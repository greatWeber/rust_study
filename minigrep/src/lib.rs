use std::{error::Error, fs, env};

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}


impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len()<3 {
         return Err("not enough arguments");
      }
      let config = Config {
          query: args[1].clone(),
          file_path: args[2].clone(),
          ignore_case: env::var("IGNORE_CASE").is_ok(),
      };
      Ok(config)
  }

  pub fn run(config: &Config) -> Result<(),Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    println!("file content:\n {content}");
    // let rs = search(&config.query, &content);
    let rs = if config.ignore_case {
      search_insensitive(&config.query,&content)
    }else {
      search(&config.query, &content)
    };
    println!("query result:\n {:?}", rs);
    Ok(())
  }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      result.push(line.trim());
    }
  }
  result
}

/**
 * 不区分大小写搜索
 */
pub fn search_insensitive<'a>(query: & str, content: &'a str) -> Vec<&'a str>{
  let query = query.to_lowercase();
  let mut result = Vec::new();
  for line in content.lines(){
    if line.to_lowercase().contains(&query) {
      result.push(line.trim());
    }
  }
  result

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_result(){
    let query = "hello";
    let content = "hello \n world";
    assert_eq!(vec!["hello"],search(query,content));
  }

  #[test]
  fn search_insensitive_case(){
    let query = "Hello";
    let content = "hello \n world";
    assert_eq!(vec!["hello"],search_insensitive(query,content));
  }
}