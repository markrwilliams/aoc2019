#[aoc_generator(day1)]
fn parse_module_mass(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn fuel_for(mass: &i64) -> i64 {
    mass / 3 - 2
}

#[aoc(day1, part1)]
fn day1_part1(masses: &[i64]) -> i64 {
    masses.iter().map(fuel_for).sum()
}

#[aoc(day1, part2)]
fn day1_part2(masses: &[i64]) -> i64 {
    let fuel_masses: Vec<i64> = masses.iter().map(fuel_for).collect();
    fuel_masses
        .iter()
        .map(|fuel| {
            let mut remaining = *fuel;
            let mut total = 0;
            while remaining > 0 {
                total += remaining;
                remaining = fuel_for(&remaining);
            }
            total
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(day1_part2(&parse_module_mass("14")), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(day1_part2(&parse_module_mass("1969")), 966);
    }

    #[test]
    fn example3() {
        assert_eq!(day1_part2(&parse_module_mass("100756")), 50346);
    }
}
