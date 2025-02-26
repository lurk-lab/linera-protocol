// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Request, Response};
use linera_sdk::{
    base::{ContractAbi, Owner, ServiceAbi},
    graphql::GraphQLMutationRoot, DataBlobHash,
};
use serde::{Deserialize, Serialize};

pub struct LurkMicrochainAbi;

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transition { chain_proof: DataBlobHash },
    Start {
        /// The public keys of player 1 and 2, respectively.
        accounts: [Owner; 2],
        chain_state: DataBlobHash,
    },
}

impl ContractAbi for LurkMicrochainAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for LurkMicrochainAbi {
    type Query = Request;
    type QueryResponse = Response;
}
