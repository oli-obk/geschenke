
# Install instructions

## rls

Not necessary if you use the rls extension from vscode

```bash
rustup override set nightly-2018-01-20
rustup component add rls-preview
rustup component add rust-src
rustup component add rust-analysis
```

## postgres

```bash
sudo apt-get install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
sudo echo "deb http://apt.postgresql.org/pub/repos/apt/ buster-pgdg main" > /etc/apt/sources.list.d/pgdg.list
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo apt update
sudo apt install postgresql-9.6

sudo su - postgres
psql
CREATE USER username WITH PASSWORD 'mypw' SUPERUSER;
CREATE DATABASE geschenke;
\q
exit
```

## Frontend

```bash
cargo install cargo-web
rustup target add asmjs-unknown-emscripten
```

# Testing

## Backend

getting the database setup

```bash
diesel migration run
```

## Frontend

running

```bash
cargo web start
```

# Useful things

* Json Viewer for firefox: https://addons.mozilla.org/en-US/firefox/addon/json-lite/

# TODO

* json interface
    * Rocket
        * R2D2 connection pool
* tests
    * *tests*
        * **tests**
            * **TESTS!!!!!111einself**
