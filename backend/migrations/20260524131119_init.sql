-- Add migration script here
CREATE TABLE chats(
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (NOW() at time zone 'utc')
);

CREATE TABLE messages(
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT REFERENCES chats(id) ON DELETE CASCADE,
    content TEXT,
    author_name VARCHAR(100),
    sent_at TIMESTAMPTZ DEFAULT (NOW() at time zone 'utc')
);