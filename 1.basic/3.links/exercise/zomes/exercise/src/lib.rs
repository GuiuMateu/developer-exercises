use hdk::prelude::*;

//  1. Declare an entry data type called Post and register it with the entry_defs macro

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

//  2. Create an ExternalPostData structure:
//    CreatePostInput:
//      This structure can take a string for the content of the post.
//      As all create_link function calls require something to be passed into
//      the tag option, tag-less posts will need to be passed an empty string --> ''

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePostInput {
    content: String,
}

//  3. create_post()
//      Create an entry from input data. Then, pass the EntryHash of the data
//      to the link creation call, where the base of the link is the agent public key.
//      Then return the EntryHash of the post.

#[hdk_extern]
pub fn create_post(external_data: CreatePostInput) -> ExternResult<EntryHash> {
    // Save input data into Post struct
    let post: Post = Post(external_data.content);

    // Create entry and get EntryHash
    create_entry(&post)?;
    let post_entry_hash: EntryHash = hash_entry(&post)?;

    // Get agent EntryHash
    let agent_entry_hash: EntryHash = agent_info()?.agent_latest_pubkey.into();

    // Create link between two elements
    create_link(agent_entry_hash, post_entry_hash.clone(), ())?;

    Ok(post_entry_hash)
}

//  4. get_posts_for_agent()
//      Given the agent_pubkey, find all posts created by the given agent
//      and return a vector of all the Post structures using get().

#[hdk_extern]
pub fn get_posts_for_agent(agent_pubkey: AgentPubKey) -> ExternResult<Vec<Post>> {
    let agent_entry_hash: EntryHash = agent_pubkey.into();
    let links: Links = get_links(agent_entry_hash, None)?;

    let mut posts: Vec<Post> = Vec::new();

    for link in links.into_inner() {
        posts.push(_get_post_from_link(link)?);
    }

    Ok(posts)
}

fn _get_post_from_link(link: Link) -> ExternResult<Post> {
    let entry_hash: EntryHash = link.target;
    let element: Element = get(entry_hash, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Error when get Post")))?;

    let post: Post = element.entry().to_app_option()?
        .ok_or(WasmError::Guest(String::from("No book found")))?;

    Ok(post)
}
