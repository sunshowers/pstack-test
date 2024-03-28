use std::{thread, time::Duration};

#[inline(never)]
pub fn sub_fn() {
    println!("sub_fn called");

    // Try and do something non-trivial, open a file, etc.
    let mut output = 0;
    for i in 0..100 {
        output += i;
    }
    println!("the output is {output}");

    thread::sleep(Duration::MAX);
}
