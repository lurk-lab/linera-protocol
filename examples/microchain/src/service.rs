// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Request, Response, Schema};
use microchain::Operation;
use linera_sdk::{
    base::{ApplicationId, WithServiceAbi},
    graphql::GraphQLMutationRoot,
    views::View,
    Service, ServiceRuntime,
};
use state::MicrochainState;

pub struct MicrochainService {
    state: Arc<MicrochainState>,
}

linera_sdk::service!(MicrochainService);

impl WithServiceAbi for MicrochainService {
    type Abi = microchain::MicrochainAbi;
}

impl Service for MicrochainService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = MicrochainState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MicrochainService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}
