import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { EditRunComponent } from './edit-run.component';

describe('EditRunComponent', () => {
  let component: EditRunComponent;
  let fixture: ComponentFixture<EditRunComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ EditRunComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EditRunComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
