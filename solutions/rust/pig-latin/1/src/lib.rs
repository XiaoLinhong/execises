pub fn translate(input: &str) -> String {
   input.split_whitespace()
        .map(translate_one)
        .collect::<Vec<_>>()
        .join(" ")
}

const VOWLES: &str = "aeiou";

pub fn translate_one(input: &str) -> String {
    let chars = input.chars().collect::<Vec<char>>();

    // Rule 1
    if input.starts_with("xr") || input.starts_with("yt") || VOWLES.contains(chars[0]) {
        return input.to_string() + "ay";
    }

    let mut head = vec![];
    for &c in &chars {
        if VOWLES.contains(c){
            break;
        }
        head.push(c);
    }

    if head[0] != 'y' &&  head.contains(&'y') { // Rule 4
        let i = head.iter().position(|&c| c == 'y').unwrap();
        // println!("{}", i);
        chars[i..].iter().collect::<String>() + &head[..i].iter().collect::<String>() + "ay"
    } else if *head.last().unwrap() == 'q' && chars.get(head.len()) == Some(&'u') { // Rule 3
        chars[head.len()+1..].iter().collect::<String>() + &head.iter().collect::<String>() + "uay"
    } else { // Rule 2
        chars[head.len()..].iter().collect::<String>() + &head.iter().collect::<String>() + "ay"
    }

}