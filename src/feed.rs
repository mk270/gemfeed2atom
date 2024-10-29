/*  gemfeed2atom: generates Atom feeds from gemlog directories

    by Martin Keegan

    To the extent (if any) permissible by law, Copyright (C) 2024  Martin Keegan

    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/

use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;

use chrono::{DateTime, Utc};
use std::time::SystemTime;

use crate::types::*;
use crate::heading::*;

const XMLNS: &str     = "http://www.w3.org/2005/Atom";

// GEN_* is about the <generator /> tag in the Atom header
const GEN_URI: &str  = "https://github.com/mk270/gemfeed2atom";
const GEN_VER: &str  = "1.0.0";
const GEN_NAME: &str = "gemfeed2atom";

const MAX_ENTRIES: usize = 10;

// gather metadata about this program itself
fn get_generator() -> Generator {
    Generator {
        uri: String::from(GEN_URI),
        version: String::from(GEN_VER),
        contents: String::from(GEN_NAME)
    }
}


fn get_title(filename: PathBuf) -> String {
    const DEFAULT: &str = "No title found";
    extract_first_heading(filename, DEFAULT)
}

struct NameDate(
    PathBuf,
    String,
    DateTime<Utc> // field 2
);

fn get_entries(dir_path: PathBuf, max: usize, base_url: String)
               -> Result<Vec<Entry>, std::io::Error> {
    // we want the top ten most recent regular world-readable Gemini files in
    // the directory, excluding the index file
    let mut files : Vec<NameDate> = vec![];

    // we iterate over the files twice; the second for-loop does the work
    // of opening the files; we want to avoid opening files that aren't
    // even going to be in the feed, so those are filtered out by the
    // first for-loop
    for file in fs::read_dir(dir_path)? {
        let de = file?;
        let name = de.file_name().into_string();
        let meta = de.metadata()?;
        let mode = meta.mode();
        let modified = meta.modified()?;

        // UNIXism? is this portable?
        if mode & 0o004 == 0 {
            continue; // not world readable
        }

        if !meta.is_file() {
            continue;
        }

        match name {
            Err(_) => continue,
            Ok(n) => {
                if !(n.ends_with(".gmi") || n.ends_with(".gemini")) {
                    continue;
                }
                if n.starts_with("index.") {
                    continue;
                }
                let moddate = DateTime::<Utc>::from(modified);

                files.push(NameDate(de.path(), n, moddate));
            }
        }
    }

    files.sort_by_key(|nd| nd.2); // sort by field 2, which is the date

    // get up to ten most recent entries
    let first = if files.len() > max {
        files.len() - max
    } else {
        0
    };

    let mut entries: Vec<Entry> = vec![];

    for file in files.drain(first ..) {
        let NameDate(path, name, moddate) = file;
        let id = base_url.clone() + &name; // TODO: urljoin
        let title = get_title(path);
        let link = BareLink { href: id.clone() };
        let updated = moddate.to_rfc3339();

        let e = Entry { id, title, updated, link };
        entries.push(e);
    }

    Ok(entries)
}

// this is for the <link> elements in the header, which help to find the
// overall feed, rather than individual entries
//
// in the usual course of operations, the output of this program would be
// saved in the "atom.xml" file referred to
fn get_links(base_url: String) -> Vec<Link> {
    let l1href = base_url.clone() + "atom.xml";

    let l1 = Link {
        href: String::from(l1href),
        rel: String::from("self")
    };
    let l2 = Link {
        href: base_url,
        rel: String::from("alternate")
    };

    let links = vec![l1, l2];
    links
}

// make the top-level <feed> element
pub fn get_feed(base_url: String, feed_dir: String, title: String)
            -> Result<Feed, std::io::Error> {
    let xmlns = String::from(XMLNS);
    let id    = base_url.clone();
    let links = get_links(base_url.clone());

    let current_time            = SystemTime::now();
    let datetime: DateTime<Utc> = DateTime::<Utc>::from(current_time);
    let updated: String         = datetime.to_rfc3339();

    let dir = PathBuf::from(feed_dir);

    let generator = get_generator();

    let entries = get_entries(dir, MAX_ENTRIES, base_url)?;

    let f = Feed {
        xmlns,
        id,
        title,
        updated,
        links,
        generator,
        entry: entries
    };

    Ok(f)
}
