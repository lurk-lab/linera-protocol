// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{
    views::{linera_views, RegisterView, RootView, ViewStorageContext}, DataBlobHash,
};


/// The microchain campaign's state.
#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct MicrochainState {
    /// All the proofs currently on chain.
    pub chain_proofs: RegisterView<Vec<u8>>,
    /// The program state.
    pub chain_state: RegisterView<Vec<u8>>,
    /// The zstore state
    pub zstore_view: RegisterView<Vec<u8>>,
}