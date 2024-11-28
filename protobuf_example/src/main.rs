mod addressbook;

use addressbook::person::PhoneNumber;
use addressbook::person::PhoneType;
use addressbook::AddressBook;
use addressbook::Person;

fn create_contact(
    name: String,
    id: i32,
    email: String, /*, number: Vec<PhoneNumber> */
) -> Person {
    let mut pers = Person::new();
    pers.name = name;
    pers.id = id;
    pers.email = email;
    return pers;
}

fn create_phone_num(_phone_type: PhoneType, number: String) -> PhoneNumber {
    let mut phone = PhoneNumber::new();
    phone.number = number;
    // In rust keywords cannot be used as variable.
    // Either i should compile protobuf again or live with this.
    // As I am doing it only to explore, I am planning to live with this.
    //phone.type = phone_type;
    return phone;
}
fn main() {
    let mut book = AddressBook::new();
    let mut person1 = create_contact(
        "testname".to_string(),
        100,
        "testemail@gmail.com".to_string(),
    );
    person1.phones = vec![create_phone_num(PhoneType::WORK, "91023123".to_string())];
    person1.phones.push(create_phone_num(
        PhoneType::HOME,
        "+81023103123".to_string(),
    ));
    book.people.push(person1);
    println!("book = {}", book);
}
