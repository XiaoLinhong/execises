use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn frequency1(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let letters = input.iter().map(|s| s.chars()).flatten().collect::<Vec<char>>();
    if letters.is_empty() {
        return HashMap::new();
    }

    let mut n = letters.len()/(worker_count);
    if n*worker_count < letters.len() {
        n += 1;
    }
    let result: Vec<Vec<char>> = letters.chunks(n).map(|v| v.to_vec()).collect();

    let mut handlers = vec![];

    let answer: HashMap<char, usize> = HashMap::new();
    let share_ans = Arc::new(Mutex::new(answer));

    for chunck in result {
        let ans = share_ans.clone();
        // let content = chunck.iter().map(|s|s.to_string()).collect::<Vec<String>>();
        handlers.push(thread::spawn( move || {
            // chunck.iter()
            chunck.iter()
                  .filter(|c| c.is_alphabetic())
                  .for_each(|&c|{
                     let mut map = ans.lock().unwrap();
                     let v = map.entry(c.to_ascii_lowercase()).or_insert(0);
                     *v += 1;
                  });
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    // println!("{:?}", share_ans);

    // Arc::try_unwrap(share_ans)
    Arc::into_inner(share_ans)
                                .unwrap()
                                .into_inner()
                                .unwrap()

}


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    // let letters = input.iter().map(|s| s.chars()).flatten().collect::<Vec<char>>();
    // let chunks = letters.chunks((letters.len() + worker_count -1)/worker_count);

    let chunks = input.chunks((input.len() + worker_count -1)/worker_count);

    thread::scope(|s| {
        let mut handlers =vec![];
        for chunk in chunks {
            // let chunk_owned = chunk.iter().map(|s|s.to_string()).collect::<Vec<String>>();
            let handler = s.spawn(move ||{
                let mut local_map = HashMap::new();
                for s in chunk {
                // for s in chunk_owned {
                    for c in s.chars().filter(|c| c.is_alphabetic()) {
                        *local_map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                    }
                }
                // for c in chunk.iter().filter(|c| c.is_alphabetic()) {
                // // for s in chunk_owned {
                //         *local_map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                // }
                local_map
            });
            handlers.push(handler);
        }

        let mut map = HashMap::new();
        for handler in handlers {
            let local_map = handler.join().unwrap();
            for (k, v) in local_map {
                *map.entry(k).or_insert(0) += v;
            }
        }
        map
    })

}
