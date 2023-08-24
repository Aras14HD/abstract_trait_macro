use abstract_trait_macro::abst_trait;
use core::fmt::Debug;

abst_trait!(Abs; I: IntoIterator; C: Clone; D: Debug);

fn main() {
    println!("Hello, world!");
}
