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

pub fn json_schema() -> String {
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

#[derive(Serialize)]
#[serde(tag = "type")]
enum Tool {
    #[serde(rename = "web_search")]
    WebSearch { web_search: ToolWebSearch },
    #[serde(rename = "retrieval")]
    Retrieval { retrieval: ToolRetrieval },
    #[serde(rename = "function")]
    Function { function: ToolFunction },
}

#[derive(Serialize)]
struct ToolWebSearch {
    enabled: Option<bool>,
    search_query: Option<String>,
}

#[derive(Serialize)]
struct ToolRetrieval {
    knowledge_id: String,
    prompt_template: Option<String>,
}

#[derive(Serialize)]
struct ToolFunction {
    name: String,
    description: String,
    parameters: Builder,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum ToolCall {
    #[serde(rename = "web_search")]
    WebSearch { id: String },
    #[serde(rename = "retrieval")]
    Retrieval { id: String },
    #[serde(rename = "function")]
    Function {
        id: String,
        function: ToolCallFunction,
    },
}

#[derive(Serialize)]
struct ToolCallFunction {
    name: String,
    arguments: String,
}

impl ToolCall {
    fn id(&self) -> String {
        match self {
            ToolCall::WebSearch { id } => id.to_string(),
            ToolCall::Retrieval { id } => id.to_string(),
            ToolCall::Function { id, .. } => id.to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "role")]
enum Message {
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assisant")]
    Assisant {
        content: Option<String>,
        tool_calls: Option<Vec<ToolCall>>,
    },
    #[serde(rename = "tool")]
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_schema() {
        let schema = json_schema();
        println!("schema: {}", schema);
    }

    #[test]
    fn test_message_serialize() {
        let message = Message::User {
            content: "hello".to_string(),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        println!("serialized: {}", serialized);

        let message = Message::Assisant {
            content: Some("hello".to_string()),
            tool_calls: Some(vec![ToolCall::Function {
                id: "aaa".to_string(),
                function: ToolCallFunction {
                    name: "bbb".to_string(),
                    arguments: "ccc".to_string(),
                },
            }]),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        println!("serialized: {}", serialized);
    }

    #[test]
    fn test_tool_function() {
        let tool = Tool::Function {
            function: ToolFunction {
                name: "test".to_string(),
                description: "test".to_string(),
                parameters: Builder::build(|params| {
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
                }),
            },
        };
        let serialized = serde_json::to_string(&tool).unwrap();
        println!("serialized: {}", serialized);
    }
}
