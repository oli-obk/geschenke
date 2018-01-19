
# Install instructions

```bash
rustup override set nightly-2018-01-13
rustup component add rls-preview
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

# Testing

```bash
diesel migration run
```

# TODO

* json interface
    * Rocket
        * R2D2 connection pool
