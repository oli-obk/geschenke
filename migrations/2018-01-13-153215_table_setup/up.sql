CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR(88),
    salt VARCHAR(20),
    autologin VARCHAR(100) NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);

CREATE INDEX email ON users USING btree (email);

COMMENT ON COLUMN users.password IS 'base64 of the sha3-512 hash';
COMMENT ON COLUMN users.autologin IS 'randomly generated "password" for logging in via links';

CREATE TABLE geschenke (
    id SERIAL NOT NULL PRIMARY KEY,
    short_description VARCHAR NOT NULL,
    description TEXT,
    creator INTEGER REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL,
    receiver INTEGER NOT NULL REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL,
    gifter INTEGER REFERENCES users(id) ON UPDATE RESTRICT ON DELETE SET NULL,
    obtained_date timestamp,
    gifted_date timestamp
);

COMMENT ON COLUMN geschenke.short_description IS 'title or one line description';
COMMENT ON COLUMN geschenke.description IS 'long description';
COMMENT ON COLUMN geschenke.creator IS 'the creator of this gift entry. may be null if user has been deleted';
COMMENT ON COLUMN geschenke.receiver IS 'the person this gift should be gifted to';
COMMENT ON COLUMN geschenke.gifter IS 'null or whomever wants to give this present';
COMMENT ON COLUMN geschenke.obtained_date IS 'the date the present has been obtained (bought, made, ...)';
COMMENT ON COLUMN geschenke.gifted_date IS 'the date the present has been given to the recipient';

insert into users (name, email, autologin) values ('oliver', 'oli@v.er', 'bar');
insert into users (name, email, autologin) values ('clara', 'cla@r.a', 'foo');

insert into geschenke (short_description, description, creator, receiver) values ('Foo', 'Ein tolles Foo', 1, 1);
insert into geschenke (short_description, description, creator, receiver) values ('Bar', 'Viele tolle Bars', 1, 2);
