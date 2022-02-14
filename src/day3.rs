pub fn part1(lines: &[String]) -> String {
    if lines.is_empty() {
        return format!("{}", 0);
    }

    let line_len = lines[0].chars().count();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..line_len {
        let mut ones_cnt = 0;
        for l in lines {
            if l.chars().nth(i).unwrap() == '1' {
                ones_cnt += 1;
            }
        }
        if ones_cnt > (lines.len() / 2) {
            gamma |= 1 << (line_len - i - 1);
        } else {
            epsilon |= 1 << (line_len) - i - 1;
        }
    }

    format!("{}", gamma * epsilon)
}

pub fn part2(lines: &[String]) -> String {
    if lines.is_empty() {
        return format!("{}", 0);
    }

    let line_len = lines[0].chars().count();

    let mut o2_candidates = lines.to_vec();
    let mut co2_candidates = lines.to_vec();

    for i in 0..line_len {
        if o2_candidates.len() > 1 {
            let (ones, zeroes) = split(&o2_candidates, |l| is_one(l, i));
            if ones.len() >= zeroes.len() {
                o2_candidates = ones;
            } else {
                o2_candidates = zeroes;
            }
        }

        if co2_candidates.len() > 1 {
            let (ones, zeroes) = split(&co2_candidates, |l| is_one(l, i));
            if zeroes.len() <= ones.len() {
                co2_candidates = zeroes;
            } else {
                co2_candidates = ones;
            }
        }
    }

    let o2 = u32::from_str_radix(o2_candidates[0].as_str(), 2).unwrap();
    let co2 = u32::from_str_radix(co2_candidates[0].as_str(), 2).unwrap();

    format!("{}", o2 * co2)
}

fn is_one(s: &str, i: usize) -> bool {
    s.chars().nth(i).unwrap() == '1'
}

fn split<F>(ts: &[String], p: F) -> (Vec<String>, Vec<String>)
where
    F: Fn(&str) -> bool,
{
    let mut ok = Vec::new();
    let mut nok = Vec::new();

    for t in ts {
        if p(t) {
            ok.push(t.clone());
        } else {
            nok.push(t.clone());
        }
    }

    (ok, nok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data: Vec<_> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part1(&data), "198");
    }

    #[test]
    fn test_part2() {
        let data: Vec<_> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part2(&data), "230");
    }
}
