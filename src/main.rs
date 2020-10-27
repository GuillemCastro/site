
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use crate::templates::{blogpost_html, Html};

use color_eyre::eyre::{Result};

pub mod post;

fn main() -> Result<()> {
    color_eyre::install()?;
    let posts = post::load("blog")?;
    for post in posts {
        let mut buf = Vec::new();
        blogpost_html(&mut buf, post.clone(), Html(post.html_body))?;
        let blogpost = String::from_utf8(buf)?;
        println!("{}", blogpost);
    }
    Ok(())
}