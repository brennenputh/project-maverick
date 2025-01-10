FROM rust:1.81

WORKDIR /usr/src/project-maverick
COPY . .

RUN cargo install --path .

CMD ["project-maverick"]
