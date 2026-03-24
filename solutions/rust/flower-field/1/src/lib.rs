fn count_flower(garden: &[& str], i: usize, j: usize ) -> usize {
    let ny = garden.len();
    let nx = garden.get(0).unwrap_or(&"").len();
    
    let i_beg = i.saturating_sub(1); // 如果 i < 1，就返回 0
    let j_beg = j.saturating_sub(1);

    let i_end = std::cmp::min(i + 2, nx);
    let j_end = std::cmp::min(j + 2, ny);
    println!("{:?}", (i_beg, j_beg, i_end, j_end));

    garden[j_beg..j_end].iter()
                        .flat_map(|line| line[i_beg..i_end].chars())
                        .filter(|&e| e == '*')
                        .count()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();

    for (j, &line) in  garden.iter().enumerate() {
        let mut new_line = String::new();
        for (i, c) in line.chars().enumerate() {
            match c {
                '*' => new_line.push(c),
                 _  => {
                    let n = count_flower(garden, i, j);
                    match n {
                        0 => new_line.push(' '),
                        _ => new_line.push_str(&format!("{n}").to_string()),
                    }
                 }
            }
        }
        answer.push(new_line);
    }
    answer
}
