use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file to read
    #[clap(default_value = "day_02_2024/input.txt")]
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.input)?;

    let num_safe_reports = part_one(&file_contents);

    let dampened_safe_reports = part_two(&file_contents);

    println!("Day 2 2024: Safe Reports: {num_safe_reports} Safe Reports: {dampened_safe_reports}");
    Ok(())
}

fn part_one<S: AsRef<str>>(s: S) -> usize {
    let reports = parse(s.as_ref());
    reports
        .iter()
        .filter(|report| is_safe(report))
        .count()
}

fn part_two<S: AsRef<str>>(s: S) -> usize {
    let reports = parse(s.as_ref());
    reports
        .iter()
        .filter(|report| {
            if !is_safe(report) {
                // For each index i, create a vector without the element at index i
                for skip_index in 0..report.len() {
                    let mut new_vec = Vec::with_capacity(report.len() - 1);

                    // Add all elements except the one at skip_index
                    for (current_index, element) in report.iter().enumerate() {
                        if current_index != skip_index {
                            new_vec.push(element.clone());
                        }
                    }

                    if is_safe(&new_vec) {
                        return true;
                    }
                }
                return false;
            }
            true
        })
        .count()
}

fn parse<S: AsRef<str>>(s: S) -> Vec<Vec<u32>> {
    let mut reports = vec![];
    for line in s.as_ref().lines() {
        reports.push(line.split(' ').map(|x| x.parse::<u32>().unwrap()).collect());
    }
    reports
}

fn is_safe(s: &Vec<u32>) -> bool {
    let mut last_increasing = None;
    for i in 1..s.len() {
        let a = s[i - 1];
        let b = s[i];
        let increasing = a < b;
        let diff = a.abs_diff(b);
        if last_increasing.is_some_and(|increase| increasing != increase) || diff == 0 || diff > 3 {
            return false;
        }
        last_increasing = Some(increasing);
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert!(part_one(sample) == 2);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert!(part_two(sample) == 4);
        Ok(())
    }
}
