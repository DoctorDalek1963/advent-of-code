use lazy_static::lazy_static;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use nom_regex::str::re_find;
use regex::Regex;

pub mod bin;

lazy_static! {
    static ref PASSPORT_PAIR_VALUE: Regex = Regex::new("^[a-z0-9#]+").unwrap();
    static ref VALIDATE_HEIGHT: Regex = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
    static ref VALIDATE_HAIR_COLOUR: Regex = Regex::new(r"^#[a-f0-9]{6}").unwrap();
    static ref VALIDATE_PID: Regex = Regex::new(r"\d{9}").unwrap();
}

fn parse_passport_pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        alt((tag(" "), tag("\n"))),
        separated_pair(alpha1, tag(":"), re_find(PASSPORT_PAIR_VALUE.to_owned())),
    )(input)
}

fn parse_passport_list(input: &str) -> IResult<&str, Vec<Vec<(&str, &str)>>> {
    separated_list1(tag("\n\n"), parse_passport_pairs)(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Passport<'s> {
    birth_year: Option<&'s str>,
    issue_year: Option<&'s str>,
    expiration_year: Option<&'s str>,
    height: Option<&'s str>,
    hair_colour: Option<&'s str>,
    eye_colour: Option<&'s str>,
    passport_id: Option<&'s str>,
    country_id: Option<&'s str>,
}

impl<'s> Passport<'s> {
    pub fn from_text(input: &'s str) -> Vec<Self> {
        parse_passport_list(input)
            .unwrap()
            .1
            .into_iter()
            .map(|pairs| Self::from_parsed(&pairs))
            .collect()
    }

    fn from_parsed(pairs: &[(&str, &'s str)]) -> Self {
        let mut passport = Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None,
        };

        for (field, value) in pairs {
            match *field {
                "byr" => passport.birth_year = Some(value),
                "iyr" => passport.issue_year = Some(value),
                "eyr" => passport.expiration_year = Some(value),
                "hgt" => passport.height = Some(value),
                "hcl" => passport.hair_colour = Some(value),
                "ecl" => passport.eye_colour = Some(value),
                "pid" => passport.passport_id = Some(value),
                "cid" => passport.country_id = Some(value),
                _ => (),
            };
        }

        passport
    }

    pub fn is_basic_valid_no_country_id(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_colour.is_some()
            && self.eye_colour.is_some()
            && self.passport_id.is_some()
    }

    pub fn is_proper_valid_no_country_id(&self) -> bool {
        self.is_basic_valid_no_country_id()
            && self.birth_year.is_some_and(|s| {
                s.parse::<u16>()
                    .is_ok_and(|year| (1920..=2002).contains(&year))
            })
            && self.issue_year.is_some_and(|s| {
                s.parse::<u16>()
                    .is_ok_and(|year| (2010..=2020).contains(&year))
            })
            && self.expiration_year.is_some_and(|s| {
                s.parse::<u16>()
                    .is_ok_and(|year| (2020..=2030).contains(&year))
            })
            && self.height.is_some_and(|s| {
                VALIDATE_HEIGHT
                    .captures(s)
                    .is_some_and(|caps| match caps.get(2) {
                        Some(mat) if mat.as_str() == "cm" => caps.get(1).is_some_and(|m| {
                            m.as_str()
                                .parse::<u8>()
                                .is_ok_and(|h| (150..=193).contains(&h))
                        }),
                        Some(mat) if mat.as_str() == "in" => caps.get(1).is_some_and(|m| {
                            m.as_str()
                                .parse::<u8>()
                                .is_ok_and(|h| (59..=76).contains(&h))
                        }),
                        _ => false,
                    })
            })
            && self
                .hair_colour
                .is_some_and(|s| VALIDATE_HAIR_COLOUR.is_match(s))
            && self
                .eye_colour
                .is_some_and(|s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s))
            && self.passport_id.is_some_and(|s| VALIDATE_PID.is_match(s))
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_passport_list_test() {
        let expected = vec![
            vec![
                ("ecl", "gry"),
                ("pid", "860033327"),
                ("eyr", "2020"),
                ("hcl", "#fffffd"),
                ("byr", "1937"),
                ("iyr", "2017"),
                ("cid", "147"),
                ("hgt", "183cm"),
            ],
            vec![
                ("iyr", "2013"),
                ("ecl", "amb"),
                ("cid", "350"),
                ("eyr", "2023"),
                ("pid", "028048884"),
                ("hcl", "#cfa07d"),
                ("byr", "1929"),
            ],
            vec![
                ("hcl", "#ae17e1"),
                ("iyr", "2013"),
                ("eyr", "2024"),
                ("ecl", "brn"),
                ("pid", "760753108"),
                ("byr", "1931"),
                ("hgt", "179cm"),
            ],
            vec![
                ("hcl", "#cfa07d"),
                ("eyr", "2025"),
                ("pid", "166559648"),
                ("iyr", "2011"),
                ("ecl", "brn"),
                ("hgt", "59in"),
            ],
        ];

        assert_eq!(parse_passport_list(TEST_INPUT), Ok(("\n", expected)))
    }

    #[test]
    fn passport_from_text_test() {
        let expected = vec![
            Passport {
                birth_year: Some("1937"),
                issue_year: Some("2017"),
                expiration_year: Some("2020"),
                height: Some("183cm"),
                hair_colour: Some("#fffffd"),
                eye_colour: Some("gry"),
                passport_id: Some("860033327"),
                country_id: Some("147"),
            },
            Passport {
                birth_year: Some("1929"),
                issue_year: Some("2013"),
                expiration_year: Some("2023"),
                height: None,
                hair_colour: Some("#cfa07d"),
                eye_colour: Some("amb"),
                passport_id: Some("028048884"),
                country_id: Some("350"),
            },
            Passport {
                birth_year: Some("1931"),
                issue_year: Some("2013"),
                expiration_year: Some("2024"),
                height: Some("179cm"),
                hair_colour: Some("#ae17e1"),
                eye_colour: Some("brn"),
                passport_id: Some("760753108"),
                country_id: None,
            },
            Passport {
                birth_year: None,
                issue_year: Some("2011"),
                expiration_year: Some("2025"),
                height: Some("59in"),
                hair_colour: Some("#cfa07d"),
                eye_colour: Some("brn"),
                passport_id: Some("166559648"),
                country_id: None,
            },
        ];

        assert_eq!(Passport::from_text(TEST_INPUT), expected);
    }

    #[test]
    fn proper_valid_test() {
        const INVALID: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

        const VALID: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

        assert!(
            Passport::from_text(INVALID)
                .into_iter()
                .all(|passport| !passport.is_proper_valid_no_country_id()),
            "Testing properly invalid passports"
        );
        assert!(
            Passport::from_text(VALID)
                .into_iter()
                .all(|passport| passport.is_proper_valid_no_country_id()),
            "Testing properly valid passports"
        );
    }
}
