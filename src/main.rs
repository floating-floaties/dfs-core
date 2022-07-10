mod core;

use crate::core::spec::Spec;

fn main() {
    let user_spec = Spec::default();
    println!("{}", user_spec.eval("ctx.some_var"));
}
