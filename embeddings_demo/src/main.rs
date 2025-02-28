use rig::providers::openai;
use rig::embeddings::{EmbeddingsBuilder};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_url("ollama", "http://localhost:11434/v1");

    let model = client.embedding_model("llama3:8b");

    let embedding = EmbeddingsBuilder::new(model)
        .document("llama")?
        .document("llama3")?
        .build()
        .await?;

    println!("{:?}", embedding);
    Ok(())
}