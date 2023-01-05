use std::error::Error;

use clap::Parser;

mod reddit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The Path to where the Loaded file should be saved
    #[arg(short, long, default_value = "./")]
    path: String,
    /// The Url from which to obtain the video
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(err) = run(args).await {
        eprintln!("{}", err);
    }
}

async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if !args.url.contains("reddit") {
        return Err(String::from("This is not a Reddit Url").into());
    } else {
        reddit::load_video(&args).await?;
    }

    Ok(())
}
