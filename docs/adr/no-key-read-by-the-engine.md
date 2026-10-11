# A new gate, `secrets`, reads every engine crate but analyze for the environment, file and keychain reads that could hand the engine a key it was not given

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-10, by `M5-08`
- **Decided:** 2026-10-10, by the decider
- **Touches:** the gate set. A new gate name, `secrets`, enters the expected
  gate set with its proof, and `denied_packages` gains a `keychain` group, which
  purity then also reads. It applies ANCHORS.md's invariant "Secrets live in the
  shell, never the engine" and closes, in part, the gap `price-provider.md`
  leaves under Enforcement. No anchor is edited. Nothing `price-provider.md`
  decided moves: the key stays a constructor parameter, sent only as a header
  and never printed. Nothing `price-allowance-between-sessions.md` decided
  moves either: the spent journal stays an append-only file in
  `crates/fetch/src/price/spent/`, opened at a path it is handed. How the shell
  stores the key is the shell's, and is not decided here. No entry leaves
  `denied_packages`, `allowed_edges` or the expected gate set, no name leaves
  `ambient_names`, and nothing here is implemented.

## Context

GOALS.md's first M5 criterion says the key "reaches the engine only as a
parameter". ANCHORS.md says the engine "never reads a keychain, config file, or
environment for one". `price-provider.md` designs the parameter, `Option<ApiKey>`
on the Tiingo provider in `vfi-fetch`, and then records under Enforcement that
nothing stops the engine reading `std::env` or a file for a key. `ambient`
(M5-04) reads only `crates/analyze/`, and `egress` reads only for connections.
That record's Decision review leaves the gap open. AGENTS.md makes a change with
no gate an escalation, so the provider's implementation waits on a check.

The difficulty is the one the task names. The engine already reads files and
the environment for reasons that are not a key, so a check that refused every
`std::fs` and `std::env` outside analyze would refuse main. A check that reads
source for names cannot tell a key file from a journal. What it reads for is
therefore a decision about each way a key could arrive.

One more file is fixed before any code exists. `price-allowance-between-sessions.md`
(Accepted) puts the `SpentRecord` trait and its journal, "which keeps entries
in an append-only file", in `crates/fetch/src/price/spent/`. That journal
writes to disk inside the directory `price-provider.md` gives the key's code,
and this record has to leave it able to.

## Decision

### The gate and where it sits

`secrets` is a gate of its own. It is not `ambient` read over more crates.
`ambient` reads for the clock, a process and a randomized hasher, which fetch's
pace and jobs legitimately use, and for every filesystem name, which the ledger
journal, the spent journal and the registry need. Widening it would turn its
one judgement, that analyze names none of the five, into a list of exceptions.

The name enters `expected_gates` directly after `egress`, since both read
fetch's source and `secrets` reads egress's chokepoint by name. Its proof enters
`violations()`, one plant per way below.

The gate reuses `ambient_reaches` as its matcher. A name is therefore matched
the way M5-04 matches it: as text, wherever it stands, comments included, and
never as the middle of a longer identifier.

### The crates it reads

It reads every workspace member except the one `ambient_crate` names. Today
that is `vfi-fetch`, `vfi-normalize`, `vfi-store`, `vfi-jobs` and
`vfi-contracts`. The members are read off the workspace and not listed, so a
crate added later, such as a binding the shell calls, is read from its first
commit. Within each crate it reads every Rust file: `src/`, `tests/`, any bench
and any build script. A build script that reads a key bakes it into the engine,
and a test that reads one makes CI hold one.

**`vfi-analyze` is left out by name.** `ambient` already reads every Rust file
there for every name it tags `environment`, `filesystem` and `process`, with no
exception. purity already denies its whole resolved tree every package
`denied_packages` lists, and that list gains `keychain` below. A key read in
analyze goes red there under a stricter rule. A second gate reading the same
lines would print the same failure twice.

The Python shell is not the engine, and the key is the shell's to hold.

### The names it shares with `ambient`, read from one place

A name both gates read is read out of `ambient_names` and is not written a second
time. It is selected by its `which` tag and by one rule, never by a list of
names:

- **Engine-wide,** every name `ambient_names` tags `environment` or
  `filesystem` that ends in `!`. Today these are `env!`, `option_env!`,
  `include!`, `include_str!` and `include_bytes!`. They are the compile-time
  forms. A key read through one is baked into every engine built, wherever it
  is written.
- **In the key's places** (below), every name `ambient_names` tags
  `environment`, `filesystem` or `process`, wholesale.

Package names come from `denied_packages` by group, as `wire_names` already
takes the network group. A package added to a group once is learned by every
gate that reads it.

