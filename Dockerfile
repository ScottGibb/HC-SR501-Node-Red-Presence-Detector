# Stage 1: Build the application
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build the application
RUN cargo build --release

# Stage 2: Create a minimal runtime environment
FROM debian:buster-slim

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/app /usr/local/bin/app

# Set the startup command to run the binary
CMD ["app"]