use crate::commands::info::{
    bech32_conv::Bech32ConvCommand, decimals::DecimalsCommand, name::NameCommand,
    restricted_mint::RestrictedMintCommand, symbol::SymbolCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum InfoCommands {
    /// Convert Bech32Address to Bits256
    #[clap(short_flag = 'B')]
    Bech32Conv(Bech32ConvCommand),

    /// Query asset decimals information
    #[clap(short_flag = 'D')]
    Decimals(DecimalsCommand),

    /// Query asset name information
    #[clap(short_flag = 'N')]
    Name(NameCommand),

    /// Query asset restricted mint information
    #[clap(short_flag = 'R')]
    RestrictedMint(RestrictedMintCommand),

    /// Query asset symbol information
    #[clap(short_flag = 'S')]
    Symbol(SymbolCommand),
}
