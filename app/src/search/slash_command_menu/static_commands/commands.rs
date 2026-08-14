use std::{collections::HashMap, sync::LazyLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp_core::features::FeatureFlag;

use crate::search::slash_command_menu::{static_commands::Argument, StaticCommand};

use super::Availability;

pub static AGENT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/agent",
    description: "Start a new conversation",
    icon_path: "bundled/svg/factor.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

pub const ADD_MCP: StaticCommand = StaticCommand {
    name: "/add-mcp",
    description: "Add new MCP server",
    icon_path: "bundled/svg/dataflow.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const PR_COMMENTS: StaticCommand = StaticCommand {
    name: "/pr-comments",
    description: "Pull GitHub PR review comments",
    icon_path: "bundled/svg/github.svg",
    availability: Availability::REPOSITORY.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

/// Removed from Factor product surface (local-first northstar / explicit drop).
#[allow(dead_code)]
pub const CREATE_DOCKER_SANDBOX: StaticCommand = StaticCommand {
    name: "/docker-sandbox",
    description: "Create a new docker sandbox terminal session",
    icon_path: "bundled/svg/docker.svg",
    availability: Availability::LOCAL,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static CREATE_NEW_PROJECT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/create-new-project",
    description: "Use the agent to create a new coding project",
    icon_path: "bundled/svg/plus.svg",
    availability: Availability::LOCAL | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::required().with_hint_text("<describe what you want to build>")),
});

pub static EDIT_SKILL: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/open-skill",
    description: "Open a skill's markdown file in the built-in editor",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub static INVOKE_SKILL: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/skills",
    description: "Invoke a skill",
    icon_path: "bundled/svg/stars-01.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub static ADD_PROMPT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/add-prompt",
    description: "Add new Agent prompt",
    icon_path: if FeatureFlag::AgentView.is_enabled() {
        "bundled/svg/prompt.svg"
    } else {
        "bundled/svg/agentmode.svg"
    },
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub const ADD_RULE: StaticCommand = StaticCommand {
    name: "/add-rule",
    description: "Add a new global rule for the agent",
    icon_path: "bundled/svg/book-open.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static EDIT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/open-file",
    description: "Open a file in the built-in code editor",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::LOCAL,
    auto_enter_ai_mode: false,
    argument: Some(
        Argument::optional().with_hint_text("<path/to/file[:line[:col]]> or \"@\" to search"),
    ),
});

pub static RENAME_TAB: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/rename-tab",
    description: "Rename the current tab",
    icon_path: "bundled/svg/pencil-line.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text("<tab name>")),
});

/// Rename the active agent conversation (display title), not the shell tab.
pub static RENAME_CONVERSATION: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/rename-conversation",
    description: "Rename the active agent conversation",
    icon_path: "bundled/svg/pencil-line.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text("<conversation name>")),
});

pub static FORK: LazyLock<StaticCommand> = LazyLock::new(|| {
    let hint_text = "<optional prompt to send in forked conversation>";
    StaticCommand {
        name: "/fork",
        description: "Fork the current conversation in a new pane or a new tab",
        icon_path: "bundled/svg/arrow-split.svg",
        availability: Availability::AGENT_VIEW
            | Availability::ACTIVE_CONVERSATION
            | Availability::NO_LRC_CONTROL
            | Availability::AI_ENABLED,
        auto_enter_ai_mode: true,
        argument: Some(Argument::optional().with_hint_text(hint_text)),
    }
});

