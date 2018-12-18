#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;
use hdk::holochain_core_types::{
    hash::HashString,
    error::HolochainError,
    entry::Entry,
    dna::zome::entry_types::Sharing,
    entry::entry_type::EntryType,
    json::JsonString,
    cas::content::Address
};

enum VoteValue { up, down }

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Comment {
    content: String,
    committer: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Vote {
    value: VoteValue,
    voter: String,
}

define_zome! {
entries: [
        entry!(
            name: "comment",
            description: "A comment in the tree",
            sharing: Sharing::Public,
            native_type: Comment,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list: List, _ctx: hdk::ValidationData| {
                Ok(())
            },
            links: [
                to!(
                    "Vote",
                    tag: "votes",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    "Reply",
                    tag: "replies",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "vote",
            description: "A vote on a comment",
            sharing: Sharing::Public,
            native_type: Vote,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list_item: ListItem, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {}
}
