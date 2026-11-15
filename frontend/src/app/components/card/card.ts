import { Component, input } from '@angular/core';

@Component({
  imports: [],
  selector: 'app-card',
  styleUrl: './card.scss',
  templateUrl: './card.html',
  host: {
    class:
      'relative px-4 pt-4 pb-12 block rounded-lg border border-aero-glass-border/80 bg-aero-glass/70 backdrop-blur-md drop-shadow-md',
  },
})
export class Card {
  title = input<string>();
}
