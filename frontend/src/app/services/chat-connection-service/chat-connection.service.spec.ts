import { TestBed } from '@angular/core/testing';

import { ChatConnectionService } from './chat-connection.service';

describe('ChatConnectionService', () => {
  let service: ChatConnectionService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ChatConnectionService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