### What it reads for, way by way

**The environment.** These are read across every file of every crate read:

- The runtime getters, which are this gate's own names: `env::var`,
  `env::var_os`, `env::vars` and `env::vars_os`. A key stored under any variable
  name is caught, because the check reads for the call and not for the
  variable.
- `env::{` and `env::*`, also its own. A grouped or glob import from `std::env`
  is the form in which a getter stands bare, so it is refused whatever it
  holds. A getter imported alone, `use std::env::var;`, still writes `env::var`.
- `env!` and `option_env!`, from `ambient_names`. Two exact calls are taken out
  of a line before it is read, and nothing else is: `env!("CARGO_MANIFEST_DIR")`
  and `env!("CARGO_TARGET_TMPDIR")`. Cargo sets both itself for each compile,
  to the package's own directory and the integration tests' scratch directory.
  Neither carries a value a user exported.
- The packages `denied_packages` groups as `environment`: `dotenv`, `dotenvy`,
  `envy`, `config` and `clap`. These are read in each crate's direct
  dependencies (`direct_dependencies`, which reads normal, build and dev edges)
  and not as source paths. A bare `config::` in source would also refuse a
  module of that name.

These are not read: `env::args` and `args_os`, `env::temp_dir`, `current_dir`
and `current_exe`. A process's arguments are what whoever runs it passes, which
is a parameter in a process's shape. The rest return a place the process
stands, never the value of a variable the code names.

**A file.** A name check cannot tell a key file from a journal. So the file way
is held where the key lives, and engine-wide only at the forms that locate or
bake one:

- **The key's places** are three:
  - the egress chokepoint, read from the `chokepoint` variable `egress` already
    defines;
  - `crates/fetch/src/price/`, where `price-provider.md` puts the trait and
    Tiingo's request, once it exists, except `crates/fetch/src/price/spent/`
    (see below);
  - any file in a read crate that names `ApiKey`, the type `price-provider.md`
    fixes, wherever it sits, `price/spent/` included. This set follows the key
    if the code holding it is written somewhere else.

  In these places, every `ambient_names` name tagged `filesystem`,
  `environment` or `process` is refused. So is every package `denied_packages`
  groups as `filesystem`, read as a source path, `-` to `_`, as `wire_names`
  does. Code that holds the key has no reason to touch any of them. The places
  are never empty, because `egress` already refuses an empty chokepoint.

  The directory is a place, and not only the files naming `ApiKey`, because a
  method in a file of its own reaches the key through a field, `self.key`,
  without writing the type.
- **`price/spent/` stands outside, by directory only.** The journal
  `price-allowance-between-sessions.md` places there opens, reads and appends
  to a file, so it needs `std::fs` and `std::path`. The key does not reach it
  from there, for three reasons that record fixes:
  - **An entry holds no key.** It carries the moment, the ticker and the host,
    and that record says "There is no key, no price, no status and no count in
    an entry".
  - **The record is handed to the provider, never the reverse.** Every ask
    takes `&mut` a `SpentRecord`, and the provider keeps no field of it. The
    journal's two operations take and return entries, so nothing on its
    interface carries the key in.
  - **The journal is told its host as a string.** It checks entries against
    `api.tiingo.com` and needs nothing of the provider's to do so.

  So code in `price/spent/` has no reason to name `ApiKey`. If a file there
  ever does, the third place takes it in and its filesystem names go red. The
  carve-out is a directory where the key's type cannot be written, not a
  directory where reads are allowed whatever it holds.
- **Engine-wide:**
  - `include!`, `include_str!` and `include_bytes!`, from `ambient_names`, as
    above;
  - `env::home_dir`, this gate's own, tagged `filesystem`. It is the step that
    finds a per-user file the caller never named, and the engine holds no
    per-user state.

**A keychain.** A keychain is reached through a package, through the
keychain's own command-line tool (`security`, `secret-tool`), or through a
foreign function. The first two are read across every crate read:

- **The package.** A new `keychain` group in `denied_packages` lists `keyring`,
  `security-framework`, `security-framework-sys` and `secret-service`. These are
  read in each crate's direct dependencies.

  The direct dependencies, not the resolved tree, are read for a reason.
  `native-tls` links `security-framework` and `security-framework-sys` on
  macOS for its TLS, so a tree read would go red on main for a reason that is
  not a key. A crate can name in source only a package it lists directly, so
  the direct list is where reaching a keychain through one starts.

  purity reads the group over analyze's whole tree, as it reads every group.
  That tree is `vfi-analyze` and `vfi-contracts` today, so purity stays green.
