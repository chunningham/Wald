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
pub fn handle_create_root(root: Comment) -> ZomeApiResult<Address> {
    // create root entry
    let root_entry = Entry::new(EntryType::App("root".into()), root);

    // commit it
	return hdk::commit_entry(&root_entry)
}

pub fn handle_get_roots() -> ZomeApiResult<Vec<Address>> {
    // get root addresses
    return hdk::query("root", 0, 0)
}

pub fn handle_create_reply(parent_addr: Address, reply: Comment) -> ZomeApiResult<Address> {
    // create reply entry
    let reply_entry = Entry::new(EntryType::App("comment".into()), reply);

    // commit entry and link on success
    return hdk::commit_entry(&reply_entry)
            .and_then(|reply_addr| {
                hdk::link_entries(&parent_addr, &reply_addr, "replies")
            })
}

pub fn handle_get_replies(parent_addr: Address) -> ZomeApiResult<Vec<Address>> {
    // try and get the parent entry and ensure it is the data type we expect
    let maybe_parent = hdk::get_entry(parent_addr.clone())
        .map(|entry| Comment::try_from(entry.unwrap().value()));

    // check if it is
	match maybe_parent {
		Ok(Ok(_parent)) => {

            // try and load the replies and convert them to the correct struct
            // please forgive the unwraps
			let replies = hdk::get_links(&parent_addr, "replies").unwrap().addresses()
                .iter()
                .map(|reply_address| {
                    let entry = hdk::get_entry(reply_address.to_owned()).unwrap().unwrap();
                    Comment::try_from(entry.value().clone()).unwrap()
                }).collect::<Vec<Comment>>();

            // if this was successful for all list items then return them
            json!({"replies": replies}).into()

		},  // return generic comment-not-found error if wrong type or doesnt exist or whatever
        _ => json!({"successs": false, "message": "No comment at this address"}).into()
	}
}

pub fn handle_apply_vote(target_comment_addr: Address, vote: Vote) -> ZomeApiResult<Address> {
    // create vote entry
    let vote_entry = Entry::new(EntryType::App("vote".into()), vote);

    // commit entry and link on success (should the check happen here?)
    match hdk::commit_entry(&vote_entry)
            .and_then(|vote_addr| {
                hdk::link_entries(&target_comment_addr, &vote_addr, "replies")
            })
        {
            Ok(_) => {
                json!({"success": true}).into()
            },
            Err(hdk_err) => hdk_err.into()
        }
}