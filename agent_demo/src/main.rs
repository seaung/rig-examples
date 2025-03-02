use rig::{providers::openai};
use rig::completion::Prompt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_url("ollama", "http://localhost:11434/v1");

    let agent = client.agent("qwen2:7b")
        .preamble("you are a helpful assistant.")
        .build();

    let response = agent.prompt("halo").await?;

    println!("{:#?}", response);

    Ok(())
}


// use rig::{providers::openai, Embed};
// use serde::{Serialize};
// use rig::completion::Prompt;
// use rig::embeddings::EmbeddingsBuilder;
// use rig::vector_store::in_memory_store::InMemoryVectorStore;
//
// #[derive(Embed, Serialize, Clone, Debug, Eq, PartialEq, Default)]
// struct WordDefinition {
//     id: String,
//     word: String,
//     #[embed]
//     definitions: Vec<String>,
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = openai::Client::from_url("ollama", "http://localhost:11434/v1");
//
//     let embedding_model = client.embedding_model("qwen2:7b");
//
//     let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
//         .documents(vec![
//             WordDefinition {
//                 id: "doc0".to_string(),
//                 word: "flurbo".to_string(),
//                 definitions: vec![
//                     "1. *flurbo* (name): A flurbo is a green alien that lives on cold planets.".to_string(),
//                     "2. *flurbo* (name): A fictional digital currency that originated in the animated series Rick and Morty.".to_string()
//                 ]
//             },
//             WordDefinition {
//                 id: "doc1".to_string(),
//                 word: "glarb-glarb".to_string(),
//                 definitions: vec![
//                     "1. *glarb-glarb* (noun): A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.".to_string(),
//                     "2. *glarb-glarb* (noun): A fictional creature found in the distant, swampy marshlands of the planet Glibbo in the Andromeda galaxy.".to_string()
//                 ]
//             },
//             WordDefinition {
//                 id: "doc2".to_string(),
//                 word: "linglingdong".to_string(),
//                 definitions: vec![
//                     "1. *linglingdong* (noun): A term used by inhabitants of the far side of the moon to describe humans.".to_string(),
//                     "2. *linglingdong* (noun): A rare, mystical instrument crafted by the ancient monks of the Nebulon Mountain Ranges on the planet Quarm.".to_string()
//                 ]
//             },
//         ])?
//         .build()
//         .await?;
//
//     // Create vector store with the embeddings
//     let vector_store = InMemoryVectorStore::from_documents(embeddings);
//
//     // Create vector store index
//     let index = vector_store.index(embedding_model);
//
//     let rag_agent = client.agent("qwen2:7b")
//         .preamble("
//             You are a dictionary assistant here to assist the user in understanding the meaning of words.
//             You will find additional non-standard word definitions that could be useful below.
//         ")
//         .dynamic_context(1, index)
//         .build();
//
//     let resp = rag_agent.prompt("what does \"glarb-glarb\" mean?").await?;
//
//     println!("{}", resp);
//
//
//     Ok(())
// }