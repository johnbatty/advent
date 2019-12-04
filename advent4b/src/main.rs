use failure::Error;
use itertools::Itertools;

fn last_digit(n: u32) -> u32 {
    n % 10
}

fn last_but_one_digit(n: u32) -> u32 {
    (n / 10) % 10
}

fn digits_never_decrease(n: u32) -> bool {
    if n == 0 {
        true
    } else {
        (last_digit(n) >= last_but_one_digit(n)) && digits_never_decrease(n / 10)
    }
}

fn has_pair_of_digits(n: u32) -> bool {
    n.to_string()
        .chars()
        .map(|c| c.to_string())
        .coalesce(|x, y| {
            if x.ends_with(&y) {
                Ok(x + &y)
            } else {
                Err((x, y))
            }
        })
        .any(|s| s.len() == 2)
}

fn main() -> Result<(), Error> {
    let input = "183564-657474";
    let range: Vec<u32> = input
        .split('-')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();
    let start = range[0];
    let end = range[1];

    let c = (start..=end)
        .filter(|n| digits_never_decrease(*n))
        .filter(|n| has_pair_of_digits(*n))
        .count();

    println!("c: {}", c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_pair_of_digits() {
        let test_data = [
            (123456, false),
            (121212, false),
            (113456, true),
            (122456, true),
            (123356, true),
            (123446, true),
            (123455, true),
            (555555, false),
            (122234, false),
            (122244, true),
        ];

        for (n, expected) in &test_data {
            assert_eq!(has_pair_of_digits(*n), *expected);
        }
    }

    #[test]
    fn test_digits_never_decrease() {
        let test_data = [
            (955555, false),
            (123454, false),
            (121212, false),
            (113456, true),
            (122456, true),
            (123356, true),
            (123446, true),
            (123455, true),
            (555555, true),
        ];

        for (n, expected) in &test_data {
            assert_eq!(digits_never_decrease(*n), *expected);
        }
    }
}
