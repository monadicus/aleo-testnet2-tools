## Aleo Testnet2 Wasm Suite

A few simple functions for interacting with Aleo testnet2 addresses and signatures.

### Run Website with Docker

1. `docker build . -t wasm-suite`
1. `docker run --rm -p 8080:80 wasm-suite` (change `8080` to whatever port you want)
1. Open http://localhost:8080/ in your browser (change `8080` to the port from the previous step)

### Build Manually

1. Install rust
1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
1. `wasm-pack build --release --target web`
