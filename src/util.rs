use crate::memory::Memory;

pub fn parse_memory(mut memory: Memory) -> String {
    if memory.get_contents().is_empty() {
        return String::from("[]");
    }

    let mut parsed_memory = String::from("[");

    let memory_contents = memory.get_contents();

    memory_contents
        .keys()
        .collect::<Vec<&i32>>()
        .iter()
        .for_each(|key| {
            let value = memory_contents.get(key).unwrap();
            parsed_memory.push_str(&*format!("{} : {}, ", key, value));
        });

    parsed_memory = parsed_memory[0..parsed_memory.len() - 2].parse().unwrap();
    parsed_memory.push_str("]");
    return parsed_memory;
}

pub fn count(string: &String, predicate: fn(char) -> bool) -> isize {
    let mut cnt = 0 as isize;

    string.as_str().chars().for_each(|c| {
        if predicate(c) {
            cnt = cnt + 1;
        }
    });

    return cnt;
}