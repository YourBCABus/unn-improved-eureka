ALTER TABLE ONLY Teachers ADD firstname varchar(255) NULL;
ALTER TABLE ONLY Teachers ADD lastname  varchar(255) NULL;

UPDATE teachers SET firstname = SUBSTRING(teachername, 0, STRPOS(teachername, ' '));
UPDATE teachers SET lastname  = SUBSTRING(teachername, STRPOS(teachername, ' ') + 1);

ALTER TABLE ONLY Teachers ALTER COLUMN firstname SET NOT NULL;
ALTER TABLE ONLY Teachers ALTER COLUMN lastname  SET NOT NULL;

ALTER TABLE ONLY Teachers DROP COLUMN teachername;
ALTER TABLE ONLY Teachers
    ADD CONSTRAINT teachers_teachername_key UNIQUE (firstname, lastname);


ALTER TABLE ONLY Teachers ADD comments text NULL;

