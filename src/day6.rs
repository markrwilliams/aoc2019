use std::collections;

struct Orbit {
    object: String,
    orbiting: String,
}

#[aoc_generator(day6)]
fn parse_orbits(input: &str) -> Vec<Orbit> {
    input
        .lines()
        .filter(|line| line.len() > 1)
        .map(|line| {
            let parsed: Vec<&str> = line.split(")").collect();
            Orbit {
                object: String::from(parsed[1]),
                orbiting: String::from(parsed[0]),
            }
        })
        .collect()
}

#[aoc(day6, part1)]
fn day6_part1(orbits: &[Orbit]) -> usize {
    Orbits::new(orbits).total()
}

#[aoc(day6, part2)]
fn day6_part2(orbits: &[Orbit]) -> usize {
    Orbits::new(orbits).transfers("YOU", "SAN")
}

struct Orbits<'a> {
    objs: collections::HashSet<&'a str>,
    adj: collections::HashMap<&'a str, &'a str>,
}

impl<'a> Orbits<'a> {
    fn new(orbits: &[Orbit]) -> Orbits {
        let mut objs = collections::HashSet::new();
        let mut adj = collections::HashMap::new();

        for orbit in orbits {
            objs.insert(orbit.object.as_str());
            objs.insert(orbit.orbiting.as_str());
            if adj
                .insert(orbit.object.as_str(), orbit.orbiting.as_str())
                .is_some()
            {
                panic!("{} has a value!", orbit.object);
            }
        }

        Orbits { objs, adj }
    }

    fn direct(&self, obj: &str) -> Option<&str> {
        self.adj.get(obj).map(|r| *r)
    }

    fn closure(&self, obj: &str) -> Vec<&str> {
        let mut closure: Vec<&str> = Vec::new();

        let mut obj = obj;

        while match self.direct(obj) {
            None => false,
            Some(next_obj) => {
                closure.push(next_obj);
                obj = next_obj;
                true
            }
        } {}

        closure
    }

    fn indirect(&self, obj: &str) -> Vec<&str> {
        let mut indirect = self.closure(obj);

        // remove the direct orbit
        if indirect.len() > 1 {
            indirect.drain(0..1);
        }

        indirect
    }

    fn total(&self) -> usize {
        self.objs
            .iter()
            .map(|o|
                 match self.direct(o) {
                    Some(_) => 1,
                    None => 0,
                 } + self.indirect(o).iter().count())
            .sum::<usize>() - 1
    }

    fn debug(&self) {
        for o in self.objs.iter() {
            println!(
                "{} directly orbits {}",
                o,
                match self.direct(o) {
                    None => "Nothing",
                    Some(other) => other,
                }
            );
            println!("{} indirectly orbits {:?}", o, self.indirect(o));
        }
    }

    fn path(&self, a: &str, b: &str) -> Vec<&str> {
        let a_closure = self.closure(a);
        let b_closure = self.closure(b);
        let a_ridx = (0..a_closure.len()).rev();
        let b_ridx = (0..b_closure.len()).rev();

        let mut path: Vec<&str> = Vec::new();

        for (a_ri, b_ri) in a_ridx.zip(b_ridx) {
            if a_closure[a_ri] != b_closure[b_ri] {
                let least_common_ancester = a_closure[a_ri + 1];
                path.extend(a_closure[..a_ri + 1].iter());
                path.push(least_common_ancester);
                path.extend(b_closure[..b_ri + 1].iter());
                break;
            }
        }

        path
    }

    fn transfers(&self, a: &str, b: &str) -> usize {
        self.path(a, b).windows(2).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_1() -> Vec<Orbit> {
        parse_orbits(
            "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
",
        )
    }

    #[test]
    fn test_example_1() {
        let example = example_1();
        let orbits = Orbits::new(&example);

        assert_eq!(orbits.direct("D").unwrap(), "C");
        assert_eq!(orbits.indirect("D"), vec!["B", "COM"]);

        assert_eq!(orbits.direct("L").unwrap(), "K");
        assert_eq!(orbits.indirect("L"), vec!["J", "E", "D", "C", "B", "COM"]);

        assert_eq!(orbits.direct("COM"), None);
        assert_eq!(orbits.indirect("COM"), Vec::<&str>::new());

        orbits.debug();
        assert_eq!(orbits.total(), 42);
    }

    fn example_2() -> Vec<Orbit> {
        parse_orbits(
            "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
",
        )
    }

    #[test]
    fn test_example_2() {
        let example = example_2();
        let orbits = Orbits::new(&example);

        //                           YOU
        //                          /
        //         G - H       J - K - L
        //        /           /
        // COM - B - C - D - E - F
        //                \
        //                 I - SAN

        assert_eq!(orbits.path("YOU", "SAN"), vec!["K", "J", "E", "D", "I"]);
        assert_eq!(orbits.transfers("YOU", "SAN"), 4);
    }
}
