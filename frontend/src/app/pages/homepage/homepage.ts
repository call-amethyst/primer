import { Component } from '@angular/core';
import { Card } from '../../components/card/card';
import { CalMonthView } from '@ascentsparksoftware/angular-calendar';

@Component({
  imports: [Card, CalMonthView],
  selector: 'app-homepage',
  styleUrl: './homepage.scss',
  templateUrl: './homepage.html',
  host: {
    class: 'flex flex-col gap-10',
  },
})
export class Homepage {
  viewDate = new Date();
}
