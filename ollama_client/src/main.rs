use rig::providers;
use rig::completion::request::CompletionModel;
use rig::completion::Prompt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");

    let model = client.completion_model("llama3:8b");

    let chat = model.completion_request("complex query")
        .send()
        .await?;

    println!("{:#?}", chat.choice.first());

    Ok(())
}
