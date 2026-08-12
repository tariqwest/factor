//! Offline checks that shell bootstrap assets for warpify still ship expected hooks.
//! M5 H1–H2: full preexec PTY behavior is manual; presence + markers are unit-testable.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    fn bootstrap_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/bundled/bootstrap")
    }

    fn read(name: &str) -> String {
        let path = bootstrap_dir().join(name);
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    }

    #[test]
    fn bootstrap_shell_scripts_exist() {
        let dir = bootstrap_dir();
        for name in [
            "bash.sh",
            "bash_init_shell.sh",
            "zsh.sh",
            "zsh_init_shell.sh",
            "fish.sh",
            "fish_init_shell.sh",
            "pwsh.ps1",
            "pwsh_init_shell.ps1",
        ] {
            let p = dir.join(name);
            assert!(p.is_file(), "missing bootstrap asset {}", p.display());
        }
    }

    #[test]
    fn bash_and_zsh_init_shell_carry_session_id_and_initshell_hook() {
        for name in ["bash_init_shell.sh", "zsh_init_shell.sh"] {
            let body = read(name);
            assert!(
                body.contains("WARP_SESSION_ID") || body.contains("session_id"),
                "{name} must stamp session_id for warpify"
            );
            assert!(
                body.contains("InitShell") || body.contains("InitShell"),
                "{name} must emit InitShell hook"
            );
        }
    }

    #[test]
    fn fish_init_shell_present_and_nonempty() {
        let body = read("fish_init_shell.sh");
        assert!(body.len() > 40, "fish_init_shell.sh looks empty");
        assert!(
            body.contains("session") || body.contains("InitShell") || body.contains("9278"),
            "fish init should reference session or DCS hook"
        );
    }

    #[test]
    fn subshell_bootstrap_markers_exist() {
        for name in [
            "bash_init_subshell.sh",
            "zsh_init_subshell.sh",
            "fish_init_subshell.sh",
        ] {
            let body = read(name);
            assert!(
                !body.trim().is_empty(),
                "{name} missing for remote/subshell warpify"
            );
        }
    }
}
