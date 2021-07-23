use hdk::prelude::*;

entry_defs![Book::entry_def()];

#[hdk_entry(id = "book")]
pub struct Book {
    title: String,
    content: String,
}

#[hdk_extern]
pub fn add_book(external_input: Book) -> ExternResult<EntryHash> { 
    // let book: Book = external_input;
    // match create_entry(&book) {
    //     Ok(x) => x,
    //     Err(e) => panic!("{}", e)
    // };

    let book: Book = external_input;
    let _header_hash: HeaderHash = create_entry(&book)?;

    let entry_hash: EntryHash = hash_entry(&book)?;

    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_book(external_input: EntryHash) -> ExternResult<Book> {
    let entry_hash: EntryHash = external_input;
    // let element: Element = match get(entry_hash.into_hash(), GetOptions::default()) {
    //     Ok(entry_hash_option) => { match entry_hash_option {
    //         Some(entry_hash) => entry_hash,
    //         None => panic!("Book not found")
    //     }},
    //     Err(e) => panic!("Error: {}", e)
    // };

    let element: Element = get(entry_hash.into_hash(), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find book")))?;

    // let book: Book = match element.entry().to_app_option() {
    //     Ok(book_option) => { match book_option {
    //         Some(book) => book,
    //         None => panic!("No book in option")
    //     }},
    //     Err(e) => panic!("Error {}", e)
    // };

    // let book_option: Option<Book> = element.entry().to_app_option()?;
    // let book: Book = book_option.expect("No book in option");

    let book: Book = element.entry().to_app_option()?
        .ok_or(WasmError::Guest(String::from("No book found")))?;

    Ok(book)
}
