import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ListRunsComponent } from './list-runs.component';

describe('ListRunsComponent', () => {
  let component: ListRunsComponent;
  let fixture: ComponentFixture<ListRunsComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ListRunsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ListRunsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
