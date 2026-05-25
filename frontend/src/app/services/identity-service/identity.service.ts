import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {ISignUpUser} from './ISignUpUser';
import {ISimpleAnswer} from './ISimpleAnswer';
import {Observable} from 'rxjs';
import {ILoginUser} from './ILoginUser';
import {IUser} from './IUser';

@Injectable({
  providedIn: 'root'
})
export class IdentityService {

  constructor(private http: HttpClient) { }

  signUpUser(data: ISignUpUser) : Observable<ISimpleAnswer> {
    return this.http.post<ISimpleAnswer>("/api/users", data);
  }

  logInUser(data: ILoginUser) : Observable<IUser> {
    return this.http.post<IUser>("/api/users/auth", data);
  }
}
