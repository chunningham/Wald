use hdk::{
    self,
    error::ZomeApiResult,
    entry_definition::{
        ValidatingEntryType,
    },
    holochain_core_types::{
        cas::content::Address,
        entry::{
            Entry,
            entry_type::EntryType,
        },
        dna::entry_types::Sharing,
    }
};

pub mod comment;
pub mod vote;

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

pub fn create_root(root: comment::Comment) -> ZomeApiResult<Address> {
    // create root entry
    let root_entry = Entry::new(EntryType::App("root".into()), root);

    // commit it
	return hdk::commit_entry(&root_entry)
}

pub fn get_roots() -> ZomeApiResult<Vec<Address>> {
    // get root addresses
    return hdk::query("root".into(), 0, 0)
}
