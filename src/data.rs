use crate::models::Trophy;

pub fn create_trophies(n: u32) -> Vec<Trophy<'static>> {
    let mut trophies = Vec::new();
    for _ in 0..n {
        trophies.push(Trophy::default());
    }
    trophies
}