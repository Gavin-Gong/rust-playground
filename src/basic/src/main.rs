mod flow;
mod module;
mod util;

use util::greet;

fn main() {
    greet();
    module::foo::greet();
    module::bar::bar::greet();
    module::bar::greet();
    module::baz::greet();
    module::greet();
    util::greet();
}

mod config {
    pub fn init() {}
}
