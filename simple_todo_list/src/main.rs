use std::cell::Cell;
use std::cell::RefCell;

#[derive(Debug, Clone)] // This will help to print the object directly, instead of implementing display method
#[derive(Default)] // Derives the Default trait for the struct
struct TodoItems {
    s_no: u32,
    one_liner: RefCell<String>,
    detail: RefCell<String>,
    status: Cell<bool>,
}

impl TodoItems {
    fn new_item(s_no: u32) -> Self {
        TodoItems {
            s_no: s_no,
            ..Default::default()
        }
    }

    fn display(self) {
        println!(
            "sNo: {}, oneLiner: {}, detail: {}, status: {}",
            self.s_no,
            self.get_oneliner(),
            self.get_detail(),
            self.get_status()
        );
    }

    fn update_status(&self, flag: bool) {
        self.status.set(flag);
    }

    fn update_oneliner(&self, oneliner: String) {
        *self.one_liner.borrow_mut() = oneliner;
    }

    fn update_detail(&self, detail: String) {
        *self.detail.borrow_mut() = detail;
    }

    fn get_status(&self) -> bool {
        self.status.get()
    }

    fn get_oneliner(&self) -> String {
        self.one_liner.borrow().clone()
    }

    fn get_detail(&self) -> String {
        self.detail.borrow().clone()
    }
}

fn main() {
    let item = TodoItems::new_item(1);
    item.update_status(false);
    item.update_oneliner("go to schoool".to_string());
    item.update_detail("do homework first....".to_string());
    item.display()
}
