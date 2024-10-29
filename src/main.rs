
use clap::Parser;

mod types;
mod feed;

use crate::types::Feed;

#[derive(Parser, Debug)]
#[command(name = "cli-tool")]
#[command(about = "Processes base URL, feed directory, and title")]
struct Cli {
    /// The base URL for the feed; use a trailing slash for best results
    #[arg(long)]
    base_url: String,

    /// The filesystem directory containing the posts
    #[arg(long)]
    feed_dir: String,

    /// The title of the gemlog
    #[arg(long)]
    title: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let f: Feed = feed::get_feed(args.base_url, args.feed_dir, args.title)?;

    let yaserde_cfg = yaserde::ser::Config{
        perform_indent: true,
        .. Default::default()
    };

    let output = yaserde::ser::to_string_with_config(&f, &yaserde_cfg)
        .ok()
        .unwrap();

    println!("{}", output);

    Ok(())
}
