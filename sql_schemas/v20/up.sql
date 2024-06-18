ALTER TABLE clients
ADD COLUMN is_google
boolean NOT NULL DEFAULT false;

INSERT INTO clients (id, client_key, is_google) VALUES (
    '00000000-0000-0000-0000-000000000000',
    md5(random()::text),
    true
);
