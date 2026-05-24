use axum::extract::{Query, State, WebSocketUpgrade};
use axum::extract::ws::{WebSocket};
use axum::extract::ws;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use sqlx::PgPool;
use tokio::sync::broadcast;
use crate::app_state::AppState;
use crate::contracts::connect_to_chat_query::ConnectToChatQuery;
use crate::contracts::create_chat_request::CreateChatRequest;
use crate::contracts::create_chat_response::CreateChatResponse;
use crate::contracts::income_message::IncomeMessage;
use crate::core::chat::Chat;
use crate::core::message::Message;

#[axum::debug_handler]
pub async fn create_chat(
    app_state: State<AppState>,
    body: Json<CreateChatRequest>) -> (StatusCode, Json<CreateChatResponse>) {

    let new_chat = create_chat_inner(&app_state.db_pool, body.0).await;

    (StatusCode::CREATED, Json(CreateChatResponse {
        id: new_chat.id,
    }))
}

async fn create_chat_inner(
    pool: &PgPool,
    body: CreateChatRequest
) -> Chat {
    let new_chat = sqlx::query_as!(
        Chat,
        "insert into chats (title) values ($1) returning *",
        body.title)
        .fetch_one(pool)
        .await
        .unwrap();

    new_chat
}

async fn create_message(pool: &PgPool, chat_id: i64, body: &IncomeMessage) -> Message {
    let new_message = sqlx::query_as!(
        Message,
        "insert into messages (chat_id, content, author_name)
         values ($1, $2, $3) returning *", chat_id, body.content, body.author_name)
        .fetch_one(pool)
        .await
        .unwrap();

    new_message
}

async fn chat_exists_by_name(name: &str, pool: &PgPool) -> bool {
    sqlx::query!("select exists(select 1 from chats where title = $1)", name)
        .fetch_one(pool)
        .await
        .unwrap().exists.unwrap()
}

async fn get_chat_by_name(pool: &PgPool, chat_name: &str) -> Chat {
    sqlx::query_as!(Chat, "select * from chats where title = $1", chat_name)
        .fetch_one(pool)
        .await
        .unwrap()
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

    if !chat_exists_by_name(&params.chat_name, &state.db_pool).await {
        create_chat_inner(&state.db_pool, CreateChatRequest {title: params.chat_name.clone()}).await;
    }

    let sender = {
        let mut lock = state.rooms.lock().unwrap();
        lock.entry(params.chat_name.clone())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    let mut receiver = sender.subscribe();

    let (mut ws_sender,
        mut ws_receiver) = socket.split();

    let chat = get_chat_by_name(&state.db_pool, &params.chat_name).await;

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    None => {
                        tracing::info!("client disconnected");
                        break
                    },
                    Some(result) => {
                        let message = match result {
                            Ok(message) => message,
                            Err(_e) => break,
                        };
                        let text = message.to_text().unwrap().to_string();

                        let mess = serde_json::from_str::<IncomeMessage>(&text).unwrap();

                        sender.send(create_message(&state.db_pool, chat.id, &mess).await)
                        .expect("Error when sending messange");

                        tracing::info!("message '{}' from {} sent to chat {}", &mess.content.clone(),  params.username, chat.id);
                    }
                }
            }
             msg = receiver.recv() => {
                  if let Ok(message) = msg {
                      let json = serde_json::to_string(&message).unwrap();
                      ws_sender.send(ws::Message::Text
                        (json.into()))
                    .await.unwrap();
                  }
            }
        }
    }
}
