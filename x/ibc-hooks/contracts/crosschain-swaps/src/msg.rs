use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use swaprouter::msg::Slippage;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    /// This should be an instance of the Osmosis swaprouter contract
    pub swap_contract: String,

    /// These are the channels that will be accepted by the contract. This is
    /// needed to avoid sending packets to addresses not supported by the
    /// receiving chain. The channels are specified as (bech32_prefix, channel_id)
    pub channels: Vec<(String, String)>,
}

/// An enum specifying what resolution the user expects in the case of a bad IBC
/// delviery
#[cw_serde]
pub enum FailedDeliveryAction {
    DoNothing,
    /// An osmosis addres used to recover any tokens that get stuck in the
    /// contract due to IBC failures
    LocalRecoveryAddr(Addr),
    // Here we could potentially add new actions in the future
    // example: SendBackToSender, SwapBackAndReturn, etc
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    /// Execute a swap and forward it to the receiver address on the specified ibc channel
    OsmosisSwap {
        /// The amount and denom to be swapped
        input_coin: Coin,
        /// The final denom to be received (as represented on osmosis)
        output_denom: String,
        /// The receiver of the IBC packet to be sent after the swap
        receiver: Addr,
        /// Slippage for the swap
        slippage: Slippage,
        /// IBC packets can contain an optional memo. If a sender wants the sent
        /// packet to include a memo, this is the field where they can specify
        /// it. If provided, the memo is expected to be a valid JSON object
        next_memo: Option<String>,
        /// If for any reason the swap were to fail, users can specify a
        /// "recovery address" that can clain the funds on osmosis after a
        /// confirmed failure.
        on_failed_delivery: FailedDeliveryAction,
    },
    /// Executing a recover will transfer any recoverable tokens that the sender
    /// has in this contract to its account.
    ///
    /// This is only usable if the contract is configured with track_ibc_sends.
    ///
    /// The only tokens that are considered recoverable for a "sender" are those
    /// returned by an IBC transfer sent by this contract, that are known to
    /// have failed, and that originated with a message specifying the "sender"
    /// as its recovery address.
    Recover {},
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the list of transfers that are recoverable for an Addr
    #[returns(Vec<crate::state::ibc::IBCTransfer>)]
    Recoverable { addr: Addr },
}

// tmp structure for crosschain response
#[cw_serde]
pub struct CrosschainSwapResponse {
    pub msg: String, // Do we want to provide more detailed information here?
}

impl CrosschainSwapResponse {
    pub fn base(amount: &Uint128, denom: &str, channel_id: &str, receiver: &str) -> Self {
        CrosschainSwapResponse {
            msg: format!("Sent {amount}{denom} to {channel_id}/{receiver}"),
        }
    }
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Weather an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

/// Message type for `sudo` entry_point
#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
