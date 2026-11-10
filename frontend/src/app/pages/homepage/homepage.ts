import { Component } from '@angular/core';
import { Card } from '../../components/card/card';

@Component({
  imports: [Card],
  selector: 'app-homepage',
  styleUrl: './homepage.scss',
  templateUrl: './homepage.html',
  host: {
    class: 'flex flex-col gap-10',
  },
})
export class Homepage {}
