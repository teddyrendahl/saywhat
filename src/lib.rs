use std::collections::HashMap;
use std::hash::Hash;

fn lv_matrix(m: usize, n: usize) -> Vec<Vec<usize>> {
    let mut mx = vec![vec![0; n]; m];
    for (i, v) in mx.iter_mut().enumerate() {
        v[0] = i
    }
    for j in 0..n {
        mx[0][j] = j;
    }
    mx
}

fn forward_step<T: PartialEq>(
    lv_matrix: &mut [Vec<usize>],
    i: usize,
    j: usize,
    c1: &T,
    c2: &T,
    transpositions: Option<(usize, usize)>,
    substitution_cost: usize,
) {
    // Cost of skipping a char in s1
    let a = lv_matrix[i - 1][j] + 1;
    // Cost of skipping a char in s2
    let b = lv_matrix[i][j - 1] + 1;
    // Substitution cost
    let c = lv_matrix[i - 1][j - 1] + if c1 != c2 { substitution_cost } else { 0 };
    let d = match transpositions {
        Some((last_left, last_right)) if last_right > 0 && last_left > 0 => {
            lv_matrix[last_left - 1][last_right - 1] + i - last_left + j - last_right - 1
        }
        Some(_) | None => c + 1,
    };
    lv_matrix[i][j] = [a, b, c, d].into_iter().min().unwrap();
}

/// Calculate the Levenshtein edit distance between two values
///
/// The edit distance is the number of elements that need to be substituted,
/// inserted or deleted to transform s1 into s2. The `transpositions` flag
/// allows for adjacent transpositions to be counted as a single option i.e
/// "ab" -> "ba"
pub fn edit_distance<T: Eq + PartialEq + Hash>(
    s1: &[T],
    s2: &[T],
    substitution_cost: usize,
    transpositions: bool,
) -> usize {
    let mut lv_matrix = lv_matrix(s1.len() + 1, s2.len() + 1);
    let mut last_seen = HashMap::new();

    for (i, c1) in s1.iter().enumerate() {
        let mut last_right_buf = 0;
        for (j, c2) in s2.iter().enumerate() {
            let last_right = last_right_buf;
            if c1 == c2 {
                last_right_buf = j + 1;
            }
            forward_step(
                &mut lv_matrix,
                i + 1,
                j + 1,
                &c1,
                &c2,
                if transpositions {
                    Some((last_seen.get(c2).cloned().unwrap_or(0), last_right))
                } else {
                    None
                },
                substitution_cost,
            );
        }
        last_seen.insert(c1, i + 1);
    }
    lv_matrix[s1.len()][s2.len()]
}

#[cfg(test)]
mod tests {
    use crate::edit_distance;

    #[test]
    fn test_edit_distance() {
        assert_eq!(
            edit_distance(
                &"kitten".chars().collect::<Vec<char>>(),
                &"sitting".chars().collect::<Vec<char>>(),
                1,
                false
            ),
            3
        )
    }

    #[test]
    fn test_edit_distance_with_transpostions() {
        assert_eq!(
            edit_distance(
                &"abcd".chars().collect::<Vec<char>>(),
                &"abdc".chars().collect::<Vec<char>>(),
                1,
                true
            ),
            1
        )
    }
}
