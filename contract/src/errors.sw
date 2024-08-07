library;

pub enum AssetError {
    AssetNotFound: AssetId,
    AssetAlreadyExists: AssetId,
}

pub enum ValueError {
    ZeroStringLength: (),
    ZeroValue: (),
}