pub const OPEN_CODE_REVIEW: StaticCommand = StaticCommand {
    name: "/open-code-review",
    description: "Open code review",
    icon_path: "bundled/svg/diff.svg",
    availability: Availability::REPOSITORY,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const INDEX: StaticCommand = StaticCommand {
    name: "/index",
    description: "Index this codebase",
    icon_path: "bundled/svg/find-all.svg",
    availability: Availability::REPOSITORY
        .union(Availability::CODEBASE_CONTEXT)
        .union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const INIT: StaticCommand = StaticCommand {
    name: "/init",
    description: "Index this codebase and generate an AGENTS.md file",
    icon_path: "bundled/svg/factor-2.svg",
    availability: Availability::REPOSITORY
        .union(Availability::AGENT_VIEW)
        .union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

pub const OPEN_PROJECT_RULES: StaticCommand = StaticCommand {
    name: "/open-project-rules",
    description: "Open the project rules file (AGENTS.md)",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::REPOSITORY.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_MCP_SERVERS: StaticCommand = StaticCommand {
    name: "/open-mcp-servers",
    description: "Open MCP servers",
    icon_path: "bundled/svg/dataflow.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_SETTINGS_FILE: StaticCommand = StaticCommand {
    name: "/open-settings-file",
    description: "Open settings file (TOML)",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::LOCAL,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_REPO: StaticCommand = StaticCommand {
    name: "/open-repo",
    description: "Switch to another indexed repository",
    icon_path: "bundled/svg/folder.svg",
    availability: Availability::LOCAL.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_RULES: StaticCommand = StaticCommand {
    name: "/open-rules",
    description: "View all of your global and project rules",
    icon_path: "bundled/svg/book-open.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static NEW: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/new",
    description: "Start a new conversation (alias for /agent)",
    icon_path: "bundled/svg/new-conversation.svg",
    availability: Availability::NO_LRC_CONTROL | Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

pub static MODEL: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/model",
    description: "Switch the base agent model",
    icon_path: "bundled/svg/factor.svg",
    availability: Availability::AGENT_VIEW | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: None,
});

pub static PROFILE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/profile",
    description: "Switch the active execution profile",
    icon_path: "bundled/svg/psychology.svg",
    availability: Availability::AGENT_VIEW | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: None,
});

pub const PLAN_NAME: &str = "/plan";

pub static PLAN: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: PLAN_NAME,
    description: "Prompt the agent to do some research and create a plan for a task",
    icon_path: "bundled/svg/file-06.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<describe your task>")),
});

pub const ORCHESTRATE_NAME: &str = "/orchestrate";

pub static ORCHESTRATE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: ORCHESTRATE_NAME,
    description: "Break a task into subtasks and run them in parallel with multiple agents",
    icon_path: "bundled/svg/factor.svg",
    availability: Availability::LOCAL | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<describe your task>")),
});

/// If `query` starts with the given command `name` followed by a space,
/// returns the remainder of the query. Otherwise returns `None`.
pub fn strip_command_prefix(query: &str, name: &str) -> Option<String> {
    query
        .strip_prefix(name)
        .and_then(|rest| rest.strip_prefix(' '))
        .map(|rest| rest.to_string())
}

pub static COMPACT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/compact",
    description: "Free up context by summarizing convo history",
    icon_path: "bundled/svg/collapse_content.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::NO_LRC_CONTROL
        | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(
        Argument::optional().with_hint_text("<optional custom summarization instructions>"),
    ),
});

pub static COMPACT_AND: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/compact-and",
    description: "Compact conversation and then send a follow-up prompt",
    icon_path: "bundled/svg/collapse_content.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::NO_LRC_CONTROL
        | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<prompt to send after compaction>")),
});

pub static QUEUE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/queue",
    description: "Queue a prompt to send after the agent finishes responding",
    icon_path: "bundled/svg/clock-plus.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::NO_LRC_CONTROL
        | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::required().with_hint_text("<prompt to send when agent is done>")),
});

pub static FORK_AND_COMPACT: LazyLock<StaticCommand> = LazyLock::new(|| {
    let hint_text = "<optional prompt to send after compaction>";
    StaticCommand {
        name: "/fork-and-compact",
        description: "Fork current conversation and compact it in the forked copy",
        icon_path: "bundled/svg/fork_and_compact.svg",
        availability: Availability::AGENT_VIEW
            | Availability::ACTIVE_CONVERSATION
            | Availability::NO_LRC_CONTROL
            | Availability::AI_ENABLED,
        auto_enter_ai_mode: true,
        argument: Some(Argument::optional().with_hint_text(hint_text)),
    }
});

