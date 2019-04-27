use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main()
{
    let re = Regex::new(r"\((\S*(?: +\S+)*) +\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
    let mut line_counts: HashMap<String, u32> = HashMap::new();

    // loop through stdin lines
    let stdin = io::stdin();
    for line in stdin.lock().lines()
    {
        // get name
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();
        let name: String = cap[1].to_string();

        // increment count
        if let Some(count) = line_counts.get_mut(&name)
        {
            *count += 1;
        }
        else
        {
            line_counts.insert(name, 1);
        }
    }

    for (key, val) in line_counts.iter()
    {
        println!("{}: {}", key, val);
    }
}
