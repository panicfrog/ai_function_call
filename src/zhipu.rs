use serde::{Deserialize, Serialize};
use std::fmt;
use valico::json_schema::{Builder, PrimitiveType};
const api_host: &'static str = "https://open.bigmodel.cn/api/paas/v4/chat/completions";

enum Model {
    GLM3Turbo,
    GLM4,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::GLM3Turbo => write!(f, "glm-3-turbo"),
            Model::GLM4 => write!(f, "glm-4"),
        }
    }
}

pub fn a() -> String {
    let mut params = Builder::build(|params| {
        params.properties(|params| {
            params.insert("location", |params| {
                params.string();
                params.desc("城市，如：北京")
            });
            params.insert("unit", |params| {
                params.enum_(|params| {
                    params.push("c");
                    params.push("f");
                });
                params.desc("温度单位，c:摄氏度，f:华氏度")
            });
        });
        params.required(vec!["location".to_string(), "unit".to_string()]);
    });
    params.into_json().to_string()
}

struct Params {
    model: Model,
    messages: Vec<Message>,
    request_id: Option<String>,
    do_sample: Option<bool>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<i32>,
    stop: Option<Vec<String>>,
    tools: Option<Vec<Tool>>,
    tool_choices: Option<String>,
}

struct ToolWrapper {
    id: String,
    tool: Tool,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Tool {
    #[serde(rename = "web_search")]
    WebSearch {
        enabled: Option<bool>,
        search_query: Option<String>,
    },
    #[serde(rename = "retrieval")]
    Retrieval {
        knowledge_id: String,
        prompt_template: Option<String>,
    },
    #[serde(rename = "function")]
    Function {
        name: String,
        description: String,
        parameters: Builder,
    },
}

// TODO: 自定义序列化
#[derive(Serialize)]
enum ToolCalls {
    WebSearch {
        id: String,
    },
    Retrieval {
        id: String,
    },
    Function {
        id: String,
        name: String,
        arguments: String,
    },
}

impl ToolCalls {
    fn id(&self) -> String {
        match self {
            ToolCalls::WebSearch { id } => id.to_string(),
            ToolCalls::Retrieval { id } => id.to_string(),
            ToolCalls::Function { id, .. } => id.to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum Message {
    System {
        role: String,
        content: String,
    },
    User {
        role: String,
        content: String,
    },
    Assisant {
        role: String,
        content: Option<String>,
        tool_calls: Option<Vec<ToolCalls>>,
    },
    Tool {
        role: String,
        content: String,
        tool_call_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_schema() {
        let schema = a();
        println!("schema: {}", schema);
    }
}