- **A child process.** `process::Command` and `Command::new`, this gate's own,
  tagged `keychain`. A child can also read the environment or a file for the
  engine, and this catches those too.

The failure names the file, the line, the name, and which of `environment`,
`filesystem`, `process` or `keychain` it reached. That is `ambient`'s shape.
For a package, it names the crate and the package instead.

### What main holds, and what the check does about each

Main as it stands passes. These are every environment, filesystem and process
read main holds in the crates read. A file read the check reads for elsewhere,
or one it refuses, appears nowhere on main.

| Where | What | What the check does |
| :--- | :--- | :--- |
| `crates/fetch/src/bin/record.rs:33`, `:108` | `use std::env;`, `env::args()`. Who is asking and where to record arrive as arguments. | Passes, because argv is not read. |
| `crates/fetch/src/ledger/journal/mod.rs:262`, in its `#[cfg(test)]` module | `std::env::temp_dir()` | Passes, because `temp_dir` is not read. |
| `crates/fetch/tests/funnel.rs:995` | `std::env::temp_dir()` | Passes, for the same reason. |
| `crates/contracts/src/lib.rs:211` (in `#[cfg(test)] mod published`); `crates/fetch/tests/golden.rs:301`; `crates/normalize/tests/golden.rs:48`, `registry.rs:21`, `bench.rs:297`, `:384`, `fixture/mod.rs:44`, `:144` | `env!("CARGO_MANIFEST_DIR")`, eight calls | Passes, as the first exact call taken out. |
| `crates/normalize/tests/registry.rs:27`, `:646`; `crates/normalize/tests/fixture/mod.rs:50` | `env!("CARGO_TARGET_TMPDIR")`, three calls | Passes, as the second exact call taken out. |
| `crates/fetch/src/bin/record.rs:34`, `:35`, `:145`, `:148`, `:193` | `std::fs`, `std::path`. It writes the recordings under the case it is given. | Passes. It is not a key's place. |
| `crates/fetch/src/bin/record.rs:36`, `:64`, `:70`, `:104`, `:186`, `:188` | `std::process::ExitCode` | Passes. Only `process::Command` and `Command::new` are read outside the key's places. |
| `crates/fetch/src/ledger/journal/mod.rs:262`; `crates/fetch/tests/funnel.rs:995` | `std::process::id()`, naming a scratch directory | Passes, for the same reason. |
| `crates/fetch/src/ledger/journal/mod.rs:54`, `:56` and its tests | `File`, `OpenOptions`, `std::path`. The journal reads and writes the one path `Journal::at` is handed. | Passes. It is not a key's place. |
| `crates/normalize/src/registry.rs:51`; `crates/normalize/src/registry/tree.rs:22`, `:23`, `:103`, `:124` | `std::path::Path`, `fs::read_dir`, `fs::read` over the registry directory it is handed. | Passes. It is not a key's place. |
| `crates/contracts/src/lib.rs:212`, in the same test module | `std::fs::read_to_string` of a published contract | Passes. It is not a key's place. |
| `crates/fetch/tests/funnel.rs`, `golden.rs`; `crates/normalize/tests/bench.rs`, `fixture/mod.rs`, `golden.rs`, `registry.rs` | `std::fs` and `std::path` over fixtures, benchmarks, the registry and scratch copies | Passes. None is a key's place. |

The chokepoint names no filesystem, environment or process name today.
`crates/fetch/src/price/` does not exist, and no file names `ApiKey`. No crate
read names `env::var`, `var_os`, `vars`, `vars_os`, `home_dir`, `env::{`,
`env::*`, `option_env!`, an `include` macro, `process::Command` or
`Command::new`. Of the packages, `vfi-fetch` lists `native-tls`, `serde`,
`serde_json` and `vfi-contracts` directly, and `vfi-normalize` lists
`vfi-contracts`. Store, jobs and contracts list none, so no group above is
listed.

### The proof the implementing task brings

The implementing task brings one plant per way the check claims to catch, each
in a copy of its own. Each plant is pinned, as `ambient`'s are, to the planted
line and the way it reached. The plants are spread across crates and over a
library file, a build script and a test, because the gate claims to read all of
them.

- **The environment:**
  - a runtime getter, `std::env::var`, in a library file of `vfi-fetch`;
  - a grouped import, `use std::env::{var as …}`, in `vfi-normalize`;
  - `option_env!` in a build script `vfi-store` does not yet have;
  - `env!` on a variable that is not one of Cargo's two, in a test;
  - an `environment` package listed in a read crate's manifest. This is a local
    path package of that name sitting beside the copy, as `violate_purity`
    plants `rand`.
