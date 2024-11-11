// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use crate::state::ProofVerifierState;
use async_graphql::{Data, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::WithServiceAbi, views::View, DataBlobHash, Service, ServiceRuntime};

pub struct ProofVerifierService {
    state: ProofVerifierState,
}

linera_sdk::service!(ProofVerifierService);

impl WithServiceAbi for ProofVerifierService {
    type Abi = proof_verifier::ProofVerifierAbi;
}

impl Service for ProofVerifierService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = ProofVerifierState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ProofVerifierService { state }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            QueryRoot {
                value: self.state.value.get().clone(),
            },
            MutationRoot {},
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn run(&self, value: DataBlobHash) -> Vec<u8> {
        bcs::to_bytes(&value).unwrap()
    }
}

struct QueryRoot {
    value: bool,
}

#[Object]
impl QueryRoot {
    async fn value(&self) -> &bool {
        &self.value
    }
}
