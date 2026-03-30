
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn get_factors_sum(n: u64) -> u64{
    (2..=((n as f32).sqrt() as u64))
                      .into_iter()
                      .filter(|&i| n % i == 0)
                      .map(|i| if i != n/i {i + n/i} else {i})
                     // .map(|i| if i != n/i {dbg!(i) + dbg!(n/i)} else {i})
                      .sum::<u64>() + 1
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => {
            match num.cmp(&get_factors_sum(num)) {
                std::cmp::Ordering::Equal => Some(Classification::Perfect),
                std::cmp::Ordering::Less => Some(Classification::Abundant),
                std::cmp::Ordering::Greater => Some(Classification::Deficient),
                
            }
        }
    }

}
