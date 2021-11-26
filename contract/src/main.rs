#![no_std]
#![no_main]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::String,
    vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, Key, URef, Parameter,
};

const ALICE_KEY: &str = "ALICE";
const BOB_KEY: &str = "BOB";
const CHARLIE_KEY: &str = "CHARLIE";

#[no_mangle]
pub extern "C" fn voter_inc() {
    // エントリーポイントから、candidate_nameを受け取る
    let candidate_name: String = runtime::get_named_arg("candidate_name");
    
    // candidate_nameのURefを取得
    let uref: URef = runtime::get_key(&candidate_name)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    
    // URefのインクリメント
    storage::add(uref, 1);
}

#[no_mangle]
pub extern "C" fn call() {
    // Named Keyの作成
    let mut voter_named_keys: BTreeMap<String, Key> = BTreeMap::new();

    let alice_key_name = String::from(ALICE_KEY);
    let bob_key_name = String::from(BOB_KEY);
    let charlie_key_name = String::from(CHARLIE_KEY);

    // それぞれのlocal_keyを作成する必要があります。
    let alice_local_key = storage::new_uref(0);
    let bob_local_key = storage::new_uref(0);
    let charlie_local_key = storage::new_uref(0);

    // Named Keyの挿入
    voter_named_keys.insert(alice_key_name, alice_local_key.into());
    voter_named_keys.insert(bob_key_name, bob_local_key.into());
    voter_named_keys.insert(charlie_key_name, charlie_local_key.into());

    let mut voter_entry_points = EntryPoints::new();
    voter_entry_points.add_entry_point(EntryPoint::new(
        "voter_inc",
        vec![
            Parameter::new("candidate_name", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash, _) =
        storage::new_locked_contract(voter_entry_points, Some(voter_named_keys), None, None);
    runtime::put_key("voter", contract_hash.into());
}
