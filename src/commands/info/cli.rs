use crate::commands::info::{decimals::DecimalsCommand, name::NameCommand, symbol::SymbolCommand};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum InfoCommands {
    /// Query asset decimals information
    #[clap(short_flag = 'D')]
    Decimals(DecimalsCommand),

    /// Query asset name information
    #[clap(short_flag = 'N')]
    Name(NameCommand),

    /// Query asset symbol information
    #[clap(short_flag = 'S')]
    Symbol(SymbolCommand),
}
