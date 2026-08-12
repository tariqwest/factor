//! Specialized BYOP model roles (Q-06).
//!
//! Optional overrides for title generation, Active AI, and next-command
//! completions. Empty values fall back to the base / agent picker model via
//! `LLMPreferences::get_active_*`. Model ids are free-form strings
//! (`byop:{provider}:{model}` or a base catalog id).

use settings::Setting;
use warpui::elements::{Container, Flex, ParentElement, Text};
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::{AppContext, Element, SingletonEntity, ViewContext, ViewHandle};

use crate::appearance::Appearance;
use crate::editor::{
    EditorView, Event as EditorEvent, SingleLineEditorOptions, TextColors, TextOptions,
};
use crate::report_if_error;
use crate::settings::AISettings;

use super::ai_page::AISettingsPageView;
use super::settings_page::{build_sub_header, SettingsWidget, HEADER_PADDING};

const CONTENT_FONT_SIZE: f32 = 12.;
const PLACEHOLDER: &str = "byop:provider:model (empty = base model)";

/// Search / settings-find terms for this widget.
pub const SEARCH_TERMS: &str = "specialized models byop title model active ai next command \
    completions agent model id conversation title suggestions grey";

pub struct SpecializedByopModelsWidget {
    title_model_editor: ViewHandle<EditorView>,
    active_ai_model_editor: ViewHandle<EditorView>,
    next_command_model_editor: ViewHandle<EditorView>,
}

impl SpecializedByopModelsWidget {
    pub fn new(ctx: &mut ViewContext<AISettingsPageView>) -> Self {
        let ai = AISettings::as_ref(ctx);
        let title = ai.byop_title_model_id.value().clone();
        let active_ai = ai.byop_active_ai_model_id.value().clone();
        let next_command = ai.byop_next_command_model_id.value().clone();

        let title_model_editor = Self::create_editor(ctx, title.as_deref());
        Self::subscribe_option_string_setting(
            ctx,
            &title_model_editor,
            |settings, value, ctx| {
                report_if_error!(settings.byop_title_model_id.set_value(value, ctx));
            },
        );

        let active_ai_model_editor = Self::create_editor(ctx, active_ai.as_deref());
        Self::subscribe_option_string_setting(
            ctx,
            &active_ai_model_editor,
            |settings, value, ctx| {
                report_if_error!(settings.byop_active_ai_model_id.set_value(value, ctx));
            },
        );

        let next_command_model_editor = Self::create_editor(ctx, next_command.as_deref());
        Self::subscribe_option_string_setting(
            ctx,
            &next_command_model_editor,
            |settings, value, ctx| {
                report_if_error!(settings.byop_next_command_model_id.set_value(value, ctx));
            },
        );

        Self {
            title_model_editor,
            active_ai_model_editor,
            next_command_model_editor,
        }
    }

    fn create_editor(
        ctx: &mut ViewContext<AISettingsPageView>,
        value: Option<&str>,
    ) -> ViewHandle<EditorView> {
        let initial = value.map(|s| s.to_owned());
        ctx.add_typed_action_view(move |ctx| {
            let appearance = Appearance::handle(ctx).as_ref(ctx);
            let options = SingleLineEditorOptions {
                is_password: false,
                text: TextOptions {
                    font_size_override: Some(appearance.ui_font_size()),
                    font_family_override: Some(appearance.monospace_font_family()),
                    text_colors_override: Some(TextColors {
                        default_color: appearance.theme().active_ui_text_color(),
                        disabled_color: appearance.theme().disabled_ui_text_color(),
                        hint_color: appearance.theme().disabled_ui_text_color(),
                    }),
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text(PLACEHOLDER, ctx);
            if let Some(value) = initial.as_ref() {
                editor.set_buffer_text(value, ctx);
            }
            editor
        })
    }

    fn subscribe_option_string_setting(
        ctx: &mut ViewContext<AISettingsPageView>,
        editor: &ViewHandle<EditorView>,
        set: impl Fn(&mut AISettings, Option<String>, &mut warpui::ModelContext<AISettings>)
            + 'static,
    ) {
        ctx.subscribe_to_view(editor, move |_, editor, event, ctx| {
            if matches!(event, EditorEvent::Blurred | EditorEvent::Enter) {
                let buffer_text = editor.as_ref(ctx).buffer_text(ctx);
                let value = {
                    let trimmed = buffer_text.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_owned())
                    }
                };
                AISettings::handle(ctx).update(ctx, |settings, ctx| {
                    set(settings, value, ctx);
                });
            }
        });
    }

