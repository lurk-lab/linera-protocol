// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use crate::state::ProofVerifierState;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
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
                verifying_key: self.state.verifying_key.get().clone(),
                verified_proof: self.state.verified_proof.get().clone(),
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
    async fn verify_proof(&self, proof_hash: DataBlobHash) -> Vec<u8> {
        bcs::to_bytes(&proof_hash).unwrap()
    }
}

struct QueryRoot {
    verifying_key: Vec<u8>,
    verified_proof: bool,
}

#[Object]
impl QueryRoot {
    async fn verifying_key(&self) -> &[u8] {
        &self.verifying_key
    }
    async fn verified_proof(&self) -> &bool {
        &self.verified_proof
    }
}
