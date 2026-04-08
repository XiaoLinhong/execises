pub fn count(lines: &[&str]) -> u32 {
    let mut lines = lines.iter()
                      .map(|&line| line.chars().collect::<Vec<char>>())
                      .collect::<Vec<Vec<char>>>();
    count_one(&mut lines)
}

fn count_one(lines: &mut [Vec<char>]) -> u32 {
    if lines.len() < 2 { return 0;}
    let mut n = 0;
    let top = &mut lines[0];

    let top_index = top.iter()
                 .enumerate()
                 .filter(|&(_, c)| *c == '+')
                 .map(|(i, _)| i )
                 .collect::<Vec<usize>>();

    if top_index.len() < 2 {
         return count_one(&mut lines[1..]);
    }
    let right_index = lines.iter()
                .enumerate()
                .filter(|&(_, line)| line[top_index[0]] == '+')
                .map(|(i, _)| i )
                .collect::<Vec<usize>>();

    if right_index.len() < 2 {
        lines[0][top_index[0]] = '-';
         return count_one(lines); // 递归
    }

    let this = top_index[0];
    for &i in &top_index[1..] {
        for &j in &right_index[1..] {
            if lines[j][i] == '+' {
                let diff = lines[0][this..i].iter()
                        .zip(lines[j][this..i].iter())
                        .filter(|&(&c1, &c2)| !"-+".contains(c1) || !"-+".contains(c2))
                        .count();
                // println!("{:?} {:?} {:?}", diff, &lines[0][this..i], &lines[j][this..i]);
                if diff == 0 { // top == bottom
                    let diff = lines[1..j].iter()
                                            .filter(|line| !"|+".contains(line[this]) || !"|+".contains(line[i]))
                                            .count(); 
                    // println!("{:?}", diff);
                    if  diff == 0 { // left = right
                        n += 1;
                    }
                }
            }
        }
    }
    // 递归
    lines[0][top_index[0]] = '-';
    n + count_one( lines)
}
