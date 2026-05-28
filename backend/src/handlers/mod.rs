pub mod error;

use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::extract::ws::{WebSocket};
use axum::extract::ws;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use crate::app_state::AppState;
use crate::contracts::connect_to_chat_query::ConnectToChatQuery;
use crate::contracts::create_chat_request::CreateChatRequest;
use crate::contracts::create_chat_response::CreateChatResponse;
use crate::contracts::income_message::IncomeMessage;
use crate::core::message::Message;
use crate::db::db::{chat_exists_by_name, create_chat_inner, create_message, get_chat_by_name, get_chat_messages};
use crate::handlers::error::AppError;

#[axum::debug_handler]
pub async fn create_chat(
    app_state: State<AppState>,
    body: Json<CreateChatRequest>,
) -> Result<(StatusCode, Json<CreateChatResponse>), AppError> {
    let new_chat = create_chat_inner(&app_state.db_pool, body.0).await?;

    Ok((StatusCode::CREATED, Json(CreateChatResponse {
        id: new_chat.id,
    })))
}

pub async fn get_messages(
    State(app_state): State<AppState>,
    Path(chat_name): Path<String>,
) -> Result<Json<Vec<Message>>, AppError> {
    let chat = get_chat_by_name(&app_state.db_pool, &chat_name).await?;
    let messages = get_chat_messages(&app_state.db_pool, chat.id).await?;
    Ok(Json(messages))
}

pub async fn connect_to_chat(
    web_socket_upgrade: WebSocketUpgrade,
    State(app_state): State<AppState>,
    Query(params): Query<ConnectToChatQuery>) -> impl IntoResponse {
    web_socket_upgrade.on_upgrade(
            |websocket|
                handle_socket(websocket, app_state, params))
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    params: ConnectToChatQuery) {

    if let Err(e) = run_socket(socket, state, params).await {
        tracing::error!("websocket session ended with error: {e}");
    }
}

async fn run_socket(
    socket: WebSocket,
    state: AppState,
    params: ConnectToChatQuery,
) -> anyhow::Result<()> {
    if !chat_exists_by_name(&params.chat_name, &state.db_pool).await? {
        create_chat_inner(&state.db_pool, CreateChatRequest { title: params.chat_name.clone() }).await?;
    }

    let sender = {
        let mut lock = state.rooms.lock().unwrap_or_else(|e| e.into_inner());
        lock.entry(params.chat_name.clone())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    let mut receiver = sender.subscribe();
    let mut redis_conn = state.redis.clone();

    let (mut ws_sender, mut ws_receiver) = socket.split();

    let chat = get_chat_by_name(&state.db_pool, &params.chat_name).await?;

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    None => {
                        tracing::info!("{} disconnected", params.username);
                        break;
                    }
                    Some(result) => {
                        let message = result?;
                        match message {
                            ws::Message::Text(text) => {
                                let mess = serde_json::from_str::<IncomeMessage>(&text)?;
                                let saved = create_message(&state.db_pool, chat.id, &mess).await?;
                                sender.send(saved.clone())?;
                                let _: redis::Value = redis::cmd("PUBLISH")
                                    .arg(format!("chat:{}", chat.id))
                                    .arg(serde_json::to_string(&saved)?)
                                    .query_async(&mut redis_conn)
                                    .await?;
                                tracing::info!("message '{}' from {} sent to chat {}", mess.content, params.username, chat.id);
                            }
                            ws::Message::Close(_) => {
                                tracing::info!("{} disconnected", params.username);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            msg = receiver.recv() => {
                match msg {
                    Ok(message) => {
                        let json = serde_json::to_string(&message)?;
                        ws_sender.send(ws::Message::Text(json.into())).await?;
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("{} lagged, missed {} messages", params.username, n);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        tracing::info!("broadcast channel for '{}' closed", params.chat_name);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}