
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use crate::templates::{blogpost_html, Html, index_html};

use color_eyre::eyre::{Result};

use std::fs;

pub mod post;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut buf = Vec::new();
    index_html(&mut buf)?;
    let index = String::from_utf8(buf)?;
    println!("{}", index);
    
    fs::write("index.html", index)?;

    let posts = post::load("blog")?;
    for post in posts {
        let mut buf = Vec::new();
        blogpost_html(&mut buf, post.clone(), Html(post.html_body))?;
        let blogpost = String::from_utf8(buf)?;
        println!("{}", blogpost);
    }
    Ok(())
}