use crate::day_solver::DaySolver;

pub struct Day4;

#[derive(Builder, Clone)]
pub struct Passport<'a> {
    birth_year: &'a [u8],
    issue_year: &'a [u8],
    exp_year: &'a [u8],
    height: &'a [u8],
    hair_color: &'a [u8],
    eye_color: &'a [u8],
    pass_id: &'a [u8],
}

impl<'a> DaySolver<'a> for Day4 {
    type Parsed = Vec<Passport<'a>>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut pass = PassportBuilder::default();
        let mut results = Vec::new();

        for field in input.as_bytes().split(|&x| x == b'\n' || x == b' ') {
            if field.is_empty() {
                if let Ok(p) = pass.build() {
                    results.push(p);
                }
                pass = PassportBuilder::default();
            } else {
                let colon = 3;
                let key = &field[..colon];
                let value = &field[colon..];

                match key {
                    b"byr" => pass.birth_year(value),
                    b"iyr" => pass.issue_year(value),
                    b"eyr" => pass.exp_year(value),
                    b"hgt" => pass.height(value),
                    b"hcl" => pass.hair_color(value),
                    b"ecl" => pass.eye_color(value),
                    b"pid" => pass.pass_id(value),
                    b"cid" => &mut pass,
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
        todo!()
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
    fn d4p2() {}
}
