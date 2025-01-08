use std::vec;

mod garden;

mod outer {

    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

mod pouter {
    pub struct Foo {
        pub val: i32,
        is_big: bool,
    }

    impl Foo {
        pub fn new(val: i32) -> Foo {
            Foo {
                val,
                is_big: val > 100,
            }
        }
    }

    pub mod inner {
        use super::Foo;

        pub fn print_foo(foo: &Foo) {
            println!("Is {} big? {}", foo.val, foo.is_big);
        }
    }
}

mod internals {
    mod a {
        pub fn x() {}
    }
   
    mod b {
        fn y() { 
            super::a::x();
            // works because `b` module is in `internals`
            // which is the same namespace as `a`
        } 
    }
}

fn main() {
    garden::harvest(&mut garden::Garden {
        plants: vec![vec![garden::Plant {
            age: 0,
            height: 0,
            health: 0,
        }]],
    });
    println!("Hello, world!");
    outer::public();
    use pouter::Foo;
    pouter::inner::print_foo(&Foo::new(101));
}
