use chrono::{NaiveDate};

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

struct DataStore {
    queue: Vec<EmpDetails>
}

impl DataStore {
    fn new() -> Self {
        DataStore { queue: Vec::new() }
    }

    fn enqueue (&mut self) -> () {
        for i in 1..30 {
            let name = "bharath".to_string() + &i.to_string();
            let e1 = EmpDetails::new_employee(name.clone(), 38+u8::from(i), NaiveDate::from_ymd_opt(1987, 3, u32::from(i)));
            println!("Enqueued element");
            e1.display();
            self.queue.push(e1);
        }
    }

    fn dequeue (&mut self) -> () {
        println!("Dequeue is invoked");
        let mut count = 0;
        loop {
            if self.queue.len() == 0 { 
                continue;
            }
            let emp = self.queue.pop();
            println!("Dequeued element: {:?}", emp);
            //emp.display();
            count = count + 1;
            if count == 29 {
                break;
            }
        }
    }
}

fn main() {
    let mut data_store = DataStore::new();
    data_store.enqueue();
    data_store.dequeue();
}
