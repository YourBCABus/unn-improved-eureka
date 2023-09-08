START TRANSACTION;


CREATE TABLE periods (
    id uuid NOT NULL PRIMARY KEY,
    
    name text NOT NULL UNIQUE,

    start_time TIME NOT NULL,
    end_time TIME NOT NULL,

    temp_start TIME,
    temp_end TIME
);

CREATE TABLE absence_xref (
    id uuid NOT NULL PRIMARY KEY,
    
    period_id uuid NOT NULL REFERENCES periods(id) ON DELETE CASCADE,
    teacher_id uuid NOT NULL REFERENCES teachers(id) ON DELETE CASCADE
);

ALTER TABLE teachers ADD COLUMN fully_absent boolean NOT NULL DEFAULT false;


COMMIT;
