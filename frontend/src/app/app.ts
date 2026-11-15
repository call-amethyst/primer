import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],

  templateUrl: './app.html',
  styleUrl: './app.scss',
  host: {
    class: 'flex flex-col items-center',
  },
})
export class App {
  message = signal('');
  loading = signal(false);

  async sayHello() {
    this.loading.set(true);
    this.message.set('');

    try {
      const response = await fetch('/api/hello');

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const text = await response.text();
      this.message.set(text);
    } catch (error) {
      console.error(error);
      this.message.set('Could not connect to backend!');
    } finally {
      this.loading.set(false);
    }
  }
}