pub const FORK_FROM: StaticCommand = StaticCommand {
    name: "/fork-from",
    description: "Fork conversation from a specific query",
    icon_path: "bundled/svg/arrow-split.svg",
    availability: Availability::AGENT_VIEW
        .union(Availability::NO_LRC_CONTROL)
        .union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

pub const CONVERSATIONS: StaticCommand = StaticCommand {
    name: "/conversations",
    description: "Open conversation history",
    icon_path: "bundled/svg/conversation.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static PROMPTS: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/prompts",
    description: "Search saved prompts",
    icon_path: "bundled/svg/prompt.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub const REWIND: StaticCommand = StaticCommand {
    name: "/rewind",
    description: "Rewind to a previous point in the conversation",
    icon_path: "bundled/svg/clock-rewind.svg",
    availability: Availability::AGENT_VIEW.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

pub const EXPORT_TO_CLIPBOARD: StaticCommand = StaticCommand {
    name: "/export-to-clipboard",
    description: "Export current conversation to clipboard in markdown format",
    icon_path: "bundled/svg/copy.svg",
    availability: Availability::AGENT_VIEW.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

pub static EXPORT_TO_FILE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/export-to-file",
    description: "Export current conversation to a markdown file",
    icon_path: "bundled/svg/download-01.svg",
    availability: Availability::AGENT_VIEW | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<optional filename>")),
});

pub struct CommandRegistry {
    registry: std::sync::Mutex<Option<Registry>>,
}

impl CommandRegistry {
    pub const fn new() -> Self {
        Self {
            registry: std::sync::Mutex::new(None),
        }
    }

    fn with_registry<T>(&self, f: impl FnOnce(&Registry) -> T) -> T {
        let mut guard = self.registry.lock().unwrap();
        if guard.is_none() {
            *guard = Some(Registry::new());
        }
        f(guard.as_ref().unwrap())
    }

    pub fn all_commands(&self) -> Vec<StaticCommand> {
        self.with_registry(|r| r.all_commands().cloned().collect())
    }

    pub fn all_commands_by_id(&self) -> Vec<(SlashCommandId, StaticCommand)> {
        self.with_registry(|r| {
            r.all_commands_by_id()
                .map(|(id, command)| (id, command.clone()))
                .collect()
        })
    }

    pub fn get_command(&self, id: &SlashCommandId) -> Option<StaticCommand> {
        self.with_registry(|r| r.get_command(id).cloned())
    }

    pub fn get_command_with_name(&self, name: &str) -> Option<StaticCommand> {
        self.with_registry(|r| r.get_command_with_name(name).cloned())
    }

    #[cfg(test)]
    pub fn get_command_id_with_name(&self, name: &str) -> Option<SlashCommandId> {
        self.with_registry(|r| r.get_command_id_with_name(name).copied())
    }

    #[cfg(test)]
    pub fn reset(&self) {
        *self.registry.lock().unwrap() = None;
    }
}

pub static COMMAND_REGISTRY: CommandRegistry = CommandRegistry::new();

/// A unique identifier for a static slash command.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SlashCommandId(Uuid);

impl SlashCommandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SlashCommandId {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Registry {
    commands: HashMap<SlashCommandId, StaticCommand>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        for command in all_commands().into_iter() {
            debug_assert!(
                !command
                    .availability
                    .contains(Availability::TERMINAL_VIEW | Availability::AGENT_VIEW),
                "command `{}` sets both TERMINAL_VIEW and AGENT_VIEW, which is unsatisfiable",
                command.name,
            );
            commands.insert(SlashCommandId::new(), command);
        }
        Self { commands }
    }

    pub fn all_commands_by_id(&self) -> impl Iterator<Item = (SlashCommandId, &StaticCommand)> {
        self.commands.iter().map(|(id, cmd)| (*id, cmd))
    }

    pub fn all_commands(&self) -> impl Iterator<Item = &StaticCommand> {
        self.commands.values()
    }

    pub fn get_command(&self, id: &SlashCommandId) -> Option<&StaticCommand> {
        self.commands.get(id)
    }

    pub fn get_command_with_name(&self, name: &str) -> Option<&StaticCommand> {
        self.commands.values().find(|command| command.name == name)
    }

    #[cfg(test)]
    pub fn get_command_id_with_name(&self, name: &str) -> Option<&SlashCommandId> {
        self.commands
            .iter()
            .find(|(_, command)| command.name == name)
            .map(|(id, _)| id)
    }
}

pub(crate) fn all_commands() -> Vec<StaticCommand> {
    let mut commands = vec![
        ADD_MCP,
        ADD_PROMPT.clone(),
        ADD_RULE,
        INDEX,
        INIT,
        OPEN_PROJECT_RULES,
        OPEN_MCP_SERVERS,
        OPEN_RULES,
        AGENT.clone(),
        NEW.clone(),
        PLAN.clone(),
        RENAME_TAB.clone(),
        RENAME_CONVERSATION.clone(),
        CONVERSATIONS,
        EXPORT_TO_CLIPBOARD,
        MODEL.clone(),
    ];

    // Warper approach: register when LocalDockerSandbox is enabled (dogfood).
    if FeatureFlag::LocalDockerSandbox.is_enabled() {
        commands.push(CREATE_DOCKER_SANDBOX);
    }

    if FeatureFlag::AgentView.is_enabled() {
        commands.push(PROMPTS.clone());
    }

    commands.push(OPEN_CODE_REVIEW);

    if FeatureFlag::CreateProjectFlow.is_enabled() {
        commands.push(CREATE_NEW_PROJECT.clone());
    }

    if FeatureFlag::SummarizationConversationCommand.is_enabled() {
        commands.push(COMPACT.clone());
        commands.push(COMPACT_AND.clone());
    }

    if FeatureFlag::QueueSlashCommand.is_enabled() {
        commands.push(QUEUE.clone());
    }

    commands.extend([FORK.clone(), FORK_AND_COMPACT.clone()]);

    if FeatureFlag::ForkFromCommand.is_enabled() {
        commands.push(FORK_FROM);
    }

    commands.extend([EDIT.clone(), EXPORT_TO_FILE.clone()]);

    if FeatureFlag::ListSkills.is_enabled() {
        commands.push(EDIT_SKILL.clone());
        commands.push(INVOKE_SKILL.clone());
    }

    if FeatureFlag::PRCommentsSlashCommand.is_enabled()
        && !FeatureFlag::PRCommentsSkill.is_enabled()
    {
        commands.push(PR_COMMENTS);
    }

    if FeatureFlag::InlineProfileSelector.is_enabled() {
        commands.push(PROFILE.clone());
    }

    if FeatureFlag::RevertToCheckpoints.is_enabled() && FeatureFlag::RewindSlashCommand.is_enabled()
    {
        commands.push(REWIND);
    }

    if FeatureFlag::InlineRepoMenu.is_enabled() {
        commands.push(OPEN_REPO);
    }

    if FeatureFlag::Orchestration.is_enabled() {
        commands.push(ORCHESTRATE.clone());
    }

    if FeatureFlag::SettingsFile.is_enabled() && cfg!(feature = "local_fs") {
        commands.push(OPEN_SETTINGS_FILE);
    }

    commands
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn command_names_are_unique() {
        let names = COMMAND_REGISTRY
            .all_commands()
            .into_iter()
            .map(|command| command.name);
        let mut seen = HashSet::new();
        for name in names {
            assert!(seen.insert(name), "duplicate slash command name: {name}");
        }
    }

    #[test]
    fn rename_tab_command_requires_argument() {
        let command = COMMAND_REGISTRY
            .get_command_with_name(RENAME_TAB.name)
            .expect("expected /rename-tab to be registered");
        let argument = command
            .argument
            .as_ref()
            .expect("expected /rename-tab to require an argument");

        assert!(!argument.is_optional);
        assert!(!argument.should_execute_on_selection);
        assert_eq!(argument.hint_text, Some("<tab name>"));
    }

    #[test]
    fn rename_conversation_command_requires_argument() {
        let command = COMMAND_REGISTRY
            .get_command_with_name(RENAME_CONVERSATION.name)
            .expect("expected /rename-conversation to be registered");
        let argument = command
            .argument
            .as_ref()
            .expect("expected /rename-conversation to require an argument");

        assert!(!argument.is_optional);
        assert_eq!(argument.hint_text, Some("<conversation name>"));
    }

    #[test]
    fn strip_command_prefix_matches_orchestrate() {
        let result = strip_command_prefix("/orchestrate deploy services", "/orchestrate");
        assert_eq!(result, Some("deploy services".to_string()));
    }

    #[test]
    fn strip_command_prefix_no_match() {
        let result = strip_command_prefix("just a normal query", "/plan");
        assert_eq!(result, None);
    }

    #[test]
    fn strip_command_prefix_empty() {
        let result = strip_command_prefix("", "/plan");
        assert_eq!(result, None);
    }

    #[test]
    fn strip_command_prefix_no_trailing_space() {
        // "/plan" alone (no trailing space) should NOT be stripped
        let result = strip_command_prefix("/plan", "/plan");
        assert_eq!(result, None);
    }

    #[test]
    fn strip_command_prefix_trailing_space_only() {
        // "/plan " with nothing after should strip to empty string
        let result = strip_command_prefix("/plan ", "/plan");
        assert_eq!(result, Some(String::new()));
    }

    #[test]
    fn strip_command_prefix_substring_not_matched() {
        // "/planning" should not match "/plan"
        let result = strip_command_prefix("/planning something", "/plan");
        assert_eq!(result, None);
    }

    /// Always-on subset that is actually unflagged in all_commands().
    const FACTOR_UNFLAGGED_KEEP: &[&str] = &[
        "/agent",
        "/new",
        "/conversations",
        "/fork",
        "/fork-and-compact",
        "/export-to-file",
        "/export-to-clipboard",
        "/model",
        "/plan",
        "/add-mcp",
        "/add-prompt",
        "/add-rule",
        "/open-file",
        "/open-code-review",
        "/open-mcp-servers",
        "/open-project-rules",
        "/open-rules",
        "/rename-tab",
        "/rename-conversation",
        "/index",
        "/init",
    ];

    /// Explicit Factor product drops — must never appear in the registry.
    const FACTOR_DROPPED: &[&str] = &["/set-tab-color"];

    #[test]
    fn factor_unflagged_keep_commands_are_registered() {
        let names: HashSet<_> = all_commands().into_iter().map(|c| c.name).collect();
        for name in FACTOR_UNFLAGGED_KEEP {
            assert!(
                names.contains(name),
                "Factor KEEP command `{name}` missing from slash registry (SC matrix)"
            );
        }
    }

    #[test]
    fn factor_dropped_commands_are_not_registered() {
        let names: HashSet<_> = all_commands().into_iter().map(|c| c.name).collect();
        for name in FACTOR_DROPPED {
            assert!(
                !names.contains(name),
                "Factor DROP command `{name}` must not be registered"
            );
        }
        assert!(
            !names.contains("/docker-sandbox"),
            "/docker-sandbox must stay unregistered when LocalDockerSandbox is off"
        );
    }

    #[test]
    fn factor_agent_and_new_are_aliases() {
        let agent = all_commands()
            .into_iter()
            .find(|c| c.name == "/agent")
            .expect("/agent");
        let new = all_commands()
            .into_iter()
            .find(|c| c.name == "/new")
            .expect("/new");
        assert!(
            agent.description.contains("conversation") || new.description.contains("alias"),
            "expected /new to document alias relationship"
        );
        assert_eq!(
            new.description,
            "Start a new conversation (alias for /agent)"
        );
    }

    #[test]
    fn factor_flagged_keep_commands_register_when_enabled() {
        use warp_core::features::FeatureFlag;

        let _g1 = FeatureFlag::SummarizationConversationCommand.override_enabled(true);
        let _g2 = FeatureFlag::QueueSlashCommand.override_enabled(true);
        let _g3 = FeatureFlag::ForkFromCommand.override_enabled(true);
        let _g4 = FeatureFlag::ListSkills.override_enabled(true);
        let _g5 = FeatureFlag::InlineProfileSelector.override_enabled(true);
        let _g6 = FeatureFlag::RevertToCheckpoints.override_enabled(true);
        let _g7 = FeatureFlag::RewindSlashCommand.override_enabled(true);
        let _g8 = FeatureFlag::InlineRepoMenu.override_enabled(true);
        let _g9 = FeatureFlag::Orchestration.override_enabled(true);
        let _g10 = FeatureFlag::SettingsFile.override_enabled(true);
        let _g11 = FeatureFlag::CreateProjectFlow.override_enabled(true);
        let _g12 = FeatureFlag::AgentView.override_enabled(true);

        let names: HashSet<_> = all_commands().into_iter().map(|c| c.name).collect();

        let expected_flagged = [
            "/compact",
            "/compact-and",
            "/queue",
            "/fork-from",
            "/skills",
            "/open-skill",
            "/profile",
            "/rewind",
            "/open-repo",
            "/orchestrate",
            "/create-new-project",
            "/prompts",
        ];
        for name in expected_flagged {
            assert!(
                names.contains(name),
                "Factor KEEP command `{name}` missing when required flags enabled (SC matrix)"
            );
        }

        // Settings file also needs local_fs cfg — only assert if compiled with local_fs
        #[cfg(feature = "local_fs")]
        assert!(
            names.contains("/open-settings-file"),
            "/open-settings-file missing with SettingsFile + local_fs"
        );
    }

    #[test]
    fn factor_docker_sandbox_registers_when_flag_enabled() {
        use warp_core::features::FeatureFlag;
        let _g = FeatureFlag::LocalDockerSandbox.override_enabled(true);
        let names: HashSet<_> = all_commands().into_iter().map(|c| c.name).collect();
        assert!(
            names.contains("/docker-sandbox"),
            "/docker-sandbox must register when LocalDockerSandbox is enabled"
        );
    }
}
