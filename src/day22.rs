use std::collections::{BTreeMap, HashMap, HashSet};
use itertools::Itertools;
use crate::common::load_from;

type Coord = (u32, u32, u32);

pub fn run_day() {
    let data = load_from("day22.txt");
    let gravity_bricks = prepare_bricks(data.as_str());
    println!("Part 1: {}", run_day22a(&gravity_bricks));
    println!("Part 2: {}", run_day22b(&gravity_bricks));
}

fn prepare_bricks(data: &str) -> Vec<Brick> {
    let mut bricks = parse_bricks(&data);
    sort_bricks_in_z(&mut bricks);
    apply_gravity(bricks).0
}

fn run_day22a(bricks: &Vec<Brick>) -> u32 {
    let (min_z_slice, max_z_slice) = gather_slices(bricks);

    // for each brick, see if the bricks above will fall because of it.
    let mut counter: u32 = 0;

    for (idx, slice) in max_z_slice {
        // get the slice above
        let bricks_above = min_z_slice.get(&(idx + 1));
        if let Some(ba) = bricks_above {
            // find each brick above that only has one brick below it, and return that brick.
            let unsafe_blocks = ba.iter().filter_map(|b| {
                // for each block in the bottom row, does it overlap the brick above?
                // If so, get it.
                let overlapping_under = slice.iter().filter(|below| b.xy_overlap(*below)).collect_vec();
                // If there is only one, log it.
                if overlapping_under.len() == 1 {
                    overlapping_under.first().map(|x| (**x).clone())
                } else {
                    None
                }
            }).sorted().dedup().count(); // then sort to allow deduplication, and count how many support one brick.
            counter += (slice.len() - unsafe_blocks) as u32;
        } else {
            counter += slice.len() as u32;
        }
    }

    counter
}

fn run_day22b(bricks: &Vec<Brick>) -> u64 {
    let mut changed = 0u64;
    // let (min_z_slice, max_z_slice) = gather_slices(bricks);
    for brick in bricks {
        let vec = bricks.iter().filter_map(|x| {
            if x == brick {
                None
            } else {
                Some(x.clone())
            }
        }).collect_vec();
        changed += apply_gravity(vec).1;
    }
    changed
}

// TODO: For later -- use dynamic programming for this.
// fn run_day22b(bricks: &Vec<Brick>) {
//     let map: HashMap<&Brick, HashSet<&Brick>> = HashMap::new();
//     let (min_z_slice, max_z_slice) = gather_slices(bricks);
//     for brick in bricks.iter().rev() {
//         let affected = affected_by_disintegration();
//     }
// }

fn affected_by_disintegration<'a>(target: &'a Brick, above: Vec<&'a Brick>) -> HashSet<&'a Brick> {
    above.iter().filter_map(|b| {
        if target.xy_overlap(*b) {
            Some(*b)
        } else {
            None
        }
    }).collect()
}

#[allow(dead_code)]
fn can_be_disintergrated<'a>(target: &'a Brick, current_row_bricks: Box<dyn Iterator<Item = &&'a Brick> + '_>, bricks_above: Box<dyn Iterator<Item = &&'a Brick> + '_>) -> bool {
    // All bricks in the same z level as the target brick, minus the target.
    let bottom_bricks: Vec<&Brick> = current_row_bricks.filter(|x| **x != target).map(|x| *x).collect_vec();
    // Bricks that are above target, then checks to see if each brick has another "bottom_brick" that it overlaps with.
    let bricks_above_that_overlap_target = bricks_above.filter(|top_brick| top_brick.xy_overlap(target)).map(|x| *x).collect_vec(); // only bricks that overlap the target

    // We have each brick that sits on the target. We now need to figure out if any of the bricks have any other overlaps.
    // If ALL of them do, then we can disintegrate, otherwise, we do not.
    // Therefore, we filter on if a brick DOES NOT have an overlap -- if any do not, then we return something,
    // and that means do not disintegrate.
    bricks_above_that_overlap_target
        .iter()
        .filter(|x| bottom_bricks.iter().all(|g| !x.xy_overlap(g))) // if any overlap, that means it's supported. We want any that are not.
        .next() // blocks that are not supported
        .is_none() // If we don't get anything, nothing will fall, so disintegrate.
}

