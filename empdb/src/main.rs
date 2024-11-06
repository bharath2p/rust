/* Used the type conversions
 * 1. To convert u8 i.to_string() is used.
 * 2. To append a value to a string, & should be used 
 * 3. To convert a u8 to u32, u32::from(i) is used
 * 4. String cannot be passed directly to a Structure. Clone should be done.
 */

use chrono::{NaiveDate};
use std::collections::HashMap;


#[derive(Debug, Clone)] // This will help to print the object directly, instead of implementing display method
struct EmpDetails {
    name: String,
    age: u8,
    // date of joining
    doj: Option<NaiveDate>
}

impl EmpDetails {
    fn new_employee(name: String, age: u8, doj: Option<NaiveDate>) -> EmpDetails {
        EmpDetails {name: name, age: age, doj: doj}
    }

    fn display(&self) {
        println!("name = {}, age = {}, doj = {:?}", self.name, self.age, self.doj);
    }
}

fn main() {
    let _i = 0u8;
    let mut empdb = HashMap::new();
    for i in 1..30 {
        let name = "bharath".to_string() + &i.to_string();
        let e1 = EmpDetails::new_employee(name.clone(), 38+u8::from(i), NaiveDate::from_ymd_opt(1987, 3, u32::from(i)));
        empdb.insert(name.clone(), e1.clone());
    }
    println!("Inserted elements: (count = {})", empdb.len());
    for (name, employee) in &empdb {
        println!("key = {}, value = {:?}", name, employee);
    }

    for i in 11..=20 {
        let name = "bharath".to_string() + &i.to_string();
        empdb.remove(&name);
    }

    println!("After deleting elements 11 to 20: (count = {})", empdb.len());
    for (name, employee) in &empdb {
        println!("key = {}, value = {:?}", name, employee);
    }
}
