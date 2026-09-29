//! What a fixture's `claims` can prove against its `expected`, as
//! `docs/adr/the-harness-checks-the-claims.md` decides it: that a claimed line
//! is in `expected`, that a group is consecutive there, that a listing is in
//! `expected`'s order, and that a group sits under the period an anchor names,
//! in the order written. The prose around those lines is the decider's to
//! re-derive; nothing here reads it.
//!
//! Only `golden.rs` declares this module. Every test target includes
//! `tests/fixture/`, so a case there would also run under `cargo test`, and a
//! false claim would turn the tests gate red as well as the fixtures gate.
//!
//! It reads the two texts it is handed and nothing else: it runs no stage,
//! builds no claim from `expected`, and imports nothing from the crate. The
//! word `registry` in a claim is matched as that word, because that is how the
//! stage renders the registry a fixture runs under; matching through the
//! stage's constant would let the check follow the engine.

use std::collections::HashMap;

mod planted;

/// One claim that does not hold, or one `claims` that holds none. `line` is the
/// claims line number, counted from one; the two failures that are about the
/// file as a whole have none.
pub struct Failure {
    pub line: Option<usize>,
    pub text: String,
    pub reason: String,
}

impl Failure {
    fn at(line: &Line, reason: impl Into<String>) -> Failure {
        Failure {
            line: Some(line.number),
            text: line.text.to_owned(),
            reason: reason.into(),
        }
    }

    fn whole(reason: impl Into<String>) -> Failure {
        Failure {
            line: None,
            text: String::new(),
            reason: reason.into(),
        }
    }
}

/// Every failure in one fixture's `claims`, in the order its lines are written.
/// `None` is a fixture directory with no `claims`.
pub fn check(claims: Option<&str>, expected: &str) -> Vec<Failure> {
    let Some(claims) = claims else {
        return vec![Failure::whole(
            "the fixture holds no claims, so nothing it records has been derived",
        )];
    };

    let (blocks, mut failures) = read(claims);
    if !blocks.iter().any(|block| matches!(block, Block::Group(_))) {
        failures.push(Failure::whole(
            "the claims hold no line beginning `| `, so they would pass over nothing",
        ));
        return failures;
    }

    let expected = Expected::new(expected);
    let mut scope: Option<Scope> = None;

    for block in &blocks {
        let group = match block {
            Block::End => {
                scope = None;
                continue;
            }
            Block::Group(group) => group,
        };

        if group.len() > 1 && group.iter().all(|line| is_period(line.text)) {
            scope = None;
            listing(group, &expected, &mut failures);
            continue;
        }

        let whole = absent(group, &expected, &mut failures);

        if is_period(group[0].text) {
            scope = Some(anchor(group, whole, &expected, &mut failures));
            continue;
        }
        if !whole {
            continue;
        }

        match scope.as_mut() {
            Some(Scope::Placing { anchor, next, end }) => match expected.after(group, *next) {
                Some(at) if at + group.len() <= *end => *next = at + group.len(),
                _ if expected.anywhere(group).is_none() => {
                    failures.push(Failure::at(&group[0], NOT_CONSECUTIVE));
                }
                _ => failures.push(Failure::at(
                    &group[0],
                    format!(
                        "placed under {:?} (line {}), and not there after the group before it \
                             and before the next period line",
                        anchor.text, anchor.number
                    ),
                )),
            },
            Some(Scope::Unplaceable) | None => {
                if expected.anywhere(group).is_none() {
                    failures.push(Failure::at(&group[0], NOT_CONSECUTIVE));
                }
            }
        }
    }

    failures.sort_by_key(|failure| failure.line);
    failures
}

const NOT_CONSECUTIVE: &str = "the group this line begins is not consecutive in expected";

/// Each line of a listing after the one before it. Consecutiveness is not
/// asked: every period line in `expected` is followed by its concept lines, so
/// no two are ever adjacent.
fn listing(group: &[Line], expected: &Expected, failures: &mut Vec<Failure>) {
    let mut next = 0;
    for line in group {
        match expected.first_from(line.text, next) {
            Some(at) => next = at + 1,
            None if expected.holds(line.text) => {
                failures.push(Failure::at(
                    line,
                    "a listing line not after the one before it in expected",
                ));
            }
            None => failures.push(Failure::at(line, "not a line of expected")),
        }
    }
}

/// Each line of the group that `expected` does not hold, as a failure of its
/// own. True when there is none, so that the group can be looked for whole.
fn absent(group: &[Line], expected: &Expected, failures: &mut Vec<Failure>) -> bool {
    let mut whole = true;
    for line in group {
        if !expected.holds(line.text) {
            failures.push(Failure::at(line, "not a line of expected"));
            whole = false;
        }
    }
    whole
}

