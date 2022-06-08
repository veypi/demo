type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();
pub struct FieldTest {
    name: &'static str,
    bitmask: u16,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for FieldTest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self {
                name: ref __self_0_0,
                bitmask: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "FieldTest");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "bitmask",
                    &&(*__self_0_1),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
pub struct Field {
    name: &'static str,
    bitmask: u16,
}
impl ::core::fmt::Debug for Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self {
                name: ref name,
                bitmask: ref bitmask,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Field");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*name));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "bitmask", &&(*bitmask));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
fn main() {}
