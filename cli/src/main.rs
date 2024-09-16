use std::ops::Deref;

use clap::{Parser, Subcommand};
use clap_stdin::{FileOrStdin, MaybeStdin};
use mainnet::{AddressMainnet, PrivateKeyMainnet};
use testnet2::{Address2, ComputeKey2, PrivateKey2, Signature2};

/// A collection of tools for working with Aleo testnet2 addresses and signatures
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Enable JSON output
    #[clap(short, long)]
    json: bool,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
pub struct FromKeyArgs {
    /// Private Key to derive the address from (or `-` for stdin)
    private_key: MaybeStdin<PrivateKey2>,
}

#[derive(Parser, Debug)]
pub struct FromKeyFileArgs {
    /// Private Key file to derive the address from (or `-` for stdin)
    private_key_file: FileOrStdin<PrivateKey2>,
}

#[derive(Parser, Debug)]
pub struct FromSigArgs {
    /// Signature to derive the address from (or `-` for stdin)
    signature: MaybeStdin<Signature2>,
}

#[derive(Parser, Debug)]
pub struct SignArgs {
    /// Private key to sign the message with (or `-` for stdin)
    #[clap(short, long)]
    private_key: Option<MaybeStdin<PrivateKey2>>,
    /// File containing the private key to sign the message with (or `-` for stdin)
    #[clap(short = 'f', long)]
    private_key_file: Option<FileOrStdin<PrivateKey2>>,
    /// Message to sign
    #[clap(short, long)]
    message: String,
}

#[derive(Parser, Debug)]
pub struct VerifyArgs {
    /// Address that created the signature
    #[clap(short, long)]
    address: Address2,

    /// Signature to verify
    #[clap(short, long)]
    signature: Signature2,

    /// Message used to create the signature
    #[clap(short, long)]
    message: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Derive a testnet2 and mainnet address from a private key
    #[clap(alias = "from-private-key")]
    FromKey(FromKeyArgs),
    /// Derive a testnet2 and mainnet address from a private key file
    #[clap(alias = "from-private-key-file")]
    FromKeyFile(FromKeyFileArgs),
    /// Derive a testnet2 and mainnet address from a signature
    #[clap(alias = "from-signature")]
    FromSig(FromSigArgs),
    /// Sign a message with a testnet2 private key
    Sign(SignArgs),
    /// Verify a signature with a testnet2 address and message
    Verify(VerifyArgs),
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::FromKeyFile(FromKeyFileArgs { private_key_file }) => {
            let pk = private_key_file.contents()?;
            let pk_mainnet: PrivateKeyMainnet = pk.to_string().parse()?;
            if args.json {
                println!(
                    r#"{{"testnet2": "{}", "mainnet": "{}"}}"#,
                    pk.to_address(),
                    AddressMainnet::try_from(pk_mainnet)?
                );
            } else {
                println!("testnet2 address: {}", pk.to_address());
                println!("mainnet address: {}", AddressMainnet::try_from(pk_mainnet)?);
            }
        }

        Commands::FromKey(FromKeyArgs { private_key }) => {
            let pk_mainnet: PrivateKeyMainnet = private_key.to_string().parse()?;
            if args.json {
                println!(
                    r#"{{"testnet2": "{}", "mainnet": "{}"}}"#,
                    private_key.to_address(),
                    AddressMainnet::try_from(pk_mainnet)?
                );
            } else {
                println!("testnet2 address: {}", private_key.to_address());
                println!("mainnet address: {}", AddressMainnet::try_from(pk_mainnet)?);
            }
        }

        Commands::FromSig(FromSigArgs { signature }) => {
            let addr = Address2::from_compute_key(&ComputeKey2::from_signature(&signature)?);
            if args.json {
                println!(r#"{{"testnet2": "{addr}"}}"#);
            } else {
                println!("testnet2 address: {addr}");
            }
        }

        Commands::Sign(SignArgs {
            private_key,
            private_key_file,
            message,
        }) => {
            let pk = match (private_key, private_key_file) {
                (Some(pk), _) => pk.deref().clone(),
                (_, Some(pk)) => pk.contents()?,
                _ => anyhow::bail!("Either a private key or a private key file must be provided"),
            };
            let sig = pk.sign(message.as_bytes(), &mut rand::thread_rng())?;
            if args.json {
                println!(r#"{{"signature": "{}"}}"#, sig);
            } else {
                println!("{}", sig);
            }
        }

        Commands::Verify(VerifyArgs {
            address,
            signature,
            message,
        }) => {
            let res = address.verify_signature(message.as_bytes(), &signature)?;
            if args.json {
                println!(r#"{{"valid": {}}}"#, res);
            } else {
                println!("{}", res);
            }
        }
    };

    Ok(())
}
