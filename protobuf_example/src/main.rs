mod addressbook;

use addressbook::AddressBook;
use addressbook::Person;
fn main() {
    let mut book = AddressBook::new();
    book.people = vec![Person::new()];
    book.people[0].id = 100;
    println!("book = {}", book);
}
