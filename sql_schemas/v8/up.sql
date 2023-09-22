CREATE TABLE teacher_oauths (
    teacher uuid NOT NULL REFERENCES teachers(id) ON DELETE CASCADE,
    provider varchar(63) NOT NULL,
    sub varchar(255) NOT NULL,

    CONSTRAINT unique_sub_for_provider UNIQUE (teacher, provider)
);
