CREATE TABLE pronoun_sets (
    id uuid NOT NULL PRIMARY KEY,
    sub varchar(63) NOT NULL,
    obj varchar(63) NOT NULL,
    pos_adj varchar(63) NOT NULL,
    pos_pro varchar(63) NOT NULL,
    refx varchar(63) NOT NULL,
    gramm_plu boolean NOT NULL,
    CONSTRAINT nopronounsetduplicates UNIQUE (sub, obj, pos_adj, pos_pro, refx, gramm_plu)
);

CREATE TABLE teachers (
    id uuid NOT NULL PRIMARY KEY,
    pronouns uuid NOT NULL,
    CONSTRAINT teachers_pronouns_fkey FOREIGN KEY (pronouns) REFERENCES pronoun_sets(id)
);

CREATE TABLE names (
    name_of uuid NOT NULL,
    honorific varchar(255) NOT NULL,
    first text NOT NULL,
    last text NOT NULL,
    middle_texts text[] NOT NULL,
    middle_display boolean[] NOT NULL,
    CONSTRAINT names_name_of_fkey FOREIGN KEY (name_of) REFERENCES teachers(id)
);
