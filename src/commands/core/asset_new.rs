use crate::commands::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use multiasset_sdk::MultiAssetContract;

#[derive(Args, Clone)]
#[command(about = "Creates a new asset")]
pub(crate) struct AssetNewCommand {
    /// The name of a new asset
    /// Ex. USDT
    #[clap(long)]
    pub(crate) name: String,

    /// The symbol of a new asset
    /// Ex. USDT
    #[clap(long)]
    pub(crate) symbol: String,

    /// The decimals of a new asset
    /// Ex. 9
    #[clap(long)]
    pub(crate) decimals: u8,

    /// The True if only owner can mint
    /// Ex. true
    #[clap(long)]
    pub(crate) restricted_mint: bool,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl AssetNewCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = MultiAssetContract::new(contract_id, wallet.clone()).await;

        // Create a new asset
        let asset = contract
            .asset_new(
                &self.name,
                &self.symbol,
                self.decimals,
                self.restricted_mint,
            )
            .await?
            .value;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nA new asset created with id: 0x{}", asset);
        println!("Transaction cost: {}", balance - new_balance);
        println!("Creator: 0x{}", wallet.address().hash());

        Ok(())
    }
}
