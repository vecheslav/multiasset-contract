library;

use std::string::String;

pub struct AssetNew {
    pub asset: AssetId,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub creator: Identity,
}

pub struct AssetMinted {
    pub recipient: Identity,
    pub asset: AssetId,
    pub amount: u64,
    pub minter: Identity,
}
