# MultiAsset Contract

MultiAsset is a temporary asset contract developed by the Composability Labs team for smoother testing of the Spark Orderbook on the testnet. It allows users to mint a specified amount of tokens. Please note, the asset standard for the mainnet is still under consideration.

## Features

- **SRC20 Compatible**: The MultiAsset contract is designed to be SRC20-compatible, enabling flexible token minting.
- **Rust SDK**: Provides an SDK with transactional and informational methods for interacting with the MultiAsset contract.

---

## MultiAsset Rust SDK

### Transactional Methods

#### Deploy a New Contract
```rust
pub async fn deploy(wallet: &WalletUnlocked) -> anyhow::Result<Self>
```
Deploys a new MultiAsset contract and returns a `MultiAssetContract` object.

#### Create a New Asset
```rust
pub async fn asset_new(
    &self,
    name: &String,
    symbol: &String,
    decimals: u8,
) -> anyhow::Result<CallResponse<AssetId>>
```
Creates a new asset with `name`, `symbol`, and `decimals`. Returns the `AssetId`.

#### Mint Asset Amount
```rust
pub async fn mint(
    &self,
    recipient: Identity,
    asset: &AssetId,
    amount: u64,
) -> anyhow::Result<CallResponse<()>> {
```
Mints the specified `amount` of `asset` to the `recipient`.

---

### Read Methods

#### Total Assets
```rust
pub async fn total_assets(&self) -> anyhow::Result<CallResponse<u64>>
```
Returns the total number of assets deployed.

#### Total Supply
```rust
pub async fn total_supply(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u64>>>
```
Returns the total minted supply of the specified `asset`.

#### Name
```rust
pub async fn name(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<String>>>
```
Returns the name of the specified `asset`.

#### Decimals
```rust
pub async fn decimals(&self, asset: &AssetId) -> anyhow::Result<CallResponse<Option<u8>>>
```
Returns the number of decimals for the specified `asset`.

#### Get Asset by Name
```rust
pub async fn asset_get(&self, name: &String) -> anyhow::Result<CallResponse<Option<AssetId>>>
```
Returns the `AssetId` of an asset based on its `name`.

---

## CLI Core Functions

These contract calls change the state of the market contract and require a funded wallet.

### Deploy
To deploy the MultiAsset contract, run the following command from the project root:
```bash
./target/release/multiasset_sdk core deploy --rpc "testnet.fuel.network"
```
Example output:
```
MultiAsset contract deployed to: 0xdc527289bdef8ec452f350c9b2d36d464a9ebed88eb389615e512a78e26e3509
Deployment cost: 24112
Deployer: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf
```

### Create a New Asset
```bash
./target/release/multiasset_sdk core asset-new \
    --name USDC \
    --symbol USDC \
    --decimals 6 \
    --contract-id 0xdc527289bdef8ec452f350c9b2d36d464a9ebed88eb389615e512a78e26e3509 \
    --rpc "testnet.fuel.network"
```
Example output:
```
A new asset created with id: 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05
Transaction cost: 5216
Creator: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf
```

### Mint Tokens
```bash
./target/release/multiasset_sdk core mint \
    --recipient-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --recipient-type address \
    --asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --amount 200000000 \
    --contract-id 0xdc527289bdef8ec452f350c9b2d36d464a9ebed88eb389615e512a78e26e3509 \
    --rpc "testnet.fuel.network"
```
Example output:
```
An asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 amount minted 200000000 to: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf
Transaction cost: 2164
Minter: 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf
```

---

## CLI Info Commands

### Retrieve Asset Name
```bash
./target/release/multiasset_sdk info name \
    --asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --contract-id 0xdc527289bdef8ec452f350c9b2d36d464a9ebed88eb389615e512a78e26e3509 \
    --rpc "testnet.fuel.network"
```

### Retrieve Asset Symbol
```bash
./target/release/multiasset_sdk info symbol \
    --asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --contract-id 0xdc527289bdef8ec452f350c9b2d36d464a9ebed88eb389615e512a78e26e3509 \
    --rpc "testnet.fuel.network"
```

### Retrieve Asset Decimals
```bash
./target/release/multiasset_sdk info decimals \
    --asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --contract-id 0xdc527289bdef8ec452f350c9b2d36d464a9ebed88eb389615e512a78e26e3509 \
    --rpc "testnet.fuel.network"
```
