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
//!
//! The macros, and the reading of TOML those tests share, sit here rather than
//! in whichever module needed them first, so that a second contract compiles
//! against one reader rather than a copy of one.

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

/// A set of names a contract closes, none of which carries anything.
///
/// The enum is the closure, the way it is in `shapes!`: a value is one of these
/// names or it does not exist, so there is no default member, no empty case,
/// and nothing to check for at run time.
///
/// `ALL` is the set in declaration order, generated rather than written out
/// because a hand-kept list is one a new member can be left out of, and the
/// comparison would then read a set the type no longer has. `declared` is those
/// same names one value at a time, which is how a value is compared against the
/// bytes it was read from.
macro_rules! names {
    (
        $(#[$type_doc:meta])*
        pub enum $name:ident {
            $( $(#[$member_doc:meta])* $member:ident, )+
        }
    ) => {
        $(#[$type_doc])*
        pub enum $name {
            $( $(#[$member_doc])* $member, )+
        }

        impl $name {
            /// Every member, in the order this type declares them.
            pub const ALL: &'static [$name] = &[$( $name::$member, )+];
        }

        #[cfg(test)]
        impl $name {
            fn declared(&self) -> &'static str {
                match self {
                    $( Self::$member => stringify!($member), )+
                }
            }
        }
    };
}

/// One contract's published bytes, and enough TOML to read them.
///
/// Enough to read a frozen file of a known shape, and no more: full lines, a
/// `#` only at the start of one, and one value per key. Anything it does not
/// understand inside a table it is reading stops the test, because a reader
/// that skipped a line it could not parse would agree with a type that had
/// dropped the same field.
#[cfg(test)]
mod published {
    pub struct Contract {
        path: &'static str,
        text: String,
    }

    impl Contract {
        /// The bytes the contracts gate freezes, read from the crate's own
        /// directory rather than the working one, so what the check reads does
        /// not depend on where the runner was standing.
        pub fn at(path: &'static str) -> Self {
            let full = format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"));
            let text = std::fs::read_to_string(&full)
                .unwrap_or_else(|why| panic!("{path} could not be read: {why}"));
            Contract { path, text }
        }

        /// Every occurrence of the array-of-tables `table`, each as the
        /// key/value pairs it states, in the order the file states them.
        pub fn occurrences(&self, table: &str) -> Vec<Vec<(&str, &str)>> {
            self.entries(&format!("[[{table}]]"))
        }

        /// The one table `table`, as the key/value pairs it states.
        pub fn keys_of(&self, table: &str) -> Vec<(&str, &str)> {
            let mut stated = self.entries(&format!("[{table}]"));
            assert_eq!(
                stated.len(),
                1,
                "{} states [{table}] {} times, and one is what this reads",
                self.path,
                stated.len()
            );
            stated.remove(0)
        }

        /// The `name` of every entry under `table`, in the order the file
        /// states them.
        pub fn names_under(&self, table: &str) -> Vec<&str> {
            self.occurrences(table)
                .iter()
                .map(|pairs| self.unquoted(self.value_of(pairs, "name", table)))
                .collect()
        }

        /// The name of every `[prefix.name]` table, in the order the file
        /// states them — the shape a set takes when each member is a definition
        /// rather than an entry.
        pub fn tables_under(&self, prefix: &str) -> Vec<&str> {
            let opening = format!("[{prefix}.");
            let mut found = Vec::new();

            for line in self.text.lines() {
                let line = line.trim();
                if line.starts_with('#') {
                    continue;
                }
                if let Some(name) = line
                    .strip_prefix(&opening)
                    .and_then(|rest| rest.strip_suffix(']'))
                {
                    found.push(name);
                }
            }

            assert!(
                !found.is_empty(),
                "{} states no [{prefix}.…] table, so this reading of it is unusable",
                self.path
            );
            found
        }

        pub fn value_of<'a>(
            &self,
            pairs: &[(&'a str, &'a str)],
            key: &str,
            table: &str,
        ) -> &'a str {
            let mut stated = pairs.iter().filter(|(name, _)| *name == key);
            let value = stated
                .next()
                .unwrap_or_else(|| {
                    panic!("an entry under {table} in {} states no `{key}`", self.path)
                })
                .1;
            assert!(
                stated.next().is_none(),
                "an entry under {table} in {} states `{key}` more than once",
                self.path
            );
            value
        }

        pub fn unquoted<'a>(&self, value: &'a str) -> &'a str {
            value
                .strip_prefix('"')
                .and_then(|inner| inner.strip_suffix('"'))
                .unwrap_or_else(|| {
                    panic!(
                        "{} states `{value}` where a quoted string was expected",
                        self.path
                    )
                })
        }

        /// The members of an inline array, each as the file states it.
        pub fn members<'a>(&self, value: &'a str) -> Vec<&'a str> {
            value
                .strip_prefix('[')
                .and_then(|inner| inner.strip_suffix(']'))
                .unwrap_or_else(|| {
                    panic!("{} states `{value}` where an array was expected", self.path)
                })
                .split(',')
                .map(str::trim)
                .filter(|member| !member.is_empty())
                .collect()
        }

        fn entries(&self, header: &str) -> Vec<Vec<(&str, &str)>> {
            let mut found: Vec<Vec<(&str, &str)>> = Vec::new();
            let mut inside = false;

            for line in self.text.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if line.starts_with('[') {
                    inside = line == header;
                    if inside {
                        found.push(Vec::new());
                    }
                    continue;
                }
                if !inside {
                    continue;
                }
                let Some((key, value)) = line.split_once('=') else {
                    panic!(
                        "{} states `{line}` under {header}, which is not a key and a value",
                        self.path
                    );
                };
                found
                    .last_mut()
                    .expect("a table was entered before its keys were read")
                    .push((key.trim(), value.trim()));
            }

            assert!(
                !found.is_empty(),
                "{} states no {header}, so this reading of it is unusable",
                self.path
            );
            found
        }
    }
}

pub mod canonical_concepts;
pub mod fetch_normalize;
