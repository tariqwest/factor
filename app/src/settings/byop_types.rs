//! BYOP provider settings types (ported from Zap for Factor).
//! API keys live in `AgentProviderSecrets` (OS keychain), not in these structs.

use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use strum_macros::EnumIter;


// ---------------------------------------------------------------------------
// 自定义 Agent 提供商配置(进程内 Provider)
// ---------------------------------------------------------------------------

/// Agent 提供商支持的协议类型。
///
/// 第一阶段仅支持 OpenAI 兼容协议(适用于 OpenAI、DeepSeek、智谱 GLM、
/// Moonshot、通义千问 DashScope-OpenAI 兼容端点、SiliconFlow、OpenRouter、
/// 任何 OpenAI 兼容的本地服务等)。后续可在此扩展 Anthropic、Google、Bedrock。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AgentProviderKind {
    /// OpenAI 兼容的 Chat Completions / `/v1/models` 协议。
    #[default]
    OpenAiCompatible,
}

/// BYOP provider 实际使用的 API 协议类型 — 显式指定,
/// 由 chat_stream 通过 genai `ServiceTargetResolver` 一对一映射到对应的
/// `AdapterKind`,完全绕过"按模型名识别"的默认行为,避免误识别。
///
/// **注意**:这是相对 [`AgentProviderKind`] 的更细粒度维度。
/// `AgentProviderKind` 目前只有 `OpenAiCompatible`(语义"用户自管 endpoint"),
/// `AgentProviderApiType` 决定 genai 用哪种原生协议序列化请求 / 解析响应。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AgentProviderApiType {
    /// OpenAI Chat Completions(`POST /v1/chat/completions`)。
    /// 适用于:OpenAI 官方、DeepSeek、SiliconFlow、OpenRouter、智谱 GLM、
    /// Moonshot、DashScope-OpenAI 兼容、本地 vLLM/llama.cpp 等。
    #[default]
    OpenAi,
    /// OpenAI Responses API(`POST /v1/responses`)。
    /// 适用于:GPT-5 / Codex / Pro 等较新模型。
    OpenAiResp,
    /// Google Gemini 原生协议(generativelanguage.googleapis.com)。
    Gemini,
    /// Anthropic Messages API 原生协议(`POST /v1/messages`,默认 `api.anthropic.com/v1/`)。
    Anthropic,
    /// Ollama 原生协议(本地或自建 Ollama)。
    Ollama,
    /// DeepSeek 原生协议。与 OpenAI 兼容相比:多轮 thinking 模式必须把
    /// `reasoning_content` 字段带回服务端(否则 400),仅 genai DeepSeek
    /// adapter 处理这个非标字段。`deepseek-reasoner / deepseek-v4-flash` 等
    /// thinking-mode 模型必须选这个类型,普通 chat 模型(`deepseek-chat`)
    /// 选 OpenAI 也可以工作。
    DeepSeek,
}

/// Provider 级别的 reasoning effort(思考深度)偏好。
///
/// 语义说明:
/// - `Auto`(默认):不向 genai 传 effort。OpenAI / Anthropic adapter 内部会按
///   模型名后缀(`-low` / `-high` / `-zero` 等)自动推断;Gemini / DeepSeek 不推断。
/// - `Off`:对支持 reasoning 的模型显式发送 `none`,关闭思考链。
/// - 其他档位:client 端先用 `reasoning::model_supports_reasoning` 判定,**仅在该
///   模型支持时**注入,避免向 claude-3-5-haiku / gpt-4o / gemini-1.5-pro 等老模型
///   注入 thinking 参数被上游 400 拒绝。
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffortSetting {
    #[default]
    Auto,
    Off,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}

impl ReasoningEffortSetting {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Off => "Off",
            Self::Minimal => "Minimal",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::XHigh => "XHigh",
            Self::Max => "Max",
        }
    }

    /// 转成 genai `ReasoningEffort`。`Auto` 返回 None(调用方不要 set)。
    pub fn to_genai(self) -> Option<genai::chat::ReasoningEffort> {
        use genai::chat::ReasoningEffort as GE;
        Some(match self {
            Self::Auto => return None,
            Self::Off => GE::None,
            Self::Minimal => GE::Minimal,
            Self::Low => GE::Low,
            Self::Medium => GE::Medium,
            Self::High => GE::High,
            Self::XHigh => GE::XHigh,
            Self::Max => GE::Max,
        })
    }
}

