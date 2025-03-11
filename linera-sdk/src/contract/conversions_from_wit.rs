// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types generated by [`wit-bindgen`] to types declared in [`linera-sdk`].

use linera_base::{
    crypto::CryptoHash,
    data_types::{Amount, BlockHeight, LurkMicrochainData},
    identifiers::{AccountOwner, ApplicationId, ChainId, MessageId, ModuleId},
    ownership::{ChangeApplicationPermissionsError, CloseChainError},
    vm::VmRuntime,
};

use super::wit::contract_runtime_api as wit_contract_api;

impl From<wit_contract_api::CryptoHash> for CryptoHash {
    fn from(crypto_hash: wit_contract_api::CryptoHash) -> Self {
        CryptoHash::from([
            crypto_hash.part1,
            crypto_hash.part2,
            crypto_hash.part3,
            crypto_hash.part4,
        ])
    }
}

impl From<wit_contract_api::AccountOwner> for AccountOwner {
    fn from(account_owner: wit_contract_api::AccountOwner) -> Self {
        match account_owner {
            wit_contract_api::AccountOwner::Reserved(value) => AccountOwner::Reserved(value),
            wit_contract_api::AccountOwner::Address32(value) => {
                AccountOwner::Address32(value.into())
            }
        }
    }
}

impl From<wit_contract_api::ModuleId> for ModuleId {
    fn from(module_id: wit_contract_api::ModuleId) -> Self {
        ModuleId::new(
            module_id.contract_blob_hash.into(),
            module_id.service_blob_hash.into(),
            module_id.vm_runtime.into(),
        )
    }
}

impl From<wit_contract_api::VmRuntime> for VmRuntime {
    fn from(vm_runtime: wit_contract_api::VmRuntime) -> Self {
        match vm_runtime {
            wit_contract_api::VmRuntime::Wasm => VmRuntime::Wasm,
            wit_contract_api::VmRuntime::Evm => VmRuntime::Evm,
        }
    }
}

impl From<wit_contract_api::MessageId> for MessageId {
    fn from(message_id: wit_contract_api::MessageId) -> Self {
        MessageId {
            chain_id: message_id.chain_id.into(),
            height: BlockHeight(message_id.height.inner0),
            index: message_id.index,
        }
    }
}

impl From<wit_contract_api::ApplicationId> for ApplicationId {
    fn from(application_id: wit_contract_api::ApplicationId) -> Self {
        ApplicationId::new(application_id.application_description_hash.into())
    }
}

impl From<wit_contract_api::ChainId> for ChainId {
    fn from(chain_id: wit_contract_api::ChainId) -> Self {
        ChainId(chain_id.inner0.into())
    }
}

impl From<wit_contract_api::Amount> for Amount {
    fn from(balance: wit_contract_api::Amount) -> Self {
        let (lower_half, upper_half) = balance.inner0;
        let value = ((upper_half as u128) << 64) | (lower_half as u128);
        Amount::from_attos(value)
    }
}

impl From<wit_contract_api::LurkMicrochainData> for LurkMicrochainData {
    fn from(data: wit_contract_api::LurkMicrochainData) -> Self {
        LurkMicrochainData {
            chain_proofs: data.chain_proofs,
            chain_state: data.chain_state,
            zstore_view: data.zstore_view,
        }
    }
}

impl From<wit_contract_api::CloseChainError> for CloseChainError {
    fn from(guest: wit_contract_api::CloseChainError) -> Self {
        match guest {
            wit_contract_api::CloseChainError::NotPermitted => CloseChainError::NotPermitted,
        }
    }
}

impl From<wit_contract_api::ChangeApplicationPermissionsError>
    for ChangeApplicationPermissionsError
{
    fn from(guest: wit_contract_api::ChangeApplicationPermissionsError) -> Self {
        match guest {
            wit_contract_api::ChangeApplicationPermissionsError::NotPermitted => {
                ChangeApplicationPermissionsError::NotPermitted
            }
        }
    }
}
