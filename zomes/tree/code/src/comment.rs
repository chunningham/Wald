
use boolinator::Boolinator;
use hdk::entry_definition::ValidatingEntryType;
/// This file holds everything that represents the "post" entry type.
use hdk::holochain_core_types::{
    cas::content::Address, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

// each comment is content and a commiting actor
// TODO figure out if the commiting actor can be discovered without storing it in the entry
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Comment {
    content: String,
    timestamp: u64,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "comment",
        description: "A comment in the tree",
        sharing: Sharing::Public,
        native_type: Comment,
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |comment: Comment, _ctx: hdk::ValidationData| {
            Ok(())
        },
        links: [
            comment_vote_link(),
            comment_reply_link(),
            comment_author_link(),
            author_submissions_link()
        ]
    )
}

pub fn comment_vote_link() -> ValidatingLinkDefinition {
    to!(
        "Vote",
        tag: "votes",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn comment_reply_link() -> ValidatingLinkDefinition {
    to!(
        "Reply",
        tag: "replies",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn comment_author_link() -> ValidatingLinkDefinition {
    to!(
        "Author",
        tag: "author",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn author_submissions_link() -> ValidatingLinkDefinition {
    from!(
        "Submissions",
        tag: "submissions",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}