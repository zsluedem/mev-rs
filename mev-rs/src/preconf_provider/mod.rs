use alloy::{
    primitives::{Address, Bytes, U256},
    rpc::types::beacon::BlsSignature,
};
use axum::async_trait;
use ethereum_consensus::deneb::Slot;
use serde::{Deserialize, Serialize};

type Transaction = Vec<u8>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PreconfRequest {
    pub tip_tx: TipTransaction,
    pub preconf_conditions: PreconfCondition,
    pub init_signature: BlsSignature,
    tip_tx_signature: Bytes,
    pub preconfer_signature: Bytes,
    pub preconf_tx: Option<Transaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TipTransaction {
    pub gas_limit: U256,
    pub from: Address,
    pub to: Address,
    pub pre_pay: U256,
    pub after_pay: U256,
    pub nonce: U256,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PreconfCondition {
    inclusion_meta_data: InclusionMetaData,
    ordering_meta_data: OrderingMetaData,
    pub block_number: u64,
    pub slot: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InclusionMetaData {
    starting_block_number: U256,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderingMetaData {
    transaction_count: U256,
    index: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PreconfRequestQuery {
    pub slot: Slot,
}
#[async_trait]
pub trait PreconfProvider {
    async fn get_preconf_requests(&self, slot: Slot) -> Vec<PreconfRequest>;
    async fn publish_preconf_request(&self, preconf_request: PreconfRequest) -> Result<(), String>;
}
