
# Install instructions

```bash
sudo apt-get install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
sudo echo "deb http://apt.postgresql.org/pub/repos/apt/ buster-pgdg main" > /etc/apt/sources.list.d/pgdg.list
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo apt update
sudo apt install postgresql-9.6
```

# Testing

```bash
diesel migrations run
```

# TODO

* json interface
    * Rocket
        * R2D2 connection pool
