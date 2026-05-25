import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ChatEnterComponent } from './chat-enter.component';

describe('ChatEnterComponent', () => {
  let component: ChatEnterComponent;
  let fixture: ComponentFixture<ChatEnterComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ChatEnterComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ChatEnterComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
