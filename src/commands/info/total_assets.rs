use crate::commands::utils::{setup, validate_contract_id};
use clap::Args;
use multiasset_sdk::MultiAssetContract;

#[derive(Args, Clone)]
#[command(about = "Query an asset number on contract")]
pub(crate) struct TotalAssetsCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl TotalAssetsCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MultiAssetContract::new(contract_id, wallet.clone()).await;

        // Create a new asset
        let assets = contract.total_assets().await?.value;

        println!("\nA total assets is: {}", assets);

        Ok(())
    }
}
