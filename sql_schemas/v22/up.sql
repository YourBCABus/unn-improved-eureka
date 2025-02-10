START TRANSACTION;

ALTER TABLE google_emails
ADD COLUMN hosted_domains text[] NOT NULL
DEFAULT '{}'::text[];

UPDATE google_emails SET
    hosted_domains = '{"bergen.org"}'::text[],
    email_regexes = ARRAY_REMOVE(email_regexes, '^.+@bergen\.org$')
WHERE '^.+@bergen\.org$' = ANY(email_regexes);
