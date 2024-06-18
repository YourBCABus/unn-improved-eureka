DELETE FROM clients WHERE is_google;
ALTER TABLE clients DROP COLUMN is_google;
