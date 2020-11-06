use std::env;
use std::error::Error;
use std::fs;

const HELP_TEXT: &str = "<query> <file> [-i]";
const UNSUFFICIENT_ARGUMENTS: &str = "Unsufficient arguments";
const UNSUPPORTED_OPTION: &str = "Unsupported option";
const UNEXPECTED_ARGUMENT: &str = "Unexpected argument";

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, String> {
    let programname = match args.next() {
      Some(f) => f,
      None => panic!("Unexpected error"),
    };
    let arguments_err = Err(format!(
      "{prog}: {err}\nUsage: {prog} {help}",
      prog = programname,
      err = UNSUFFICIENT_ARGUMENTS,
      help = HELP_TEXT
    ));
    let query = match args.next() {
      Some(q) => q,
      None => return arguments_err,
    };
    let filename = match args.next() {
      Some(f) => f,
      None => return arguments_err,
    };

    let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    for arg in args {
      // Parse argument options
      let mut arg_chars = arg.chars();
      let first_token = arg_chars.next().unwrap();
      if first_token == '-' {
        for option in arg_chars {
          match option {
            'i' => case_sensitive = false,
            _ => {
              return Err(format!(
                "{prog}: {err} `{op}`\nUsage: {prog} {help}",
                prog = programname,
                err = UNSUPPORTED_OPTION,
                op = option,
                help = HELP_TEXT
              ));
            }
          }
        }
      } else {
        return Err(format!(
          "{prog}: {err} {first_token}{rest}\nUsage: {prog} {help}",
          prog = programname,
          err = UNEXPECTED_ARGUMENT,
          first_token = first_token,
          rest = arg_chars.as_str(),
          help = HELP_TEXT
        ));
      }
    }

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
