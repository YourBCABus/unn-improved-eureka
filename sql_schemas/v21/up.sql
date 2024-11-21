START TRANSACTION;

CREATE TABLE google_emails (
    school_id uuid NOT NULL REFERENCES schools(id),
    email_regexes text[] NOT NULL
);

INSERT INTO google_emails (school_id, email_regexes) VALUES (
    (SELECT school_id FROM clients WHERE is_google),
    '{"^.+@bergen\.org$"}'::text[]
);
