use std::fs::read_to_string;

fn main() {
    let (left, right) = parse_puzzle_input("data/input.txt");

    let i = find_smallest_distance(&left, &right);
    let ii = calculate_similarity_score(&left, &right);

    println!("(i): {}", i);
    println!("(ii): {}", ii)
}

fn find_smallest_distance(left: &[u32], right: &[u32]) -> u32 {
    let mut left_mut = left.to_vec();
    let mut right_mut = right.to_vec();

    left_mut.sort();
    right_mut.sort();

    left_mut
        .iter()
        .zip(right_mut.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn calculate_similarity_score(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .map(|num| right.iter().filter(|val| **val == *num).count() as u32 * *num)
        .sum()
}

fn parse_puzzle_input(path: &str) -> (Vec<u32>, Vec<u32>) {
    read_to_string(&path)
        .expect(&format!("Error reading file: {}", &path))
        .split("\n")
        .map(|s| {
            let vec: Vec<_> = s
                .split("   ")
                .map(|num| {
                    num.parse::<u32>()
                        .expect(&format!("Error parsing int: {}", num))
                })
                .collect();
            return (vec[0], vec[1]);
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_LEFT: [u32; 6] = [3, 4, 2, 1, 3, 3];
    const TEST_INPUT_RIGHT: [u32; 6] = [4, 3, 5, 3, 9, 3];

    #[test]
    fn test_parse_puzzle_input() {
        let (a, b) = parse_puzzle_input("data/example_input.txt");
        assert_eq!(&TEST_INPUT_LEFT, a.as_slice());
        assert_eq!(&TEST_INPUT_RIGHT, b.as_slice());
    }

    #[test]
    fn test_find_smallest_distance() {
        let result = find_smallest_distance(&TEST_INPUT_LEFT, &TEST_INPUT_RIGHT);
        assert_eq!(11, result);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let result = calculate_similarity_score(&TEST_INPUT_LEFT, &TEST_INPUT_RIGHT);
        assert_eq!(31, result)
    }
}
