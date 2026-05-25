import {Component} from '@angular/core';
import {FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators} from '@angular/forms';
import {Router} from '@angular/router';

@Component({
  selector: 'app-log-in',
  imports: [
    FormsModule,
    ReactiveFormsModule,
  ],
  templateUrl: './log-in.component.html',
  styleUrl: './log-in.component.scss'
})
export class LogInComponent {
  loginForm: FormGroup = new FormGroup({
    name: new FormControl('', [Validators.required])
  });

  constructor(private router: Router) {}

  getError(): string {
    const errors = this.loginForm.controls['name'].errors;
    if (!errors || Object.keys(errors).length === 0) return '';
    return 'This field is required';
  }

  onSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (this.loginForm.valid) {
      sessionStorage.setItem('name', this.loginForm.value['name'].trim());
      sessionStorage.setItem('id', this.loginForm.value['name'].trim());
      this.router.navigateByUrl('/chat_enter');
    }
  }
}
