use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Cycle([i32; 100], RefCell<Option<Rc<Cycle>>>);

fn main() {
    let mut total = 0;
    loop {
        let d1 = Rc::new(Cycle([100; 100], RefCell::new(None)));
        let d2 = Rc::new(Cycle([100; 100], RefCell::new(Some(Rc::clone(&d1)))));
        *d1.1.borrow_mut() = Some(Rc::clone(&d2));
        total += 2;
        println!("{total}");
    }
}
