use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::Config;


const SYSTEM_PROMPT: &str = "你是一位专业的命令行/终端错误诊断专家，专注于分析各类命令行工具、脚本、程序的输出日志与报错信息。
你的核心任务是：基于用户提供的【原始命令行输出内容】，快速、精准地定位问题根源，并给出可直接执行的解决方案。

=== 你的工作规则（必须严格遵守）===
1. 只分析用户提供的命令行原始输出文本，不编造未出现的日志、错误码和上下文
2. 优先识别：错误类型、报错码、异常堆栈、关键提示词、失败原因
3. 按以下结构输出诊断结果：
   🚨 错误类型：简明归类（如：命令不存在、权限不足、依赖缺失、语法错误、网络失败、配置错误等）
   🔍 问题根源：精准说明根本原因（1-3句话）
   ✅ 解决方案：提供可直接复制执行的命令/步骤，清晰易懂
   💡 补充说明：可选，给出排查建议、常见误区、依赖版本说明等

4. 语言必须：简洁、专业、无废话、可直接用于CLI工具展示
5. 若用户提供的内容不是报错/无异常，直接回复：未检测到命令行错误，输出内容正常
6. 不解释无关概念，不发散无关内容，只专注解决当前报错
7. 适配所有操作系统：Linux、macOS、Windows、容器环境等";

#[derive(Debug, Deserialize, Serialize)]
struct LLMResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Choice {
    message: Messages,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Messages {
    role: String,
    content: String,
}


pub async fn call_llm(input: &str, config: &Config) -> Result<String, String> {
    let client = Client::new();
    let res = client
        .post(&config.base_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&json!({
            "model": &config.model,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": input}
            ]
        }))
        .send()
        .await;
    match res {
        Ok(response) => {
            let base = response.text().await.unwrap();
            let base: LLMResponse = serde_json::from_str(&base).unwrap();
            Ok(base.choices[0].message.content.clone())
        }
        Err(e) => Err(e.to_string()),
    }
}
