
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
/// 


// pub fn convert<T: Copy + TryInto<u32> >(number: &[T], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    use Error::*;
    const MAX_BASE: u32 = 100; 
    if from_base > MAX_BASE || from_base < 2 {return Err(InvalidInputBase);}
    if to_base   > MAX_BASE || to_base < 2   {return Err(InvalidOutputBase);}

    if number.is_empty() {return Ok(vec![0]);}

    for &i in number {

        // match TryInto::<u32>::try_into(*i) {
        //     Ok(n) if n > from_base => return Err(InvalidDigit(n)),
        //     _ => return Err(InvalidDigit(0)),
        // }

        if i >= from_base {
            return   Err(InvalidDigit(i));
        }
    }
    
    let ndigit = number.len();
    let mut n10 = number.iter().enumerate().rev()
                    // .map(|(i, &e)| TryInto::<u32>::try_into(e).unwrap_or(0 as u32) * from_base.pow(i as u32) )
                    .map(|(i, &n)| dbg!( n ) * from_base.pow(dbg!(ndigit - 1 - i) as u32) )
                    .sum::<u32>();
    
    if n10 == 0 {return Ok(vec![0]);}

    let ndigit = (n10 as f32).log(to_base as f32) as u32 + 1;
    let mut v = vec![];
    for i in (0..ndigit).rev() {
        v.push(n10 / to_base.pow(i));
        println!("{}, {}", n10, i);
        n10 -=  v[(ndigit - i - 1) as usize] * to_base.pow(i);
    }
    Ok(v)
}
