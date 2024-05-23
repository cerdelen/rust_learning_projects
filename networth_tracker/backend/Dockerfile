FROM rust:latest

WORKDIR /usr/src/myapp
COPY ./backend .
COPY .env .

RUN cargo install cargo-watch
RUN cargo install --path .

# EXPOSE 3000

CMD ["cargo", "watch", "-x", "run"]