    fn render_input(
        label: &'static str,
        editor: ViewHandle<EditorView>,
        appearance: &Appearance,
        enabled: bool,
    ) -> Box<dyn Element> {
        let color = if enabled {
            appearance.theme().active_ui_text_color()
        } else {
            appearance.theme().disabled_ui_text_color()
        };

        let label = Text::new_inline(label, appearance.ui_font_family(), CONTENT_FONT_SIZE)
            .with_color(color.into())
            .finish();

        let input = appearance
            .ui_builder()
            .text_input(editor)
            .with_style(UiComponentStyles {
                padding: Some(Coords {
                    top: 10.,
                    bottom: 10.,
                    left: 16.,
                    right: 16.,
                }),
                background: Some(appearance.theme().surface_2().into()),
                ..Default::default()
            })
            .build()
            .finish();

        Flex::column()
            .with_spacing(8.)
            .with_child(label)
            .with_child(input)
            .finish()
    }
}

impl SettingsWidget for SpecializedByopModelsWidget {
    type View = AISettingsPageView;

    fn search_terms(&self) -> &str {
        SEARCH_TERMS
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let is_any_ai_enabled = AISettings::as_ref(app).is_any_ai_enabled(app);
        let header_color = if is_any_ai_enabled {
            appearance.theme().active_ui_text_color()
        } else {
            appearance.theme().disabled_ui_text_color()
        };

        let description = appearance
            .ui_builder()
            .paragraph(
                "Optional model overrides for specialized roles. Leave empty to use the base agent model from the picker. Use a BYOP id (byop:provider:model) or a catalog id present in Agents → Providers.",
            )
            .with_style(UiComponentStyles {
                font_color: Some(
                    appearance
                        .theme()
                        .active_ui_text_color()
                        .with_opacity(60)
                        .into(),
                ),
                font_size: Some(CONTENT_FONT_SIZE),
                ..Default::default()
            })
            .build()
            .finish();

        let column = Flex::column()
            .with_spacing(16.)
            .with_child(
                build_sub_header(appearance, "Specialized models", Some(header_color))
                    .with_padding_bottom(HEADER_PADDING)
                    .finish(),
            )
            .with_child(description)
            .with_child(Self::render_input(
                "Title model",
                self.title_model_editor.clone(),
                appearance,
                is_any_ai_enabled,
            ))
            .with_child(Self::render_input(
                "Active AI model",
                self.active_ai_model_editor.clone(),
                appearance,
                is_any_ai_enabled,
            ))
            .with_child(Self::render_input(
                "Next-command model",
                self.next_command_model_editor.clone(),
                appearance,
                is_any_ai_enabled,
            ));

        Container::new(column.finish())
            .with_margin_bottom(HEADER_PADDING)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specialized_byop_models_search_terms_cover_roles() {
        let terms = SEARCH_TERMS.to_lowercase();
        assert!(terms.contains("title"));
        assert!(terms.contains("active"));
        assert!(terms.contains("next command"));
        assert!(terms.contains("byop"));
    }

    #[test]
    fn specialized_byop_settings_fields_exist_on_ai_settings() {
        // Compile-time / reflection-style check: the define_settings_group! macros
        // expose these as Setting fields used by the widget and LLMPreferences.
        use settings::Setting as _;
        assert_eq!(
            crate::settings::ByopTitleModelId::toml_path(),
            Some("agents.byop.title_model_id")
        );
        assert_eq!(
            crate::settings::ByopActiveAiModelId::toml_path(),
            Some("agents.byop.active_ai_model_id")
        );
        assert_eq!(
            crate::settings::ByopNextCommandModelId::toml_path(),
            Some("agents.byop.next_command_model_id")
        );
    }
}
