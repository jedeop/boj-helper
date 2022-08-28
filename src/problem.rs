use std::{fs, path::Path};

use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    id: String,
    url: String,
    title: String,
    examples: Vec<Example>,
}

impl Problem {
    pub async fn create(id: &str) -> Result<()> {
        let problem = Problem::get(id).await?;

        let path = Path::new("./problems").join(id);

        fs::create_dir_all(&path)?;

        let contents = toml::to_string(&problem).unwrap();

        fs::write(&path.join("problem.toml"), contents)?;
        fs::write(&path.join("solution.py"), "")?;

        Ok(())
    }

    async fn get(id: &str) -> Result<Problem> {
        let url = format!("https://www.acmicpc.net/problem/{}", id);

        let body = reqwest::get(&url).await?.text().await?;

        let document = Html::parse_document(&body);

        let title_selector = Selector::parse("#problem_title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .context("no title element")?
            .text()
            .collect::<String>();

        let ex_in_selector = Selector::parse("pre[id^='sample-input']").unwrap();
        let ex_in = document
            .select(&ex_in_selector)
            .map(|el| el.text().collect::<String>());

        let ex_out_selector = Selector::parse("pre[id^='sample-output']").unwrap();
        let ex_out = document
            .select(&ex_out_selector)
            .map(|el| el.text().collect::<String>());

        let examples = ex_in.zip(ex_out).map(Example::new).collect();

        Ok(Problem {
            id: id.to_string(),
            url,
            title,
            examples,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    input: String,
    output: String,
}

impl Example {
    fn new(e: (String, String)) -> Example {
        Example {
            input: e.0,
            output: e.1,
        }
    }
}
