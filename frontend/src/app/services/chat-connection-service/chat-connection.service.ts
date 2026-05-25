import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { ChatMessagesService } from '../chat-message/chat-messages.service';

@Injectable({
  providedIn: 'root'
})
export class ChatConnectionService {
  private ws: WebSocket | null = null;
  private currentChatName = '';
  private currentUsername = '';
  private closed = false;
  private reconnectAttempt = 0;
  private readonly reconnectDelays = [0, 1000, 2000, 5000];
  private readonly WS_BASE = `${window.location.protocol === 'https:' ? 'wss' : 'ws'}://${window.location.host}/api/chats/connect`;

  constructor(private router: Router, private chatMessages: ChatMessagesService) {}

  joinPublicChat(chatName: string, username: string) {
    this.openChat(chatName, username);
    this.router.navigate(['/chats', chatName]);
  }

rejoinChat(chatName: string, username: string): void {
    if (!this.ws || this.ws.readyState === WebSocket.CLOSED || this.ws.readyState === WebSocket.CLOSING) {
      this.openChat(chatName, username);
    }
  }

  sendMessage(content: string, authorName: string): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ content, author_name: authorName }));
    }
  }

  closeConnection(): void {
    this.closed = true;
    this.ws?.close();
    this.ws = null;
  }

  private openChat(chatName: string, username: string): void {
    this.currentChatName = chatName;
    this.currentUsername = username;
    this.closed = false;
    this.reconnectAttempt = 0;
    this.chatMessages.clearMessages();
    this.connect();
  }

  private connect(): void {
    if (this.ws) {
      this.ws.onclose = null;
      this.ws.close();
    }
    const url = `${this.WS_BASE}?username=${encodeURIComponent(this.currentUsername)}&chat_name=${encodeURIComponent(this.currentChatName)}`;
    this.ws = new WebSocket(url);

    this.ws.onopen = () => {
      console.log('[WS]: Connected to', this.currentChatName);
      this.reconnectAttempt = 0;
    };

    this.ws.onmessage = (event: MessageEvent) => {
      const msg = JSON.parse(event.data);
      this.chatMessages.addMessage(msg);
    };

    this.ws.onclose = () => {
      if (this.closed) return;
      console.log('[WS]: Disconnected, reconnecting...');
      this.scheduleReconnect();
    };

    this.ws.onerror = (error) => {
      console.error('[WS]: Error', error);
    };
  }

  private scheduleReconnect(): void {
    const delay = this.reconnectDelays[Math.min(this.reconnectAttempt, this.reconnectDelays.length - 1)];
    this.reconnectAttempt++;
    setTimeout(() => {
      if (!this.closed) this.connect();
    }, delay);
  }
}
