START TRANSACTION;

DROP TABLE absence_xref;
DROP TABLE periods;

ALTER TABLE teachers DROP COLUMN fully_absent;

COMMIT;
