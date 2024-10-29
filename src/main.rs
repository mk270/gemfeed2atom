/*  gemfeed2atom: generates Atom feeds from gemlog directories

    by Martin Keegan

    To the extent (if any) permissible by law, Copyright (C) 2024  Martin Keegan

    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/

use clap::Parser;

mod types;
mod feed;
mod heading;

use crate::types::Feed;

#[derive(Parser, Debug)]
#[command(name = "gemfeed2atom")]
#[command(about = "generates Atom feeds from gemlog directories")]
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
