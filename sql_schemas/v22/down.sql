START TRANSACTION;

UPDATE google_emails SET
    email_regexes = email_regexes || '^.+@bergen\.org$'::text
WHERE 'bergen.org' = ANY(hosted_domains);

ALTER TABLE google_emails DROP COLUMN hosted_domains;