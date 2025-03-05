// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::{Arc, Mutex};

use async_graphql::{EmptySubscription, Request, Response, Schema};
use lurk_microchain::Operation;
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};

use self::state::LurkMicrochainState;

#[derive(Clone)]
pub struct LurkMicrochainService {
    runtime: Arc<Mutex<ServiceRuntime<LurkMicrochainService>>>,
    state: Arc<LurkMicrochainState>,
}

linera_sdk::service!(LurkMicrochainService);

impl WithServiceAbi for LurkMicrochainService {
    type Abi = lurk_microchain::LurkMicrochainAbi;
}

impl Service for LurkMicrochainService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = LurkMicrochainState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        LurkMicrochainService {
            runtime: Arc::new(Mutex::new(runtime)),
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        )
        .data(self.runtime.clone())
        .finish();
        schema.execute(request).await
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{futures_util::FutureExt, Request};
    use linera_sdk::{util::BlockingWait, views::View, Service, ServiceRuntime};
    use serde_json::json;

    use super::*;

    #[test]
    fn query() {
        let runtime = ServiceRuntime::<LurkMicrochainService>::new();
        let state = LurkMicrochainState::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store");

        let service = LurkMicrochainService {
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        };

        let response = service
            .handle_query(Request::new("{ clock { increment } }"))
            .now_or_never()
            .expect("Query should not await anything")
            .data
            .into_json()
            .expect("Response should be JSON");

        assert_eq!(response, json!({"clock" : {"increment": 0}}))
    }
}
