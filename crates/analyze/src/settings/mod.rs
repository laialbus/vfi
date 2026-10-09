//! The settings: every free parameter of the derivation, each a field of the
//! one [`Settings`] a caller hands analyze on every call.
//!
//! What a setting is, and what each defines beside it — its meaning, unit,
//! domain, preset and the preset's reason — is
//! `docs/adr/what-analyze-takes-and-returns.md`'s. A method that has settings
//! adds its file here and its fields to the one declaration below.

use vfi_contracts::analyze_store::Setting;

/// A setting's value as the characters analyze writes it, which is how the
/// premises carry it. How a value is written is under the method version, so
/// a setting's type implements this where that writing is defined.
pub trait Written {
    fn written(&self) -> Box<str>;
}

/// Declares [`Settings`] and, from the same fields, every setting a result
/// records: each field by its name, with its value written.
///
/// The names are read off the declaration, so a setting added is recorded by
/// adding its field, and there is no second list to leave it out of.
macro_rules! settings {
    (
        $(#[$type_doc:meta])*
        pub struct Settings {
            $( $(#[$field_doc:meta])* $field:ident : $field_type:ty, )*
        }
    ) => {
        $(#[$type_doc])*
        pub struct Settings {
            $( $(#[$field_doc])* $field: $field_type, )*
        }

        impl Settings {
            /// Every setting by name, with the value used.
            pub(crate) fn used(&self) -> Vec<Setting> {
                vec![$(
                    Setting {
                        name: stringify!($field).into(),
                        value: Written::written(&self.$field),
                    },
                )*]
            }
        }
    };
}

settings! {
    /// Every setting the derivation reads.
    ///
    /// `non_exhaustive` keeps the struct literal out of other crates, so a
    /// caller has one only from a constructor here.
    #[non_exhaustive]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Settings {}
}

impl Settings {
    /// Every setting at its preset value. The caller calls this, and the
    /// derivation never does.
    pub fn preset() -> Self {
        Settings {}
    }
}
