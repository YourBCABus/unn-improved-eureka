CREATE TABLE teacher_future_schedules (
    teacher uuid NOT NULL REFERENCES teachers(id) ON DELETE CASCADE,
    date date NOT NULL,
    
    periods uuid[] NOT NULL,
    fully_absent boolean NOT NULL,
    comment text,

    CONSTRAINT unique_teacher_day_schedule UNIQUE (teacher, date)
);
