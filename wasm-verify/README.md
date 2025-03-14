## Aleo Testnet2 Wasm Verify

This is a simple wasm library based on the utilities provided by [aleo-testnet2-tools](https://github.com/monadicus/aleo-testnet2-tools). The output is built for Nodejs, not web.

This package contains a few simple functions for verifying aleo Aleo addresses and signatures:

|Signature|Returns|Description|
|-|-|-|
|`testnet2_verify(address, message, signature)`|`boolean`|Returns true for valid testnet2 signatures. Throws errors for parsing errors|
|`testnet2_verify_addr(address)`|`boolean`|Returns true for valid testnet2 addresses.|
|`mainnet_verify(address, message, signature)`|`boolean`|Returns true for valid mainnet signatures. Throws errors for parsing errors.|
|`mainnet_verify_addr(address)`|`boolean`|Returns true for valid testnet2 addresses.|
|`signature(address, message, signature)`|`number`|Returns 0 for invalid, 1 for valid mainnet, and 2 for valid testnet2 signatures.|

### Compiling from source

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
1. `wasm-pack build --release --target nodejs`