- **A file:**
  - `std::fs::read_to_string` in `crates/fetch/src/price/`, which the plant
    makes;
  - the same in the chokepoint;
  - the same in a file outside the chokepoint and `price/` that names
    `ApiKey`;
  - the same in a file under `crates/fetch/src/price/spent/` that names
    `ApiKey`, which shows the carve-out ends where the type is written;
  - `include_str!` in a test of `vfi-jobs`;
  - `env::home_dir` in `vfi-contracts`.
- **A keychain:**
  - a `keychain` package listed in a read crate's manifest, planted the same
    way;
  - `use std::process::Command;`;
  - `Command::new("security")` behind a grouped import that writes no
    `process::Command`.

Beside these comes an accept copy that must stay green. It holds:

- `env::args()`, `env::temp_dir()`, and both Cargo calls;
- `std::fs` in a library file outside the key's places;
- `std::fs`, `std::path`, `std::env::temp_dir()` and `std::process::id()` in a
  file under `crates/fetch/src/price/spent/` that does not name `ApiKey`, the
  shape the spent journal and its tests take;
- `std::process::id()` and `ExitCode` outside the key's places;
- a module named `config`;
- an identifier that holds a read name inside a longer one, such as
  `env::variables`.

A gate that refused these would refuse main's own reads, or the journal an
accepted record requires.

## Alternatives

- **`ambient` read over every engine crate.** Rejected, for the reasons under
  "The gate and where it sits". Its clock, process and hasher names refuse
  fetch's pace and jobs. Its filesystem names refuse the ledger journal, the
  spent journal and the registry. Its `env::` refuses a bin's argv. Making it
  pass would mean writing exceptions into the one place M5-04 wrote its
  judgement down, and analyze would then be read under a list someone else had
  loosened.
- **All of `crates/fetch/src/price/` a key's place, `spent/` included.**
  Rejected. The spent journal cannot append to a file without `std::fs`, so
  the gate would go red on the journal `price-allowance-between-sessions.md`
  requires. Passing it would mean moving the journal, which changes what that
  accepted record decided.
- **Only the files naming `ApiKey`, with no directory.** Rejected. A method in
  a file of its own reaches the key through a field without writing the type,
  and could then read a file beside it unseen.
- **A disk chokepoint: filesystem names stand only in listed places,
  engine-wide.** This is the shape `egress` has for the network. Rejected for
  three reasons:
  - Store's whole job at M6 is the disk, so every new reader of a file would
    become an edit to a protected script.
  - A key read inside a listed place, the journal for one, still passes.
  - The network has one way out, but the disk has as many as the engine has
    data.

  It would cost a great deal and still could not tell a key file from a
  journal.
- **A test that runs the provider with a variable set and a key file present,
  and asserts every answer is absent.** Rejected as the check. It proves the
  provider does not read the variable and file the test chose, not that no code
  reads another under a name nobody guessed. It is still worth writing beside
  the gate, and belongs to the provider's task.
- **Hold it in the types: `ApiKey` constructible only at the boundary the shell
  calls.** Rejected for now. Any code holding a `String` can build one, and the
  boundary crate does not exist until the shell's binding is decided.

## Consequences

- **Easier:**
  - The provider's implementation has a gate to land under, which AGENTS.md
    requires.
  - Every engine crate is read for a key from its first commit.
  - A keychain package added once to `denied_packages` is refused to analyze
    and to every other crate together.
  - The spent journal lands where its accepted record put it, with no change
    to that record.
- **Harder:**
  - Any engine code that wants an environment variable needs a record. This
    includes a test reading an opt-in flag, a build script reading a Cargo
    variable other than the two places, and a bin adopting `clap`.
  - So does any engine code that spawns a process.
  - The code holding the key cannot touch the disk. Opening the spent journal
    at its path stays inside `price/spent/`, and the provider is handed an
    open record. A test that names `ApiKey` runs on the in-memory record,
    which `price-allowance-between-sessions.md` already has the provider's
    tests do.
  - A comment in `price/spent/` that names `ApiKey` draws its file in, since
    the matcher reads comments. It is reworded, as `ambient`'s are.
- **Expensive to reverse:** the gate itself. Removing or narrowing it is
  weakening a gate, which `TEMPLATE.md` makes Constitutional. Adding a name, a
  group or a place is cheap.

## Enforcement

This applies an anchor-derived invariant. The gate above is the check, and the
proof above is what shows it holds. What it cannot see, said here rather than
left to be found:

