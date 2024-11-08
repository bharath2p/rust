use std::cell::RefCell;

struct MyStruct {
    u8_field: u8,
    u16_field: u16,
    u32_field: u32,
    string_field: RefCell<String>, // Interior mutability for String
    bool_field: RefCell<bool>,     // Interior mutability for bool
}

impl MyStruct {
    // Constructor for creating a new instance of MyStruct
    fn new(u8_field: u8, u16_field: u16, u32_field: u32, string_field: String, bool_field: bool) -> Self {
        MyStruct {
            u8_field,
            u16_field,
            u32_field,
            string_field: RefCell::new(string_field),
            bool_field: RefCell::new(bool_field),
        }
    }

    // Accessor methods for the fields
    fn get_u8(&self) -> u8 {
        self.u8_field
    }

    fn get_u16(&self) -> u16 {
        self.u16_field
    }

    fn get_u32(&self) -> u32 {
        self.u32_field
    }

    fn get_string(&self) -> String {
        self.string_field.borrow().clone()
    }

    fn get_bool(&self) -> bool {
        *self.bool_field.borrow()
    }

    // Mutator methods for the fields that are allowed to change
    fn set_string(&self, new_string: String) {
        *self.string_field.borrow_mut() = new_string;
    }

    fn set_bool(&self, new_bool: bool) {
        *self.bool_field.borrow_mut() = new_bool;
    }
}

fn main() {
    // Create an instance of MyStruct
    let my_struct = MyStruct::new(10, 20, 30, "Initial String".to_string(), true);

    // Accessing fields
    println!("u8_field: {}", my_struct.get_u8());
    println!("u16_field: {}", my_struct.get_u16());
    println!("u32_field: {}", my_struct.get_u32());
    println!("string_field: {}", my_struct.get_string());
    println!("bool_field: {}", my_struct.get_bool());

    // Updating mutable fields
    my_struct.set_string("Updated String".to_string());
    my_struct.set_bool(false);

    // Accessing fields after update
    println!("Updated string_field: {}", my_struct.get_string());
    println!("Updated bool_field: {}", my_struct.get_bool());
}

