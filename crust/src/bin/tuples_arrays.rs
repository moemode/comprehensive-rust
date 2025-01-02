fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("{:?}", a);

    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for p in primes {
        for i in 2..p {
            assert_ne!(p % i, 0);
        }
    }

    let (left, right) = (1, 2);
    println!("left: {}, right: {}", left, right);
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("matrix: {:#?}", matrix);
    let transposed = transpose3(matrix);
    println!("transposed: {:#?}", transposed);
}

fn transpose3(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut T = [[0; 3]; 3];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            T[i][j] = matrix[j][i];
        }
    }
    T
}

fn transpose<const R: usize, const C: usize, T>(m: [[T; C]; R]) -> [[T; R]; C] {
    let mut rows = m.map(|r| r.into_iter());
    use std::array;
    array::from_fn(|_| array::from_fn(|i| rows[i].next().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected = [[1, 4, 7], [2, 5, 8], [3, 6, 9]];
        assert_eq!(transpose3(matrix), expected);
        assert_eq!(transpose(matrix), expected);
        let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
        let transposed = transpose3(matrix);
        let expected = [[101, 201, 301], [102, 202, 302], [103, 203, 303]];
        assert_eq!(transposed, expected);
        assert_eq!(transpose(matrix), expected)
    }
}
