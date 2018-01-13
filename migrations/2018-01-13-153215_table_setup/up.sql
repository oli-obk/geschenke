CREATE TABLE users (
    name text,
    email text,
    id serial NOT NULL PRIMARY KEY
);

CREATE INDEX email ON users USING btree (email);


CREATE TABLE geschenke (
    id serial NOT NULL PRIMARY KEY,
    short_description text,
    description text,
    creator integer REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL,
    receiver integer NOT NULL REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL,
    gifter integer REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL,
    obtained_date date,
    gifted_date date
);

COMMENT ON COLUMN geschenke.short_description IS 'title or one line description';
COMMENT ON COLUMN geschenke.description IS 'long description';
COMMENT ON COLUMN geschenke.creator IS 'the creator of this gift entry. may be null if user has been deleted';
COMMENT ON COLUMN geschenke.receiver IS 'the person this gift should be gifted to';
COMMENT ON COLUMN geschenke.gifter IS 'null or whomever wants to give this present';
COMMENT ON COLUMN geschenke.obtained_date IS 'the date the present has been obtained (bought, made, ...)';
COMMENT ON COLUMN geschenke.gifted_date IS 'the date the present has been given to the recipient';
