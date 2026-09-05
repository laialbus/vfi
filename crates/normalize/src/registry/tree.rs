//! The registry's bytes, read once, and the version that names them.
//!
//! One read, not two. The version is the digest of the bytes the registry was
//! built from, so those bytes are read a single time and both the digest and
//! the reading are taken from the same copy — a version computed by a second
//! walk would name whatever the tree held at the moment of that walk, which is
//! not necessarily what answered the question.
//!
//! The order is fixed by the paths, sorted by their bytes, so one tree always
//! digests one way however the filesystem happens to hand its entries over.
//! Paths are rendered relative to the root the caller gave, which is the
//! repository-relative path the record names less the one directory every entry
//! shares; dropping it is what lets the same tree digest the same wherever it is
//! read from, and it cannot change the order because every path loses the same
//! prefix.
//!
//! Each file enters the digest as its path, a zero byte, its length, and then
//! its bytes. The path is in because a byte moved from one file to another is a
//! different tree, and the length is in because without it the boundary between
//! two files is guesswork and two trees could stream the same way.

use std::fs;
use std::path::Path;

use super::sha256::Sha256;

pub(super) struct File {
    pub(super) path: String,
    pub(super) bytes: Vec<u8>,
}

pub(super) struct Tree {
    files: Vec<File>,
}

impl Tree {
    /// Every file under `root`, in the order their paths sort in.
    pub(super) fn read(root: &Path) -> Result<Tree, String> {
        let mut files = Vec::new();
        collect(root, "", &mut files)?;
        files.sort_by(|one, other| one.path.cmp(&other.path));
        Ok(Tree { files })
    }

    pub(super) fn digest(&self) -> [u8; 32] {
        let mut taken = Sha256::new();
        for file in &self.files {
            taken.write(file.path.as_bytes());
            taken.write(&[0]);
            taken.write(&(file.bytes.len() as u64).to_be_bytes());
            taken.write(&file.bytes);
        }
        taken.finish()
    }

    /// The files whose path lies directly inside `directory`, and the paths
    /// under it that are not files of that shape — a nested one, or one whose
    /// name is not a `.toml`. The second list is refused by the caller rather
    /// than passed over: a file inside a half of the registry that nothing reads
    /// is one whose bytes moved the version and whose content moved nothing.
    ///
    /// A file under `registry/` that is in neither half is a different thing and
    /// is left alone here, as it is by the gate over the data: it is not a
    /// misplaced rule, it is not read, and the version names it because the
    /// version names the tree.
    pub(super) fn inside<'t>(&'t self, directory: &str) -> (Vec<&'t File>, Vec<&'t str>) {
        let mut held = Vec::new();
        let mut misplaced = Vec::new();

        for file in &self.files {
            let Some(rest) = file.path.strip_prefix(directory) else {
                continue;
            };
            if rest.contains('/') || !rest.ends_with(".toml") || rest.len() == ".toml".len() {
                misplaced.push(file.path.as_str());
            } else {
                held.push(file);
            }
        }

        (held, misplaced)
    }
}

impl File {
    /// The file's stem: what is left of its name once the directory it sits in
    /// and the extension every file here carries are taken off.
    pub(super) fn stem<'f>(&'f self, directory: &str) -> &'f str {
        self.path
            .strip_prefix(directory)
            .and_then(|name| name.strip_suffix(".toml"))
            .expect("a file this is asked about is one `inside` returned")
    }

    /// The file as text, or nothing where its bytes are not text at all.
    pub(super) fn text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }
}

fn collect(at: &Path, prefix: &str, into: &mut Vec<File>) -> Result<(), String> {
    let entries =
        fs::read_dir(at).map_err(|why| format!("{}: cannot be read ({why})", at.display()))?;

    for entry in entries {
        let entry =
            entry.map_err(|why| format!("{}: an entry cannot be read ({why})", at.display()))?;
        let path = entry.path();

        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            return Err(format!(
                "{}: is named in bytes that are not text, and the registry is read by path",
                path.display()
            ));
        };
        let named = format!("{prefix}{name}");

        if path.is_dir() {
            collect(&path, &format!("{named}/"), into)?;
            continue;
        }

        let bytes =
            fs::read(&path).map_err(|why| format!("{}: cannot be read ({why})", path.display()))?;
        into.push(File { path: named, bytes });
    }

    Ok(())
}
