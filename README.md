
# Install instructions

## rls

Not necessary if you use the rls extension from vscode

```bash
rustup override set nightly-2018-04-19
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

## https

```bash
apt install certbot -t stretch-backports
# make sure no http server is running
certbot certonly
# choose "standalone"
# fill out rest of stuff
```

Add

```
[global.tls]
certs = "/path/to/certs.pem"
key = "/path/to/key.pem"
```

to `Rocket.toml`

## cookie secret key

generate a new one with

```bash
openssl rand -base64 32
```

and add it to `Rocket.toml` where appropriate

# Testing

## Backend

getting the database setup

```bash
diesel migration run
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
