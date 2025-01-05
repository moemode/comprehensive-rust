/* An Fn (e.g. add_3) neither consumes nor mutates captured values. 
It can be called needing only a shared reference to the closure, 
which means the closure can be executed repeatedly and even concurrently.

An FnMut (e.g. accumulate) might mutate captured values. 
The closure object is accessed via exclusive reference, so it can be called repeatedly but not concurrently.

If you have an FnOnce (e.g. multiply_sum), you may only call it once. 
Doing so consumes the closure and any values captured by move.

FnMut is a subtype of FnOnce. Fn is a subtype of FnMut and FnOnce. 
I.e. you can use an FnMut wherever an FnOnce is called for, 
and you can use an Fn wherever an FnMut or FnOnce is called for.

When you define a function that takes a closure, 
you should take FnOnce if you can (i.e. you call it once), or FnMut else, and last Fn. 
This allows the most flexibility for the caller.

In contrast, when you have a closure, the most flexible you can have is Fn 
(which can be passed to a consumer of any of the 3 closure traits), then FnMut, and lastly FnOnce.

The compiler also infers Copy (e.g. for add_3) and Clone (e.g. multiply_sum), 
depending on what the closure captures. Function pointers (references to fn items) implement Copy and Fn.

By default, closures will capture each variable from an outer scope by the least demanding 
form of access they can (by shared reference if possible, then exclusive reference, then by move). 
The move keyword forces capture by value.
*/

fn apply_and_log(f: impl FnOnce(i32) -> i32, func_name: &str, input: i32) -> i32 {
    let result = f(input);
    println!("{}({}) = {}", func_name, input, result);
    result
}

fn make_greeter(prefix: String) -> impl Fn(&str) {
    move |name| println!("{} {}", prefix, name)
}

fn main() {
    let add_one = |x| x + 1;
    let add_two = |x| x + 2;
    let add_three = |x| x + 3;

    let x = 5;
    let y = apply_and_log(add_one, "add_one", x);
    let z = apply_and_log(add_two, "add_two", y);
    let w = apply_and_log(add_three, "add_three", z);

    println!("{} + 1 + 2 + 3 = {}", x, w);

    let g = make_greeter("Hello".into());
    g("world");
}