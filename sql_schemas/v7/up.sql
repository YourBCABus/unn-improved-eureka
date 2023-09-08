ALTER TABLE names
ADD CONSTRAINT no_name_duplicates
UNIQUE (honorific, first, last, middle_texts, middle_display);
