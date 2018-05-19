FROM liuchong/rustup

RUN rustup default nightly-2018-04-19

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080

RUN rustup target install x86_64-unknown-linux-musl

RUN apt update && apt install -y --no-install-recommends libpq-dev

ADD . /app

WORKDIR /app

RUN cargo build 

CMD ["cargo", "run"]