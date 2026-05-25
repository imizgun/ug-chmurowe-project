import {Component, EventEmitter, Input, Output} from '@angular/core';

@Component({
  selector: 'app-edit-pop-up',
  imports: [],
  templateUrl: './edit-pop-up.component.html',
  styleUrl: './edit-pop-up.component.scss'
})
export class EditPopUpComponent {
  @Output() onEditing: EventEmitter<boolean> = new EventEmitter<boolean>();
  @Input() messageId: number = 0;
  @Input() isActive: boolean = false;
}
