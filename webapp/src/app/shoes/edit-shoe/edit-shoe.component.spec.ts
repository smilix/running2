import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { EditShoeComponent } from './edit-shoe.component';

describe('EditShoeComponent', () => {
  let component: EditShoeComponent;
  let fixture: ComponentFixture<EditShoeComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ EditShoeComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EditShoeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
