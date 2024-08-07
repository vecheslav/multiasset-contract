use crate::setup::setup;

use multiasset_sdk::AssetMinted;

use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, Identity},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn mint() -> anyhow::Result<()> {
        let (contract, minter, user) = setup().await?;

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        let asset = contract
            .with_account(&minter.wallet)
            .await?
            .asset_new(&name, &symbol, decimals)
            .await?
            .value;

        assert_eq!(user.wallet.get_asset_balance(&asset).await?, 0);

        let amount = 1_000_000_000;
        let recipient: Identity = user.wallet.address().into();

        let response = contract
            .with_account(&minter.wallet)
            .await?
            .mint(recipient, &asset, amount)
            .await?;

        let log = response.decode_logs_with_type::<AssetMinted>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            AssetMinted {
                recipient: recipient,
                asset: asset.clone(),
                amount: amount,
                minter: minter.wallet.address().into(),
            }
        );

        assert_eq!(user.wallet.get_asset_balance(&asset).await?, amount);

        contract
            .with_account(&minter.wallet)
            .await?
            .mint(recipient, &asset, amount)
            .await?;

        assert_eq!(user.wallet.get_asset_balance(&asset).await?, 2 * amount);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn mint_not_owner() {
        let (contract, minter, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&minter.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals)
            .await
            .unwrap()
            .value;

        let amount = 1_000_000_000;
        let recipient: Identity = user.wallet.address().into();

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .mint(recipient, &AssetId::zeroed(), amount)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ZeroValue")]
    async fn mint_zero_amount() {
        let (contract, minter, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        let asset = contract
            .with_account(&minter.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals)
            .await
            .unwrap()
            .value;

        assert_eq!(user.wallet.get_asset_balance(&asset).await.unwrap(), 0);

        let amount = 0;
        let recipient: Identity = user.wallet.address().into();

        contract
            .with_account(&minter.wallet)
            .await
            .unwrap()
            .mint(recipient, &asset, amount)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AssetNotFound")]
    async fn mint_bad_asset() {
        let (contract, minter, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&minter.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals)
            .await
            .unwrap()
            .value;

        let amount = 1_000_000_000;
        let recipient: Identity = user.wallet.address().into();

        contract
            .with_account(&minter.wallet)
            .await
            .unwrap()
            .mint(recipient, &AssetId::zeroed(), amount)
            .await
            .unwrap();
    }
}
