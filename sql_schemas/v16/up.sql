ALTER TABLE config
ADD attribs jsonb NOT NULL DEFAULT '{}'::jsonb;
