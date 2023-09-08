ALTER TABLE Teachers ADD Honorific varchar(255) NOT NULL DEFAULT('<honorific>');
ALTER TABLE Teachers ADD Pronouns varchar(255) NOT NULL DEFAULT('<sub>;<obj>;<posadj>;<pospro>;<ref>;false');
ALTER TABLE Teachers ALTER COLUMN Honorific DROP DEFAULT;
ALTER TABLE Teachers ALTER COLUMN Pronouns DROP DEFAULT;
