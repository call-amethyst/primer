import { ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter, withComponentInputBinding } from '@angular/router';
import { routes } from './app.routes';
import {
  provideCalendar,
  withDateAdapter,
  withDefaults,
} from '@ascentsparksoftware/angular-calendar';
import { provideDateFnsAdapter } from '@ascentsparksoftware/angular-calendar/date-fns';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes, withComponentInputBinding()),
    provideCalendar(
      withDateAdapter(provideDateFnsAdapter()),
      withDefaults({ timezone: 'Europe/Berlin', weekStartsOn: 0 }),
    ),
  ],
};
