pub mod bar;
pub mod foo;

pub mod baz {
    pub fn greet() {
        println!("{}", "baz")
    }
}

pub fn greet() {
    println!("{}", "baz")
}
