
use boolinator::Boolinator;
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

// each comment is content and a timestamp
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
            comment_reply_link(),
            comment_author_link(),
            author_submissions_link(),
            vote::comment_upvote_link(),
            vote::comment_downvote_link(),
            vote::agent_upvoted_link(),
            vote::agent_downvoted_link()
        ]
    )
}

pub fn get_comment(comment_addr: Address) -> ZomeApiResult<Option<Entry>> {
    return hdk::get_entry(&comment_addr)
}

pub fn get_comment_author(comment_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&comment_addr, "author")
}

pub fn get_my_submissions() -> ZomeApiResult<GetLinksResult> {
    return get_agent_submissions(&AGENT_ADDRESS)
}

pub fn get_agent_submissions(agent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(agent_addr, "submissions")
}

pub fn create_reply(parent_addrs: Vec<Address>, reply: Comment) -> ZomeApiResult<Address> {
    // create reply entry
    let reply_entry = Entry::new(EntryType::App("comment".into()), reply);

    // commit entry and link on success
    return hdk::commit_entry(&reply_entry)
            .and_then(|reply_addr| {
                parent_addrs.iter().for_each(|parent_addr| hdk::link_entries(&parent_addr, &reply_addr, "replies"));
                hdk::link_entries(&reply_addr, &AGENT_ADDRESS, "author");
                hdk::link_entries(&AGENT_ADDRESS, &reply_addr, "submissions");
            })
}

pub fn get_replies(parent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&parent_addr, "replies")
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