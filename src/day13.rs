use std::cmp::{max, min};

use super::input_lines;

#[allow(dead_code)]
fn read_folding_input(filename: &str) -> std::io::Result<Vec<(char, usize)>> {
    let lines = input_lines(filename)?;

    let mut result = Vec::new();

    for l in lines {
        let l = l?;
        let mut fold = l.split_terminator("=");
        let axis = fold.next().unwrap().chars().nth(0).unwrap();
        let pos = fold.next().unwrap().parse::<usize>().unwrap();

        result.push((axis, pos));
    }

    Ok(result)
}

#[allow(dead_code)]
fn read_coord_input(filename: &str) -> std::io::Result<Vec<Vec<bool>>> {
    let lines = input_lines(filename)?;

    let mut coords = Vec::new();

    for l in lines {
        let l = l?;
        let mut xy = l.split_terminator(",");
        let x = xy.next().unwrap().parse::<usize>().unwrap();
        let y = xy.next().unwrap().parse::<usize>().unwrap();

        coords.push((x, y));
    }

    let mut x_max = 0;
    let mut y_max = 0;

    for c in &coords {
        x_max = max(x_max, c.0);
        y_max = max(y_max, c.1);
    }

    let mut result = vec![vec![false; x_max + 1]; y_max + 1];

    for (x, y) in coords {
        result[y][x] = true;
    }

    Ok(result)
}

#[allow(dead_code)]
fn fold_x(map: Vec<Vec<bool>>, pos: usize) -> Vec<Vec<bool>> {
    let prio_size = pos;
    let later_size = map[0].len() - pos - 1;

    let result_size = min(prio_size, later_size);

    let mut result = vec![vec![false; result_size]; map.len()];

    for i in 0..result.len() {
        for j in 0..result_size {
            result[i][result_size - j - 1] = map[i][pos - j - 1];
            let o = &mut result[i][result_size - j - 1];
            *o = map[i][pos + j + 1] || *o;
        }
    }

    result
}

#[allow(dead_code)]
fn fold_y(map: Vec<Vec<bool>>, pos: usize) -> Vec<Vec<bool>> {
    let prio_size = pos;
    let later_size = map.len() - pos - 1;

    let result_size = max(prio_size, later_size);

    let mut result = vec![vec![false; map[0].len()]; result_size];

    for i in 0..result_size {
        for j in 0..result[0].len() {
            if i < prio_size {
                result[result_size - i - 1][j] = map[pos - i - 1][j];
            }
            if i < later_size {
                let o = &mut result[result_size - i - 1][j];
                *o = map[pos + i + 1][j] || *o;
            }
        }
    }

    result
}

#[allow(dead_code)]
fn print_status(map: &Vec<Vec<bool>>) {
    for l in map {
        let t = l
            .iter()
            .map(|p| if *p { "0" } else { " " })
            .collect::<Vec<_>>()
            .join("");
        println!("{}", t);
    }
    println!("--------");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some() {
        println!("{:?}", (0..10).zip(0..5).collect::<Vec<_>>());
    }

    #[test]
    fn test_read_input() {
        let map = read_coord_input("inputs/day13_test.txt").unwrap();

        print_status(&map);

        let folds = read_folding_input("inputs/day13_test_fold.txt").unwrap();

        println!("{:?}", folds);
    }

    #[test]
    fn test_read_real_input() {
        let map = read_coord_input("inputs/day13.txt").unwrap();

        print_status(&map);

        let folds = read_folding_input("inputs/day13_fold.txt").unwrap();

        println!("{:?}", folds);
    }

    #[test]
    fn test_part1() {
        let map = read_coord_input("inputs/day13_test.txt").unwrap();
        let folds = read_folding_input("inputs/day13_test_fold.txt").unwrap();

        let result = if folds[0].0 == 'x' {
            fold_x(map, folds[0].1)
        } else {
            fold_y(map, folds[0].1)
        };

        print_status(&result);

        let mut n = 0;
        for l in &result {
            for c in l {
                if *c {
                    n += 1;
                }
            }
        }

        assert_eq!(n, 17);

        let result = if folds[1].0 == 'x' {
            fold_x(result, folds[1].1)
        } else {
            fold_y(result, folds[1].1)
        };

        print_status(&result);
    }

    #[test]
    fn do_part1() {
        let map = read_coord_input("inputs/day13.txt").unwrap();
        let folds = read_folding_input("inputs/day13_fold.txt").unwrap();

        let result = if folds[0].0 == 'x' {
            fold_x(map, folds[0].1)
        } else {
            fold_y(map, folds[0].1)
        };

        let mut n = 0;
        for l in &result {
            for c in l {
                if *c {
                    n += 1;
                }
            }
        }

        assert_eq!(n, 802);
    }

    #[test]
    fn do_part2() {
        let map = read_coord_input("inputs/day13.txt").unwrap();
        let folds = read_folding_input("inputs/day13_fold.txt").unwrap();

        let mut result = map;

        for f in folds {
            result = if f.0 == 'x' {
                fold_x(result, f.1)
            } else {
                fold_y(result, f.1)
            };
        }

        print_status(&result);
    }
}