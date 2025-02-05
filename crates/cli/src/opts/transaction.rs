use crate::utils::{parse_ether_value, parse_u256};
use clap::Parser;
use ethers::types::U256;
use serde::Serialize;

#[derive(Parser, Debug, Clone, Serialize)]
#[clap(next_help_heading = "Transaction options")]
pub struct TransactionOpts {
    /// Gas limit for the transaction.
    #[clap(long, env = "ETH_GAS_LIMIT", value_parser = parse_u256)]
    pub gas_limit: Option<U256>,

    /// Gas price for legacy transactions, or max fee per gas for EIP1559 transactions.
    #[clap(
        long,
        env = "ETH_GAS_PRICE",
        value_parser = parse_ether_value,
        value_name = "PRICE"
    )]
    pub gas_price: Option<U256>,

    /// Max priority fee per gas for EIP1559 transactions.
    #[clap(
        long,
        env = "ETH_PRIORITY_GAS_PRICE",
        value_parser = parse_ether_value,
        value_name = "PRICE"
    )]
    pub priority_gas_price: Option<U256>,

    /// Ether to send in the transaction, either specified in wei, or as a string with a unit type.
    ///
    ///
    ///
    /// Examples: 1ether, 10gwei, 0.01ether
    #[clap(long, value_parser = parse_ether_value)]
    pub value: Option<U256>,

    /// Nonce for the transaction.
    #[clap(long, value_parser = parse_u256)]
    pub nonce: Option<U256>,

    /// Send a legacy transaction instead of an EIP1559 transaction.
    ///
    /// This is automatically enabled for common networks without EIP1559.
    #[clap(long)]
    pub legacy: bool,
}
