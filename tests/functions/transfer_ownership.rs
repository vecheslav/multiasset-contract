use crate::setup::setup;

use multiasset_sdk::OwnershipTransferred;

mod success {

    use super::*;

    #[tokio::test]
    async fn transfer_ownership() -> anyhow::Result<()> {
        let (contract, owner, user) = setup().await?;

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&owner.wallet)
            .asset_new(&name, &symbol, decimals, false)
            .await?;

        let response = contract
            .with_account(&owner.wallet)
            .transfer_ownership(user.wallet.address().into())
            .await?;

        let log = response
            .decode_logs_with_type::<OwnershipTransferred>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OwnershipTransferred {
                new_owner: user.wallet.address().into(),
                previous_owner: owner.wallet.address().into(),
            }
        );

        let symbol = String::from("BTC_II");
        contract
            .with_account(&user.wallet)
            .asset_new(&name, &symbol, decimals, false)
            .await?;

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn transfer_ownership_not_owner() {
        let (contract, owner, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&owner.wallet)
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();

        contract
            .with_account(&user.wallet)
            .transfer_ownership(user.wallet.address().into())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn transfer_ownership_verify_old_owner() {
        let (contract, owner, user) = setup().await.unwrap();

        let name = String::from("BTC");
        let symbol = String::from("BTC");
        let decimals = 8;

        contract
            .with_account(&owner.wallet)
            .asset_new(&name, &symbol, decimals, false)
            .await
            .unwrap();

        contract
            .with_account(&user.wallet)
            .transfer_ownership(user.wallet.address().into())
            .await
            .unwrap();

        contract
            .with_account(&owner.wallet)
            .asset_new(&name, &symbol, decimals, true)
            .await
            .unwrap();
    }
}
