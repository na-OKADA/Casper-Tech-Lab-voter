#![no_std]
#![no_main]

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
    api_error::ApiError, CLType, Parameter, Key, NamedKeys,
    EntryPointAccess, EntityEntryPoint as EntryPoint, EntryPoints, EntryPointType, EntryPointPayment,
};

const ALICE_KEY: &str = "ALICE";
const BOB_KEY: &str = "BOB";
const CHARLIE_KEY: &str = "CHARLIE";

#[no_mangle]
pub extern "C" fn voter_inc() {
    let candidate_name: String = runtime::get_named_arg("candidate_name");

    let key = runtime::get_key(&candidate_name)
        .unwrap_or_revert_with(ApiError::MissingKey);

    let uref = key
        .as_uref()
        .cloned()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    storage::add(uref, 1i32);
}

#[no_mangle]
pub extern "C" fn call() {
    let btree_map: BTreeMap<String, Key> = BTreeMap::new();
    // Convert into NamedKeys
    let mut named_keys: NamedKeys = btree_map.into();

    let alice = storage::new_uref(0);
    let bob = storage::new_uref(0);
    let charlie = storage::new_uref(0);

    named_keys.insert(ALICE_KEY.into(), alice.into());
    named_keys.insert(BOB_KEY.into(), bob.into());
    named_keys.insert(CHARLIE_KEY.into(), charlie.into());

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "voter_inc",
        vec![Parameter::new("candidate_name", CLType::String)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    ));

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points.into(), Some(named_keys), None, None, None);

    runtime::put_key("voter", contract_hash.into());
}
