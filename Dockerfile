# Stage 1: Build the application
FROM rust:bookworm AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    openssl \
    libssl-dev \
    libftdi1 \
    libftdi1-dev && \
    rm -rf /var/lib/apt/lists/*
COPY . .
# Build the application in release mode
RUN cargo build --release --no-default-features --features=prod && cargo build --no-default-features --features dev


# Stage 2: Create the runtime image
FROM debian:bookworm-slim AS prod
# Required for static linking
RUN apt-get update && apt-get install -y \ 
    openssl \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*
# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/presence-detector /usr/local/bin/presence-detector
# Set the startup command to run the binary
CMD ["presence-detector"]

# Stage 3: Create the development image
FROM debian:bookworm-slim AS dev
# Required for static linking
RUN apt-get update && apt-get install -y \
    openssl \
    libssl-dev \
    libftdi1 \
    libftdi1-dev &&\
    rm -rf /var/lib/apt/lists/*
# Copy the build artifact from the builder stage
COPY --from=builder /app/target/debug/presence-detector /usr/local/bin/presence-detector
# Set the startup command to run the binary
CMD ["presence-detector"]
