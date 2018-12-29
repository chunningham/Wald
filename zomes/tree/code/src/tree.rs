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

use comment::Comment;

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
    )
}


// roots will have no visible parents. is an invisible "true root" necessary for finding the visible roots?
// something with a very simple entry that the address can be easily found?
// nah a query can be run for the "root" entries :)
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

pub fn create_reply(parent_addr: Address, reply: Comment) -> ZomeApiResult<Address> {
    // create reply entry
    let reply_entry = Entry::new(EntryType::App("comment".into()), reply);

    // commit entry and link on success
    return hdk::commit_entry(&reply_entry)
            .and_then(|reply_addr| {
                hdk::link_entries(&parent_addr, &reply_addr, "replies")
            })
}

pub fn get_reply_addresses(parent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&parent_addr, "replies")
}

pub fn get_comment(comment_addr: Address) -> ZomeApiResult<Option<Entry>> {
    return hdk::get_entry(&comment_addr)
}

pub fn apply_vote(target_comment_addr: Address, vote: Vote) -> ZomeApiResult<Address> {
    // create vote entry
    let vote_entry = Entry::new(EntryType::App("vote".into()), vote);

    // commit entry and link on success (should the check happen here?)
    return hdk::commit_entry(&vote_entry)
            .and_then(|vote_addr| {
                hdk::link_entries(&target_comment_addr, &vote_addr, "replies")
            })
}

pub fn get_vote_addresses(comment: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&comment, "votes")
}

pub fn get_vote()