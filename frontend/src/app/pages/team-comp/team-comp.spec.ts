import { ComponentFixture, TestBed } from '@angular/core/testing';
import { TeamComp } from './team-comp';

describe('TeamComp', () => {
  let component: TeamComp;
  let fixture: ComponentFixture<TeamComp>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TeamComp],
    }).compileComponents();

    fixture = TestBed.createComponent(TeamComp);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
