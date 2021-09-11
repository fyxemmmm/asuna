
use std::rc::Rc;
fn main() {
    // 堆上的数据有了三个共享的所有者。
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
}

// 

use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);
    {
        // 获得 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("data: {:?}", data.borrow());
}

// =================================================================
