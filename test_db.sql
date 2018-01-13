--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.6
-- Dumped by pg_dump version 10.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

SET search_path = public, pg_catalog;

ALTER TABLE ONLY public.geschenke DROP CONSTRAINT receiver;
ALTER TABLE ONLY public.geschenke DROP CONSTRAINT gifter;
ALTER TABLE ONLY public.geschenke DROP CONSTRAINT creator;
DROP INDEX public.email;
ALTER TABLE ONLY public.users DROP CONSTRAINT id;
ALTER TABLE ONLY public.geschenke DROP CONSTRAINT geschenke_pkey;
ALTER TABLE public.users ALTER COLUMN id DROP DEFAULT;
ALTER TABLE public.geschenke ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE public.users_id_seq;
DROP TABLE public.users;
DROP SEQUENCE public.geschenke_id_seq;
DROP TABLE public.geschenke;
DROP EXTENSION plpgsql;
DROP SCHEMA public;
--
-- Name: public; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA public;


ALTER SCHEMA public OWNER TO postgres;

--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA public IS 'standard public schema';


--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET search_path = public, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: geschenke; Type: TABLE; Schema: public; Owner: oliver
--

CREATE TABLE geschenke (
    id integer NOT NULL,
    short_description text,
    description text,
    creator integer,
    receiver integer NOT NULL,
    gifter integer,
    obtained_date date,
    gifted_date date
);


ALTER TABLE geschenke OWNER TO oliver;

--
-- Name: COLUMN geschenke.short_description; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.short_description IS 'title or one line description';


--
-- Name: COLUMN geschenke.description; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.description IS 'long description';


--
-- Name: COLUMN geschenke.creator; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.creator IS 'the creator of this gift entry. may be null if user has been deleted';


--
-- Name: COLUMN geschenke.receiver; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.receiver IS 'the person this gift should be gifted to';


--
-- Name: COLUMN geschenke.gifter; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.gifter IS 'null or whomever wants to give this present';


--
-- Name: COLUMN geschenke.obtained_date; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.obtained_date IS 'the date the present has been obtained (bought, made, ...)';


--
-- Name: COLUMN geschenke.gifted_date; Type: COMMENT; Schema: public; Owner: oliver
--

COMMENT ON COLUMN geschenke.gifted_date IS 'the date the present has been given to the recipient';


--
-- Name: geschenke_id_seq; Type: SEQUENCE; Schema: public; Owner: oliver
--

CREATE SEQUENCE geschenke_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE geschenke_id_seq OWNER TO oliver;

--
-- Name: geschenke_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: oliver
--

ALTER SEQUENCE geschenke_id_seq OWNED BY geschenke.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: oliver
--

CREATE TABLE users (
    name text,
    email text,
    id integer NOT NULL
);


ALTER TABLE users OWNER TO oliver;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: oliver
--

CREATE SEQUENCE users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE users_id_seq OWNER TO oliver;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: oliver
--

ALTER SEQUENCE users_id_seq OWNED BY users.id;


--
-- Name: geschenke id; Type: DEFAULT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY geschenke ALTER COLUMN id SET DEFAULT nextval('geschenke_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY users ALTER COLUMN id SET DEFAULT nextval('users_id_seq'::regclass);


--
-- Data for Name: geschenke; Type: TABLE DATA; Schema: public; Owner: oliver
--

COPY geschenke (id, short_description, description, creator, receiver, gifter, obtained_date, gifted_date) FROM stdin;
1	Foo	Ein tolles Foo!	1	1	\N	\N	\N
2	Bar	Mindestens zwei Bars	1	2	\N	\N	\N
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: oliver
--

COPY users (name, email, id) FROM stdin;
oliver	oli@v.er	1
clara	cl@a.ra	2
\.


--
-- Name: geschenke_id_seq; Type: SEQUENCE SET; Schema: public; Owner: oliver
--

SELECT pg_catalog.setval('geschenke_id_seq', 2, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: oliver
--

SELECT pg_catalog.setval('users_id_seq', 2, true);


--
-- Name: geschenke geschenke_pkey; Type: CONSTRAINT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY geschenke
    ADD CONSTRAINT geschenke_pkey PRIMARY KEY (id);


--
-- Name: users id; Type: CONSTRAINT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY users
    ADD CONSTRAINT id PRIMARY KEY (id);


--
-- Name: email; Type: INDEX; Schema: public; Owner: oliver
--

CREATE INDEX email ON users USING btree (email);


--
-- Name: geschenke creator; Type: FK CONSTRAINT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY geschenke
    ADD CONSTRAINT creator FOREIGN KEY (creator) REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL;


--
-- Name: geschenke gifter; Type: FK CONSTRAINT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY geschenke
    ADD CONSTRAINT gifter FOREIGN KEY (gifter) REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL;


--
-- Name: geschenke receiver; Type: FK CONSTRAINT; Schema: public; Owner: oliver
--

ALTER TABLE ONLY geschenke
    ADD CONSTRAINT receiver FOREIGN KEY (receiver) REFERENCES users(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- Name: public; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

