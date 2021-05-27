mod path {
    use std::path::PathBuf;

    #[test]
    fn from_git_dir() -> crate::Result {
        let dir = repo_path()?.join(".git");
        assert_eq!(
            git_repository::discover::path(&dir)?.canonicalize()?,
            dir,
            "the .git dir is directly returned if valid"
        );
        Ok(())
    }

    #[test]
    fn from_working_dir() -> crate::Result {
        let dir = repo_path()?;
        assert_eq!(
            git_repository::discover::path(&dir)?.canonicalize()?,
            dir.join(".git"),
            "a working tree dir yields the git dir"
        );
        Ok(())
    }

    #[test]
    fn from_nested_dir() -> crate::Result {
        let working_dir = repo_path()?;
        let dir = working_dir.join("some/very/deeply/nested/subdir");
        assert_eq!(
            git_repository::discover::path(&dir)?.canonicalize()?,
            working_dir.join(".git"),
            "a working tree dir yields the git dir"
        );
        Ok(())
    }

    fn repo_path() -> crate::Result<PathBuf> {
        git_testtools::scripted_fixture_repo_read_only("make_basic_repo.sh")
    }
}