mod flow;
mod module;
mod util;

fn main() {
    module::foo::greet();
    module::bar::bar::greet();
    module::bar::greet();
    module::baz::greet();
    module::greet();
    util::greet();
    flow::run();
}
