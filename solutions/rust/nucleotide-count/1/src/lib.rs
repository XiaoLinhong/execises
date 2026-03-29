use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let answer = nucleotide_counts(dna)?;
    answer.get(&nucleotide).map_or(Err(nucleotide), |&n|Ok(n))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let nucleotides = vec!['A', 'C', 'G', 'T'];

     let mut answer = 
     nucleotides.iter()
                .zip([0; 4])
                .map(|(&c, i)| (c, i))
                .collect::<HashMap<char, usize>>();
    
    // let mut answer = HashMap::<char, usize>::new();

    for c in dna.chars() {
        if nucleotides.contains(&c){
            // let n = answer.entry(c).or_insert(0);
            let n = answer.get_mut(&c).unwrap();
            *n += 1;
        } else {
            return Err(c);
        }
    }
    Ok(answer)
}
