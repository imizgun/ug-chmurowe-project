import {Component, Input} from '@angular/core';
import {NgClass, NgStyle} from '@angular/common';

@Component({
  selector: 'app-chat-message',
  imports: [
    NgStyle,
    NgClass,
  ],
  templateUrl: './chat-message.component.html',
  styleUrl: './chat-message.component.scss'
})
export class ChatMessageComponent {
  @Input() content: string = '';
  @Input() id: number = 0;
  @Input() author: string = '';
  @Input() isLeft: boolean = false;
  @Input() sentAt: string = '';
  @Input() chatId: number = 0;
}
