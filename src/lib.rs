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
    substitution_cost: usize,
) {
    // Cost of skipping a char in s1
    let a = lv_matrix[i - 1][j] + 1;
    // Cost of skipping a char in s2
    let b = lv_matrix[i][j - 1] + 1;
    // Substitution cost
    let c = lv_matrix[i - 1][j - 1] + if c1 != c2 { substitution_cost } else { 0 };
    lv_matrix[i][j] = [a, b, c].into_iter().min().unwrap();
}

/// Calculate the Levenshtein edit distance between two values
///
/// The edit distance is the number of elements that need to be substituted,
/// inserted or deleted to transform s1 into s2.
pub fn edit_distance<T: PartialEq>(s1: &[T], s2: &[T], substitution_cost: usize) -> usize {
    let mut lv_matrix = lv_matrix(s1.len() + 1, s2.len() + 1);

    for (i, c1) in s1.iter().enumerate() {
        for (j, c2) in s2.iter().enumerate() {
            forward_step(&mut lv_matrix, i + 1, j + 1, &c1, &c2, substitution_cost);
        }
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
                1
            ),
            3
        )
    }
}
