use std::slice::SliceIndex;
use std::cmp::max;

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let forest_width = input.first().unwrap().len();
    let mut forest: Vec<Vec<(bool, usize)>> = vec![vec![(false, 0); forest_width]; input.len()];
    

    let mut vertical_max = vec![0;forest_width];
    for i in 0..input.len(){
        let line = input.get(i).unwrap();
        if i == 0 || i == input.len()-1{
            forest.get_mut(i).unwrap().iter_mut().for_each(|(x,y)| *x = true);
        }else{
            forest[i][0].0 = true;
            forest[i][forest_width-1].0 = true;
        }

        let mut horizontal_max = 0;
        // look from left and top
        for j in 0..line.len(){
            forest[i][j].1 = line.chars().nth(j).unwrap().to_digit(10).unwrap() as usize;
            // from left to right
            if  horizontal_max < forest[i][j].1{
                forest[i][j].0 = true;
                horizontal_max = forest[i][j].1;
                // println!("Visible from left! {} {}", i, j);
            }
            // from top to bottom  
            if vertical_max[j] < forest[i][j].1{
                forest[i][j].0 = true;
                vertical_max[j] = forest[i][j].1;
                // println!("Visible from top! {} {}", i, j);
            }
        }
    }

    
    vertical_max= vec![0; forest_width];
    for i in (0..input.len()).rev(){
        let line = input.get(i).unwrap();

        let mut horizontal_max = 0;
        // look from right and bottom
        for j in (0..line.len()).rev(){
            // from right to left
            if horizontal_max < forest[i][j].1{
                forest[i][j].0 = true;
                horizontal_max = forest[i][j].1;
                // println!("Visible from right! {} {}", i, j);
            }
            // from bottom to top
            if vertical_max[j] < forest[i][j].1{
                forest[i][j].0 = true;
                vertical_max[j] = forest[i][j].1;
                // println!("Visible from bottom! {} {}", i, j);
            }
        }
    }
    // forest.iter().for_each(|x| println!("{:?}", x));

    let count: usize = forest.iter().map(|b| b.iter().filter(|a| a.0).count()).sum();
    count.to_string()
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let forest_width = input.first().unwrap().len();
    let forest_height = input.len();
    let mut forest: Vec<Vec<usize>> = vec![vec![(0); forest_width]; input.len()];
    
    let mut max_scenic_score = 0;

    // build forest
    for i in 0..input.len(){
        let line = input.get(i).unwrap();
        for j in 0..line.len(){
            forest[i][j] = line.chars().nth(j).unwrap().to_digit(10).unwrap() as usize;
        }
    }

    for i in 1..forest.len()-1{
        for j in 1..forest[i].len()-1{
            // calculate all 4 directions viewing
            let mut l_viewing = 0;
            let mut r_viewing = 0;
            let mut t_viewing = 0;
            let mut b_viewing = 0;
            for a in 1..j+1{
                l_viewing += 1;
                if forest[i][j-a] >= forest[i][j]{
                    break;
                }
            }
            for a in 1..(forest_width - j){
                r_viewing += 1;
                if forest[i][j+a] >= forest[i][j]{
                    break;
                }
            }
            for a in 1..i+1{
                t_viewing += 1;
                if forest[i-a][j] >= forest[i][j]{
                    break;
                }
            }
            for a in 1..(forest_height - i){
                b_viewing += 1;
                if forest[i+a][j] >= forest[i][j]{
                   break; 
                }
            }
            let scenic_score = l_viewing * r_viewing * t_viewing * b_viewing;
            // println!("Scenic score {} {} is {} = {} * {} * {} * {}",i,j,scenic_score, l_viewing, r_viewing, t_viewing, b_viewing);
            if scenic_score > max_scenic_score{
                max_scenic_score = scenic_score;
            }
        }
    }
    max_scenic_score.to_string()
}