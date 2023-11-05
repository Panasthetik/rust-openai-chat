use std::error::Error;
use std::io::{stdout, Write};
use std::env;
use dotenv::dotenv;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv::from_filename(".env").ok();
    let api_key = dotenv::var("OPENAI_API_KEY").unwrap();
    env::set_var("OPENAI_API_KEY", &api_key);

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        // .model("gpt-3.5-turbo")
        .model("gpt-4")
        .max_tokens(1024u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            // .content("Write a marketing blog praising and introducing Rust library async-openai")
            .content("Write a marketing blog praising and introducing Damien Hirst as an artist to invest in.")
            .role(Role::User)
            .build()?])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}