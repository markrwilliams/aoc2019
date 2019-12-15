extern crate image;

use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
struct Move {
    direction: Direction,
    distance: i64,
}

impl Move {
    fn parse(input: &str) -> Move {
        Move {
            direction: match input.chars().nth(0).unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("unknown move"),
            },
            distance: input[1..].parse().unwrap(),
        }
    }

    fn go(&self, panel: &mut Panel, wire_id: i8, steps: &mut i64) {
        let (cycle_x, cycle_y) = ([panel.ox], [panel.oy]);
        let (xs, ys): (Box<dyn Iterator<Item = i64>>, Box<dyn Iterator<Item = i64>>) =
            match self.direction {
                Direction::Up => (
                    Box::new(cycle_x.iter().copied().cycle()),
                    Box::new((panel.oy - self.distance..panel.oy).rev()),
                ),
                Direction::Down => (
                    Box::new(cycle_x.iter().copied().cycle()),
                    Box::new(panel.oy + 1..panel.oy + self.distance + 1),
                ),
                Direction::Left => (
                    Box::new((panel.ox - self.distance..panel.ox).rev()),
                    Box::new(cycle_y.iter().copied().cycle()),
                ),
                Direction::Right => (
                    Box::new(panel.ox + 1..panel.ox + self.distance + 1),
                    Box::new(cycle_y.iter().copied().cycle()),
                ),
            };

        for (x, y) in xs.zip(ys) {
            panel.ox = x;
            panel.oy = y;
            *steps += 1;
            let cell = &mut panel
                .grid
                .entry((panel.ox, panel.oy))
                .or_insert_with(|| HashMap::new());
            cell.entry(wire_id).or_insert(*steps);
            match cell
                .iter()
                .find(|(other_wire_id, _)| **other_wire_id != wire_id)
            {
                Some((_, other_wire_steps)) => {
                    panel
                        .overlaps
                        .insert((panel.ox, panel.oy), *steps + *other_wire_steps);
                }
                None => (),
            };
        }
    }
}

#[derive(PartialEq, Debug)]
struct Wire {
    pub path: Vec<Move>,
}

impl Wire {
    fn parse(input: &str) -> Wire {
        Wire {
            path: input.split(",").map(Move::parse).collect(),
        }
    }
}

#[aoc_generator(day3)]
fn parse_coordinates(input: &str) -> Vec<Wire> {
    input.lines().map(Wire::parse).collect()
}

#[derive(PartialEq, Debug)]
struct Panel {
    ox: i64,
    oy: i64,
    grid: HashMap<(i64, i64), HashMap<i8, i64>>,
    overlaps: HashMap<(i64, i64), i64>,
}

impl Panel {
    fn new(wires: &[Wire]) -> Panel {
        let mut panel = Panel {
            ox: 0,
            oy: 0,
            grid: HashMap::new(),
            overlaps: HashMap::new(),
        };

        for (i, wire) in wires.iter().enumerate() {
            let mut steps = 0;
            for m in wire.path.iter() {
                m.go(&mut panel, i as i8, &mut steps);
            }
            panel.ox = 0;
            panel.oy = 0;
        }
        return panel;
    }

    fn closest(&self) -> i64 {
        let ox = self.ox as i64;
        let oy = self.oy as i64;
        self.overlaps
            .iter()
            .map(|((x, y), _)| (*x - ox).abs() + (*y - oy).abs())
            .min()
            .expect("no min?")
    }

    fn fewest(&self) -> i64 {
        self.overlaps
            .iter()
            .map(|((_, _), steps)| *steps)
            .min()
            .expect("no min?")
    }

    fn png(&self, path: &str) {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            i64::max_value(),
            i64::max_value(),
            i64::min_value(),
            i64::min_value(),
        );

        for ((x, y), _) in self.grid.iter() {
            min_x = cmp::min(min_x, *x);
            max_x = cmp::max(max_x, *x);
            min_y = cmp::min(min_y, *y);
            max_y = cmp::max(max_y, *y);
        }

        let mut img: image::RgbImage = image::ImageBuffer::new(
            (max_x - min_x + 1).abs() as u32,
            (max_y - min_y + 1).abs() as u32,
        );

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            // translate coordinates
            let translated_x = x as i64 + min_x;
            let translated_y = y as i64 + min_y;
            *pixel = image::Rgb(match self.grid.get(&(translated_x, translated_y)) {
                Some(_) => [255, 255, 255],
                None => [0, 0, 0],
            });
        }

        img.save(path).unwrap();
    }
}

#[aoc(day3, part1)]
fn day3_part1(wires: &[Wire]) -> i64 {
    Panel::new(wires).closest()
}

#[aoc(day3, part2)]
fn day3_part2(wires: &[Wire]) -> i64 {
    Panel::new(wires).fewest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        assert_eq!(
            parse_coordinates("R75"),
            vec![Wire {
                path: vec![Move {
                    direction: Direction::Right,
                    distance: 75
                }]
            }],
        )
    }

    #[test]
    fn example2_parse() {
        use Direction::*;
        assert_eq!(
            parse_coordinates(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            vec![
                Wire {
                    path: vec![
                        Move {
                            direction: Right,
                            distance: 75
                        },
                        Move {
                            direction: Down,
                            distance: 30
                        },
                        Move {
                            direction: Right,
                            distance: 83
                        },
                        Move {
                            direction: Up,
                            distance: 83
                        },
                        Move {
                            direction: Left,
                            distance: 12
                        },
                        Move {
                            direction: Down,
                            distance: 49
                        },
                        Move {
                            direction: Right,
                            distance: 71
                        },
                        Move {
                            direction: Up,
                            distance: 7
                        },
                        Move {
                            direction: Left,
                            distance: 72
                        }
                    ]
                },
                Wire {
                    path: vec![
                        Move {
                            direction: Up,
                            distance: 62
                        },
                        Move {
                            direction: Right,
                            distance: 66
                        },
                        Move {
                            direction: Up,
                            distance: 55
                        },
                        Move {
                            direction: Right,
                            distance: 34
                        },
                        Move {
                            direction: Down,
                            distance: 71
                        },
                        Move {
                            direction: Right,
                            distance: 55
                        },
                        Move {
                            direction: Down,
                            distance: 58
                        },
                        Move {
                            direction: Right,
                            distance: 83
                        }
                    ]
                }
            ],
        )
    }

    #[test]
    fn example1_dist() {
        let p = Panel::new(&parse_coordinates(
            "R8,U5,L5,D3
U7,R6,D4,L4",
        ));
        p.png("/tmp/example1.png");
        assert_eq!(p.closest(), 6);
    }

    #[test]
    fn example2_dist() {
        let p = Panel::new(&parse_coordinates(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        ));
        assert_eq!(p.closest(), 159);
    }

    #[test]
    fn example3_dist() {
        let p = Panel::new(&parse_coordinates(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ));
        p.png("/tmp/example2.png");
        assert_eq!(p.closest(), 135);
    }

    #[test]
    fn example1_part2_fewest() {
        let p = Panel::new(&parse_coordinates(
            "R8,U5,L5,D3
U7,R6,D4,L4",
        ));
        assert_eq!(p.fewest(), 30);
    }
}
