
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
```

### old

```bash
sudo echo "deb http://apt.postgresql.org/pub/repos/apt/ buster-pgdg main" > /etc/apt/sources.list.d/pgdg.list
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo apt update
sudo apt install postgresql-9.6

sudo su - postgres
psql
CREATE USER geschenke WITH PASSWORD 'geschenke' SUPERUSER;
CREATE DATABASE geschenke;
\q
exit
```

Show all databases: `\l`
Show all tables in the current database: `\dt`
Connect to a database: `\c DATABASENAME`
Delete a database (connect to a different database first!): `DROP DATABASE databasename;`
Show current user: `select current_user;`
Change role: `set role USERNAME;`

### docker

```bash
apt-get install apt-transport-https ca-certificates curl gnupg2 software-properties-common
curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
apt-key fingerprint 0EBFCD88
vi /etc/apt/sources.list # add next line
deb [arch=amd64] https://download.docker.com/linux/debian stretch stable
apt-get update
apt-get install docker-ce
usermod -G docker USER
docker run --name geschenke-postgres -e POSTGRES_PASSWORD=geschenke -e POSTGRES_USER=geschenke -p=5432:5432 -d postgres && docker ps
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
