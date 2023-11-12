START TRANSACTION;

CREATE TABLE privileges (
    teacher_id uuid NOT NULL PRIMARY KEY REFERENCES teachers(id) ON DELETE CASCADE,
    secretary bool NOT NULL,
    admin bool NOT NULL
);

INSERT INTO privileges (teacher_id, secretary, admin) 
    (SELECT id as teacher_id, false as secretary, false as admin FROM teachers);

COMMIT;
