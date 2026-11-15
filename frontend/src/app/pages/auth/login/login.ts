import { booleanAttribute, Component, input, signal } from '@angular/core';
import { Card } from '../../../components/card/card';
import { form, FormField, required, SchemaPath, validate } from '@angular/forms/signals';
import passwordMatchValidator from '../../../validation/rules/password-match-validator';
import { RouterLink } from '@angular/router';

@Component({
  imports: [Card, FormField, RouterLink],
  selector: 'app-login',
  templateUrl: './login.html',
})
export class Login {
  register = input<boolean>(undefined, { transform: booleanAttribute });

  loginModel = signal<LoginForm>({
    username: '',
    password: '',
  });

  loginForm = form(this.loginModel);

  registerModel = signal<RegisterForm>({
    username: '',
    password: '',
    confirmPassword: '',
  });

  registerForm = form(this.registerModel, (schemaPath) => {
    required(schemaPath.username, { message: 'Username is required' });
    required(schemaPath.password, { message: 'Password is required' });
    required(schemaPath.confirmPassword, { message: 'Confirm Password is required' });
    passwordMatchValidator(schemaPath.password, schemaPath.confirmPassword, {
      message: 'Passwords do not match',
    });
    passwordMatchValidator(schemaPath.confirmPassword, schemaPath.password, {
      message: 'Passwords do not match',
    });
  });

  submitLogin() {}

  submitRegister() {
    
  }
}

interface LoginForm {
  username: string;
  password: string;
}

interface RegisterForm {
  username: string;
  password: string;
  confirmPassword: string;
}
