use crate::models::*;
use strum::IntoEnumIterator;

pub fn process_challenge(challenge: &Challenge) -> Vec<Challenge> {
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
                if s == Species::Unknown {
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
                if s == Species::Unknown {
                    continue;
                }
                let mut c = challenge.clone();
                c.reserve = r;
                c.species = s;
                challenges.push(c);
            }
        }
    } else if challenge.kills > 0 {
        challenges.push(challenge.clone());
    }
    challenges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_diamond_challenge() {
        let mut challenge = Challenge::default();
        challenge.species = Species::All;
        challenge.rating = Rating::Diamond;
        challenge.gender = Gender::Male;
        challenge.mods = Boolean::True;
        challenge.kills = 2;
        
        let mut expected = Vec::new();
        for s in Species::iter() {
            if s == Species::Unknown {
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
        challenge.rating = Rating::Diamond;
        challenge.kills = 10;
        let expected = vec![challenge.clone()];
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual); 
    }

    #[test]
    fn test_species_on_reserve() {
        let mut challenge = Challenge::default();
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
        challenge.species = Species::All;
        challenge.reserve = Reserve::LaytonLakeDistrict;
        challenge.kills = 10;
        let mut expected = Vec::new();
        let rs = reserve_species();
        let r_species = rs.get(&challenge.reserve).unwrap();
        for species in r_species  {
            let mut c = Challenge::default();
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
        challenge.reserve = Reserve::All;
        challenge.species = Species::All;

        let mut expected = Vec::new();
        for r in Reserve::iter() {
            if r == Reserve::Unknown {
                continue;
            }
            for s in Species::iter() {
                if s == Species::Unknown {
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
        let challenge = Challenge::default();
        let expected: Vec<Challenge> = Vec::new();
        let actual = process_challenge(&challenge);
        assert_eq!(expected, actual);
    }
}