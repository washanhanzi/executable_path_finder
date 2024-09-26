//from rust-analyzer/crates/toolchain/src/lib.rs

use std::{env, iter, path::PathBuf};

use camino::Utf8PathBuf;

pub mod home;

/// find return a `PathBuf` for the given executable, it tries to find it in PATH and environment variables.
///
/// The current implementation checks three places for an executable to use:
/// 1) $PATH/`<executable_name>`
///      example: for cargo, this tries all paths in $PATH with appended `cargo`, returning the
///      first that exists
/// 2) Appropriate environment variable (erroring if this is set but not a usable executable)
///      example: for cargo, this checks $CARGO environment variable; for rustc, $RUSTC; etc
pub fn find(exec: &str) -> Option<Utf8PathBuf> {
    find_in_path(exec).or_else(|| find_in_env(exec))
}

/// find_with_cargo_home return a `PathBuf` for the given executable, it tries to find it in PATH, environment variables and CARGO_HOME.
///
/// The current implementation checks three places for an executable to use:
/// 1) $PATH/`<executable_name>`
///      example: for cargo, this tries all paths in $PATH with appended `cargo`, returning the
///      first that exists
/// 2) Appropriate environment variable (erroring if this is set but not a usable executable)
///      example: for cargo, this checks $CARGO environment variable; for rustc, $RUSTC; etc
/// 3) `$CARGO_HOME/bin/<executable_name>`
///      where $CARGO_HOME defaults to ~/.cargo (see <https://doc.rust-lang.org/cargo/guide/cargo-home.html>)
///      example: for cargo, this tries $CARGO_HOME/bin/cargo, or ~/.cargo/bin/cargo if $CARGO_HOME is unset.
///      It seems that this is a reasonable place to try for cargo, rustc, and rustup
pub fn find_with_cargo_home(exec: &str) -> Option<Utf8PathBuf> {
    find_in_path(exec)
        .or_else(|| find_in_env(exec))
        .or_else(|| find_in_cargo_home(exec))
}

pub fn find_in_cargo_home(exec: &str) -> Option<Utf8PathBuf> {
    let mut path = get_cargo_home()?;
    path.push("bin");
    path.push(exec);
    probe_for_binary(path)
}

pub fn find_in_env(exec: &str) -> Option<Utf8PathBuf> {
    env::var_os(exec.to_ascii_uppercase())
        .map(PathBuf::from)
        .map(Utf8PathBuf::try_from)
        .and_then(Result::ok)
}

pub fn find_in_path(exec: &str) -> Option<Utf8PathBuf> {
    let paths = env::var_os("PATH").unwrap_or_default();
    env::split_paths(&paths)
        .map(|path| path.join(exec))
        .map(PathBuf::from)
        .map(Utf8PathBuf::try_from)
        .filter_map(Result::ok)
        .find_map(probe_for_binary)
}

pub fn probe_for_binary(path: Utf8PathBuf) -> Option<Utf8PathBuf> {
    let with_extension = match env::consts::EXE_EXTENSION {
        "" => None,
        it => Some(path.with_extension(it)),
    };
    iter::once(path)
        .chain(with_extension)
        .find(|it| it.is_file())
}

fn get_cargo_home() -> Option<Utf8PathBuf> {
    if let Some(path) = env::var_os("CARGO_HOME") {
        return Utf8PathBuf::try_from(PathBuf::from(path)).ok();
    }

    if let Some(mut path) = home::home_dir() {
        path.push(".cargo");
        return Utf8PathBuf::try_from(path).ok();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_in_path() {
        let temp_dir = TempDir::new().unwrap();
        let fake_bin = temp_dir.path().join("fake-binary");
        fs::write(&fake_bin, "").unwrap();

        let old_path = env::var_os("PATH");
        env::set_var("PATH", temp_dir.path());

        let expected_path = Utf8PathBuf::try_from(fake_bin).unwrap();
        assert_eq!(find_in_path("fake-binary"), Some(expected_path));
        assert_eq!(find_in_path("non-existent-binary"), None);

        // Restore the original PATH
        if let Some(path) = old_path {
            env::set_var("PATH", path);
        } else {
            env::remove_var("PATH");
        }
    }
    #[test]
    fn test_find_in_env() {
        env::set_var("TESTEXEC", "/path/to/testexec");
        assert!(find_in_env("testexec").is_some());
        assert!(find_in_env("nonexistent").is_none());
        env::remove_var("TESTEXEC");
    }

    #[test]
    fn test_find_with_cargo_home() {
        let temp_dir = TempDir::new().unwrap();
        let fake_cargo_home = temp_dir.path().join(".cargo");
        fs::create_dir_all(fake_cargo_home.join("bin")).unwrap();
        let fake_bin = fake_cargo_home.join("bin").join("fake-cargo-binary");
        fs::write(&fake_bin, "").unwrap();

        env::set_var("CARGO_HOME", fake_cargo_home);

        assert!(find_with_cargo_home("fake-cargo-binary").is_some());
        assert!(find_with_cargo_home("non-existent-binary").is_none());

        env::remove_var("CARGO_HOME");
    }

    #[test]
    fn test_probe_for_binary() {
        let temp_dir = TempDir::new().unwrap();
        let fake_bin = temp_dir.path().join("fake-binary");
        fs::write(&fake_bin, "").unwrap();

        assert!(probe_for_binary(Utf8PathBuf::try_from(fake_bin).unwrap()).is_some());
        assert!(probe_for_binary(
            Utf8PathBuf::try_from(temp_dir.path().join("non-existent")).unwrap()
        )
        .is_none());
    }
}
