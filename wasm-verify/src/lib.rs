use snarkvm_console::network::MainnetV0;
use snarkvm_dpc::testnet2::Testnet2;
use wasm_bindgen::prelude::*;

type AddressMainnet = snarkvm_console::account::Address<MainnetV0>;

type Address2 = snarkvm_dpc::Address<Testnet2>;
type Signature2 = <Testnet2 as snarkvm_dpc::Network>::AccountSignature;

/// Verify the network of an aleo address. 1 is mainnet, 2 is testnet2
#[wasm_bindgen]
pub fn address_verify(address: &str) -> u8 {
    if address.parse::<AddressMainnet>().is_ok() {
        1
    } else if address.parse::<Address2>().is_ok() {
        2
    } else {
        0
    }
}

/// Verify a testnet2 signature with a testnet2 address and message
#[wasm_bindgen]
pub fn testnet2_verify(address: &str, message: &str, signature: &str) -> Result<bool, String> {
    let address = address.parse::<Address2>().map_err(|e| e.to_string())?;
    let signature = signature.parse::<Signature2>().map_err(|e| e.to_string())?;
    address
        .verify_signature(message.as_bytes(), &signature)
        .map_err(|e| e.to_string())
}

/// Verify a mainnet signature with a mainnet address and message
#[wasm_bindgen]
pub fn mainnet_verify(address: &str, message: &str, signature: &str) -> Result<bool, String> {
    let address = address
        .parse::<AddressMainnet>()
        .map_err(|e| e.to_string())?;
    let signature = signature
        .parse::<snarkvm_console::account::Signature<MainnetV0>>()
        .map_err(|e| e.to_string())?;
    Ok(signature.verify_bytes(&address, message.as_bytes()))
}

/// Verify a signature, returning 0 for invalid, 1 for valid mainnet, and 2 for valid testnet2
#[wasm_bindgen]
pub fn signature_verify(address: &str, message: &str, signature: &str) -> u8 {
    let addr_mainnet = address.parse::<AddressMainnet>();
    let sig_mainnet = signature.parse::<snarkvm_console::account::Signature<MainnetV0>>();

    if let (Ok(addr_mainnet), Ok(sig_mainnet)) = (addr_mainnet, sig_mainnet) {
        if sig_mainnet.verify_bytes(&addr_mainnet, message.as_bytes()) {
            return 1;
        }
    }

    let addr2 = address.parse::<Address2>();
    let sig2 = signature.parse::<Signature2>();

    if let (Ok(addr2), Ok(sig2)) = (addr2, sig2) {
        if addr2
            .verify_signature(message.as_bytes(), &sig2)
            .is_ok_and(|t| t)
        {
            return 2;
        }
    }

    // invalid signature or address
    0
}
