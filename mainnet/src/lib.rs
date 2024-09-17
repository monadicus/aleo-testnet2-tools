use snarkvm_console_account::{Address, PrivateKey, Signature};
use snarkvm_console_network::MainnetV0;

pub type PrivateKeyMainnet = PrivateKey<MainnetV0>;
pub type AddressMainnet = Address<MainnetV0>;
pub type SignatureMainnet = Signature<MainnetV0>;
