use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day11.txt");
    let observed_gal = build_observed_galaxy(&data);
    let expanded_gal = expand_galaxy(&observed_gal);
    println!("Part 1: {}", day11a(&expanded_gal));
}

fn day11a(gal: &Vec<(usize, usize)>) -> u64 {
    determine_pairs(gal).iter().sum::<usize>() as u64
}

fn build_observed_galaxy(data: &str) -> Vec<(usize, usize)> {
    data.lines().enumerate().flat_map(|(y, line)| {
        line.char_indices().filter_map(move |(x, char)| {
            match char {
                '#' => Some((x, y.clone())),
                _ => None
            }
        })
    }).collect()
}

fn expand_galaxy(observed: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // we don't care about the last lines in both directions so we don't need the size of the field
    // just what's between or to the 0-side.
    let populated_x: Vec<usize> = observed.iter().map(|(x, _)| x.clone()).collect();
    let populated_y: Vec<usize> = observed.iter().map(|(_, y)| y.clone()).collect();

    // in the observed universe, figure out what lines are empty
    let expand_x: Vec<usize> = (0usize..*populated_x.iter().max().unwrap()).filter(|entry| !populated_x.contains(entry)).collect();
    let expand_y: Vec<usize> = (0usize..*populated_y.iter().max().unwrap()).filter(|entry| !populated_y.contains(entry)).collect();

    // now, we need to adjust each observed parameter based on the above.
    observed.iter().map(|(x, y)| {
        (
            x + expand_x.iter().filter(|e| *e < x).count(),
            y + expand_y.iter().filter(|e| *e < y).count(),
        )
    }).collect()
}

fn determine_pairs(expanded: &Vec<(usize, usize)>) -> Vec<usize> {
    expanded.iter().enumerate().take(expanded.len() - 1).flat_map(|(idx, (x1, y1))| {
        expanded.iter().skip(idx+1).map(|(x2, y2)| x1.abs_diff(*x2) + y1.abs_diff(*y2))
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use structopt::lazy_static::lazy_static;
    use crate::day11::{build_observed_galaxy, day11a, determine_pairs, expand_galaxy};

    const TEST_DATA_1: &str = "...#......\n\
                               .......#..\n\
                               #.........\n\
                               ..........\n\
                               ......#...\n\
                               .#........\n\
                               .........#\n\
                               ..........\n\
                               .......#..\n\
                               #...#.....";

    lazy_static! {
        static ref OBESERVABLE_GAL_PAIRS: Vec<(usize, usize)> = vec![
            (3, 0),
            (7, 1),
            (0, 2),
            (6, 4),
            (1, 5),
            (9, 6),
            (7, 8),
            (0, 9),
            (4, 9)
        ];
    }

    lazy_static! {
        static ref EXPANDED_GAL_PAIRS: Vec<(usize, usize)> = vec![
            (4, 0),
            (9, 1),
            (0, 2),
            (8, 5),
            (1, 6),
            (12, 7),
            (9, 10),
            (0, 11),
            (5, 11)
        ];
    }

    #[test]
    fn test_build_observed_galaxy() {
        assert_eq!(build_observed_galaxy(TEST_DATA_1), *OBESERVABLE_GAL_PAIRS.deref())
    }

    #[test]
    fn test_expand_galaxy() {
        assert_eq!(expand_galaxy(OBESERVABLE_GAL_PAIRS.deref()), *EXPANDED_GAL_PAIRS.deref())
    }

    #[test]
    fn test_shortest_distance() {
        assert_eq!(determine_pairs(&vec![(1, 6), (5, 11)]), vec![9]); // 5 and 9
        assert_eq!(determine_pairs(&vec![(4, 0), (9, 10)]), vec![15]); // 1 and 7
        assert_eq!(determine_pairs(&vec![(0, 2), (12, 7)]), vec![17]); // 3 and 6
        assert_eq!(determine_pairs(&vec![(0, 11), (5, 11)]), vec![5]); // 8 and 9
    }

    #[test]
    fn test_day11a() {
        assert_eq!(day11a(EXPANDED_GAL_PAIRS.deref()), 374);
    }

}