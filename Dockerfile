# Download base image ubuntu.
FROM ubuntu:18.04

# Install dependencies.
RUN apt-get update; apt-get install gcc openssl libssl-dev pkg-config curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH $PATH:/root/.cargo/bin

ADD ./ ./

# Build the app ready for production use.
RUN cargo build --release
EXPOSE 80/tcp
EXPOSE 80/udp

CMD ["/bin/bash"]
ENTRYPOINT cargo run --release