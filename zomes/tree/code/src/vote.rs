
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

pub fn get_upvotes(comment_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&comment_addr, "upvotes")
}

pub fn get_downvotes(comment_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&comment_addr, "downvotes")
}

pub fn get_upvoted_comments(agent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&agent_addr, "upvoted")
}

pub fn get_downvoted_comments(agent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    return hdk::get_links(&agent_addr, "downvoted")
}

pub fn apply_vote(comment_addr: Address, vote: bool) -> Result<(), ZomeApiError> {
    // making this real explicit
    if (vote == true) {
        hdk::link_entries(&AGENT_ADDRESS, &comment_addr, "upvoted");
        return hdk::link_entries(&comment_addr, &AGENT_ADDRESS, "upvotes")
    } else {
        hdk::link_entries(&AGENT_ADDRESS, &comment_addr, "downvoted");
        return hdk::link_entries(&comment_addr, &AGENT_ADDRESS, "downvotes")
    }
}

// votes are immutable until links can be unmade lmao, fight me
pub fn comment_upvote_link() -> ValidatingLinkDefinition {
    to!(
        "Upvote",
        tag: "upvotes",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn comment_downvote_link() -> ValidatingLinkDefinition {
    to!(
        "Downvote",
        tag: "downvotes",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn agent_upvoted_link() -> ValidatingLinkDefinition {
    from!(
        "Upvoted",
        tag: "upvoted",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn agent_downvoted_link() -> ValidatingLinkDefinition {
    from!(
        "Downvoted",
        tag: "downvoted",
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}