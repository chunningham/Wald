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
                ),
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
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list_item: ListItem, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {}
}

fn handle_create_root(root: Comment) -> JsonString {
    let root_entry = Entry::new(EntryType::App("comment".into()), root);
	match hdk::commit_entry(&root_entry) {
		Ok(address) => json!({"success": true, "address": address}).into(),
		Err(hdk_err) => hdk_err.into()
	}
}

fn handle_create_reply(parent_addr: Address, reply: Comment) -> JsonString {
    let reply_entry = Entry::new(EntryType::App("comment".into()), reply);

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

		},
        _ => json!({"successs": false, "message": "No comment at this address"}).into()
	}
}