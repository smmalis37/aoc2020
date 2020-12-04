use crate::day_solver::DaySolver;

pub struct Day4;

#[derive(Builder, Clone)]
pub struct Passport<'a> {
    birth_year: &'a str,
    issue_year: &'a str,
    exp_year: &'a str,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    pass_id: &'a str,
}

impl<'a> DaySolver<'a> for Day4 {
    type Parsed = Vec<Passport<'a>>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut pass = PassportBuilder::default();
        let mut results = Vec::new();

        for field in input.split(&['\n', ' '][..]) {
            if field.is_empty() {
                if let Ok(p) = pass.build() {
                    results.push(p);
                }
                pass = PassportBuilder::default();
            } else {
                let colon = 3;
                let key = &field[..colon];
                let value = &field[colon + 1..];

                match key {
                    "byr" => pass.birth_year(value),
                    "iyr" => pass.issue_year(value),
                    "eyr" => pass.exp_year(value),
                    "hgt" => pass.height(value),
                    "hcl" => pass.hair_color(value),
                    "ecl" => pass.eye_color(value),
                    "pid" => pass.pass_id(value),
                    "cid" => &mut pass,
                    _ => unreachable!(),
                };
            }
        }

        if let Ok(p) = pass.build() {
            results.push(p);
        }

        results
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.len()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut count = 0;

        for p in data {
            if !(1920..=2002).contains(&p.birth_year.parse().unwrap()) {
                continue;
            }

            if !(2010..=2020).contains(&p.issue_year.parse().unwrap()) {
                continue;
            }

            if !(2020..=2030).contains(&p.exp_year.parse().unwrap()) {
                continue;
            }

            let height_num_index = p.height.len() - 2;
            let height_range = match &p.height[height_num_index..] {
                "in" => 59..=76,
                "cm" => 150..=193,
                _ => continue,
            };
            if !height_range.contains(&p.height[..height_num_index].parse().unwrap()) {
                continue;
            }

            if !(p.hair_color.len() == 7
                && &p.hair_color[0..1] == "#"
                && p.hair_color[1..].chars().all(|c| c.is_ascii_hexdigit()))
            {
                continue;
            }

            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&p.eye_color) {
                continue;
            }

            if !(p.pass_id.len() == 9 && p.pass_id.chars().all(|c| c.is_ascii_digit())) {
                continue;
            }

            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4p1() {
        assert_eq!(
            Day4::part1(Day4::parse(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            )),
            2
        );
    }

    #[test]
    fn d4p2() {
        assert_eq!(
            Day4::part2(Day4::parse(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            )),
            0
        );

        assert_eq!(
            Day4::part2(Day4::parse(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            )),
            4
        );
    }
}