impl AgentProviderApiType {
    /// 设置 UI dropdown 显示文字。
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::OpenAi => "OpenAI",
            Self::OpenAiResp => "OpenAI-Response",
            Self::Gemini => "Gemini",
            Self::Anthropic => "Anthropic",
            Self::Ollama => "Ollama",
            Self::DeepSeek => "DeepSeek",
        }
    }

    /// 反向解析 Debug 格式名(`OpenAi` / `DeepSeek` 等),用于 BYOPLastUsedReasoningMap
    /// 这种 `<api_type>:<model_id>` 复合 key 的 hydrate 场景。未知字符串返回 None。
    pub fn from_debug_str(s: &str) -> Option<Self> {
        Some(match s {
            "OpenAi" => Self::OpenAi,
            "OpenAiResp" => Self::OpenAiResp,
            "Gemini" => Self::Gemini,
            "Anthropic" => Self::Anthropic,
            "Ollama" => Self::Ollama,
            "DeepSeek" => Self::DeepSeek,
            _ => return None,
        })
    }

    /// 当用户没填 base_url 时使用的默认 endpoint。新建 provider / 切换 ApiType
    /// 时,UI 可调用此方法预填,便于新手。
    ///
    /// **必须以 `/` 结尾**:genai 0.6.x 的 adapter 内部用 `format!("{base_url}messages")` /
    /// `Url::join` 拼接 service path,缺尾随 `/` 会拼出乱地址(Anthropic 是 `.devmessages` 直接连)
    /// 或被 `Url::join` 吃掉 path 最后一段。client 端 `build_client` 也会兜底补 `/`,
    /// 这里依然要求显式尾随 `/`,保证 UI 预填值落到 settings.toml 后即使绕过 client 兜底也是对的。
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Self::OpenAi => "https://api.openai.com/v1/",
            Self::OpenAiResp => "https://api.openai.com/v1/",
            Self::Gemini => "https://generativelanguage.googleapis.com/v1beta/",
            Self::Anthropic => "https://api.anthropic.com/v1/",
            Self::Ollama => "http://localhost:11434/",
            Self::DeepSeek => "https://api.deepseek.com/v1/",
        }
    }
}

/// 一条用户自定义的 Agent 提供商配置。
///
/// `api_key` **不**保存在这里,而是保存在 `AgentProviderSecrets` 单例(secure storage),
/// 通过 `id` 关联。这样设置文件 (settings.toml) 不会泄漏敏感信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AgentProvider {
    /// 提供商唯一 ID,首次创建时生成,持久化到设置中作为 secret 的关联键。
    #[serde(default = "AgentProvider::default_id")]
    pub id: String,

    /// 用户给这个提供商起的显示名(例如 "DeepSeek 官方"、"本地 Ollama")。
    pub name: String,

    /// 协议类型,目前固定为 OpenAI 兼容(语义"用户自管 endpoint")。
    /// 实际请求/响应序列化协议由 [`AgentProvider::api_type`] 决定。
    #[serde(default)]
    pub kind: AgentProviderKind,

    /// 显式指定的 API 协议类型(OpenAI / OpenAI-Response / Gemini / Anthropic / Ollama)。
    /// 老配置(无此字段)反序列化为 `OpenAi` 兼容老语义。
    #[serde(default)]
    pub api_type: AgentProviderApiType,

    /// API base URL,例如 `https://api.deepseek.com/v1`、`http://localhost:11434`。
    /// 不要带尾随斜杠,但代码侧会做容错。
    pub base_url: String,

    /// 用户配置的、希望暴露给 Agent 选择的模型列表。
    /// 每条同时含 `name`(显示名) 与 `id`(发送给上游 API 的 model 字段值)。
    #[serde(default)]
    pub models: Vec<AgentProviderModel>,

    /// 附加 HTTP 请求头,发给上游 provider 时逐条 merge 进请求。
    /// 用于需要额外路由头的 gateway(如 Portkey 的 `x-portkey-provider`)。
    /// `api_key` 仍走 `Authorization: Bearer` 标准路径。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_headers: Vec<(String, String)>,
}

