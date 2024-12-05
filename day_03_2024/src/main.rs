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

    let total = part_one(&file_contents);

    let enabled_total = part_two(&file_contents);

    println!("Day 3 2024: sum: {total}, enabled: {enabled_total}");
    Ok(())
}

fn part_one<S: AsRef<str>>(s: S) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(s.as_ref()).map(|c| c.extract()) {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }
    total
}

fn part_two<S: AsRef<str>>(s: S) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't?\(\)|do?\(\)").unwrap();
    let mut tabulate = true;
    let mut total = 0;
    for (full, _) in re.captures_iter(s.as_ref()).map(|c| c.extract::<0>()) {
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
                    total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert_eq!(part_one(sample), 161);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample2.txt")?;
        let result = part_two(sample);
        assert_eq!(result, 48);
        Ok(())
    }
}
