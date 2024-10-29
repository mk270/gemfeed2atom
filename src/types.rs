/*  gemfeed2atom: generates Atom feeds from gemlog directories

    by Martin Keegan

    To the extent (if any) permissible by law, Copyright (C) 2024  Martin Keegan

    NOTE: the contents of this file are unlikely to be subject to copyright
    in the United States or other jurisdictions with similar laws, as the
    material's form is semi-mechanically determined by a technical standard,
    rather than an independent intellectual creation.

    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/


use yaserde_derive::YaSerialize;

// used for the <link> elements in the header
#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "link")]
pub struct Link {
    #[yaserde(attribute)]
    pub href: String,
    #[yaserde(attribute)]
    pub rel: String
}

// used for the <link> elements in the individual gemlog post entries
#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "link")]
pub struct BareLink {
    #[yaserde(attribute)]
    pub href: String
}

#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "generator")]
pub struct Generator {
    #[yaserde(attribute)]
    pub uri: String,
    #[yaserde(attribute)]
    pub version: String,
    #[yaserde(text)]
    pub contents: String
}

#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "entry")]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub updated: String,
    pub link: BareLink
}

#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "feed")]
pub struct Feed {
    #[yaserde(attribute)]
    pub xmlns: String,
    pub id: String,
    pub title: String,
    pub updated: String,
    pub links: Vec<Link>,
    pub generator: Generator,
    pub entry: Vec<Entry> //sic: the singular needs to be the name of each tag
}
