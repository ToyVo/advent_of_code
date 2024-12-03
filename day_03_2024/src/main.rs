use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file to read
    #[clap(default_value = "day_03_2024/input.txt")]
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.input)?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(&file_contents).map(|c| c.extract()) {
        total += (a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap());
    }

    let enabled_re = Regex::new(r"mul\(\d+,\d+\)|don't?\(\)|do?\(\)").unwrap();
    let mut tabulate = true;
    let mut enabled_total = 0;
    for (full, _) in enabled_re
        .captures_iter(&file_contents)
        .map(|c| c.extract::<0>())
    {
        match full {
            "do()" => {
                tabulate = true;
            }
            "don't()" => {
                tabulate = false;
            }
            _ => {
                if tabulate {
                    let (a, b) = full[4..(full.len() - 1)].split_once(',').unwrap();
                    enabled_total += (a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap());
                }
            }
        }
    }

    println!("Day 3 2024: sum: {total}, enabled: {enabled_total}");
    Ok(())
}
