use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file to read
    #[clap(default_value = "day_01_2024/input.txt")]
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.input)?;

    let total_distance = part_one(&file_contents);

    let similarity_score = part_two(&file_contents);

    println!("Day 1 2024: Total distance: {total_distance} Similarity score: {similarity_score}");
    Ok(())
}

fn part_one<S: AsRef<str>>(s: S) -> u32 {
    let (list_1, list_2) = parse(s.as_ref());
    std::iter::zip(&list_1, &list_2).fold(0, |total, (a, b)| total + u32::abs_diff(*a, *b))
}

fn part_two<S: AsRef<str>>(s: S) -> u32 {
    let (list_1, list_2) = parse(s.as_ref());
    list_1.iter().fold(0, |total, a| {
        let num_a_in_list_2 = list_2.iter().filter(|x| x == &a).count() as u32;
        total + (a * num_a_in_list_2)
    })
}

fn parse<S: AsRef<str>>(s: S) -> (Vec<u32>, Vec<u32>) {
    let mut list_1 = vec![];
    let mut list_2 = vec![];
    for line in s.as_ref().lines() {
        let (num_1, num_2) = line
            .trim()
            .split_once(' ')
            .expect("Expected 2 number each line separated by spaces");
        list_1.push(num_1.trim().parse::<u32>().expect("Expected numbers"));
        list_2.push(num_2.trim().parse::<u32>().expect("Expected numbers"));
    }
    list_1.sort();
    list_2.sort();
    (list_1, list_2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert!(part_one(sample) == 11);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert!(part_two(sample) == 31);
        Ok(())
    }
}
