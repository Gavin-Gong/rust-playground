pub mod sub;
use self::sub as saz;
use super::bar;
use crate::module::bar as baz;

pub fn greet() {
    bar::greet();
    baz::greet();
    saz::foo();
}
