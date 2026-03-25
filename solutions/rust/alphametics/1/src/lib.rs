use std::collections::HashMap;

// "HE + SEES + THE == LIGHT"
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut words: Vec<Vec<char>> = input
        .replace("==", "+")
        .split("+")
        .map(|s| s.trim().chars().rev().collect::<Vec<char>>() ) 
        .collect();

    let sum_word = words.pop().unwrap();
    let n = sum_word.len();
    if words.iter().filter(|w| w.len() > n).count() > 0 {
        return  None;
    }

    let mut nums = HashMap::<char, i32>::new();

    // let mut equations = Vec::new();
    for (i, &c) in sum_word.iter().enumerate() {
        let digit = 10_i32.pow(i as u32);
        nums.entry(c)
            .and_modify(|count| *count -= digit)
            .or_insert(- (digit as i32));

        // let mut equation = vec![c];
        for word in words.iter() {
            if let Some(&c) = word.get(i) {
            nums.entry(c)
                .and_modify(|count| *count += digit)
                .or_insert( digit as i32);
                // equation.push(c);
            }
        }
        // equations.push(equation);
    }

    let last =  sum_word.last().unwrap();

    let chars =  nums.keys().cloned().collect::<Vec<char>>();

    let combines = get_combine(&chars, last);

    println!("{:?}",  nums);
    for this in combines {
        // println!("{:?}",  this);
        if this.iter().map(|c| (nums[&c.0] as i64) * (c.1 as i64) ).sum::<i64>() == 0 {            
            return Some(this.iter().map(|(c, v)| (*c, *v)).collect::<HashMap<char, u8>>());
        }
    }
    None
}

fn get_combine(chars: &[char], last: &char) -> Vec<Vec<(char, u8)>> {
    // 这部分 AI写的
    let mut results = Vec::new();
    let mut current_assignment = Vec::new();
    let mut used_digits = [false; 10]; // 记录 0-9 哪些数字已被占用

    // 内部递归函数
    fn backtrack(
        chars: &[char],
        last: &char,
        used_digits: &mut [bool; 10],
        current: &mut Vec<(char, u8)>,
        results: &mut Vec<Vec<(char, u8)>>,
    ) {
        // 基准情形：所有字符都已分配数字
        if current.len() == chars.len() {
            results.push(current.clone());
            return;
        }

        let target_char = chars[current.len()];

        for digit in 0..=9 {
            // 约束 1：数字不能重复使用
            if used_digits[digit as usize] {
                continue;
            }

            // 约束 2：如果是特定的 last 字符（首字母），数字不能为 0
            if target_char == *last && digit == 0 {
                continue;
            }

            // 尝试分配
            used_digits[digit as usize] = true;
            current.push((target_char, digit));

            // 递归下一步
            backtrack(chars, last, used_digits, current, results);

            // 回溯：恢复现场
            current.pop(); // 这个太难想到了吧！
            used_digits[digit as usize] = false;
        }
    }

    backtrack(chars, last, &mut used_digits, &mut current_assignment, &mut results);
    results
}