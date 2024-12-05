use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file to read
    #[clap(default_value = "day_04_2024/input.txt")]
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.input)?;

    let xmas = part_one(&file_contents);

    let x_mas = part_two(&file_contents);

    println!("Day 4 2024: XMAS: {xmas}, X-MAS: {x_mas}");
    Ok(())
}

fn part_one<S: AsRef<str>>(s: S) -> u32 {
    let matrix: Vec<Vec<char>> = s.as_ref().lines().map(|x| x.chars().collect()).collect();

    let mut count = 0;

    let height = matrix.len();
    for n in 0..height {
        let width = matrix[n].len();
        for m in 0..width {
            if matrix[n][m] == 'X' {
                if n > 2
                    && m > 2
                    && matrix[n - 1][m - 1] == 'M'
                    && matrix[n - 2][m - 2] == 'A'
                    && matrix[n - 3][m - 3] == 'S'
                {
                    count += 1;
                }
                if n > 2
                    && matrix[n - 1][m] == 'M'
                    && matrix[n - 2][m] == 'A'
                    && matrix[n - 3][m] == 'S'
                {
                    count += 1;
                }
                if n > 2
                    && m < width - 3
                    && matrix[n - 1][m + 1] == 'M'
                    && matrix[n - 2][m + 2] == 'A'
                    && matrix[n - 3][m + 3] == 'S'
                {
                    count += 1;
                }
                if m > 2
                    && matrix[n][m - 1] == 'M'
                    && matrix[n][m - 2] == 'A'
                    && matrix[n][m - 3] == 'S'
                {
                    count += 1;
                }
                if m < width - 3
                    && matrix[n][m + 1] == 'M'
                    && matrix[n][m + 2] == 'A'
                    && matrix[n][m + 3] == 'S'
                {
                    count += 1;
                }
                if n < height - 3
                    && m > 2
                    && matrix[n + 1][m - 1] == 'M'
                    && matrix[n + 2][m - 2] == 'A'
                    && matrix[n + 3][m - 3] == 'S'
                {
                    count += 1;
                }
                if n < height - 3
                    && matrix[n + 1][m] == 'M'
                    && matrix[n + 2][m] == 'A'
                    && matrix[n + 3][m] == 'S'
                {
                    count += 1;
                }
                if n < height - 3
                    && m < width - 3
                    && matrix[n + 1][m + 1] == 'M'
                    && matrix[n + 2][m + 2] == 'A'
                    && matrix[n + 3][m + 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_two<S: AsRef<str>>(s: S) -> u32 {
    let matrix: Vec<Vec<char>> = s.as_ref().lines().map(|x| x.chars().collect()).collect();

    let mut count = 0;

    let height = matrix.len();
    for n in 0..height {
        let width = matrix[n].len();
        for m in 0..width {
            if n > 0 && m > 0 && n < height - 1 && m < width - 1 && matrix[n][m] == 'A' {
                if (matrix[n - 1][m - 1] == 'M' && matrix[n + 1][m + 1] == 'S'
                    || matrix[n + 1][m + 1] == 'M' && matrix[n - 1][m - 1] == 'S')
                    && (matrix[n + 1][m - 1] == 'M' && matrix[n - 1][m + 1] == 'S'
                        || matrix[n - 1][m + 1] == 'M' && matrix[n + 1][m - 1] == 'S')
                {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        assert_eq!(part_one(sample), 18);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> std::io::Result<()> {
        let sample = std::fs::read_to_string("./sample.txt")?;
        let result = part_two(sample);
        assert_eq!(result, 9);
        Ok(())
    }
}
