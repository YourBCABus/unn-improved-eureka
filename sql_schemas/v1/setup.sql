-- periods
CREATE TABLE public.periods (
    periodid uuid DEFAULT gen_random_uuid() NOT NULL,
    periodname character varying(255) NOT NULL
);

ALTER TABLE public.periods OWNER TO <yourusername>;

ALTER TABLE ONLY public.periods
    ADD CONSTRAINT periods_periodname_key UNIQUE (periodname);

ALTER TABLE ONLY public.periods
    ADD CONSTRAINT periods_pkey PRIMARY KEY (periodid);

GRANT ALL ON TABLE public.periods TO eureka;


-- teachers
CREATE TABLE public.teachers (
    teacherid uuid DEFAULT gen_random_uuid() NOT NULL,
    teachername character varying(255) NOT NULL,
    isabsent boolean NOT NULL,
    fullyabsent boolean NOT NULL
);


ALTER TABLE public.teachers OWNER TO <yourusername>;

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_pkey PRIMARY KEY (teacherid);

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_teachername_key UNIQUE (teachername);
    
GRANT ALL ON TABLE public.teachers TO eureka;




-- xref
CREATE TABLE public.teachers_periods_absence_xref (
    periodid uuid NOT NULL,
    teacherid uuid NOT NULL
);

ALTER TABLE public.teachers_periods_absence_xref OWNER TO <yourusername>;

ALTER TABLE ONLY public.teachers_periods_absence_xref
    ADD CONSTRAINT pkabsenceentry PRIMARY KEY (teacherid, periodid);

ALTER TABLE ONLY public.teachers_periods_absence_xref
    ADD CONSTRAINT teachers_periods_absence_xref_periodid_fkey FOREIGN KEY (periodid) REFERENCES public.periods(periodid);

ALTER TABLE ONLY public.teachers_periods_absence_xref
    ADD CONSTRAINT teachers_periods_absence_xref_teacherid_fkey FOREIGN KEY (teacherid) REFERENCES public.teachers(teacherid);

GRANT ALL ON TABLE public.teachers_periods_absence_xref TO eureka;

