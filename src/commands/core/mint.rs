use crate::commands::utils::{setup, validate_contract_id, IdentityType};
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{Address, AssetId, ContractId, Identity},
};
use multiasset_sdk::MultiAssetContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Mints an asset amount to recipient")]
pub(crate) struct MintCommand {
    /// The b256 id of the account
    #[clap(long)]
    pub(crate) recipient_id: String,

    /// The type of account
    #[clap(long)]
    pub(crate) recipient_type: IdentityType,

    /// The asset id
    #[clap(long)]
    pub(crate) asset: String,

    /// The amount to mint
    /// Ex. 10000000
    #[clap(long)]
    pub(crate) amount: u64,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl MintCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        if self.asset.len() as u64 != 66 {
            anyhow::bail!("Invalid fuel asset length");
        }

        let asset_id = AssetId::from_str(&self.asset).expect("Invalid asset");

        // Connect to the deployed contract via the rpc
        let contract = MultiAssetContract::new(contract_id, wallet.clone()).await;

        // Mint asset amount
        let minted = match self.recipient_type {
            IdentityType::Address => {
                let address = Address::from_str(&self.recipient_id).expect("Invalid address");
                let asset_balance = wallet
                    .try_provider()?
                    .get_asset_balance(&address.into(), asset_id.clone())
                    .await?;
                contract
                    .mint(Identity::Address(address), &asset_id, self.amount)
                    .await?;
                let new_asset_balance = wallet
                    .try_provider()?
                    .get_asset_balance(&address.into(), asset_id.clone())
                    .await?;
                new_asset_balance - asset_balance
            }
            IdentityType::Contract => {
                let address =
                    ContractId::from_str(&self.recipient_id).expect("Invalid contract id");
                //let asset_balance = wallet.try_provider()?.get_asset_balance(&address.into(), asset_id.clone()).await?;
                contract
                    .mint(Identity::ContractId(address), &asset_id, self.amount)
                    .await?;
                //let new_asset_balance = wallet.try_provider()?.get_asset_balance(&address.into(), asset_id.clone()).await?;
                0 // Todo new_asset_balance - asset_balance
            }
        };

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nAn asset {} amount minted {} to: {}",
            self.asset, minted, self.recipient_id
        );
        println!("Transaction cost: {}", balance - new_balance);
        println!("Minter: 0x{}", wallet.address().hash());

        Ok(())
    }
}
