#![feature(try_from)]
use std::convert::TryFrom;

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

// for entries this simple, and for vote organising, we need to record some metadata to prevent collisions (especially with something as simple as votes)
// a timestamp or order record should be enough, as long as it's something outside the control of the user (to prevent intentional collisions for whatever reason)

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
// each comment is content and a commiting actor
// TODO figure out if the commiting actor can be discovered without storing it in the entry
struct Comment {
    content: String,
    committer: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
// each vote is just a simple enum value, changing a vote probably requires an update or delete/commit cycle
// afaik, updating an entry leave it and appends a pointer to the updated entry which the DHT retrieves
// deleting I think appends a notice telling everyone's local HT to just ignore it (or even to drop it from the table?)
// imo, deleting is a better option for managing DHT size, even if votes are small, 10k of them could still be a few Mbs
// TODO as above with the comment/actor thing
struct Vote {
    value: VoteValue,
    voter: String,
    timestamp: u64,
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
                ),
                // is delcaring a from! one way identicle to declaring a to! the other way?
                // TODO figure that out/wait for more documentation
                // from!(
                //     "Parent",
                //     tag: "parents",
                //     validation_package: || hdk::ValidationPackageDefinition::Entry,
                //     validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                //         Ok(())
                //     }
                // )
            ]
        ),
        entry!(
            name: "vote",
            description: "A vote on a comment",
            sharing: Sharing::Public,
            native_type: Vote,
            // TODO validate so each person can only vote once, changing votes is possible though
            // we'd have to have that validation done during linking, not committing? or in the handling function?
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list_item: ListItem, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {}
}

// roots will have no visible parents. is an invisible "true root" necessary for finding the visible roots?
// something with a very simple entry that the address can be easily found?
fn handle_create_root(root: Comment) -> JsonString {
    // create root entry
    let root_entry = Entry::new(EntryType::App("comment".into()), root);

    // commit it
	match hdk::commit_entry(&root_entry) {
		Ok(address) => json!({"success": true, "address": address}).into(),
		Err(hdk_err) => hdk_err.into()
	}
}

fn handle_create_reply(parent_addr: Address, reply: Comment) -> JsonString {
    // create reply entry
    let reply_entry = Entry::new(EntryType::App("comment".into()), reply);

    // commit entry and link on success
    match hdk::commit_entry(&reply_entry)
            .and_then(|reply_addr| {
                hdk::link_entries(&parent_addr, &reply_addr, "replies")
            })
        {
            Ok(_) => {
                json!({"success": true}).into()
            },
            Err(hdk_err) => hdk_err.into()
        }
}

fn handle_get_replies(parent_addr: Address) -> JsonString {
    // try and get the parent entry and ensure it is the data type we expect
    let maybe_parent = hdk::get_entry(parent_addr.clone())
        .map(|entry| Comment::try_from(entry.unwrap().value()));

    // check if it is
	match maybe_parent {
		Ok(Ok(parent)) => {

            // try and load the replies and convert them to the correct struct
            // please forgive the unwraps
			let replies = hdk::get_links(&parent_addr, "replies").unwrap().addresses()
                .iter()
                .map(|reply_address| {
                    let entry = hdk::get_entry(reply_address.to_owned()).unwrap().unwrap();
                    Comment::try_from(entry.value().clone()).unwrap()
                }).collect::<Vec<Comment>>();

            // if this was successful for all list items then return them
            json!({"name": parent.name, "replies": list_items}).into()

		},  // return generic comment-not-found error if wrong type or doesnt exist or whatever
        _ => json!({"successs": false, "message": "No comment at this address"}).into()
	}
}

fn handle_apply_vote(target_comment_addr: Address, vote: Vote) -> JsonString {
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