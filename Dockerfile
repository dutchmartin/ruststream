# Download base image ubuntu.
FROM ubuntu:18.04

# Install dependencies.
RUN apt-get update; apt-get install gcc openssl libssl-dev pkg-config curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY ./ ./

# Build the app ready for production use.
RUN $HOME/.cargo/bin/cargo build --release

ENTRYPOINT ["cargo run"]