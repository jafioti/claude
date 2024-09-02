use std::io::{stdin, stdout, Write};

use colored::Colorize;
use misanthropy::*;

#[tokio::main]
async fn main() {
    let client = Anthropic::from_env().unwrap();
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Failed to get input");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let mut request = MessagesRequest::default()
            .with_stream(true)
            .with_model("claude-3-5-sonnet-20240620");
        request.add_user(Content::text(s));

        let mut stream = client
            .messages_stream(&request)
            .expect("Failed to reach the anthropic API.");
        while let Some(event) = stream.next().await {
            match event {
                Ok(StreamEvent::ContentBlockDelta {
                    delta: ContentBlockDelta::TextDelta { text },
                    ..
                }) => {
                    print!("{}", text.bold());
                }
                Err(e) => eprintln!("Error: {}", e),
                _ => {}
            }
        }
        println!();
    }
}
