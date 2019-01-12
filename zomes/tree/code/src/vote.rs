
use boolinator::Boolinator;
use hdk::entry_definition::ValidatingEntryType;
/// This file holds everything that represents the "post" entry type.
use hdk::holochain_core_types::{
    cas::content::Address, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

// each vote is just a simple bool (true == upvote, false == downvote), changing a vote probably requires an update or delete/commit cycle
// afaik, updating an entry leave it and appends a pointer to the updated entry which the DHT retrieves
// deleting I think appends a notice telling everyone's local HT to just ignore it (or even to drop it from the table?)
// imo, deleting is a better option for managing DHT size, even if votes are small, 10k of them could still be a few Mbs
// TODO as with the comment/actor thing
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Vote {
    value: bool,
    voter: String,
    timestamp: u64,
}

// TODO could vote entries be replaced by up/down links to agent addrs? are links unique?
// not until links can be deleted or unlinked, so keep this for now


pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "vote",
        description: "A vote up or down",
        sharing: Sharing::Public,
        native_type: Vote,
        // TODO validate so each person can only vote once, changing votes is possible though
        // we'd have to have that validation done during linking, not committing? or in the handling function?
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |vote: Vote, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}