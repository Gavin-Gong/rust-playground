mod data_type;
mod flow;
mod module;
mod util;
mod var;

fn main() {
    module::foo::greet();
    module::foo::sub::foo();
    module::bar::bar::greet();
    module::bar::greet();
    module::baz::greet();
    module::greet();
    util::greet();
    flow::run();
    var::run();
    data_type::run();
}
