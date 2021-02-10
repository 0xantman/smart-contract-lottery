use crate::state::{PollStatus, Proposal, State, WinnerInfoState};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, CanonicalAddr, HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub denomStable: String,
    pub denomStableDecimal: Uint128,
    pub denomShare: String,
    pub blockTimePlay: u64,
    pub everyBlockTimePlay: u64,
    pub publicSaleEndBlock: u64,
    pub pollEndHeight: u64,
    pub tokenHolderSupply: Uint128,
    pub terrandContractAddress: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// Registering to the lottery
    Register { combination: String },
    /// Run the lottery
    Play {},
    /// Public sale buy the token holders with 1:1 ratio
    PublicSale {},
    /// Claim holder reward
    Reward {},
    /// Claim jackpot
    Jackpot {},
    /// DAO
    /// Make a proposal
    Proposal {
        description: String,
        proposal: Proposal,
        amount: Option<Uint128>,
        prizePerRank: Option<Vec<u8>>,
    },
    /// Vote the proposal
    Vote { pollId: u64, approve: bool },
    /// Valid a proposal
    PresentProposal { pollId: u64 },
    /// Reject a proposal
    RejectProposal { pollId: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the config state
    Config {},
    /// Combination lottery numbers and address
    Combination {},
    /// Winner lottery rank and address
    Winner {},
    /// Get specific poll
    GetPoll { pollId: u64 },
    /// Get the needed round for workers adding randomness to Terrand
    GetRound {},
    /// Query Terrand smart contract to get the needed randomness to play the lottery
    GetTerrand { round: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CombinationInfo {
    pub key: String,
    pub addresses: Vec<CanonicalAddr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllCombinationResponse {
    pub combination: Vec<CombinationInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WinnerInfo {
    pub rank: u8,
    pub winners: Vec<WinnerInfoState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllWinnerResponse {
    pub winner: Vec<WinnerInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPollResponse {
    pub creator: HumanAddr,
    pub status: PollStatus,
    pub end_height: u64,
    pub start_height: u64,
    pub description: String,
    pub amount: Uint128,
    pub prizePerRank: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Round {
    pub nextRound: u64,
}

pub type RoundResponse = Round;

// We define a custom struct for each query response
pub type ConfigResponse = State;