/// The scope an anchor opens. `whole` is whether `expected` holds every line of
/// it, which `absent` has already said where it does not.
fn anchor<'a>(
    group: &'a [Line<'a>],
    whole: bool,
    expected: &Expected,
    failures: &mut Vec<Failure>,
) -> Scope<'a> {
    let period = &group[0];
    let at = match expected.positions(period.text) {
        [] => return Scope::Unplaceable,
        [at] => *at,
        many => {
            failures.push(Failure::at(
                period,
                format!(
                    "an anchor must name one period, and expected writes this line {} times",
                    many.len()
                ),
            ));
            if whole && expected.anywhere(group).is_none() {
                failures.push(Failure::at(period, NOT_CONSECUTIVE));
            }
            return Scope::Unplaceable;
        }
    };

    let next = if whole && expected.consecutive_at(group, at) {
        at + group.len()
    } else {
        if whole {
            failures.push(Failure::at(period, NOT_CONSECUTIVE));
        }
        at + 1
    };

    Scope::Placing {
        anchor: period,
        next,
        end: expected.next_period(at),
    }
}

enum Scope<'a> {
    /// Groups after this anchor must lie at or after `next` and before `end`,
    /// the index of the period line that follows the anchor's.
    Placing {
        anchor: &'a Line<'a>,
        next: usize,
        end: usize,
    },
    /// An anchor that names no one period places nothing: the groups under it
    /// are held only to what any group is, and the anchor is already red.
    Unplaceable,
}

/// In `expected`, only a period line begins this way.
fn is_period(text: &str) -> bool {
    text.starts_with("at ") || text.starts_with("from ")
}

struct Line<'a> {
    number: usize,
    text: &'a str,
}

enum Block<'a> {
    Group(Vec<Line<'a>>),
    /// A line beginning `#`: it ends an anchor's scope.
    End,
}

/// The groups of `claims` and the scope ends between them. A group is a
/// maximal run of claimed lines; any other line ends it, a blank one included.
fn read(claims: &str) -> (Vec<Block<'_>>, Vec<Failure>) {
    let mut blocks = Vec::new();
    let mut failures = Vec::new();
    let mut group = Vec::new();

    for (index, text) in lines(claims).into_iter().enumerate() {
        let line = Line {
            number: index + 1,
            text,
        };
        if let Some(claimed) = text.strip_prefix("| ") {
            group.push(Line {
                number: line.number,
                text: claimed,
            });
            continue;
        }
        if !group.is_empty() {
            blocks.push(Block::Group(std::mem::take(&mut group)));
        }
        if text.starts_with('|') {
            // Read as prose, it would drop a claim out of the check unseen.
            failures.push(Failure::at(
                &line,
                "begins `|` without the space a claimed line has",
            ));
        } else if text.starts_with('#') {
            blocks.push(Block::End);
        }
    }
    if !group.is_empty() {
        blocks.push(Block::Group(group));
    }

    (blocks, failures)
}

/// Split on `\n` alone, so a trailing `\r` or space stays part of the line and
/// has to match byte for byte.
fn lines(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    text.strip_suffix('\n')
        .unwrap_or(text)
        .split('\n')
        .collect()
}

struct Expected<'a> {
    lines: Vec<&'a str>,
    at: HashMap<&'a str, Vec<usize>>,
}

impl<'a> Expected<'a> {
    fn new(text: &'a str) -> Expected<'a> {
        let lines = lines(text);
        let mut at: HashMap<&str, Vec<usize>> = HashMap::new();
        for (index, line) in lines.iter().enumerate() {
            at.entry(line).or_default().push(index);
        }
        Expected { lines, at }
    }

    fn positions(&self, line: &str) -> &[usize] {
        self.at.get(line).map_or(&[], Vec::as_slice)
    }

    fn holds(&self, line: &str) -> bool {
        !self.positions(line).is_empty()
    }

    fn first_from(&self, line: &str, from: usize) -> Option<usize> {
        self.positions(line).iter().copied().find(|&at| at >= from)
    }

    fn consecutive_at(&self, group: &[Line], at: usize) -> bool {
        group
            .iter()
            .enumerate()
            .all(|(offset, line)| self.lines.get(at + offset) == Some(&line.text))
    }

    /// The first place at or after `from` where the whole group stands.
    fn after(&self, group: &[Line], from: usize) -> Option<usize> {
        self.positions(group[0].text)
            .iter()
            .copied()
            .find(|&at| at >= from && self.consecutive_at(group, at))
    }

    fn anywhere(&self, group: &[Line]) -> Option<usize> {
        self.after(group, 0)
    }

    /// The index of the first period line after `at`, or the end of `expected`.
    fn next_period(&self, at: usize) -> usize {
        (at + 1..self.lines.len())
            .find(|&index| is_period(self.lines[index]))
            .unwrap_or(self.lines.len())
    }
}
