# MultiAsset Contract
SRC20 compatible MultiAsset contract

# MultiAsset Rust SDK - MultiAssetContract method description

# Transactional methods

## Deploy a new contract

pub async fn deploy(wallet: &WalletUnlocked) -> anyhow::Result<Self>

Returns MultiAssetContract object.


## Create a new asset

pub async fn asset_new(
        &self,
        name: &String,
        symbol: &String,
        decimals: u8,
    ) -> anyhow::Result<CallResponse<AssetId>>

Creates a new asset with `name`, `symbol` and `decimals`.
Returns AssetId.


## Mint asset amount

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

Mints `amount` of `asset` to `recipient`.


# Read methods

## Total assets

pub async fn total_assets(&self) -> anyhow::Result<CallResponse<u64>>

Returns the number of assets deployed.

## Total supply

pub async fn total_supply(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u64>>>

Returns a total minted amount of given `asset`

## Name

pub async fn name(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<String>>>

Returns a `name` of given `asset`

## Decimals

pub async fn decimals(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u8>>>

Returns a `decimals` of given `asset`

## Get asset

pub async fn asset_get(&self, name: &String) -> anyhow::Result<CallResponse<Option<AssetId>>>

Returns AssetId by `name`


## Name

pub async fn name(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u8>>>

Returns a `name` of given `asset`




# CLI Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from project root folder

## Deploy

./target/release/multiasset_sdk core deploy --rpc "testnet.fuel.network"

Sample output:

MultiAsset contract deployed to: 0xe7d84947aa22d83e4e570184db554ccef978a64d0acd12b1efc10bfcc357080f
Deployment cost: 23679
Deployer: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## AssetNew

./target/release/multiasset_sdk core asset-new \
    --name USDC \
    --symbol USDC \
    --decimals 6 \
    --contract-id 0xe7d84947aa22d83e4e570184db554ccef978a64d0acd12b1efc10bfcc357080f \
    --rpc "testnet.fuel.network"

Sample output:

A new asset created with id: 0x08f52ffa220c075aa46da0f31a76ed40fd1f7019f317d3295a8b5880e63ab94e
Transaction cost: 5232
Creator: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Mint

./target/release/multiasset_sdk core mint \
    --recipient-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --recipient-type address \
    --asset 0x08f52ffa220c075aa46da0f31a76ed40fd1f7019f317d3295a8b5880e63ab94e \
    --amount 2000000 \
    --contract-id 0xe7d84947aa22d83e4e570184db554ccef978a64d0acd12b1efc10bfcc357080f \
    --rpc "testnet.fuel.network"

Sample output:

An asset 0x08f52ffa220c075aa46da0f31a76ed40fd1f7019f317d3295a8b5880e63ab94e amount minted 2000000 to: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf
Transaction cost: 2164
Minter: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf


# CLI Info Commands

## Name

./target/release/multiasset_sdk info name \
    --asset 0x08f52ffa220c075aa46da0f31a76ed40fd1f7019f317d3295a8b5880e63ab94e \
    --contract-id 0xe7d84947aa22d83e4e570184db554ccef978a64d0acd12b1efc10bfcc357080f \
    --rpc "testnet.fuel.network"

## Symbol

./target/release/multiasset_sdk info symbol \
    --asset 0x08f52ffa220c075aa46da0f31a76ed40fd1f7019f317d3295a8b5880e63ab94e \
    --contract-id 0xe7d84947aa22d83e4e570184db554ccef978a64d0acd12b1efc10bfcc357080f \
    --rpc "testnet.fuel.network"

## Decimals

./target/release/multiasset_sdk info decimals \
    --asset 0x08f52ffa220c075aa46da0f31a76ed40fd1f7019f317d3295a8b5880e63ab94e \
    --contract-id 0xe7d84947aa22d83e4e570184db554ccef978a64d0acd12b1efc10bfcc357080f \
    --rpc "testnet.fuel.network"
