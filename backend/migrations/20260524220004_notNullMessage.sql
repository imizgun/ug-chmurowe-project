-- Add migration script here
ALTER TABLE messages ALTER COLUMN chat_id SET NOT NULL;