fn gather_slices<'a>(bricks: &'a Vec<Brick>) -> (BTreeMap<u32, Vec<&'a Brick>>, BTreeMap<u32, Vec<&'a Brick>>) {
    let mut min_z_slice = BTreeMap::<u32, Vec<&'a Brick>>::new();
    let mut max_z_slice = BTreeMap::<u32, Vec<&'a Brick>>::new();
    for brick in bricks {
        min_z_slice.entry(brick.min.2).and_modify(|vec| vec.push(brick)).or_insert(Vec::from([brick]));
        max_z_slice.entry(brick.max.2).and_modify(|vec| vec.push(brick)).or_insert(Vec::from([brick]));
    }
    (min_z_slice, max_z_slice)
}

fn parse_coord(coord: &str) -> Coord {
    let mut c = coord.split(",");
    (
        c.next().unwrap().parse().unwrap(),
        c.next().unwrap().parse().unwrap(),
        c.next().unwrap().parse().unwrap()
    )
}

fn parse_bricks(data: &str) -> Vec<Brick> {
    data.lines()
        .map(|line| {
            line.split_once("~").map(|(first, second)| {
                let a = parse_coord(first);
                let b = parse_coord(second);
                Brick {
                    min: (a.0.min(b.0), a.1.min(b.1), a.2.min(b.2)),
                    max: (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2))
                }
            }).unwrap()
        })
        .collect()
}

fn sort_bricks_in_z(bricks: &mut Vec<Brick>) {
    bricks.sort_unstable_by_key(|Brick { min, max: _ }|  min.2)
}

