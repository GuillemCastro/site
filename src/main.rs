
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use crate::templates::{blogpost_html, Html, index_html, blogindex_html};

use color_eyre::eyre::{Result};
use std::{fs::{self, File}, path::PathBuf};
use std::env;
use fs_extra::{self, dir::CopyOptions};
use std::path::{Path};
use structopt::StructOpt;
use rouille::Request;
use rouille::Response;

pub mod post;

#[derive(StructOpt, Debug)]
#[structopt(name = "site")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    Build,
    Serve
}

fn build_blog() -> Result<()> {
    fs::create_dir("public/blog")?;
    let posts = post::load("blog")?;
    let mut buf = Vec::new();
    blogindex_html(&mut buf, posts.clone())?;
    let blogindex = String::from_utf8(buf)?;
    fs::write("public/blog/index.html", blogindex)?;
    for post in posts {
        let mut buf = Vec::new();
        blogpost_html(&mut buf, post.clone(), Html(post.html_body))?;
        let blogpost = String::from_utf8(buf)?;
        let path = format!("public/blog/{}", post.name);
        fs::create_dir(path.clone())?;
        fs::write(format!("{}/index.html", path), blogpost)?;
    }
    Ok(())
}

fn build_main() -> Result<()> {
    let mut buf = Vec::new();
    index_html(&mut buf)?;
    let index = String::from_utf8(buf)?;
    fs::write("public/index.html", index)?;
    Ok(())
}

fn build() -> Result<()> {
    if Path::new("public").exists() {
        fs::remove_dir_all("public")?;
    }
    fs::create_dir("public")?;
    let copy_options = CopyOptions::new();
    fs_extra::dir::copy("static", "public", &copy_options)?;
    fs_extra::dir::copy("css", "public", &copy_options)?;
    build_main()?;
    build_blog()?;
    Ok(())
}

fn serve() -> Result<()> {
    println!("Serving at 0.0.0.0:1337 (available at http://localhost:1337)");
    rouille::start_server("0.0.0.0:1337", move |request| {
        let url = request.url();
        let path = url.strip_prefix("/").unwrap();
        let mut response = rouille::match_assets(&request, "public");
        if response.is_error() && Path::new("public").join(path).is_dir()
            && Path::new("public").join(path).join("index.html").exists() {
            let file = File::open(Path::new("public").join(path).join("index.html")).unwrap();
            response = Response::from_file("text/html; charset=utf8", file);
        }
        println!("{} {} {}", request.method(), request.url(), response.status_code);
        return response;
    });
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Build => {
            build()?;
        }
        Command::Serve => {
            serve()?;
        }
    }
    Ok(())
}