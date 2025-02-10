// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/*! ABI of the microchain Example Application */

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    base::{AccountOwner, Amount, ContractAbi, ServiceAbi, Timestamp},
    graphql::GraphQLMutationRoot, DataBlobHash,
};
use serde::{Deserialize, Serialize};

pub struct MicrochainAbi;

impl ContractAbi for MicrochainAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for MicrochainAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Instantiate a Lurk program by providing a `ChainState`.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, SimpleObject)]
pub struct InstantiationArgument {
    pub chain_state: DataBlobHash,
    // TODO: potentially more stuff, but idk yet
}

impl std::fmt::Display for InstantiationArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Serialization failed")
        )
    }
}

/// Transition the ChainState by providing a ChainProof.
/// This is the only operation we support right now...
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transition { chain_proof: DataBlobHash },
    // TODO: potentially more stuff, but idk yet
}

/// Another user can send a message to transition the ChainState by providing a ChainProof.
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Transition { chain_proof: DataBlobHash },
}
