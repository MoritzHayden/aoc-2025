advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos: i64 = 50;
    let mut count: u32 = 0;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let mut chars = line.chars();
        let dir = chars.next().unwrap_or('R');
        let dist: i64 = chars.as_str().parse().unwrap_or(0);

        match dir {
            'L' => pos -= dist,
            'R' => pos += dist,
            _ => {}
        }

        pos = pos.rem_euclid(100);

        if pos == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pos: i64 = 50;
    let mut count: u32 = 0;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let mut chars = line.chars();
        let dir = chars.next().unwrap_or('R');
        let dist_i64: i64 = chars.as_str().parse().unwrap_or(0);
        let d: u64 = dist_i64.unsigned_abs();

        pos = pos.rem_euclid(100);
        let p: u64 = pos as u64;

        let t_first: u64 = match dir {
            'R' => {
                if p == 0 {
                    100
                } else {
                    100 - p
                }
            }
            'L' => {
                if p == 0 {
                    100
                } else {
                    p
                }
            }
            _ => 0,
        };

        if t_first > 0 && d >= t_first {
            let occ = 1 + (d - t_first) / 100;
            count = count.saturating_add(occ as u32);
        }

        match dir {
            'L' => pos -= dist_i64,
            'R' => pos += dist_i64,
            _ => {}
        }

        pos = pos.rem_euclid(100);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
