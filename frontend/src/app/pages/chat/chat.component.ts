import {Component, ElementRef, OnDestroy, OnInit, ViewChild} from '@angular/core';
import {ChatMessagesService} from '../../services/chat-message/chat-messages.service';
import {IMessage} from '../../services/chat-message/IMessage';
import {ChatMessageComponent} from '../../components/chat-message/chat-message.component';
import {ActivatedRoute, Router} from '@angular/router';
import {ChatConnectionService} from '../../services/chat-connection-service/chat-connection.service';
import {FormsModule} from '@angular/forms';
import {NgClass} from '@angular/common';
import {TimeFormatPipe} from './TimeFormatPipe';

@Component({
  selector: 'app-chat',
  imports: [
    ChatMessageComponent,
    FormsModule,
    NgClass,
    TimeFormatPipe
  ],
  templateUrl: './chat.component.html',
  styleUrl: './chat.component.scss'
})
export class ChatComponent implements OnInit, OnDestroy {
  chatName: string = 'Chat';
  private chatId: string = '';
  messageText: string = '';
  messages: IMessage[] = [];
  @ViewChild('scrollAnchor') private scrollAnchor!: ElementRef;

  private scrollToBottom(): void {
    this.scrollAnchor.nativeElement.scrollIntoView({ behavior: 'smooth' });
  }

  constructor(private chatMessageService: ChatMessagesService,
              private router: ActivatedRoute,
              private route: Router,
              private chatConnection: ChatConnectionService) {}

  ngOnInit(): void {
    if (sessionStorage.getItem('id') === null) {
      sessionStorage.clear();
      this.route.navigate(['/log_in']);
      return;
    }

    this.chatId = this.router.snapshot.paramMap.get('id')!;
    this.chatName = this.chatId;
    const username = sessionStorage.getItem('name') ?? '';

    console.log(`[chat]: ${username}, ${this.chatId}`);
    this.chatConnection.rejoinChat(this.chatId, username);
    this.chatMessageService.getMessages(this.chatId).subscribe();

    this.chatMessageService.messagesObs$.subscribe(msgs => {
      this.messages = [...msgs];
      setTimeout(() => this.scrollToBottom(), 0);
    });
  }

  sendMessage() {
    this.messageText = this.messageText.trim();
    if (this.messageText !== '') {
      const username = sessionStorage.getItem('name') ?? '';
      this.chatConnection.sendMessage(this.messageText, username);
      this.messageText = '';
    }
  }

  ngOnDestroy(): void {
    this.chatConnection.closeConnection();
  }

  protected readonly sessionStorage = sessionStorage;
}
