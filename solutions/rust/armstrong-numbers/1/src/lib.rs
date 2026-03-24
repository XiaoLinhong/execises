// pub fn is_armstrong_number(num: u32) -> bool {
//     let num_str = format!("{num}"); 
//     let ndigit = num_str.len() as u32;
//     num_str.chars()
//            .map(|c| (c.to_digit(10).unwrap() as u32).pow(ndigit))
//            .sum::<u32>() == num
// }

pub fn is_armstrong_number(num: u32) -> bool {
    let k: u32 = match num {
        0 => 1,
        _ => (num as f32).log10() as u32 + 1,
    };
    let ten: u32 = 10;
    (1..=k).into_iter()
          .map(|p| ((num % ten.pow(p) - num % ten.pow(p-1)) / ten.pow(p-1)).pow(k))
          .sum::<u32>() == num
}
