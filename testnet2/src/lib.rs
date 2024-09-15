use snarkvm_dpc::{testnet2::Testnet2, Address, ComputeKey, Network, PrivateKey};

pub type PrivateKey2 = PrivateKey<Testnet2>;
pub type Address2 = Address<Testnet2>;
pub type ComputeKey2 = ComputeKey<Testnet2>;
pub type Signature2 = <Testnet2 as Network>::AccountSignature;