fn apply_gravity(bricks: Vec<Brick>) -> (Vec<Brick>, u64) {
    let mut result: Vec<Brick> = Vec::new();
    let mut changed = 0u64;
    for brick in bricks {
        if brick.min.2 == 1 {
            // do nothing if it sits on the bottom.
            result.push(brick.to_owned());
        } else {
            let old_z = brick.min.2;
            let new_z = result.iter_mut().filter_map(|x| brick.would_fall_above(x)).max().unwrap_or(1);
            if old_z != new_z {
                changed += 1;
            }
            result.push(brick.drop_to(new_z))
        }
    }

    sort_bricks_in_z(&mut result);
    (result, changed)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
struct Brick {
    min: Coord,
    max: Coord
}

impl Brick {
    pub fn drop_to(&self, new_z: u32) -> Brick {
        Brick {
            min: (self.min.0, self.min.1, new_z),
            max: (self.max.0, self.max.1, self.max.2 - self.min.2 + new_z)
        }
    }
}

impl Brick {
    fn would_fall_above(&self, other: &Brick) -> Option<u32> {
        // First, we need to see if the x-y box overlaps, if it doesn't, we
        // don't even care.
        // We also check that the z value is higher than the current selected block,
        // otherwise we're not going to care because it can't fall upwards.
        if self.xy_overlap(other) && self.min.2 > other.max.2 {
            // go one above the max z
            Some(other.max.2 + 1)
        } else {
            None
        }
    }

    fn xy_overlap(&self, other: &Brick) -> bool {
        // x then y
        self.max.0 >= other.min.0 && other.max.0 >= self.min.0 &&
            self.max.1 >= other.min.1 && other.max.1 >= self.min.1
    }

    #[allow(dead_code)]
    fn calculate_constituent_coords(&self) -> HashSet<Coord> {
        let mut coords: HashSet<Coord> = HashSet::new();
        for x in self.min.0..=self.max.0 {
            for y in self.min.1..=self.max.1 {
                for z in self.min.2..=self.max.2 {
                    coords.insert((x, y, z));
                }
            }
        }
        coords
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::ops::Deref;
    use indoc::indoc;
    use itertools::Itertools;
    use proptest::bool::{ANY as BOOL_ANY};
    use proptest::proptest;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day22::{apply_gravity, Brick, can_be_disintergrated, Coord, gather_slices, parse_bricks, parse_coord, prepare_bricks, run_day22a, run_day22b};

    const TEST_DATA: &str = indoc! {
        "1,0,1~1,2,1
         0,0,2~2,0,2
         0,2,3~2,2,3
         0,0,4~0,2,4
         2,0,5~2,2,5
         0,1,6~2,1,6
         1,1,8~1,1,9"
    };

    lazy_static! {
        static ref PARSED_DATA: Vec<Brick> = vec![
            Brick { min: (1,0,1), max: (1,2,1) }, // A
            Brick { min: (0,0,2), max: (2,0,2) }, // B
            Brick { min: (0,2,3), max: (2,2,3) }, // C
            Brick { min: (0,0,4), max: (0,2,4) }, // D
            Brick { min: (2,0,5), max: (2,2,5) }, // E
            Brick { min: (0,1,6), max: (2,1,6) }, // F
            Brick { min: (1,1,8), max: (1,1,9) }  // G
        ];
    }

    lazy_static! {
        static ref GRAVITY_APPLIED_DATA: Vec<Brick> = vec![
            Brick { min: (1,0,1), max: (1,2,1) }, // A
            Brick { min: (0,0,2), max: (2,0,2) }, // B
            Brick { min: (0,2,2), max: (2,2,2) }, // C
            Brick { min: (0,0,3), max: (0,2,3) }, // D
            Brick { min: (2,0,3), max: (2,2,3) }, // E
            Brick { min: (0,1,4), max: (2,1,4) }, // F
            Brick { min: (1,1,5), max: (1,1,6) }  // G
        ];
    }

    #[test]
    fn test_parsing() {
        assert_eq!(parse_bricks(TEST_DATA), *PARSED_DATA.deref())
    }

    proptest! {
        #[test]
        fn test_parse_coords(x in 0..100u32, y in 0..100u32, z in 0..100u32) {
            let string_to_parse = format!("{x},{y},{z}");
            assert_eq!(parse_coord(string_to_parse.as_str()), (x, y, z))
        }
    }

    #[rstest]
    #[case((0, 0, 0), (0, 1, 0), (0, 1, 0), (0, 2, 0), true)]
    #[case((0, 0, 0), (0, 1, 0), (0, 0, 0), (1, 1, 0), true)]
    #[case((0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 3, 0), false)]
    #[case((0, 0, 0), (2, 2, 0), (3, 2, 0), (3, 3, 0), false)]
    #[case((0, 0, 0), (2, 2, 0), (2, 2, 0), (3, 3, 0), true)]
    fn test_overlap(#[case] min1: Coord, #[case] max1: Coord, #[case] min2: Coord, #[case] max2: Coord, #[case] overlaps: bool) {
        let first = Brick {
            min: min1,
            max: max1
        };
        let second = Brick {
            min: min2,
            max: max2
        };

        assert_eq!(first.xy_overlap(&second), overlaps);
    }

    #[test]
    fn test_apply_gravity() {
        let a = PARSED_DATA.deref().iter().map(|x| x.clone()).collect_vec();
        assert_eq!(&apply_gravity(a).0, GRAVITY_APPLIED_DATA.deref());
    }

    #[test]
    fn test_day_22a() {
        assert_eq!(run_day22a(GRAVITY_APPLIED_DATA.deref()), 5);
    }

    #[test]
    fn integration_test_day_22a() {
        let g_bricks = prepare_bricks(TEST_DATA);
        assert_eq!(run_day22a(&g_bricks), 5);
    }

    #[test]
    fn test_can_be_disintegrated_false() {
        let target = Brick { min: (0, 0, 1), max: (1, 0, 1) };
        let other_bottom = Brick { min: (2, 0, 1), max: (3, 0, 1) };
        let bottom_row = vec![
            &target,
            &other_bottom,
        ];
        let top_row = vec![
            Brick { min: (0, 0, 2), max: (1, 0, 2) }, // this brick should fall as we're taking the target below away, so we should get "false"
            Brick { min: (2, 0, 2), max: (3, 0, 2) },
        ];
        let top_row_borrow = top_row.iter().collect_vec();

        let result = can_be_disintergrated(&target, Box::new(bottom_row.iter()), Box::new(top_row_borrow.iter()));
        assert_eq!(result, false);
    }

    #[test]
    fn test_can_be_disintegrated_true() {
        let target = Brick { min: (0, 0, 1), max: (1, 0, 1) };
        let other_bottom = Brick { min: (2, 0, 1), max: (3, 0, 1) };
        let bottom_row = vec![
            &target,
            &other_bottom,
        ];
        let top_row = vec![
            Brick { min: (2, 0, 2), max: (3, 0, 2) }, // this brick should not fall
        ];
        let top_row_borrow = top_row.iter().collect_vec();

        let result = can_be_disintergrated(&target, Box::new(bottom_row.iter()), Box::new(top_row_borrow.iter()));
        assert_eq!(result, true);
    }

    #[test]
    fn test_can_be_disintegrated_overlapping_true() {
        let target = Brick { min: (1, 0, 1), max: (2, 0, 1) };
        let other_bottom = Brick { min: (2, 0, 1), max: (3, 0, 1) };
        let other_other_bottom = Brick { min:   (0, 0, 1), max: (1, 0, 1) };
        let bottom_row = vec![
            &other_other_bottom,
            &target,
            &other_bottom,
        ];
        let top_row = vec![
            Brick { min: (0, 0, 2), max: (2, 0, 3) }, // this brick should not fall as something else supports it
        ];
        let top_row_borrow = top_row.iter().collect_vec();

        let result = can_be_disintergrated(&target, Box::new(bottom_row.iter()), Box::new(top_row_borrow.iter()));
        assert_eq!(result, true);
    }

    #[test]
    fn test_gather_slices() {
        let (min, max) = gather_slices(GRAVITY_APPLIED_DATA.deref());
        assert_eq!(min, BTreeMap::from([
            (1, vec![GRAVITY_APPLIED_DATA.get(0).unwrap()]),
             (2, vec![GRAVITY_APPLIED_DATA.get(1).unwrap(), GRAVITY_APPLIED_DATA.get(2).unwrap()]),
              (3, vec![GRAVITY_APPLIED_DATA.get(3).unwrap(), GRAVITY_APPLIED_DATA.get(4).unwrap()]),
               (4, vec![GRAVITY_APPLIED_DATA.get(5).unwrap()]),
                (5, vec![GRAVITY_APPLIED_DATA.get(6).unwrap()]),
        ]));
        assert_eq!(max, BTreeMap::from([
            (1, vec![GRAVITY_APPLIED_DATA.get(0).unwrap()]),
             (2, vec![GRAVITY_APPLIED_DATA.get(1).unwrap(), GRAVITY_APPLIED_DATA.get(2).unwrap()]),
              (3, vec![GRAVITY_APPLIED_DATA.get(3).unwrap(), GRAVITY_APPLIED_DATA.get(4).unwrap()]),
               (4, vec![GRAVITY_APPLIED_DATA.get(5).unwrap()]),
                (6, vec![GRAVITY_APPLIED_DATA.get(6).unwrap()]),
        ]));
    }

    proptest! {
        #[test]
        fn test_would_fall_to(x in 0..255u32, y in 0..255u32, zmin in 20..200u32, zmaxoffset in 1..10u32, above in 1..10u32, under in BOOL_ANY) {
            let zmax = zmin + zmaxoffset;
            let brick_one = Brick { min: (x, y, zmin), max: (x, y, zmax) };
            let brick_two_zmin = if under {
                zmin - above
            } else {
                zmin + zmaxoffset + above
            };
            let brick_two = Brick { min: (x, y, brick_two_zmin), max: (x, y, brick_two_zmin + zmaxoffset) };

            if under {
                assert_eq!(brick_two.would_fall_above(&brick_one), None);
            } else {
                assert_eq!(brick_two.would_fall_above(&brick_one), Some(zmin + zmaxoffset + 1));
            }
        }
    }

    #[test]
    fn test_day_22b() {
        assert_eq!(run_day22b(GRAVITY_APPLIED_DATA.deref()), 7);
    }


}