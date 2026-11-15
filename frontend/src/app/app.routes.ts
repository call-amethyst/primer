import { Routes } from '@angular/router';
import { Homepage } from './pages/homepage/homepage';
import { Login } from './pages/auth/login/login';

export const routes: Routes = [
  {
    path: 'login',
    component: Login,
  },
  {
    path: 'home',
    component: Homepage,
  },
  {
    path: '',
    pathMatch: 'full',
    redirectTo: 'home',
  },
];
