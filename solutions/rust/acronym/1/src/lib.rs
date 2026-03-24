pub fn abbreviate(phrase: &str) -> String {
    let mut this = String::new();

    let mut last = ' ';

    for c in phrase.chars() {
        if ( last == ' ' || last == '-' || last == '_' ) && c.is_alphabetic() {
            this.push(c.to_ascii_uppercase());
        } else if last.is_lowercase() && c.is_uppercase() {
            this.push(c);
        }
        last = c;
    }

    // println!("{:?}", this);
    // let this = phrase.replace("-", " ");
    // this.split(" ")
    // .map(|w| w.chars().next().unwrap().to_ascii_uppercase())
    // .collect::<String>()
    this
}
