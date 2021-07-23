use hdk::prelude::*;

entry_defs![Greeting::entry_def()];

// Define struct forn an entry
#[hdk_entry(id = "gretting")]
pub struct Greeting(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

// Create entry
#[hdk_extern]
pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    let grett: Greeting = Greeting(input.content);
    create_entry(grett)
}
