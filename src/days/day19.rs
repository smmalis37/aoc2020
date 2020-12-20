use std::num::NonZeroUsize;

use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

use crate::{day_solver::DaySolver, util::*};

pub struct Day19;

type N = u8;

#[derive(Clone)]
pub enum Rule {
    Character(u8),
    OneSide(ArrayVec<[N; 3]>),
    TwoSide(ArrayVec<[N; 3]>, ArrayVec<[N; 3]>),
}

use Rule::*;

impl<'a> DaySolver<'a> for Day19 {
    type Parsed = (FxHashMap<N, Rule>, impl Iterator<Item = &'a [u8]> + Clone);
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut rulemap = FxHashMap::default();
        let mut lines = input.as_bytes().split(|&x| x == b'\n');

        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            let colon = l.iter().position(|&x| x == b':').unwrap();
            let id = l[..colon].parse().unwrap();
            let mut pos = colon + 2;

            rulemap.insert(
                id,
                if l[pos] == b'"' {
                    Character(l[pos + 1])
                } else {
                    let mut first = ArrayVec::new();
                    let mut second = ArrayVec::new();

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
        lines
            .filter(|l| recursive_match(l, &rules, 0).map_or(false, |x| x.get() == l.len()))
            .count()
    }

    fn part2((rules, lines): Self::Parsed) -> Self::Output {
        let mut count = 0;

        for l in lines {
            let mut ftcount = 0;
            let mut ftpos = 0;
            while let Some(x) = recursive_match(&l[ftpos..], &rules, 42) {
                ftcount += 1;
                ftpos += x.get();

                let mut topos = ftpos;
                for _ in 1..ftcount {
                    if let Some(x) = recursive_match(&l[topos..], &rules, 31) {
                        topos += x.get();
                        if topos == l.len() {
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }

        count
    }
}

fn recursive_match(l: &[u8], rules: &FxHashMap<N, Rule>, me: N) -> Option<NonZeroUsize> {
    match rules.get(&me).unwrap() {
        Character(a) => {
            if l.get(0) == Some(a) {
                NonZeroUsize::new(1)
            } else {
                None
            }
        }
        OneSide(subrules) => handle_subrules(l, rules, subrules),
        TwoSide(left, right) => {
            if let Some(x) = handle_subrules(l, rules, left) {
                Some(x)
            } else if let Some(x) = handle_subrules(l, rules, right) {
                Some(x)
            } else {
                None
            }
        }
    }
}

fn handle_subrules(l: &[u8], rules: &FxHashMap<N, Rule>, subrules: &[N]) -> Option<NonZeroUsize> {
    let mut pos = 0;

    for &rule in subrules {
        if let Some(x) = recursive_match(&l[pos..], rules, rule) {
            pos += x.get();
        } else {
            return None;
        }
    }

    NonZeroUsize::new(pos)
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
