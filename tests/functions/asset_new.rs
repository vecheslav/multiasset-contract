use crate::setup::setup;

use multiasset_sdk::AssetNew;

use fuels::types::AssetId;

mod success {

    use super::*;

    #[tokio::test]
    async fn create_asset_restricted_mint() -> anyhow::Result<()> {
        let (contract, owner, _) = setup().await?;

        let name = String::from("BTC_NAME");
        let symbol = String::from("BTC");
        let decimals = 8;
        let restricted_mint: bool = true;

        let response = contract
            .with_account(&owner.wallet)
            .await?
            .asset_new(&name, &symbol, decimals, restricted_mint)
            .await?;
        let asset = response.value;
        assert_ne!(response.value, AssetId::zeroed());

        let log = response.decode_logs_with_type::<AssetNew>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            AssetNew {
                asset: asset,
                name: name.clone(),
                symbol: symbol.clone(),
                decimals: decimals,
                creator: owner.wallet.address().into(),
            }
        );

        assert_eq!(contract.total_assets().await?.value, 1);
        assert_eq!(contract.total_supply(&asset).await?.value, Some(0));
        assert_eq!(contract.name(&asset).await?.value, Some(name.clone()));
        assert_eq!(contract.symbol(&asset).await?.value, Some(symbol.clone()));
        assert_eq!(contract.decimals(&asset).await?.value, Some(decimals));
        assert_eq!(contract.asset(&symbol).await?.value, Some(asset));
        assert_eq!(
            contract.restricted_mint(&asset).await?.value,
            Some(restricted_mint)
        );

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn create_asset_not_owner() {
        let (contract, _, user) = setup().await.unwrap();

        let name = String::from("BTC_NAME");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AssetAlreadyExists")]
    async fn create_asset_already_exists() {
        let (contract, owner, _) = setup().await.unwrap();

        let name = String::from("BTC_NAME");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&owner.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();

        _ = contract
            .with_account(&owner.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ZeroStringLength")]
    async fn create_asset_empty_name() {
        let (contract, owner, _) = setup().await.unwrap();

        let name = String::from("");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&owner.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ZeroStringLength")]
    async fn create_asset_empty_symbol() {
        let (contract, owner, _) = setup().await.unwrap();

        let name = String::from("BTC_NAME");
        let symbol = String::from("");
        let decimals = 8;

        contract
            .with_account(&owner.wallet)
            .await
            .unwrap()
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();
    }
}
