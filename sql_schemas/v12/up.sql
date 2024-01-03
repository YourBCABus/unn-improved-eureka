CREATE TABLE config (
    sheet_id varchar(255) NOT NULL,
    row_limiter boolean UNIQUE NOT NULL CHECK (row_limiter = false) DEFAULT false
);

INSERT INTO config (sheet_id) VALUES ('');
