# Aleo Testnet2 Tools

A website version of these tools is available in the [wasm-suite](./wasm-suite/) directory.

```bash
$ cargo run -q
A collection of tools for working with Aleo testnet2 addresses and signatures

Usage: aleo-testnet2-tools [OPTIONS] <COMMAND>

Commands:
  from-key       Derive a testnet2 and mainnet address from a private key
  from-key-file  Derive a testnet2 and mainnet address from a private key file
  from-sig       Derive a testnet2 and mainnet address from a signature
  sign           Sign a message with a testnet2 private key
  verify         Verify a signature with a testnet2 address and message
  help           Print this message or the help of the given subcommand(s)

Options:
  -j, --json     Enable JSON output
  -h, --help     Print help
  -V, --version  Print version
```

## Setup/Building

### Rust

1. Install [rust](https://www.rust-lang.org/tools/install)
1. Clone this repo
1. View help message: `cargo run`

### Container

1. Install a container engine such as [docker](https://docs.docker.com/engine/install/) or [podman](https://podman.io/docs/installation).
1. Build the container: `docker build . -t aleo-testnet2-tools`
1. Run the CLI in a container `docker run --rm aleo-testnet2-tools tool <COMMAND>` (replace `<COMMAND>` with `from-key ...` or `--help`)

> [!NOTE]
> The file/path based commands/options will not work in containers
>
> ```sh
> # Example of signing a message using a container with a key from the host filesystem
> docker run --rm aleo-testnet2-tools tool sign -p $(cat ~/.aleo/key) -m "hello, world!"
> ```


## Commands

All commands, when run with `--json` (eg. `cargo run -q -- --json`) will output as JSON.

### `from-key` - Derive addresses from key

```sh
# Input the private key as plaintext
$ cargo run -q from-key APrivateKey...
testnet2 address: aleo1...
mainnet address: aleo1...
```

### `from-key-file` - Derive addresses from key file

```sh
# Input the private key from a file
$ cargo run -q from-key-file ./path/to/testnet2.key
testnet2 address: aleo1...
mainnet address: aleo1...
```

### `from-sig` - Derive addresses from a signature

```sh
$ cargo run -q from-sig sign1..."
testnet2 address: aleo1...
```

### `sign` - Sign a message with a Testnet2 private key

```sh
# Sign a message from a private key file
$ cargo run -q sign -f ./path/to/testnet2.key -m "my message"
sign1...

# Sign a message from a plaintext private key
$ cargo run -q sign -p APrivate1... -m "my message"
sign1...
```

### `verify` - Verify a message signed with a Testnet2 private key

```sh
# Sign a message from a private key file
$ cargo run -q verify -a aleo1... -m "signed message" -s sign1...
true # (or false)
```
