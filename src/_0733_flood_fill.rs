#[allow(dead_code)]
pub fn flood_fill_1(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    // image[row_idx][col_idx]
    let row_count = image.len();
    let col_count = image[0].len();
    let old_color = image[sr as usize][sc as usize];

    let mut new_image = vec![vec![-1; col_count]; row_count];

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((sr as usize, sc as usize));
    while stack.len() > 0 {
        let top = stack.last().unwrap();
        let r = top.0;
        let c = top.1;

        if new_image[r][c] == -1 {
            new_image[r][c] = if image[r][c] == old_color {
                new_color
            } else {
                image[r][c]
            };
        }
        match [
            (r as i32, c as i32 + 1),
            (r as i32 + 1, c as i32),
            (r as i32, c as i32 - 1),
            (r as i32 - 1, c as i32),
        ]
        .iter()
        .find(|pair| {
            0 <= pair.0
                && pair.0 < row_count as i32
                && 0 <= pair.1
                && pair.1 < col_count as i32
                && image[pair.0 as usize][pair.1 as usize] == old_color
                    // || image[pair.0 as usize][pair.1 as usize] == new_color
                && new_image[pair.0 as usize][pair.1 as usize] == -1
        }) {
            Some((new_r, new_c)) => stack.push((*new_r as usize, *new_c as usize)),
            None => {
                stack.pop();
            }
        }
    }

    for r in 0..row_count as usize {
        for c in 0..col_count as usize {
            if new_image[r][c] == -1 {
                new_image[r][c] = image[r][c];
            }
        }
    }

    new_image
}

#[allow(dead_code)]
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    // for bounds checking
    let row_count = image.len();
    let col_count = image[0].len();

    // `usize` is more convenient than `i32`
    // as we need to perform a lot of vector access
    let start_r = sr as usize;
    let start_c = sc as usize;

    let start_color = image[start_r][start_c];
    if start_color == new_color {
        // do nothing, this guard is important for the DFS below
        // this avoids endless loop when the startng pixel
        // has the same color value as the `new_color: i32`
        return image;
    }
    // if we modify the `image: Vec<Vec<i32>>` directly, we do not need to create another 2D array
    // hence we need to create a mutable reference
    // without it the rust compiler won't let us modify the input image: `Vec<Vec<i32>>`
    let mut image = image;

    // DFS

    let mut stack = vec![(start_r, start_c)];

    while stack.len() > 0 {
        if let Some(&(r, c)) = stack.last() {
            // by checking the pixel's color we avoid DFS on
            // 1. visited pixels
            // 2. pixels that are not the same color as the starting pixel
            image[r][c] = new_color;
            if r > 0 && image[r - 1][c] == start_color {
                // up
                stack.push((r - 1, c));
            } else if r + 1 < row_count && image[r + 1][c] == start_color {
                // down
                stack.push((r + 1, c));
            } else if c > 0 && image[r][c - 1] == start_color {
                // left
                stack.push((r, c - 1));
            } else if c < col_count - 1 && image[r][c + 1] == start_color {
                // right
                stack.push((r, c + 1));
            } else {
                // no where to go, pop this pixel
                stack.pop();
            }
        }
    }

    image
}
// cargo watch -x "test _0733_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            (
                (vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], (1, 1, 2)),
                vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            ),
            (
                (vec![vec![0, 0, 0], vec![0, 1, 1]], (1, 1, 1)),
                vec![vec![0, 0, 0], vec![0, 1, 1]],
            ),
            (
                (
                    vec![
                        vec![0, 1, 1, 1],
                        vec![1, 0, 1, 0],
                        vec![0, 1, 1, 0],
                        vec![0, 0, 0, 0],
                        vec![1, 1, 0, 1],
                    ],
                    (2, 2, 2),
                ),
                vec![
                    vec![0, 2, 2, 2],
                    vec![1, 0, 2, 0],
                    vec![0, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![1, 1, 0, 1],
                ],
            ),
            (
                (
                    vec![
                        vec![0, 1, 1, 1],
                        vec![1, 0, 1, 0],
                        vec![0, 1, 1, 0],
                        vec![0, 2, 0, 1],
                        vec![1, 1, 0, 1],
                    ],
                    (2, 2, 2),
                ),
                vec![
                    vec![0, 2, 2, 2],
                    vec![1, 0, 2, 0],
                    vec![0, 2, 2, 0],
                    vec![0, 2, 0, 1],
                    vec![1, 1, 0, 1],
                ],
            ),
            (
                (
                    vec![
                        vec![0, 1, 2, 1, 1, 1, 1, 0],
                        vec![1, 0, 2, 0, 1, 0, 1, 1],
                        vec![0, 1, 2, 1, 0, 1, 1, 0],
                    ],
                    (2, 6, 2),
                ),
                vec![
                    vec![0, 1, 2, 2, 2, 2, 2, 0],
                    vec![1, 0, 2, 0, 2, 0, 2, 2],
                    vec![0, 1, 2, 1, 0, 2, 2, 0],
                ],
            ),
        ];
        for ((image, args), expect) in test_cases {
            let ans = flood_fill(image, args.0, args.1, args.2);
            assert_eq!(ans, expect);
        }
    }
}
