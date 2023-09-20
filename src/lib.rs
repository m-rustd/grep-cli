use std::{env, error, fs};

#[derive(Debug)]
pub struct Config {
  pub query: String, // 查询关键词
  pub filename: String, // 文件名
  pub case_sensitive: bool // 是否区分大小写，默认区分
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result;
    if config.case_sensitive {
        result = search(&config.query, &contents)
    } else {
        result = search_case_insensitive(&config.query, &contents)
    }
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

// 区分大小写
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 忽略大小写
pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines() {
      if line.to_lowercase().contains(&query) {
          results.push(line);
      }
  }
  results
}