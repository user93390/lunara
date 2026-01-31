FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

ENV CI=true

RUN apk add --no-cache npm
RUN npm install -g bun

RUN apk add make

COPY . .

RUN make build

FROM alpine:latest

WORKDIR /app

# Linux keyring.
RUN apk add --no-cache keyutils

COPY --from=builder /app/target/release/Lunara /usr/local/bin/Lunara
COPY --from=builder /app/static /app/static

CMD ["Lunara"]