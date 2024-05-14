CREATE TABLE schools (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    is_default boolean NOT NULL DEFAULT false
);

INSERT INTO schools (id, name, is_default) VALUES (gen_random_uuid(), 'Bergen County Academies', true);

CREATE FUNCTION default_school_id() RETURNS uuid AS $$
    SELECT id FROM schools WHERE is_default = true;
$$ LANGUAGE SQL;

ALTER TABLE teachers
    ADD COLUMN school_id uuid NOT NULL REFERENCES schools(id) DEFAULT default_school_id();
ALTER TABLE periods
    ADD COLUMN school_id uuid NOT NULL REFERENCES schools(id) DEFAULT default_school_id();

ALTER TABLE periods DROP CONSTRAINT periods_name_key;
ALTER TABLE periods ADD CONSTRAINT periods_name_key UNIQUE (name, school_id);

ALTER TABLE config
    DROP COLUMN row_limiter;
ALTER TABLE config
    ADD COLUMN school_id uuid NOT NULL REFERENCES schools(id) DEFAULT default_school_id();

-- Null means all schools (Admin Clients)
ALTER TABLE clients
    ADD COLUMN school_id uuid REFERENCES schools(id);

ALTER TABLE teachers ALTER COLUMN school_id DROP DEFAULT;
ALTER TABLE periods ALTER COLUMN school_id DROP DEFAULT;
ALTER TABLE config ALTER COLUMN school_id DROP DEFAULT;
