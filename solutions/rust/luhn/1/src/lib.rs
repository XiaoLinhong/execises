/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    if code.chars().filter(|&e| !e.is_ascii_digit() && e != ' ').count() > 0{
        return false;
    }

    let numbers: Vec<u8> = code.chars()
                               .filter(|&e| e.is_ascii_digit())
                               .map(|e| e.to_digit(10).unwrap() as u8)
                               .collect();

    if numbers.len() <= 1 {
        return  false;
    }

    numbers.iter()
            .rev()
            .enumerate()
            .map(|(i, &item)|{
                    let new_number = match i {
                        i if i % 2 == 0 => item,
                        _ if item > 4 => item*2_u8 - 9_u8,
                        _ => item*2,
                    };
                    new_number as u32
            }).sum::<u32>() % 10 == 0

}
