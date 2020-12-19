use std::collections::HashMap;

use arrayvec::ArrayVec;
use regex::bytes::Regex;

use crate::{day_solver::DaySolver, util::*};

pub struct Day19;

#[derive(Clone, Debug)]
pub enum Rule {
    Character(u8),
    OneSide(ArrayVec<[u8; 3]>),
    TwoSide(ArrayVec<[u8; 3]>, ArrayVec<[u8; 3]>),
}

use Rule::*;

impl<'a> DaySolver<'a> for Day19 {
    type Parsed = (HashMap<u8, Rule>, impl Iterator<Item = &'a [u8]> + Clone);
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut rulemap = HashMap::new();
        let mut lines = input.as_bytes().split(|&x| x == b'\n');

        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            let colon = l.iter().position(|&x| x == b':').unwrap();
            let id = l[..colon].parse().unwrap();

            rulemap.insert(
                id,
                if l[colon + 2] == b'"' {
                    Character(l[colon + 3])
                } else {
                    let mut first = ArrayVec::new();
                    let mut second = ArrayVec::new();
                    let mut pos = colon + 2;

                    loop {
                        let i = l[pos..]
                            .iter()
                            .position(|&x| x == b' ')
                            .unwrap_or(l.len() - pos);
                        if let Ok(num) = l[pos..pos + i].parse() {
                            first.push(num);
                        } else if l[pos] == b'|' {
                            std::mem::swap(&mut first, &mut second);
                        } else {
                            unreachable!()
                        }

                        pos += i + 1;
                        if pos >= l.len() {
                            break;
                        }
                    }

                    if second.is_empty() {
                        OneSide(first)
                    } else {
                        TwoSide(second, first)
                    }
                },
            );
        }

        (rulemap, lines)
    }

    fn part1((rules, lines): Self::Parsed) -> Self::Output {
        let regex = Regex::new(&construct_regex(&rules)).unwrap();
        lines.filter(|l| regex.is_match(l)).count()
    }

    fn part2((mut rules, lines): Self::Parsed) -> Self::Output {
        let eight_left = [42].iter().copied().collect();
        let eight_right = [42, 8].iter().copied().collect();
        rules.insert(8, TwoSide(eight_left, eight_right));
        let eleven_left = [42, 31].iter().copied().collect();
        let eleven_right = [42, 11, 31].iter().copied().collect();
        rules.insert(11, TwoSide(eleven_left, eleven_right));
        Self::part1((rules, lines))
    }
}

fn construct_regex(rules: &HashMap<u8, Rule>) -> String {
    format!("^{}$", recurse_construct(rules, 0))
}

fn recurse_construct(rules: &HashMap<u8, Rule>, me: u8) -> String {
    match rules.get(&me).unwrap() {
        Character(a) => (*a as char).to_string(),
        OneSide(subrules) => handle_subrules(rules, subrules),
        TwoSide(left, right) => format!(
            "(({})|({}))",
            handle_subrules(rules, left),
            handle_subrules(rules, right)
        ),
    }
}

fn handle_subrules<'a>(
    rules: &HashMap<u8, Rule>,
    list: impl IntoIterator<Item = &'a u8>,
) -> String {
    list.into_iter()
        .map(|&x| recurse_construct(rules, x))
        .fold(String::new(), |acc, e| acc + &e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d19p1() {
        assert_eq!(
            Day19::part1(Day19::parse(
                "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            )),
            2
        );

        assert_eq!(
            Day19::part1(Day19::parse(
                "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            )),
            3
        );
    }

    #[test]
    fn d19p2() {
        assert_eq!(
            Day19::part2(Day19::parse(
                "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            )),
            12
        );
    }
}