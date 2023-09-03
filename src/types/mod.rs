mod absence;
mod period;
mod teacher;

pub use teacher::{ Teacher, TeacherName, Honorific, pronouns::PronounSet };

/*
                                        Table "public.periods"
     Column      |          Type          | Collation | Nullable |              Default               
-----------------+------------------------+-----------+----------+------------------------------------
 periodid        | uuid                   |           | not null | gen_random_uuid()
 periodname      | character varying(255) |           | not null | 
 utcstartdefault | time without time zone |           | not null | '00:00:00'::time without time zone
 utcenddefault   | time without time zone |           | not null | '00:00:00'::time without time zone
 utcstartcurrent | time without time zone |           |          | 
 utcendcurrent   | time without time zone |           |          | 
Indexes:
    "periods_pkey" PRIMARY KEY, btree (periodid)
    "periods_periodname_key" UNIQUE CONSTRAINT, btree (periodname)
Referenced by:
    TABLE "teachers_periods_absence_xref" CONSTRAINT "teachers_periods_absence_xref_periodid_fkey" FOREIGN KEY (periodid) REFERENCES periods(periodid)


CREATE TABLE pronoun_sets (
    id uuid NOT NULL PRIMARY KEY,

    sub varchar(63) NOT NULL,
    obj varchar(63) NOT NULL,
    pos_adj varchar(63) NOT NULL,
    pos_pro varchar(63) NOT NULL,
    refx varchar(63) NOT NULL,

    gramm_plu bool NOT NULL,

    CONSTRAINT NoPronounSetDuplicates UNIQUE (sub, obj, pos_adj, pos_pro, refx, gramm_plu)
);

CREATE TABLE teachers (
    id uuid NOT NULL PRIMARY KEY,
    pronouns uuid NOT NULL REFERENCES pronoun_sets(id)
);

CREATE TABLE names (
    name_of uuid NOT NULL REFERENCES teachers(id),

    honorary varchar(255) NOT NULL,
    first text NOT NULL,
    last text NOT NULL,
    middle_texts text[] NOT NULL,
    middle_display bool[] NOT NULL
);

 */