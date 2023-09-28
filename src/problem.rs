use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{bail, Context, Result};
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

        fs::write(path.join("problem.toml"), contents)?;
        fs::write(path.join("solution.py"), "")?;

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

    pub fn read(id: &str) -> Result<Problem> {
        let path = Path::new("./problems").join(id).join("problem.toml");
        let data = fs::read(path)?;
        let problem: Problem = toml::from_slice(&data)?;

        if problem.id != id {
            bail!(
                "Cannot read problem info: id mismatch ({} != {})",
                id,
                problem.id
            );
        }

        Ok(problem)
    }

    pub fn run(&self, input: &str) -> Result<String> {
        let mut child = Command::new("python")
            .arg(self.get_solution_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to spawn child process")?;

        let input = input.to_string();

        let mut stdin = child.stdin.take().context("Failed to open stdin")?;
        std::thread::spawn(move || {
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        });

        let output = child.wait_with_output().context("Failed to read stdout")?;
        let string = String::from_utf8(output.stdout)?;

        Ok(string)
    }

    pub fn run_examples(&self) -> Result<()> {
        for (i, example) in self.examples.iter().enumerate() {
            let result = self.run(&example.input)?;
            println!("[example{}]", i + 1);
            println!("< input\n{}", example.input.trim());
            println!("= expected output\n{}", example.output.trim());
            println!("> output\n{}", result.trim());
            if result == example.output {
                println!("[Passed]");
            } else {
                println!("[Failed]");
            }
        }

        Ok(())
    }

    fn get_path(&self) -> PathBuf {
        Path::new("./problems").join(&self.id)
    }
    fn get_solution_path(&self) -> PathBuf {
        self.get_path().join("solution.py")
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
