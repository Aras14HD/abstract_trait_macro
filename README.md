# abstract_trait_macro
A macro to create abstraction Traits in rust, reducing generics clutter.
## Example
```Rust
use abstract_trait_macro::abst_trait;

abst_trait!(Abs; E: ParitalEq + Clone; C: Copy; D: Debug);

fn before<E: PartialEq + Clone, C: Copy, D: Debug>(e: E, c: C, D: D) {}
fn after<T: Abs>(e: T::E, c: T::C, d: T::D) {}
```
## Future Plans
Add where clauses (needs associated type where clauses).
