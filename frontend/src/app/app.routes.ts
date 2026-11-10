import { Routes } from '@angular/router';
import { Homepage } from './pages/homepage/homepage';

export const routes: Routes = [
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
