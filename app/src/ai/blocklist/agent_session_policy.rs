//! Pure session routing policy for agent tools (M5 / FACTOR-006).
//!
//! Keeps local / warpified-SSH / legacy-SSH / LRC decisions unit-testable without
//! a PTY or live host. Executors and prompt renderers should align with these
//! rules; this module is the documented source of truth for offline CI.

use super::SessionContext;
use crate::ai::agent::api::RequestParams;

/// Where `run_shell_command` must execute relative to the user's active PTY.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellExecutionTarget {
    /// Local terminal session (or unknown session type).
    Local,
    /// Warpified remote: shell hooks on remote; do not open a nested `ssh`.
    WarpifiedRemote { host_id: Option<String> },
    /// User-typed SSH (legacy): PTY is remote but session_type may still be Local.
    /// Commands run on the remote PTY; never prepend `ssh host …`.
    LegacySsh { host: String, port: String },
}

/// Classify shell tool target from [`SessionContext`] (H4 / H7).
pub fn shell_execution_target(ctx: &SessionContext) -> ShellExecutionTarget {
    if ctx.is_legacy_ssh() {
        let host = ctx
            .ssh_connection_info()
            .and_then(|i| i.host.clone())
            .filter(|h| !h.is_empty())
            .unwrap_or_else(|| "unknown".to_owned());
        let port = ctx
            .ssh_connection_info()
            .and_then(|i| i.port.clone())
            .filter(|p| !p.is_empty())
            .unwrap_or_else(|| "22".to_owned());
        return ShellExecutionTarget::LegacySsh { host, port };
    }
    if ctx.is_remote() {
        return ShellExecutionTarget::WarpifiedRemote {
            host_id: ctx.host_id().map(|h| h.as_str().to_owned()),
        };
    }
    ShellExecutionTarget::Local
}

/// D-08 / H6: local filesystem tools (`read_files`, apply diffs) must not run
/// against the client disk when the session is a warpified remote.
pub fn local_file_tools_forbidden(ctx: &SessionContext) -> bool {
    ctx.is_remote()
}

/// H5: an LRC (long-running command) is attached to this agent request.
/// Prefer `write_to_long_running_shell_command` / `read_shell_command_output`
/// over spawning a new shell (which would miss the remote TUI the user watches).
#[allow(dead_code)] // M5 SSH LRC policy wire-in
pub fn lrc_attached(params: &RequestParams) -> bool {
    params.lrc_running_command.is_some()
        || params
            .lrc_command_id
            .as_ref()
            .is_some_and(|id| !id.is_empty())
}

/// Legacy SSH requires the model to treat the PTY as remote and not nest `ssh`.
#[allow(dead_code)] // M5 SSH LRC policy wire-in
pub fn must_not_nest_ssh(ctx: &SessionContext) -> bool {
    matches!(
        shell_execution_target(ctx),
        ShellExecutionTarget::LegacySsh { .. } | ShellExecutionTarget::WarpifiedRemote { .. }
    )
}

/// Whether agent prompt should inject a legacy-SSH facts block (see chat_stream).
#[allow(dead_code)] // M5 SSH LRC policy wire-in
pub fn needs_legacy_ssh_prompt_block(ctx: &SessionContext) -> bool {
    ctx.is_legacy_ssh()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::agent::{AIAgentInput, UserQueryMode};
    use crate::terminal::model::session::SessionType;
    use crate::terminal::ssh::util::InteractiveSshCommand;
    use warp_core::HostId;

    fn empty_params() -> RequestParams {
        RequestParams::new_for_test(
            vec![AIAgentInput::UserQuery {
                query: "t".into(),
                context: Default::default(),
                static_query_type: None,
                referenced_attachments: Default::default(),
                user_query_mode: UserQueryMode::Normal,
                running_command: None,
                intended_agent: None,
            }],
            vec![],
        )
    }

    #[test]
    fn shell_target_local_default() {
        let ctx = SessionContext::new_for_test().with_session_type(Some(SessionType::Local));
        assert_eq!(shell_execution_target(&ctx), ShellExecutionTarget::Local);
        assert!(!local_file_tools_forbidden(&ctx));
        assert!(!must_not_nest_ssh(&ctx));
        assert!(!needs_legacy_ssh_prompt_block(&ctx));
    }

    #[test]
    fn shell_target_warpified_remote() {
        let host = HostId::new("h1".into());
        let ctx =
            SessionContext::new_for_test().with_session_type(Some(SessionType::WarpifiedRemote {
                host_id: Some(host),
            }));
        assert_eq!(
            shell_execution_target(&ctx),
            ShellExecutionTarget::WarpifiedRemote {
                host_id: Some("h1".into())
            }
        );
        assert!(local_file_tools_forbidden(&ctx));
        assert!(must_not_nest_ssh(&ctx));
        assert!(!needs_legacy_ssh_prompt_block(&ctx));
    }

    #[test]
    fn shell_target_legacy_ssh_overrides_local_session_type() {
        let info = InteractiveSshCommand {
            host: Some("user@box".into()),
            port: Some("2222".into()),
        };
        let ctx = SessionContext::new_for_test()
            .with_session_type(Some(SessionType::Local))
            .with_legacy_ssh(info);
        assert_eq!(
            shell_execution_target(&ctx),
            ShellExecutionTarget::LegacySsh {
                host: "user@box".into(),
                port: "2222".into(),
            }
        );
        // Legacy SSH: session_type is Local so is_remote is false — file tools
        // are NOT blocked by D-08 remote flag (local FS may still be wrong for
        // remote paths; shell still runs on remote PTY). Documented H7 tradeoff.
        assert!(!local_file_tools_forbidden(&ctx));
        assert!(must_not_nest_ssh(&ctx));
        assert!(needs_legacy_ssh_prompt_block(&ctx));
    }

    #[test]
    fn legacy_ssh_defaults_port_and_host() {
        let info = InteractiveSshCommand {
            host: None,
            port: None,
        };
        let ctx = SessionContext::new_for_test().with_legacy_ssh(info);
        assert_eq!(
            shell_execution_target(&ctx),
            ShellExecutionTarget::LegacySsh {
                host: "unknown".into(),
                port: "22".into(),
            }
        );
    }

    #[test]
    fn lrc_attached_from_command_id_or_running() {
        let mut params = empty_params();
        assert!(!lrc_attached(&params));
        params.lrc_command_id = Some(String::new());
        assert!(!lrc_attached(&params));
        params.lrc_command_id = Some("block-42".into());
        assert!(lrc_attached(&params));
    }
}
