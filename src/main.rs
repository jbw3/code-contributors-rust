use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn print_results(line_counts: &HashMap<String, u32>)
{
    // sort counts
    let mut sorted_counts: Vec<(&String, &u32)> = line_counts.iter().collect();
    sorted_counts.sort_by(|(_, count1), (_, count2)| count1.cmp(count2).reverse());

    // calculate total
    let mut total_count = 0u32;
    for (_, count) in sorted_counts.iter()
    {
        total_count += *count;
    }

    // print counts
    for (name, count) in sorted_counts.iter()
    {
        println!("{}: {}", name, count);
    }

    println!("Total: {}", total_count);
}

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

    print_results(&line_counts);
}
