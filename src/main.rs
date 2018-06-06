extern crate failure;
extern crate failure_tools;
extern crate git2;

use failure::{Error, ResultExt};
use failure_tools::ok_or_exit;
use std::io::{stdout, Write};

fn run() -> Result<(), Error> {
    let repo = git2::Repository::open(".")?;
    let mut walker = repo.revwalk()?;
    walker.push_head()?;

    let (mut unknowns, mut anys, mut commits, mut trees, mut blobs, mut tags) = (0, 0, 0, 0, 0, 0);
    for oid in walker {
        use git2::ObjectType::*;
        let object = repo.find_object(oid?, None)?;
        match object.kind() {
            Some(Tag) => tags += 1,
            Some(Commit) => commits += 1,
            Some(Tree) => trees += 1,
            Some(Blob) => blobs += 1,
            Some(Any) => anys += 1,
            None => unknowns += 1,
        }
    }
    writeln!(
        stdout(),
        "commits: {}, trees: {}, blobs: {}, tags: {}, any: {}, unknown: {}",
        commits,
        trees,
        blobs,
        tags,
        anys,
        unknowns
    ).map_err(Into::into)
}

fn main() {
    ok_or_exit(run().with_context(|_| "Failed to count git objects"))
}
