use regex::Regex;
use std::io::{self, BufRead};

fn main()
{
    let re = Regex::new(r"\((\S*(?: +\S+)*) +\d{4}").unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines()
    {
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();

        println!("'{}'", &cap[1]);
    }
}
