use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
        error::HolochainError,
        json::JsonString,
    },
    holochain_wasm_utils::api_serialization::{
        get_entry::GetEntryOptions, get_links::GetLinksResult,
    },
    AGENT_ADDRESS,
};

pub mod vote;
pub mod comment;

pub fn root_definition() -> ValidatingEntryType {
    entry!(
        name: "root",
        description: "An initial parentless topic",
        sharing: Sharing::Public,
        native_type: comment::Comment,
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |comment: comment::Comment, _ctx: hdk::ValidationData| {
            Ok(())
        },
        links: [
            comment::comment_reply_link(),
            comment::comment_author_link(),
            comment::author_submissions_link(),
            vote::comment_upvote_link(),
            vote::comment_downvote_link(),
            vote::agent_upvoted_link(),
            vote::agent_downvoted_link()
        ]
    )
}

pub fn create_root(root: Comment) -> ZomeApiResult<Address> {
    // create root entry
    let root_entry = Entry::new(EntryType::App("root".into()), root);

    // commit it
	return hdk::commit_entry(&root_entry)
}

pub fn get_roots() -> ZomeApiResult<Vec<Address>> {
    // get root addresses
    return hdk::query("root".into(), 0, 0)
}