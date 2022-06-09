pub struct Field<T, V> {
    value: T,
    #[debug = "0b{:08b}"]
    bitmask: V,
}
impl<T, V> ::core::fmt::Debug for Field<T, V>
where
    T: ::core::fmt::Debug,
    V: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self {
                value: ref value,
                bitmask: ref bitmask,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Field");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "value", &&(*value));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "bitmask",
                    &::core::fmt::Arguments::new_v1_formatted(
                        &["0b"],
                        &[::core::fmt::ArgumentV1::new_binary(&&&(*bitmask))],
                        &[::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 8u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Is(8usize),
                            },
                        }],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
fn main() {}
