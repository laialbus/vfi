//! The types the stages compile against at the boundaries between them.
//!
//! A contract is data: `contracts/<name>/v<N>.<ext>`, frozen the moment its
//! digest reaches `contracts/<name>/versions` and kept frozen by the contracts
//! gate. This crate is the code beside those bytes — one module per contract,
//! holding the Rust the stages on either side of it are written against.
//!
//! Nothing here retrieves, maps or computes. The crate depends on no other, so
//! the stage that produces a value and the stage that consumes it can both name
//! the same type without either learning anything about the other, which is
//! what anchor 3 asks of a boundary and what a stage-to-stage edge would break.
//!
//! The published bytes stay the single source, and a transcription nothing
//! compares is the unchecked duplication the one-source-of-truth invariant
//! bans. So each module carries the comparison as its own test: the field names
//! read off the type's own declaration by the macros below, the published names
//! read out of the contract file, and both printed when they part. `cargo test`
//! runs it, which is the tests gate — no gate was added for it.

/// A type that carries what one side of a boundary hands the other, together
/// with the names of the fields it carries.
///
/// The names come from the declaration itself, so the comparison this crate
/// makes is against the type. Writing them out a second time would put a third
/// reading between the published bytes and the code, and a field renamed in the
/// struct alone would then still agree with it.
///
/// `fields { … }` are the fields the contract names, and are what is compared.
/// A contract that says a value holds its fields *and* something else — a
/// collection the boundary carries rather than a named field — declares that in
/// `and { … }`, which the comparison leaves alone.
macro_rules! carries {
    (
        $(#[$type_doc:meta])*
        pub struct $name:ident {
            fields {
                $( $(#[$field_doc:meta])* pub $field:ident : $field_type:ty, )+
            }
            $(
                and {
                    $( $(#[$held_doc:meta])* pub $held:ident : $held_type:ty, )+
                }
            )?
        }
    ) => {
        $(#[$type_doc])*
        pub struct $name {
            $( $(#[$field_doc])* pub $field: $field_type, )+
            $( $( $(#[$held_doc])* pub $held: $held_type, )+ )?
        }

        #[cfg(test)]
        impl $name {
            const FIELDS: &'static [&'static str] = &[$(stringify!($field)),+];
        }
    };
}

/// A type whose value takes one of several shapes, together with each shape's
/// name and the number of fields it carries.
///
/// The enum is what makes it one shape and never both and never neither: there
/// is no state to check for at run time, because none can be constructed. What
/// the comparison checks is that the shapes are the ones the contract names.
macro_rules! shapes {
    (
        $(#[$type_doc:meta])*
        pub enum $name:ident {
            $(
                $(#[$shape_doc:meta])*
                $shape:ident { $( $carried:ident : $carried_type:ty ),+ $(,)? },
            )+
        }
    ) => {
        $(#[$type_doc])*
        pub enum $name {
            $(
                $(#[$shape_doc])*
                $shape { $( $carried: $carried_type, )+ },
            )+
        }

        #[cfg(test)]
        impl $name {
            const SHAPES: &'static [(&'static str, usize)] =
                &[$( (stringify!($shape), [$(stringify!($carried)),+].len()), )+];
        }
    };
}

pub mod fetch_normalize;
