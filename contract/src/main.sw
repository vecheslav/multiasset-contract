contract;

mod errors;
mod events;

use sway_libs::asset::base::{
    _decimals,
    _name,
    _set_decimals,
    _set_name,
    _set_symbol,
    _symbol,
    _total_assets,
    _total_supply,
};
use sway_libs::{
    asset::supply::{
        _burn,
        _mint,
    },
    ownership::{
        _owner,
        initialize_ownership,
        only_owner,
        transfer_ownership,
    },
};
use standards::src20::SRC20;
use standards::src5::{SRC5, State};
use std::{hash::{Hash, sha256}, storage::storage_string::*, string::String};
use errors::*;
use events::*;

storage {
    total_assets: u64 = 0,
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    name: StorageMap<AssetId, StorageString> = StorageMap {},
    symbol: StorageMap<AssetId, StorageString> = StorageMap {},
    decimals: StorageMap<AssetId, u8> = StorageMap {},
    sub_id: StorageMap<AssetId, SubId> = StorageMap {},
    asset: StorageMap<b256, AssetId> = StorageMap {},
    restricted_mint: StorageMap<AssetId, bool> = StorageMap {},
}

abi MultiAsset {
    #[storage(read, write)]
    fn initialize_ownership(new_owner: Identity);

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity);

    #[storage(read, write)]
    fn asset_new(
        name: String,
        symbol: String,
        decimals: u8,
        restricted_mint: bool,
    ) -> AssetId;

    #[storage(read, write)]
    fn mint(recipient: Identity, asset: AssetId, amount: u64);

    #[storage(read)]
    fn asset(symbol: String) -> Option<AssetId>;

    #[storage(read)]
    fn restricted_mint(asset: AssetId) -> Option<bool>;
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        _total_assets(storage.total_assets)
    }

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        _total_supply(storage.total_supply, asset)
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        _name(storage.name, asset)
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        _symbol(storage.symbol, asset)
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        _decimals(storage.decimals, asset)
    }
}

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        _owner()
    }
}

impl MultiAsset for Contract {
    #[storage(read, write)]
    fn initialize_ownership(new_owner: Identity) {
        initialize_ownership(new_owner);
    }

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity) {
        transfer_ownership(new_owner);
    }

    #[storage(read, write)]
    fn asset_new(
        name: String,
        symbol: String,
        decimals: u8,
        restricted_mint: bool,
    ) -> AssetId {
        only_owner();
        let sub_id = sha256((ContractId::this(), symbol));
        let asset = AssetId::new(ContractId::this(), sub_id);
        require(
            name
                .as_bytes()
                .len() > 0 && symbol
                .as_bytes()
                .len() > 0,
            ValueError::ZeroStringLength,
        );
        require(decimals <= 12, ValueError::BadIntValue(decimals));
        require(
            storage
                .sub_id
                .get(asset)
                .try_read()
                .is_none(),
            AssetError::AssetAlreadyExists(asset),
        );
        _set_name(storage.name, asset, name);
        _set_symbol(storage.symbol, asset, symbol);
        _set_decimals(storage.decimals, asset, decimals);

        storage.total_assets.write(storage.total_assets.read() + 1);
        storage.total_supply.insert(asset, 0);
        storage.sub_id.insert(asset, sub_id);
        storage.asset.insert(sha256(symbol), asset);
        storage.restricted_mint.insert(asset, restricted_mint);

        let creator = msg_sender().unwrap();
        log(AssetNew {
            asset,
            name,
            symbol,
            decimals,
            creator,
        });
        asset
    }

    #[storage(read, write)]
    fn mint(recipient: Identity, asset: AssetId, amount: u64) {
        let sub_id = storage.sub_id.get(asset).try_read();
        require(amount > 0, ValueError::ZeroValue);
        require(sub_id.is_some(), AssetError::AssetNotFound(asset));
        let sub_id = sub_id.unwrap();
        assert(
            _mint(
                storage
                    .total_assets,
                storage
                    .total_supply,
                recipient,
                sub_id,
                amount,
            ) == asset,
        );
        if storage.restricted_mint.get(asset).read() {
            only_owner();
        }
        let minter = msg_sender().unwrap();
        log(AssetMinted {
            recipient,
            asset,
            amount,
            minter,
        });
    }

    #[storage(read)]
    fn asset(symbol: String) -> Option<AssetId> {
        storage.asset.get(sha256(symbol)).try_read()
    }

    #[storage(read)]
    fn restricted_mint(asset: AssetId) -> Option<bool> {
        storage.restricted_mint.get(asset).try_read()
    }
}
