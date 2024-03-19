mod jwt;
mod zhipu;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = zhipu::Params {
        model: zhipu::Model::GLM3Turbo,
        messages: vec![zhipu::Message::User {
            content: "你好你知道深圳湾一号周边有什么酒店吗?".to_string(),
        }],
        request_id: None,
        do_sample: None,
        stream: Some(false),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: None,
        tools: None,
        tool_choices: None,
    };
    zhipu::request(params).await?;
    Ok(())
}
