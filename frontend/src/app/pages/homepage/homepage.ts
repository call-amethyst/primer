import { Component } from '@angular/core';
import { Card } from '../../components/card/card';
import { NgIcon, provideIcons } from '@ng-icons/core';
import { gameCalendar } from '@ng-icons/game-icons';
import { gameTeamUpgrade } from '@ng-icons/game-icons';
import { gameArcheryTarget } from '@ng-icons/game-icons';

@Component({
  imports: [Card, NgIcon],
  providers: [provideIcons({ gameCalendar, gameTeamUpgrade, gameArcheryTarget })],
  selector: 'app-homepage',
  templateUrl: './homepage.html',
  host: {
    class: 'flex flex-col gap-10',
  },
})
export class Homepage {}
