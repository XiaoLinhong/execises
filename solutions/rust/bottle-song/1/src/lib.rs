const WORDS: [(&str, &str, &str, &str); 10] = [
    ("Ten", "Ten", "nine", "bottles"),
    ("Nine", "Nine", "eight", "bottles"),
    ("Eight", "Eight", "seven", "bottles"),
    ("Seven", "Seven", "six", "bottles"),
    ("Six", "Six", "five", "bottles"),
    ("Five", "Five", "four", "bottles"),
    ("Four", "Four", "three", "bottles"),
    ("Three", "Three", "two", "bottles"),
    ("Two", "Two", "one", "bottle"),
    ("One", "One", "no", "bottles"),
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut poem = String::new();

    for i in 0..take_down {
        if i > 0 {poem.push('\n');}
        if i > 0 {poem.push('\n');}
        let (a, b, c, d) = WORDS[(10 - start_bottles + i) as usize];
        let mut this = format!(concat!(
            "{} green bottles hanging on the wall,\n",
            "{} green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be {} green {} hanging on the wall.",
        ), a, b, c, d);
        if start_bottles - i == 1{
            this = this.replace("One green bottles hanging", "One green bottle hanging");
        }
        poem.push_str(&this);
    }
    poem
}
 