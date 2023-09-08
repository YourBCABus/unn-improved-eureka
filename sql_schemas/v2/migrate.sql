ALTER TABLE Periods ADD UtcStartDefault Time NOT NULL DEFAULT('00:00:00');
ALTER TABLE Periods ADD UtcEndDefault Time NOT NULL DEFAULT('00:00:00');

ALTER TABLE Periods ADD UtcStartCurrent Time NULL;
ALTER TABLE Periods ADD UtcEndCurrent Time NULL;
