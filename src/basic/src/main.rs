mod data_type;
mod flow;
mod module;
mod ownership;
mod util;
mod var;
mod char;
mod testing;

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
    ownership::run();
}
