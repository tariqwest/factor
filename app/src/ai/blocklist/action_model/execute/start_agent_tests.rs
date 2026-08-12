use super::*;
use crate::ai::agent::conversation::ConversationStatus;
use crate::ai::agent::task::TaskId;
use crate::ai::agent::{
    AIAgentAction, AIAgentActionId, AIAgentActionResultType, AIAgentActionType,
    StartAgentExecutionMode, StartAgentResult,
};
use crate::ai::blocklist::BlocklistAIHistoryModel;
use ai::agent::action_result::StartAgentVersion;
use warp_core::features::FeatureFlag;
use warpui::{App, Entity, EntityId, ModelContext};

fn build_start_agent_action(
    version: StartAgentVersion,
    execution_mode: StartAgentExecutionMode,
) -> AIAgentAction {
    AIAgentAction {
        id: AIAgentActionId::from("start-agent-action".to_string()),
        action: AIAgentActionType::StartAgent {
            version,
            name: "Agent 1".to_string(),
            prompt: "Investigate the failure".to_string(),
            execution_mode,
            lifecycle_subscription: None,
        },
        task_id: TaskId::new("start-agent-task".to_string()),
        requires_result: false,
    }
}

/// Collects `StartAgentExecutorEvent` for unit assertions without pane/GUI.
#[derive(Default)]
struct StartAgentEventCollector {
    create_agent: Vec<StartAgentRequest>,
}

impl Entity for StartAgentEventCollector {
    type Event = ();
}

fn subscribe_create_agent_events(
    app: &mut App,
    executor: &warpui::ModelHandle<StartAgentExecutor>,
) -> warpui::ModelHandle<StartAgentEventCollector> {
    let collector = app.add_model(|_| StartAgentEventCollector::default());
    collector.update(app, |_, ctx: &mut ModelContext<StartAgentEventCollector>| {
        ctx.subscribe_to_model(executor, |me, event, _| match event {
            StartAgentExecutorEvent::CreateAgent(request) => {
                me.create_agent.push(request.clone());
            }
        });
    });
    collector
}

#[test]
fn execute_returns_error_when_child_startup_is_blocked_before_initialization() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V1,
            StartAgentExecutionMode::local_with_defaults(),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Async {
            execute_future,
            on_complete,
        } = execution
        else {
            panic!("expected async execution");
        };

        let child_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_child_conversation(
                terminal_view_id,
                "Agent 1".to_string(),
                parent_conversation_id,
                ctx,
            )
        });

        executor.read(&app, |executor, _| {
            assert_eq!(
                executor
                    .pending
                    .as_ref()
                    .and_then(|pending| pending.child_conversation_id),
                Some(child_conversation_id)
            );
        });

        history_model.update(&mut app, |history_model, ctx| {
            history_model.update_conversation_status(
                terminal_view_id,
                child_conversation_id,
                ConversationStatus::Blocked {
                    blocked_action:
                        "GitHub authentication required before starting the child agent."
                            .to_string(),
                },
                ctx,
            );
        });

        let async_result = execute_future.await;
        let result = app.update(|ctx| on_complete(async_result, ctx));
        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Error { error, version })
                if error
                    == "GitHub authentication required before starting the child agent."
                    && version == StartAgentVersion::V1
        ));

        executor.read(&app, |executor, _| {
            assert!(executor.pending.is_none());
        });
    });
}

#[test]
fn execute_returns_detailed_error_when_child_startup_fails_before_initialization() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V1,
            StartAgentExecutionMode::local_with_defaults(),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Async {
            execute_future,
            on_complete,
        } = execution
        else {
            panic!("expected async execution");
        };

        let child_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_child_conversation(
                terminal_view_id,
                "Agent 1".to_string(),
                parent_conversation_id,
                ctx,
            )
        });

        history_model.update(&mut app, |history_model, ctx| {
            history_model.update_conversation_status_with_error_message(
                terminal_view_id,
                child_conversation_id,
                ConversationStatus::Error,
                Some("Failed to resolve child agent skills: review-comments".to_string()),
                ctx,
            );
        });

        let async_result = execute_future.await;
        let result = app.update(|ctx| on_complete(async_result, ctx));
        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Error { error, version })
                if error == "Failed to resolve child agent skills: review-comments"
                    && version == StartAgentVersion::V1
        ));
    });
}

#[test]
fn execute_returns_error_when_local_harness_child_requires_orchestration_v2() {
    App::test((), |mut app| async move {
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V2,
            StartAgentExecutionMode::local_harness("claude".to_string()),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Sync(result) = execution else {
            panic!("expected sync execution");
        };

        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Error { error, version })
                if error == "Local harness child agents require orchestration v2."
                    && version == StartAgentVersion::V2
        ));
    });
}

#[test]
fn execute_rejects_invalid_local_harness_names_before_pane_creation() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V2,
            StartAgentExecutionMode::local_harness("codex".to_string()),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Sync(result) = execution else {
            panic!("expected sync execution");
        };

        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Error { error, version })
                if error == "Unsupported local child harness 'codex'."
                    && version == StartAgentVersion::V2
        ));
    });
}