impl AgentProvider {
    fn default_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// 构造一个新的、空的提供商。
    pub fn new_empty() -> Self {
        Self {
            id: Self::default_id(),
            name: String::new(),
            kind: AgentProviderKind::default(),
            api_type: AgentProviderApiType::default(),
            base_url: String::new(),
            models: Vec::new(),
            extra_headers: Vec::new(),
        }
    }
}

impl settings_value::SettingsValue for AgentProvider {}

/// 单条模型条目: `name` 是用户在 model picker 中看到的显示名,
/// `id` 是真正发给上游 OpenAI 兼容 API 的 `model` 字段值。
///
/// 序列化为 toml 时形如:
/// ```toml
/// [[agent_providers.models]]
/// name = "DS V3 通用"
/// id   = "deepseek-chat"
/// ```
///
/// 反序列化兼容老格式 `models = ["deepseek-chat", "deepseek-coder"]`
/// (每个字符串视为 `{ name = id, id = id }`),便于现有用户无痛升级。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct AgentProviderModel {
    pub name: String,
    pub id: String,

    /// 上下文窗口(tokens)。来源:用户填或 models.dev 自动带入。
    /// 0 表示未知 — chat_stream 退化到不做主动截断,完全交给上游服务报错。
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub context_window: u32,

    /// 单次最大输出 tokens。0 表示未指定。
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub max_output_tokens: u32,

    /// 是否支持 reasoning(思考/CoT)输出。
    #[serde(default, skip_serializing_if = "is_false")]
    pub reasoning: bool,

    /// 是否支持 function/tool calling。
    /// 默认 `true` — 老配置升级 + 用户手填新 model 时不要默认禁工具,
    /// 不支持工具调用的模型由 models.dev 数据带入显式 false。
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub tool_call: bool,

    // ----- 多模态附件 capability,三态语义:
    // - `None`(toml 字段缺省)= Auto: 运行时按 models.dev catalog → substring fallback 推断
    // - `Some(true)` = Force-On: 用户强制开,绕过推断
    // - `Some(false)` = Force-Off: 用户强制关
    //
    // 字段命名故意用 `image` 而非 `vision`,跟 models.dev `modalities.input: ["image"]`
    // 字面对应,语义最窄不歧义(避免用户误以为 vision = image+pdf+...)。
    /// 是否支持图像输入(image/* MIME)。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<bool>,
    /// 是否支持 PDF 文档输入(application/pdf)。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdf: Option<bool>,
    /// 是否支持音频输入(audio/* MIME)。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
}

fn is_zero_u32(v: &u32) -> bool {
    *v == 0
}
fn is_false(v: &bool) -> bool {
    !*v
}
fn is_true(v: &bool) -> bool {
    *v
}
fn default_true() -> bool {
    true
}

impl AgentProviderModel {
    pub fn from_id(id: String) -> Self {
        Self {
            name: id.clone(),
            id,
            context_window: 0,
            max_output_tokens: 0,
            reasoning: false,
            tool_call: true,
            image: None,
            pdf: None,
            audio: None,
        }
    }
}

impl<'de> Deserialize<'de> for AgentProviderModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Either {
            Plain(String),
            Full {
                #[serde(default)]
                name: String,
                id: String,
                #[serde(default)]
                context_window: u32,
                #[serde(default)]
                max_output_tokens: u32,
                #[serde(default)]
                reasoning: bool,
                #[serde(default = "default_true")]
                tool_call: bool,
                #[serde(default)]
                image: Option<bool>,
                #[serde(default)]
                pdf: Option<bool>,
                #[serde(default)]
                audio: Option<bool>,
            },
        }
        match Either::deserialize(deserializer)? {
            Either::Plain(id) => Ok(AgentProviderModel::from_id(id)),
            Either::Full {
                name,
                id,
                context_window,
                max_output_tokens,
                reasoning,
                tool_call,
                image,
                pdf,
                audio,
            } => {
                let name = if name.is_empty() { id.clone() } else { name };
                Ok(AgentProviderModel {
                    name,
                    id,
                    context_window,
                    max_output_tokens,
                    reasoning,
                    tool_call,
                    image,
                    pdf,
                    audio,
                })
            }
        }
    }
}

