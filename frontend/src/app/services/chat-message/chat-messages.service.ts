import { Injectable } from '@angular/core';
import { IMessage } from './IMessage';
import { BehaviorSubject, tap } from 'rxjs';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class ChatMessagesService {
  private messages: IMessage[] = [];
  messagesObs$ = new BehaviorSubject<IMessage[]>(this.messages);

  constructor(private http: HttpClient) {}

  getMessages(chatName: string) {
    return this.http.get<IMessage[]>(`http://localhost:3000/api/chats/${encodeURIComponent(chatName)}/messages`)
      .pipe(
        tap(msgs => {
          this.messages = msgs;
          this.messagesObs$.next(this.messages);
        })
      );
  }

  addMessage(message: IMessage): void {
    this.messages = [...this.messages, message];
    this.messagesObs$.next(this.messages);
  }

  clearMessages(): void {
    this.messages = [];
    this.messagesObs$.next(this.messages);
  }
}
