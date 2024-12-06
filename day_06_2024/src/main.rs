use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file to read
    #[clap(default_value = "day_06_2024/input.txt")]
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.input)?;

    let visited = part_one(&file_contents);

    let part_two = part_two(&file_contents);

    println!("Day 6 2024: Part 1: {visited}, Part 2: {part_two}");
    Ok(())
}

fn part_one<S: AsRef<str>>(s: S) -> u32 {
    0
}

fn part_two<S: AsRef<str>>(s: S) -> u32 {
    0
}

fn parse<S: AsRef<str>>(s: S) -> Vec<Vec<char>> {
    vec![vec![]]
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert_eq!(part_one(sample), 0);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        let result = part_two(sample);
        assert_eq!(result, 0);
        Ok(())
    }
}
