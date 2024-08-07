use fuels::types::{AssetId, Identity};
use fuels::{
    prelude::{
        abigen, Contract, LoadConfiguration, StorageConfiguration, TxPolicies,
        VariableOutputPolicy, WalletUnlocked,
    },
    programs::responses::CallResponse,
};

use rand::Rng;
use std::path::PathBuf;

abigen!(Contract(
    name = "MultiAsset",
    abi = "contract/out/release/multiasset-contract-abi.json"
));

const MULTIASSET_CONTRACT_BINARY_PATH: &str = "contract/out/release/multiasset-contract.bin";
const MULTIASSET_CONTRACT_STORAGE_PATH: &str =
    "contract/out/release/multiasset-contract-storage_slots.json";

pub struct MultiAssetContract {
    instance: MultiAsset<WalletUnlocked>,
}

impl MultiAssetContract {
    pub async fn deploy(wallet: &WalletUnlocked) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(root.join(MULTIASSET_CONTRACT_STORAGE_PATH));

        let contract_configuration =
            LoadConfiguration::default().with_storage_configuration(storage_configuration?);

        let contract_id = Contract::load_from(
            root.join(MULTIASSET_CONTRACT_BINARY_PATH),
            contract_configuration,
        )?
        .with_salt(salt)
        .deploy(wallet, TxPolicies::default())
        .await?;

        let multiasset = MultiAsset::new(contract_id.clone(), wallet.clone());

        let _self = Self {
            instance: multiasset,
        };

        _self.initialize_ownership(wallet.address().into()).await?;

        Ok(_self)
    }

    pub async fn with_account(&self, account: &WalletUnlocked) -> anyhow::Result<Self> {
        Ok(Self {
            instance: self.instance.clone().with_account(account.clone()),
        })
    }

    async fn initialize_ownership(&self, recipient: Identity) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .initialize_ownership(recipient)
            .call()
            .await?)
    }

    pub async fn transfer_ownership(
        &self,
        recipient: Identity,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .transfer_ownership(recipient)
            .call()
            .await?)
    }

    pub async fn asset_new(
        &self,
        name: &String,
        symbol: &String,
        decimals: u8,
    ) -> anyhow::Result<CallResponse<AssetId>> {
        Ok(self
            .instance
            .methods()
            .asset_new(name.clone(), symbol.clone(), decimals)
            .call()
            .await?)
    }

    pub async fn mint(
        &self,
        recipient: Identity,
        asset: &AssetId,
        amount: u64,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .mint(recipient, *asset, amount)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn total_assets(&self) -> anyhow::Result<CallResponse<u64>> {
        Ok(self.instance.methods().total_assets().call().await?)
    }

    pub async fn total_supply(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u64>>> {
        Ok(self
            .instance
            .methods()
            .total_supply(asset.clone())
            .call()
            .await?)
    }

    pub async fn name(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<String>>> {
        Ok(self.instance.methods().name(asset.clone()).call().await?)
    }

    pub async fn symbol(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<String>>> {
        Ok(self.instance.methods().symbol(asset.clone()).call().await?)
    }

    pub async fn decimals(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u8>>> {
        Ok(self
            .instance
            .methods()
            .decimals(asset.clone())
            .call()
            .await?)
    }
}
