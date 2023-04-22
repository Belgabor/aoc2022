use std::fs;

type Matrix<T> = Vec<Vec<T>>;
type Heights = Matrix<i8>;
type Visible = Matrix<bool>;

fn parse(content: &String) -> Heights {
    let mut heights: Heights = Vec::new();

    for line in content.split("\n") {
        let mut row: Vec<i8> = Vec::with_capacity(line.len());
        for tree in line.chars() {
            row.push(i8::from_str_radix(&tree.to_string(), 10).expect("Error parsing tree"));
        }
        heights.push(row);
    }

    return heights;
}

fn calculate_visibility(heights: &Heights) -> Visible {
    let mut visible: Visible = Vec::with_capacity(heights.len());
    let row_size = heights[0].len();
    for _ in 0..heights.len() {
        visible.push(vec![false; row_size]);
    }

    for row in 0..heights.len() {
        let mut highest_tree = -1;
        for col in 0..row_size {
            let tree = heights[row][col];
            if tree > highest_tree {
                visible[row][col] = true;
                highest_tree = tree;
            }
        }
    }
    for row in 0..heights.len() {
        let mut highest_tree = -1;
        for col in (0..row_size).rev() {
            let tree = heights[row][col];
            if tree > highest_tree {
                visible[row][col] = true;
                highest_tree = tree;
            }
        }
    }

    for col in 0..row_size {
        let mut highest_tree = -1;
        for row in 0..heights.len() {
            let tree = heights[row][col];
            if tree > highest_tree {
                visible[row][col] = true;
                highest_tree = tree;
            }
        }
    }

    for col in 0..row_size {
        let mut highest_tree = -1;
        for row in (0..heights.len()).rev() {
            let tree = heights[row][col];
            if tree > highest_tree {
                visible[row][col] = true;
                highest_tree = tree;
            }
        }
    }

    return visible;
}

fn count_visible(visible: &Visible) -> usize {
    let mut amount = 0;
    for row in visible {
        for tree in row {
            if *tree {
                amount += 1;
            }
        }
    }
    return amount;
}

fn calculate_scenic_score(heights: &Heights, row: usize, col: usize) -> u32 {
    let tree = heights[row][col];
    let row_width = heights[0].len();

    let mut score_left = 0;
    for icol in (0..col).rev() {
        let checked_tree = heights[row][icol];
        score_left += 1;
        if checked_tree >= tree {
            break;
        }
    }

    let mut score_right = 0;
    for icol in col+1..row_width {
        let checked_tree = heights[row][icol];
        score_right += 1;
        if checked_tree >= tree {
            break;
        }
    }

    let mut score_top = 0;
    for irow in (0..row).rev() {
        let checked_tree = heights[irow][col];
        score_top += 1;
        if checked_tree >= tree {
            break;
        }
    }

    let mut score_bottom = 0;
    for irow in row+1..heights.len() {
        let checked_tree = heights[irow][col];
        score_bottom += 1;
        if checked_tree >= tree {
            break;
        }
    }

    // println!("{} {} {} {} {} {}", row, col, score_left, score_top, score_right, score_bottom);

    return score_left * score_right * score_top * score_bottom;
}

fn calculate_highest_scenic_score(heights: &Heights) -> u32 {
    let row_size = heights[0].len();
    let mut score = 0;
    for row in 1..heights.len()-1 {
        for col in 1..row_size-1 {
            let tree_score = calculate_scenic_score(heights, row, col);
            if tree_score > score {
                score = tree_score;
            }
        }
    }

    return score;
}

fn part1(heights: &Heights) {
    //println!("{:?}", heights);
    //println!("{:?}", calculate_visibility(heights));
    let visible = calculate_visibility(heights);
    println!("Part 1: {}", count_visible(&visible));
}

fn part2(heights: &Heights) {
    println!("Part 2: {}", calculate_highest_scenic_score(heights));
}

fn main() {
    let files = vec!["sample.txt", "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
