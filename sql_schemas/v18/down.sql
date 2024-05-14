ALTER TABLE teachers DROP COLUMN school_id;
ALTER TABLE periods DROP COLUMN school_id;

ALTER TABLE config
    ADD COLUMN row_limiter boolean UNIQUE NOT NULL CHECK (row_limiter = false) DEFAULT false;
ALTER TABLE config DROP COLUMN school_id;

ALTER TABLE clients DROP COLUMN school_id;

DROP TABLE schools;

DROP FUNCTION default_school_id();
