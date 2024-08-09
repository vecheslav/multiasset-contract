use crate::commands::utils::setup;
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use multiasset_sdk::MultiAssetContract;

#[derive(Args, Clone)]
#[command(about = "Deploys the market to a network")]
pub(crate) struct DeployCommand {
    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl DeployCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Deploy the contract
        let contract = MultiAssetContract::deploy(&wallet).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nMultiAsset contract deployed to: 0x{}", contract.id());
        println!("Deployment cost: {}", balance - new_balance);
        println!("Deployer: 0x{}", wallet.address().hash());

        Ok(())
    }
}
