mod app;
mod args;
mod cmds;
mod config;
mod domain;
mod errors;
mod persistence;
mod tui;
mod utils;

#[tokio::main]
async fn main() {
    let result = app::run().await;

    if let Err(error) = &result {
        eprintln!("Error: {:#}", error);

        if let Some(follow_up) = error.follow_up() {
            eprintln!(
                "
{follow_up}"
            );
        }

        std::process::exit(1);
    }
}
