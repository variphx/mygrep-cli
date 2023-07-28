use std::{
    error::Error,
    fs,
    io::{self, Write},
};

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub is_case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2],
            is_case_sensitive: {
                if args.len() == 3 {
                    false
                } else if args[3] == "--case-sensitive" || args[3] == "-s" {
                    true
                } else if args[3] == "--case-insensitive" || args[3] == "-i" {
                    false
                } else {
                    return Err("No such parameters!");
                }
            },
        })
    }
}

struct Queried<'a> {
    index: usize,
    line: &'a str,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout().lock();
    let contents = fs::read_to_string(config.filename)?;

    let queried_holder = {
        if config.is_case_sensitive {
            search_case_sensitive(config.query, &contents)
        } else {
            search_case_insensitive(config.query, &contents)
        }
    };

    if queried_holder.is_empty() {
        writeln!(&mut stdout, "Found nothing!")?;
    } else {
        writeln!(&mut stdout, "Found:")?;
        for queried in queried_holder {
            writeln!(&mut stdout, "Line {}: {}", queried.index, queried.line)?;
        }
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<Queried<'a>> {
    let mut queried_holder = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if line.contains(query) {
            queried_holder.push(Queried { index, line });
        }
    }

    queried_holder
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Queried<'a>> {
    let mut queried_holder = Vec::new();
    let query = query.to_lowercase();

    for (index, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            queried_holder.push(Queried { index, line });
        }
    }

    queried_holder
}
