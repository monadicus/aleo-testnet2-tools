use rand::{rngs::StdRng, SeedableRng};
use snarkvm_console::network::MainnetV0;
use snarkvm_dpc::testnet2::Testnet2;
use wasm_bindgen::prelude::*;

type PrivateKeyMainnet = snarkvm_console::account::PrivateKey<MainnetV0>;
type AddressMainnet = snarkvm_console::account::Address<MainnetV0>;

type PrivateKey2 = snarkvm_dpc::PrivateKey<Testnet2>;
type Address2 = snarkvm_dpc::Address<Testnet2>;
type Signature2 = <Testnet2 as snarkvm_dpc::Network>::AccountSignature;

/// Generate a testnet2 address from a private key
#[wasm_bindgen]
pub fn testnet2_address(private_key: &str) -> Result<String, String> {
    Ok(private_key
        .parse::<PrivateKey2>()
        .map_err(|e| e.to_string())?
        .to_address()
        .to_string())
}

/// Generate a mainnet address from a mainnet private key
#[wasm_bindgen]
pub fn mainnet_address(private_key: &str) -> Result<String, String> {
    let key = private_key
        .parse::<PrivateKeyMainnet>()
        .map_err(|e| e.to_string())?;
    Ok(AddressMainnet::try_from(&key)
        .map_err(|e| e.to_string())?
        .to_string())
}

/// Sign a message with a mainnet private key
#[wasm_bindgen]
pub fn testnet2_sign(private_key: &str, message: &str) -> Result<String, String> {
    let key = private_key
        .parse::<PrivateKey2>()
        .map_err(|e| e.to_string())?;
    Ok(key
        .sign(message.as_bytes(), &mut StdRng::from_entropy())
        .map_err(|e| e.to_string())?
        .to_string())
}

/// Verify a signature with a testnet2 address and message
#[wasm_bindgen]
pub fn testnet2_verify(address: &str, message: &str, signature: &str) -> Result<bool, String> {
    let address = address.parse::<Address2>().map_err(|e| e.to_string())?;
    let signature = signature.parse::<Signature2>().map_err(|e| e.to_string())?;
    address
        .verify_signature(message.as_bytes(), &signature)
        .map_err(|e| e.to_string())
}
