import { AbstractControl, ValidationErrors, ValidatorFn } from '@angular/forms';

export function confirmPasswordValidator(func: (pass: string) => boolean): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    const value: string = control.value || '';
    return func(value)
      ? null
      : { passwordsDoNotMatch: `Passwords must be the same` };
  };
}
