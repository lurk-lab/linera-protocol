// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;
use linera_sdk::views::{linera_views, RegisterView, RootView, ViewStorageContext};
use linera_sdk::DataBlobHash;

/// The application state.
#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct ProofVerifierState {
    pub value: RegisterView<bool>,
}
