use std::str::FromStr;

use clap::Args;
use fuels::types::bech32::Bech32Address;

#[derive(Args, Clone)]
#[command(about = "Convert Bech32Address to Address")]
pub(crate) struct Bech32ConvCommand {
    /// The bech32 address
    #[clap(long)]
    pub(crate) bech32: String,
}

impl Bech32ConvCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let bech32_addr = Bech32Address::from_str(self.bech32.as_str()).unwrap();

        println!("\nAn address 0x{}", bech32_addr.hash());

        Ok(())
    }
}
