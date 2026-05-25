import {Component, OnInit} from '@angular/core';
import {FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators} from '@angular/forms';
import {Router} from '@angular/router';
import {ChatConnectionService} from '../../services/chat-connection-service/chat-connection.service';

@Component({
  selector: 'app-chat-enter',
  imports: [
    FormsModule,
    ReactiveFormsModule
  ],
  templateUrl: './chat-enter.component.html',
  styleUrl: './chat-enter.component.scss'
})
export class ChatEnterComponent implements OnInit {
  constructor(private router: Router, private chatConnection: ChatConnectionService) {}

  ngOnInit(): void {
    if (sessionStorage.getItem('id') === null) {
      sessionStorage.clear();
      this.router.navigate(['/log_in']);
      return;
    }
  }

  chatEnterForm: FormGroup = new FormGroup({
    chatName: new FormControl('', [Validators.required])
  });

  getError(): string {
    const errors = this.chatEnterForm.controls['chatName'].errors;
    if (!errors || Object.keys(errors).length === 0) return '';
    const val = errors[Object.keys(errors)[0]];
    return val === true ? 'This field is required' : val;
  }

  onSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (this.chatEnterForm.valid) {
      this.chatConnection.joinPublicChat(
        this.chatEnterForm.value['chatName'],
        sessionStorage.getItem('name') ?? ''
      );
    }
  }
}
