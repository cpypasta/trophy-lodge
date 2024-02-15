use crate::models::*;
use strum::IntoEnumIterator;
use convert_case::{Case, Casing};

pub fn process_challenge(challenge: &Challenge) -> Vec<Challenge> {
    if !challenge.valid() {
        return Vec::new();
    }
    let mut challenges = Vec::new();
    if challenge.species == Species::All && challenge.reserve != Reserve::All {
        if challenge.reserve != Reserve::Unknown {
           for s in reserve_species().get(&challenge.reserve).unwrap_or(&Vec::new()) {
                let mut c = challenge.clone();
                c.species = s.clone();
                challenges.push(c);
           } 
        } else {
            for s in Species::iter() {
                if !s.is_real() {
                    continue;
                }
                let mut c = challenge.clone();
                c.species = s;
                challenges.push(c);
            }
        }
    } else if challenge.reserve == Reserve::All && challenge.species != Species::All {
        if challenge.species == Species::Unknown {
            for r in Reserve::iter() {
                if r == Reserve::Unknown {
                    continue;
                }
                let mut c = challenge.clone();
                c.reserve = r;
                challenges.push(c);
            }
        } else {
            for rs in reserve_species() {
                if rs.1.contains(&challenge.species) {
                    let mut c = challenge.clone();
                    c.reserve = rs.0;
                    challenges.push(c);
                }
            }
        }
    } else if challenge.reserve == Reserve::All && challenge.species == Species::All {
        for r in Reserve::iter() {
            if r == Reserve::Unknown {
                continue;
            }
            for s in Species::iter() {
                if !s.is_real() {
                    continue;
                }
                let mut c = challenge.clone();
                c.reserve = r;
                c.species = s;
                challenges.push(c);
            }
        }
    } else {
        challenges.push(challenge.clone());
    }
    challenges
}

pub fn into_summary(challenges: &Vec<Challenge>) -> ChallengeSummary {
    if challenges.is_empty() {
        return ChallengeSummary::default();
    }

    let mut total_kills = 0;
    let mut total_kills_remaining = 0;
    for c in challenges {
        total_kills += c.kills;
        total_kills_remaining += c.kills_remaining;
    }
    let kills_completed = total_kills - total_kills_remaining;

    ChallengeSummary {
        name: challenges[0].name.clone().to_case(Case::Title), 
        description: challenges[0].description.clone(), 
        start: challenges[0].start.clone(),
        percent: kills_completed  as f32 / total_kills as f32,
        is_deleted: false,
    }
}

pub fn convert_challenge_name(name: &String) -> String {
    name.clone().replace(|c: char| !c.is_alphanumeric(), "_").to_lowercase() + ".csv"
}

pub fn create_challenge_filename(challenge: &Challenge) -> String {
    convert_challenge_name(&challenge.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_all_diamond_challenge() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.species = Species::All;
        challenge.rating = Rating::Diamond;
        challenge.gender = Gender::Male;
        challenge.mods = Boolean::True;
        challenge.kills = 2;
        
        let mut expected = Vec::new();
        for s in Species::iter() {
            if !s.is_real() {
                continue;
            }
            let mut c = challenge.clone();
            c.species = s;
            expected.push(c);
        }
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual); 
    }

    #[test]
    fn test_diamond_kills() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.rating = Rating::Diamond;
        challenge.kills = 10;
        let expected = vec![challenge.clone()];
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual); 
    }

    #[test]
    fn test_species_on_reserve() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.species = Species::WhitetailDeer;
        challenge.reserve = Reserve::LaytonLakeDistrict;
        challenge.kills = 10;
        let expected = vec![challenge.clone()];
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_all_species_on_reserve() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.species = Species::All;
        challenge.reserve = Reserve::LaytonLakeDistrict;
        challenge.kills = 10;
        let mut expected = Vec::new();
        let rs = reserve_species();
        let r_species = rs.get(&challenge.reserve).unwrap();
        for species in r_species  {
            let mut c = Challenge::default();
            c.name = "challenge".to_string();
            c.species = species.clone();
            c.reserve = Reserve::LaytonLakeDistrict;
            c.kills = challenge.kills;
            expected.push(c);
        }
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_species_on_all_reserves() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.species = Species::WhitetailDeer;
        challenge.reserve = Reserve::All;

        let mut expected = Vec::new();
        let reserves = vec![
            Reserve::LaytonLakeDistrict, 
            Reserve::RanchoDelArroyo,
            Reserve::MississippiAcresPreserve,
            Reserve::RevontuliCoast,
            Reserve::NewEnglandMountains,
        ];
        for r in reserves {
            let mut c = challenge.clone();
            c.reserve = r;
            expected.push(c);
        }
        let mut actual = process_challenge(&challenge);
        expected.sort();
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_all_reserves_without_species() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.reserve = Reserve::All;

        let mut expected = Vec::new();
        for r in Reserve::iter() {
            if r == Reserve::Unknown {
                continue;
            }
            let mut c = challenge.clone();
            c.reserve = r;
            expected.push(c);
        }
        let mut actual = process_challenge(&challenge);
        expected.sort();
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_all_reserves_all_species() {
        let mut challenge = Challenge::default();
        challenge.name = "challenge".to_string();
        challenge.reserve = Reserve::All;
        challenge.species = Species::All;

        let mut expected = Vec::new();
        for r in Reserve::iter() {
            if r == Reserve::Unknown {
                continue;
            }
            for s in Species::iter() {
                if !s.is_real() {
                    continue;
                }
                let mut c = challenge.clone();
                c.reserve = r;
                c.species = s;
                expected.push(c);
            }
        }
        let mut actual = process_challenge(&challenge);
        expected.sort();
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_challenge() {
        let mut challenge = Challenge::default();
        challenge.kills = 0;
        let expected: Vec<Challenge> = Vec::new();
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_summary() {
        let challenges = Vec::new();
        let expected = ChallengeSummary {
            name: "".to_string(),
            description: "".to_string(),
            start: "".to_string(),
            percent: 0.0,
            is_deleted: false,
        };
        let actual = into_summary(&challenges);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_challenges_into_summary() {
        let name = "my test challenge".to_string();
        let description = name.clone();
        let start = Local::now().to_rfc3339();
        let mut challenges = Vec::new();
        let mut challenge_criteria_a = Challenge::default();
        challenge_criteria_a.species = Species::WhitetailDeer;
        challenge_criteria_a.reserve = Reserve::LaytonLakeDistrict;
        challenge_criteria_a.name = name.clone();
        challenge_criteria_a.description = description.clone();
        challenge_criteria_a.start = start.clone();
        challenge_criteria_a.kills_remaining = 0;

        let mut challenge_criteria_b = Challenge::default();
        challenge_criteria_b.species = Species::WhitetailDeer;
        challenge_criteria_b.reserve = Reserve::RevontuliCoast;
        challenge_criteria_b.name = name.clone();
        challenge_criteria_b.description = description.clone();
        challenge_criteria_b.start = start.clone();

        challenges.extend(vec![challenge_criteria_a, challenge_criteria_b]);

        let expected = ChallengeSummary {
            name: name.to_case(Case::Title),
            description,
            start,
            percent: 0.5,
            is_deleted: false,
        }; 
        let actual = into_summary(&challenges);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_challenge_filename() {
        let mut challenge = Challenge::default();
        challenge.name = "My_test-challenge for you".to_string();
        let expected = "my_test_challenge_for_you.csv".to_string();
        let actual = create_challenge_filename(&challenge);
        assert_eq!(expected, actual);
    }
}