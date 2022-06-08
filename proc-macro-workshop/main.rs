// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use derive_debug::CustomDebug;
type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();

#[derive(Debug)]
pub struct FieldTest {
    name: &'static str,
    bitmask: u16,
}

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bitmask: u16,
}
fn main() {}
