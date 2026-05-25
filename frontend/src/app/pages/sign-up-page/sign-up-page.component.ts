import {Component, OnInit} from '@angular/core';
import {FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators} from '@angular/forms';
import {confirmPasswordValidator} from './repeat-password-validator';
import {RouterLink} from '@angular/router';
import {IdentityService} from '../../services/identity-service/identity.service';
import {NgIf, NgStyle} from '@angular/common';

@Component({
  selector: 'app-sign-up-page',
  imports: [
    FormsModule,
    ReactiveFormsModule,
    RouterLink,
    NgIf,
    NgStyle
  ],
  templateUrl: './sign-up-page.component.html',
  styleUrl: './sign-up-page.component.scss'
})
export class SignUpPageComponent implements OnInit {

  constructor(private identityService: IdentityService) { }

  ngOnInit(): void {
    this.signUpForm.get("repeatPassword")?.setValidators(
      confirmPasswordValidator(confPass => confPass === this.signUpForm.get('password')?.value)
    )
  }

  signUpForm: FormGroup = new FormGroup({
      email: new FormControl('', [Validators.required, Validators.email]),
      name: new FormControl('', [Validators.required]),
      password: new FormControl('', [Validators.required]),
      repeatPassword: new FormControl('', [Validators.required]),
    }
  );

  responseObj = {
    responseText: "",
    isError: false,
  };
  isResponse = false;

  getErrorByKey(key: string): string {
    if (this.signUpForm.controls[key].errors === null || Object.keys(this.signUpForm.controls[key].errors ?? {}).length === 0) {
      return "";
    }
    else {
      let error = this.signUpForm.controls[key].errors[Object.keys(this.signUpForm.controls[key].errors ?? {})[0]];
      return error === true ? "This field is required" : error;
    }
  }

  onSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (this.signUpForm.valid) {
      console.log(this.signUpForm.value);

      this.identityService.signUpUser({

        email: this.signUpForm.get('email')?.value ?? "",
        password: this.signUpForm.get('password')?.value ?? "",
        name: this.signUpForm.get('name')?.value ?? ""

      }).subscribe({
        next: r => {
          this.responseObj.responseText = r.message;
          this.responseObj.isError = false;
          this.isResponse = true;
        },
        error: e => {
          this.responseObj.responseText = e.error.message;
          this.responseObj.isError = true;
          this.isResponse = true;
        }
      });

    }
  }

  protected readonly Object = Object;
}
