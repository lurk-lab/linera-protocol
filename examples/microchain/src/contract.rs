// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{ApplicationId, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime, DataBlobHash,
};
use microchain::{InstantiationArgument, Message, MicrochainAbi, Operation};
use state::MicrochainState;

pub struct MicrochainContract {
    state: MicrochainState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(MicrochainContract);

impl WithContractAbi for MicrochainContract {
    type Abi = MicrochainAbi;
}

impl Contract for MicrochainContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = MicrochainState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MicrochainContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: InstantiationArgument) {
        // Validate that the application parameters were configured correctly.
        let _ = self.runtime.application_parameters();

        // self.runtime.

        self.runtime
            .assert_data_blob_exists(argument.chain_state.clone());
        let chain_state = self.runtime.read_data_blob(argument.chain_state);
        let (chain_proofs, chain_state, zstore_view) = self.runtime.microchain_start(chain_state);
        self.state.chain_proofs.set(chain_proofs);
        self.state.chain_state.set(chain_state);
        self.state.zstore_view.set(zstore_view);
    }

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::Transition { chain_proof } => {
                self.transition(chain_proof);
            }
        }
    }

    async fn execute_message(&mut self, message: Message) {
        match message {
            Message::Transition { chain_proof } => {
                assert_eq!(
                    self.runtime.chain_id(),
                    self.runtime.application_creator_chain_id(),
                    "Action can only be executed on the chain that created this Lurk microchain"
                );

                self.transition(chain_proof);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl MicrochainContract {
    fn transition(&mut self, chain_proof: DataBlobHash) {
        let (chain_proofs, chain_state, zstore_view) = self.runtime.microchain_transition(
            chain_proof,
            self.state.chain_proofs.get().clone(), // Hmm, this clone seems a bit strange, but we keep it for now.
            self.state.chain_state.get().clone(),
            self.state.zstore_view.get().clone(),
        );

        self.state.chain_proofs.set(chain_proofs);
        self.state.chain_state.set(chain_state);
        self.state.zstore_view.set(zstore_view);
    }
}
