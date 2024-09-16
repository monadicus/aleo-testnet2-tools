FROM rust:1 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:12-slim
RUN apt update -y
RUN apt install -y curl
COPY --from=build-env /app/target/release/aleo-testnet2-tools /
RUN ln -s /aleo-testnet2-tools /usr/bin/tool
CMD ["./aleo-testnet2-tools"]