#[test]
fn execute_returns_error_when_local_harness_child_missing_parent_run_id() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V2,
            StartAgentExecutionMode::local_harness("claude".to_string()),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Sync(result) = execution else {
            panic!("expected sync execution");
        };

        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Error { error, version })
                if error
                    == "Local harness child agents require the parent run_id to be available."
                    && version == StartAgentVersion::V2
        ));
    });
}

#[test]
fn execute_returns_error_when_remote_opencode_harness_is_requested() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V2,
            StartAgentExecutionMode::Remote {
                environment_id: "env-123".to_string(),
                skill_references: vec![],
                model_id: String::new(),
                computer_use_enabled: false,
                worker_host: String::new(),
                harness_type: "opencode".to_string(),
                title: String::new(),
            },
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Sync(result) = execution else {
            panic!("expected sync execution");
        };

        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Error { error, version })
                if error == "Remote child agents do not support the opencode harness yet."
                    && version == StartAgentVersion::V2
        ));
    });
}

/// Local StartAgentV2 path: executor emits CreateAgent with local mode so the
/// pane/controller layer can spawn a child conversation.
///
/// Does **not** cover: terminal pane creation, enter_agent_view, or the child
/// inference loop (IT-005 GUI / full controller E2E).
#[test]
fn execute_local_v2_emits_create_agent_with_local_mode() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let events = subscribe_create_agent_events(&mut app, &executor);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V2,
            StartAgentExecutionMode::local_with_defaults(),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        assert!(
            matches!(execution, AnyActionExecution::Async { .. }),
            "local start_agent waits on child startup"
        );

        events.read(&app, |collector, _| {
            assert_eq!(collector.create_agent.len(), 1);
            let request = &collector.create_agent[0];
            assert_eq!(request.name, "Agent 1");
            assert_eq!(request.prompt, "Investigate the failure");
            assert_eq!(request.parent_conversation_id, parent_conversation_id);
            assert_eq!(
                request.execution_mode,
                StartAgentExecutionMode::local_with_defaults()
            );
            assert!(request.parent_run_id.is_none());
        });

        executor.read(&app, |executor, _| {
            assert!(executor.pending.is_some());
            assert_eq!(
                executor
                    .pending
                    .as_ref()
                    .map(|p| p.parent_conversation_id),
                Some(parent_conversation_id)
            );
        });
    });
}

/// Parent→child completion without GUI: after CreateAgent, history creates a
/// child conversation and assigns a run_id; executor returns Success(agent_id).
///
/// Remaining E2E gaps for full UI:
/// - `StartAgentExecutorEvent::CreateAgent` → terminal `Event::StartAgentConversation`
/// - pane_group `create_hidden_child_agent_conversation` / visible child pane
/// - child auto-query with parent's prompt and BYOP FakeProvider turn
/// - parent UI showing child status / agent_id tool result in transcript
#[test]
fn execute_local_v2_succeeds_when_child_run_id_is_assigned() {
    App::test((), |mut app| async move {
        let _orchestration_v2 = FeatureFlag::OrchestrationV2.override_enabled(true);
        let terminal_view_id = EntityId::new();
        let history_model = app.add_singleton_model(|_| BlocklistAIHistoryModel::new_for_test());
        let executor = app.add_model(StartAgentExecutor::new);
        let parent_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_conversation(terminal_view_id, false, ctx)
        });
        let action = build_start_agent_action(
            StartAgentVersion::V2,
            StartAgentExecutionMode::local_with_defaults(),
        );

        let execution = executor.update(&mut app, |executor, ctx| {
            let input = ExecuteActionInput {
                action: &action,
                conversation_id: parent_conversation_id,
            };
            let result: AnyActionExecution = executor.execute(input, ctx).into();
            result
        });

        let AnyActionExecution::Async {
            execute_future,
            on_complete,
        } = execution
        else {
            panic!("expected async execution");
        };

        // Step 1: parent→child link via history (same path pane_group uses after CreateAgent).
        let child_conversation_id = history_model.update(&mut app, |history_model, ctx| {
            history_model.start_new_child_conversation(
                terminal_view_id,
                "Agent 1".to_string(),
                parent_conversation_id,
                ctx,
            )
        });

        executor.read(&app, |executor, _| {
            assert_eq!(
                executor
                    .pending
                    .as_ref()
                    .and_then(|pending| pending.child_conversation_id),
                Some(child_conversation_id),
                "StartedNewConversation must bind child to pending StartAgent"
            );
        });

        // Step 2: assign run_id (OrchestrationV2 agent_id). Valid non-nil UUID.
        let child_run_id = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee".to_string();
        history_model.update(&mut app, |history_model, ctx| {
            history_model.assign_run_id_for_conversation(
                child_conversation_id,
                child_run_id.clone(),
                None,
                terminal_view_id,
                ctx,
            );
        });

        executor.read(&app, |executor, _| {
            assert!(
                executor.pending.is_none(),
                "pending StartAgent should complete after run_id assignment"
            );
        });

        let async_result = execute_future.await;
        let result = app.update(|ctx| on_complete(async_result, ctx));
        assert!(matches!(
            result,
            AIAgentActionResultType::StartAgent(StartAgentResult::Success { agent_id, version })
                if agent_id == child_run_id && version == StartAgentVersion::V2
        ));
    });
}
