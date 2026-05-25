import { Routes } from '@angular/router';
import {SignUpPageComponent} from './pages/sign-up-page/sign-up-page.component';
import {LayoutComponent} from './pages/layout/layout.component';
import {LogInComponent} from './pages/log-in/log-in.component';
import {NotFoundComponent} from './pages/not-found/not-found.component';
import {ChatEnterComponent} from './pages/chat-enter/chat-enter.component';
import {ChatComponent} from './pages/chat/chat.component';

export const routes: Routes = [
  {
    path: "",
    component: LayoutComponent,
    children: [
      {
        path: "",
        redirectTo: "/log_in",
        pathMatch: "full"
      },
      {
        path: "sign_up",
        component: SignUpPageComponent
      },
      {
        path: "log_in",
        component: LogInComponent
      },
      {
        path: "chat_enter",
        component: ChatEnterComponent
      },
      {
        path: "chats/:id",
        component: ChatComponent
      }
    ]
  },
  {
    path: "**",
    component: NotFoundComponent
  }
];
