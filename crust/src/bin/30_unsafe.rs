// 5 things you can do only in unsafe rust
// dereference raw pointers
// access union fields
// use mutable static variables
// call unsafe functions (including extern)
// implemenet unsafe traits

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

extern "C" {
    fn abs(input: i32) -> i32;
}

/// Swaps the values pointed to by a and b.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn main() {
    let mut s = String::from("careful!");
    let r1 = &raw mut s;
    let r2 = r1 as *const String;
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = String::from("uhoh");
        println!("r2 is: {}", *r2);
    }

    /* undefined behavior
    let r3: &String = unsafe { &*r1 };
    drop(s);
    println!("r3 is: {}", *r3);
    */
    add_to_counter(42);
    unsafe { println!("COUNTER: {COUNTER}") }

    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    // println!("bool: {}", unsafe { u.b }); // undefined behavior

    unsafe {
        let emojis = "🗻∈🌏";
        println!("emoji: {}", emojis.get_unchecked(0..4));
        println!("abs(-3): {}", abs(-3));
    }

    let mut a = 42;
    let mut b = 66;
    unsafe {
        swap(&mut a, &mut b);
    }
    println!("a: {}, b: {}", a, b);
}
