#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod vote;
pub mod comment;
pub mod tree;

// use hdk::holochain_core_types::{
//     // hash::HashString,
//     error::HolochainError,
//     entry::Entry,
//     dna::zome::entry_types::Sharing,
//     entry::entry_type::EntryType,
//     json::JsonString,
//     cas::content::Address
//     // json::DefaultJson
// };

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{cas::content::Address, entry::Entry, json::JsonString, error::HolochainError},
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
};

define_zome! {
entries: [
        comment::definition(),
        vote::definition(),
        tree::root_definition()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_root: {
                inputs: |root: Comment|,
                outputs: |result: JsonString|,
                handler: tree::handle_create_root
            }
            get_roots: {
                inputs: | |,
                outputs: |results: ZomeApiResult<Vec<Address>>|,
                handler: tree::handle_get_roots
            }
            create_reply: {
                inputs: |parent_addr: Address, reply: Comment|,
                outputs: |result: JsonString|,
                handler: tree::handle_create_reply
            }
            get_replies: {
                inputs: |parrent_addr: Address|,
                outputs: |result: JsonString|,
                handler: tree::handle_get_replies
            }
            apply_vote: {
                inputs: |target_comment_addr: Address, vote: Vote|,
                outputs: |results: JsonString|,
                handler: tree::handle_apply_vote
            }
        }
    }
}