- **Gap: a key read from a file outside the key's places and handed in.** The
  ledger journal, the spent journal, the registry reader, a bin, a test, and
  store at M6 may all read files. Any of them could read a key file and pass
  the string to a caller that builds `ApiKey` from it, and nothing here goes
  red. A name check cannot tell that read from the one each already makes.
  That is the residue of the gap `price-provider.md` named, and it stays
  visible.
- **A key written out rather than read in.** The provider could put the key's
  text in an entry's ticker, and the spent journal would keep it on disk. This
  gate reads for a key arriving. A key leaving is held by `ApiKey` printing no
  part of itself, as `price-provider.md` decides, and otherwise by review.
- **An alias** (`use std::env as e; e::var(…)`, `use std::process::{Command as
  C}; C::new(…)`), and a name spelled with spaces around its `::`. The matcher
  is `ambient`'s, and these are its blind spots too. An alias of `ApiKey` in
  `price/spent/` is one of them.
- **A foreign function block.** `security-framework-sys` already links the
  macOS keychain library into `vfi-fetch` through `native-tls`. So an `extern`
  block declaring `SecItemCopyMatching` would call it with no new package.
  This is not read. It is not the shorter way, and an `unsafe extern` block is
  what a reader sees in a diff.
- **What a dependency does inside.** A package that reads a variable or a
  keychain on its own, such as a price client that falls back to an
  environment variable, is the dependency's code and not the engine's source.
  `price-provider.md` adds no dependency for the provider. A package that does
  this belongs in a `denied_packages` group, which this gate then learns.
- **Source pulled in from outside the crate's directory,** by a `#[path]`
  attribute or a manifest key. The gate reads files by where they sit, as
  `ambient` does.
- **A key passed in argv to a bin.** That is not read, by decision, because
  argv is its caller's parameter.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: a new gate, which the template
  places in the tier the decider accepts flagged for later human review. No
  anchor is edited, no gate is narrowed or removed, no name leaves
  `ambient_names`, no entry leaves `denied_packages`, `allowed_edges` or the
  expected gate set, and the protected-path list is untouched. The gate
  script is protected, so the task that implements this will need the human
  label on its own PR. Flagged for later human review.
- **Checked:**
  - The invariant "Secrets live in the shell, never the engine" and the
    first M5 criterion, which this applies and does not restate.
  - `price-provider.md`, left as accepted: the key stays a constructor
    parameter, sent only as a header and never printed. The gap its
    Enforcement names is the one closed here, in part, with the residue
    stated.
  - `price-allowance-between-sessions.md`, the record #300 contradicted
    without naming. The journal stays in `price/spent/`, append-only, at a
    path it is handed. The three reasons the key cannot reach it from there
    are that record's own words: no key in an entry, the record handed
    `&mut` to the provider and never the reverse, the host checked as a
    string.
  - The table of main's reads, against main at `c706c70`. Every file and
    line is as written. The eight `CARGO_MANIFEST_DIR` and three
    `CARGO_TARGET_TMPDIR` calls are the complete set. No crate read names
    `env::var`, `var_os`, `vars`, `vars_os`, `home_dir`, `env::{`, `env::*`,
    `option_env!`, an `include` macro, `process::Command` or `Command::new`.
    No build script, no `ApiKey`, no `price/` exists. Direct dependencies
    are as stated, and store, jobs and contracts list none.
  - `ambient_names`, `denied_packages`, `expected_gates`, `chokepoint` and
    `wire_names` in the gate script, which are as the record reads them.
  - The six alternatives as argued.
- **Verdict and why:** accepted. The decision that matters is where the file
  way is held. A name check cannot tell a key file from a journal, so the
  record holds it where the key lives and follows the key by its type
  wherever it moves, and that is the only shape that leaves the spent
  journal where an accepted record put it without leaving it a blind spot:
  a file under `spent/` that writes `ApiKey` is drawn back in. Reading
  direct dependencies rather than the resolved tree for the keychain group
  is right, since `native-tls` would otherwise go red on main for a reason
  that is not a key. Two things are left visible and none is settled here:
  - The residue the Enforcement section names first, a key read by any
    file reader outside the key's places and handed in as a string, stands
    open. It is the honest limit of a name check, and the type-boundary
    alternative is the shape that would close it, once a shell binding
    exists.
  - The accept copy and the plants are a proof the implementing task must
    bring in full. A gate that lands with the plants but without the accept
    copy proves it catches and not that it leaves main's own reads alone.
- **What would have changed it:** a read of the environment or the
  filesystem on main that the table missed, since the record's claim that
  main passes would then be false; or a reason the key could reach
  `price/spent/` that the allowance record's interface allows, since the
  carve-out would then be a hole and not a directory the type cannot be
  written in. Neither was found.
