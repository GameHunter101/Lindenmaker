use std::collections::HashMap;

pub type Rules = HashMap<char, String>;

pub fn progress(start: &str, constants: &[char], rules: &Rules) -> String {
    start
        .chars()
        .flat_map(|c| {
            if constants.contains(&c) {
                vec![c]
            } else {
                rules[&c].chars().collect()
            }
        })
        .collect()
}

pub fn separate_stack_strings(input: &str) -> Vec<String> {
    if input.is_empty() || !input.contains(']') {
        return vec![input.chars().filter(|c| c != &'[').collect()];
    }

    let mut end_index = -1;
    let char_indices: Vec<(usize, char)> = input.char_indices().collect();
    for (i, c) in &char_indices {
        if c == &']' {
            end_index = *i as i32;
            break;
        }
    }

    let mut start_index = -1;
    for i in (0..end_index).rev() {
        if char_indices[i as usize].1 == '[' {
            start_index = i;
            break;
        }
    }

    let base_string = char_indices[..end_index as usize]
        .iter()
        .flat_map(|(_, e)| if e == &'[' { None } else { Some(*e) })
        .collect();

    let other_strings = separate_stack_strings(
        &(input[..start_index as usize].to_string() + &input[end_index as usize + 1..]),
    );

    vec![base_string].into_iter().chain(other_strings).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Rules, progress};

    #[test]
    fn wiki_algae_model() {
        let start = "A".to_string();
        let expected: Vec<String> = [
            "A",
            "AB",
            "ABA",
            "ABAAB",
            "ABAABABA",
            "ABAABABAABAAB",
            "ABAABABAABAABABAABABA",
            "ABAABABAABAABABAABABAABAABABAABAAB",
        ]
        .iter()
        .map(|str| str.to_string())
        .collect();

        let mut rules: Rules = HashMap::new();

        rules.insert('A', "AB".to_string());
        rules.insert('B', "A".to_string());

        let result: &[String] = &(0..8)
            .map(|i| (0..i).fold(start.clone(), |acc, _| progress(&acc, &[], &rules)))
            .collect::<Vec<_>>();
        assert_eq!(&expected, result);
    }

    #[test]
    fn wiki_binary_tree_model() {
        let start = "0".to_string();
        let expected: Vec<String> = [
            "0",
            "1[0]0",
            "11[1[0]0]1[0]0",
            "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0",
        ]
        .iter()
        .map(|str| str.to_string())
        .collect();

        let mut rules: Rules = HashMap::new();

        rules.insert('1', "11".to_string());
        rules.insert('0', "1[0]0".to_string());

        let result: &[String] = &(0..4)
            .map(|i| (0..i).fold(start.clone(), |acc, _| progress(&acc, &['[', ']'], &rules)))
            .collect::<Vec<_>>();
        assert_eq!(&expected, result);
    }

    #[test]
    fn wiki_koch_curve() {
        let start = "F".to_string();
        let expected: Vec<String> = [
            "F",
            "F+F-F-F+F",
            "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F",
            "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F",
        ]
        .iter()
        .map(|str| str.to_string())
        .collect();

        let mut rules: Rules = HashMap::new();

        rules.insert('F', "F+F-F-F+F".to_string());

        let result: &[String] = &(0..4)
            .map(|i| (0..i).fold(start.clone(), |acc, _| progress(&acc, &['+', '-'], &rules)))
            .collect::<Vec<_>>();
        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }
}
