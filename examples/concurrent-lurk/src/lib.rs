// Copyright (c) Lurk Lab Systems Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{InputObject, SimpleObject};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
    linera_base_types::{AccountOwner, ChainId, MessageId},
    DataBlobHash,
};
use serde::{Deserialize, Serialize};

pub struct ConcurrentLurkAbi;

#[derive(Debug, Deserialize, Serialize, InputObject)]
pub struct TempData {
    pub kind: String,
    pub message: Vec<u8>,
    pub pid: ChainId,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transition {
        chain_proof: DataBlobHash,

        pre_kind: String,
        pre_message: Vec<u8>,
        pre_pid: ChainId,

        post_kind: String,
        post_message: Vec<u8>,
        post_pid: ChainId,

        verify: bool,
    },
    Start {
        owner: AccountOwner,
        chain_state: DataBlobHash,

        post_kind: String,
        post_message: Vec<u8>,
        post_pid: ChainId,

        verify: bool,
    },
}

/// The IDs of a temporary chain for a Lurk process.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, SimpleObject)]
pub struct ProcessId {
    /// The ID of the `OpenChain` message that created the chain.
    pub message_id: MessageId,
    /// The ID of the temporary game chain itself.
    pub chain_id: ChainId,
}

impl ProcessId {
    pub fn new(message_id: MessageId, chain_id: ChainId) -> Self {
        Self {
            message_id,
            chain_id,
        }
    }
}

impl ContractAbi for ConcurrentLurkAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for ConcurrentLurkAbi {
    type Query = async_graphql::Request;
    type QueryResponse = async_graphql::Response;
}
