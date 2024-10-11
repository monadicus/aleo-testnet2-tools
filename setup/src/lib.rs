use snarkvm_dpc::{
    parameters::testnet2::Testnet2Parameters as Setup, Address, Parameters, PrivateKey,
};

pub type PrivateKeySetup = PrivateKey<Setup>;
pub type AddressSetup = Address<Setup>;
pub type SignatureSetup = <Setup as Parameters>::AccountSignature;

#[cfg(test)]
mod test {
    use snarkvm_algorithms::{EncryptionScheme, SignatureScheme};

    use crate::*;

    #[test]
    fn parse() {
        let pk = "APrivateKey1tvYedpZs4PHtCmjU7nHFPV3onCvHkrQopbXHGP3faiE8iLa"
            .parse::<PrivateKeySetup>()
            .unwrap();
        let sig = Setup::account_signature_scheme()
            .sign(&pk.sk_sig, b"foo", &mut rand::thread_rng())
            .unwrap();
        println!("{:?}", sig);
        let addr = AddressSetup::from_private_key(&pk).unwrap();
        println!("{}", addr);

        let random = Setup::account_encryption_scheme()
            .generate_randomness(addr.to_encryption_key(), &mut rand::thread_rng())
            .unwrap();
        let msg = Setup::account_encryption_scheme()
            .encrypt(addr.to_encryption_key(), &random, b"foo")
            .unwrap();
        let dec = Setup::account_encryption_scheme()
            .decrypt(&pk.to_decryption_key().unwrap(), &msg)
            .unwrap();

        println!("{}", String::from_utf8(dec).unwrap());
    }
}
