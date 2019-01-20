#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;

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
    holochain_core_types::{cas::content::Address, entry::Entry},
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
};

mod tree;

define_zome! {
    entries: [
            tree::comment::definition(),
            tree::root_definition()
        ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            read_comment: {
                inputs: |addr: Address|,
                outputs: |results: ZomeApiResult<Option<Entry>>|,
                handler: tree::comment::get_comment
            }
            create_root: {
                inputs: |root: tree::comment::Comment|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: tree::create_root
            }
            get_roots: {
                inputs: | |,
                outputs: |results: ZomeApiResult<Vec<Address>>|,
                handler: tree::get_roots
            }
            create_reply: {
                inputs: |parent_addr: Vec<Address>, reply: tree::comment::Comment|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: tree::comment::create_reply
            }
            get_replies: {
                inputs: |parrent_addr: Address|,
                outputs: |result: ZomeApiResult<GetLinksResult>|,
                handler: tree::comment::get_replies
            }
            get_comments_by: {
                inputs: |agent: Address|,
                outputs: |results: ZomeApiResult<GetLinksResult>|,
                handler: tree::comment::get_agent_submissions
            }
            get_my_comments: {
                inputs: | |,
                outputs: |results: ZomeApiResult<GetLinksResult>|,
                handler: tree::comment::get_my_submissions
            }
        }
    }
}
