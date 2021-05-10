use chrono::prelude::*;
use color_eyre::eyre::{Result, eyre};
use glob::glob;
use std::{cmp::Ordering, fs};
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Post {
    pub name: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub body: String,
    pub html_body: String
}

impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

pub fn load(dir: &str) -> Result<Vec<Post>> {
    let mut res: Vec<Post> = Vec::new();
    for path in glob(&format!("{}/*.md", dir))?.filter_map(Result::ok) {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let body = fs::read_to_string(path)?;
        let post = parse(filename, body)?;
        res.push(post);
    }
    Ok(res)
}

pub fn parse(filename: String, body: String) -> Result<Post> {
    let mut start_header = false;
    let mut end_header = false;
    let lines = body.lines();
    let mut date: DateTime<FixedOffset> = DateTime::from(Utc::now());
    let mut title: String = String::new();
    let mut body: String = String::new();
    for line in lines {
        match line {
            "---" => {
                if !start_header {
                    start_header = true;
                }
                else if start_header && !end_header {
                    end_header = true;
                }
            }
            _ => {
                if start_header && !end_header {
                    let split: Vec<&str> = line.split(":").collect();
                    match split[0] {
                        "title" => {
                            title = split.into_iter()
                                .skip(1)
                                .flat_map(|f | f.chars())
                                .collect();
                        }
                        "date" => {
                            let parsed_date = NaiveDate::parse_from_str(split[1].trim(), "%Y-%m-%d")?;
                            date = DateTime::<Utc>::from_utc(
                                NaiveDateTime::new(parsed_date, NaiveTime::from_hms(0, 0, 0)),
                                Utc,
                            )
                            .with_timezone(&Utc)
                            .into();
                        }
                        _ => {
                            return Err(eyre!("Unsupported metadata"))
                        }
                    }
                }
                else if start_header && end_header {
                    body.push_str(&format!("\n{}", line));
                }
            }
        }
    }
    let html_body = markdown_to_html(&body, &ComrakOptions::default());
    Ok(Post {
        name: filename,
        title: title,
        date: date,
        body: body,
        html_body: html_body
    })
}