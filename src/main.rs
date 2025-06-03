mod pchat;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use pchat::cli;
use pchat::config_parse;
use pchat::get_project_context;
use tokio::io::{self, AsyncWriteExt};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let pchat_config = config_parse::parse_conf();
    let project_context: String = get_project_context::check_if_rust_project();
    let ollama = Ollama::new(pchat_config.chat_url, pchat_config.chat_port);
    let model = pchat_config.model;

    let prompt = cli::run();
    let full_prompt = format!("{}:\n{}", prompt, project_context);

    let mut stream = ollama
        .generate_stream(GenerationRequest::new(model, full_prompt))
        .await
        .unwrap();
    let mut stdout = io::stdout();
    while let Some(res) = stream.next().await {
        let responses = res.unwrap();
        for resp in responses {
            stdout.write_all(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
        }
    }
    println!("\n");
}