impl settings_value::SettingsValue for AgentProviderModel {}

/// Keys are regex patterns (insertion-ordered), values are serialized CLIAgent names (e.g. "Claude").
/// An empty string value means "Any CLI Agent" (CLIAgent::Unknown).
///
/// Uses `IndexMap` to preserve insertion order so the settings UI list is deterministic.
/// Supports backward-compatible deserialization from the legacy `Vec<String>` format,
/// where each string is converted to a key with an empty agent value.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ToolbarCommandMap(IndexMap<String, String>);

impl ToolbarCommandMap {
    pub(crate) fn new(map: IndexMap<String, String>) -> Self {
        Self(map)
    }
}

impl<'de> Deserialize<'de> for ToolbarCommandMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MapOrVec {
            Map(IndexMap<String, String>),
            Vec(Vec<String>),
        }

        match MapOrVec::deserialize(deserializer) {
            Ok(MapOrVec::Map(map)) => Ok(ToolbarCommandMap::new(map)),
            Ok(MapOrVec::Vec(vec)) => {
                let map = vec
                    .into_iter()
                    .map(|pattern| (pattern, String::new()))
                    .collect();
                Ok(ToolbarCommandMap::new(map))
            }
            Err(e) => Err(e),
        }
    }
}

impl schemars::JsonSchema for ToolbarCommandMap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ToolbarCommandMap")
    }

    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        gen.subschema_for::<HashMap<String, String>>()
    }
}

impl std::ops::Deref for ToolbarCommandMap {
    type Target = IndexMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl settings_value::SettingsValue for ToolbarCommandMap {
    fn to_file_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.0).unwrap_or_default()
    }

    fn from_file_value(value: &serde_json::Value) -> Option<Self> {
        // Try map format first (using from_value to preserve insertion order), then legacy array format.
        if value.is_object() {
            if let Ok(map) = serde_json::from_value::<IndexMap<String, String>>(value.clone()) {
                return Some(ToolbarCommandMap::new(map));
            }
        }
        if let Some(arr) = value.as_array() {
            let result: IndexMap<String, String> = arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| (s.to_string(), String::new())))
                .collect();
            return Some(ToolbarCommandMap::new(result));
        }
        None
    }
}

/// 持久化记忆"上次某 (api_type, model) 用过的 reasoning effort 档位"。
/// key 形式:`<api_type>:<model_id>`,例如 `DeepSeek:deepseek-v4-pro`。
/// value 是 `ReasoningEffortSetting` 枚举(serde_json snake_case)。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BYOPLastUsedReasoningMap(pub IndexMap<String, ReasoningEffortSetting>);

impl BYOPLastUsedReasoningMap {
    pub fn new(map: IndexMap<String, ReasoningEffortSetting>) -> Self {
        Self(map)
    }

    /// 拼 key:`<api_type>:<model_id>`。api_type 用 Debug 拼出 `DeepSeek` 等驼峰名,
    /// 跟 ReasoningEffortSetting 的 serde 形式无关。
    pub fn make_key(api_type: AgentProviderApiType, model_id: &str) -> String {
        format!("{api_type:?}:{model_id}")
    }
}

impl std::ops::Deref for BYOPLastUsedReasoningMap {
    type Target = IndexMap<String, ReasoningEffortSetting>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl schemars::JsonSchema for BYOPLastUsedReasoningMap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "BYOPLastUsedReasoningMap".into()
    }

    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        gen.subschema_for::<HashMap<String, String>>()
    }
}

impl settings_value::SettingsValue for BYOPLastUsedReasoningMap {
    fn to_file_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.0).unwrap_or_default()
    }

    fn from_file_value(value: &serde_json::Value) -> Option<Self> {
        if value.is_object() {
            if let Ok(map) =
                serde_json::from_value::<IndexMap<String, ReasoningEffortSetting>>(value.clone())
            {
                return Some(Self::new(map));
            }
        }
        None
    }
}
