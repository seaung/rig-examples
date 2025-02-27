use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let agent = model.agent("llama3:8b")
        .preamble("你是一个中国人")
        .build();

    let response = agent.prompt("你是哪里人?").await?;

    println!("{:?}", response);
    Ok(())
}