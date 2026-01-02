//! cherry picking

use super::{CommitId, RepoPath};
use crate::{
	error::Result,
	sync::{repository::repo, utils::read_file},
};
use scopetime::scope_time;

const GIT_CHERRY_PICK_HEAD_FILE: &str = "CHERRY_PICK_HEAD";

///
pub fn cherry_pick_commit(
	repo_path: &RepoPath,
	commit: CommitId,
) -> Result<()> {
	scope_time!("cherry_pick");

	let repo = repo(repo_path)?;

	let commit = repo.find_commit(commit.into())?;

	repo.cherrypick(&commit, None)?;

	Ok(())
}

///
pub fn cherry_pick_head(repo_path: &RepoPath) -> Result<CommitId> {
	scope_time!("cherry_pick_head");

	let path =
		repo(repo_path)?.path().join(GIT_CHERRY_PICK_HEAD_FILE);

	let file_content = read_file(&path)?;

	let id = git2::Oid::from_str(file_content.trim())?;

	Ok(id.into())
}
