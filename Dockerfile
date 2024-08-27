# -- Build image, based on the official Rust image --
FROM rust:latest as builder
LABEL authors="daniel,kristian"

# Set the workdir
WORKDIR /build

# Copy all files in the current directory to the workdir of the image
COPY . .

# Compile the Rust application
RUN cargo build --release

# -- Runtime image, based on Debian --
FROM debian:latest

# Install necessary packages
RUN apt-get update
RUN apt-get install sqlite3 -y

COPY --from=builder /build/target/release/ventilator /app/ventilator

WORKDIR /app

# Copy non-binary dependencies
COPY static static
COPY templates templates
COPY init_db.sql init_db.sql
COPY Rocket.toml Rocket.toml

RUN sqlite3 /app/ventilator_db.sqlite < init_db.sql

ENV DATABASE_URL=/app/ventilator_db.sqlite

# Since this will be run in production in the Docker image, listen on all interfaces.
ENV ROCKET_ADDRESS=0.0.0.0

# Expose the port (used mainly for documentation purposes)
EXPOSE 8000


ENTRYPOINT ["./ventilator"]