use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file to read
    #[clap(default_value = "day_05_2024/input.txt")]
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.input)?;

    let middle_sum = part_one(&file_contents);

    let disordered_sum = part_two(&file_contents);

    println!("Day 5 2024: Part 1: {middle_sum}, Part 2: {disordered_sum}");
    Ok(())
}

fn part_one<S: AsRef<str>>(s: S) -> u32 {
    let (rules, updates) = parse(&s);
    let in_order_updates = updates.iter().filter(|update| in_order(&rules, &update));
    let mut sum = 0;
    for update in in_order_updates {
        sum += update[update.len() / 2];
    }
    sum
}

fn part_two<S: AsRef<str>>(s: S) -> u32 {
    let (rules, updates) = parse(&s);
    let out_of_order_updates = updates.iter().filter(|update| !in_order(&rules, &update));
    let mut sum = 0;
    for update in out_of_order_updates {
        let ordered_update = order(&rules, update.clone());
        sum += ordered_update[ordered_update.len() / 2];
    }
    sum
}

fn parse<S: AsRef<str>>(s: S) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, updates) = s.as_ref().split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .collect();
    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|n| n.trim().parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn in_order(rules: &[(u32, u32)], update: &[u32]) -> bool {
    for i in 0..update.len() {
        // make sure i + 1 isn't page a in any rules
        // make sure i - 1 isn't page b in any rules
        for (a, b) in rules {
            if (i < update.len() - 1 && &update[i] == b && &update[i + 1] == a)
                || (i > 0 && &update[i] == a && &update[i - 1] == b)
            {
                return false;
            }
        }
    }
    true
}

fn order(rules: &[(u32, u32)], mut update: Vec<u32>) -> Vec<u32> {
    update.sort_by(|a, b| {
        for (a1, b1) in rules {
            if a == a1 && b == b1 {
                return std::cmp::Ordering::Greater;
            }
            if a == b1 && b == a1 {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    });
    update
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert_eq!(part_one(sample), 143);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        let result = part_two(sample);
        assert_eq!(result, 123);
        Ok(())
    }
}
