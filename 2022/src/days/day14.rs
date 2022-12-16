use std::ops::{Range, RangeInclusive};

#[derive(Debug, Clone, Copy)]
enum Tile{
    Rock,
    Air,
    Sand
}

struct Cave{
    tiles: Vec<Vec<Tile>>, // top left is 0,0
    sand_source: (usize, usize),
}

impl Cave{
    fn new(size: (usize, usize), sand_source: (usize, usize)) -> Self{
        Cave { 
            tiles: vec![vec![Tile::Air; size.1]; size.0],
            sand_source: sand_source 
        }
    }

    fn size(&self) -> (usize, usize){
        match self.tiles.first(){
            Some(y) => return (self.tiles.len(), y.len()),
            None => return (self.tiles.len(), 0)
        }
    }

    fn draw_rock(&mut self, input: &String) {
        let parts: Vec<(usize, usize)> = input.split(" -> ").map(
            |x|{
                (x.split(",").nth(0).unwrap().parse().unwrap(), x.split(",").nth(1).unwrap().parse().unwrap())
            }).collect();
        
        let mut previous_part: (usize, usize) = (usize::MAX,usize::MAX);
        let selfsize = self.size();

        for part in parts{
            if part.0 > selfsize.0 || part.1 > selfsize.1{
                panic!("WHOOPS to small initial cave")
            }
            if previous_part != (usize::MAX, usize::MAX){
                for x in absolute_range(previous_part.0, part.0){
                    for y in absolute_range(previous_part.1, part.1){
                        self.tiles[x][y] = Tile::Rock;
                    } 
                }
            }
            previous_part = part;
        }
    }

    fn put_sand(&mut self) -> bool{
        let mut sand_location = self.sand_source;
        let mysize = self.size();
        loop{
            match self.get_below(sand_location){
                Some(below) =>{
                    match below.1{
                        Tile::Rock | Tile::Sand=>{
                            // attempt diagonal left below
                            match self.get_left(below.0){
                                Some(left_below) =>{
                                    match left_below.1{
                                        Tile::Rock | Tile::Sand =>{
                                            // we cannot go left, as this is already filled
                                        },
                                        Tile::Air =>{
                                            sand_location = left_below.0;
                                            continue;
                                        }
                                    }
                                },
                                None => {
                                    // we cannot go left, as this does not exist
                                }
                            }

                            // attempt diagonal right below
                            match self.get_right(below.0){
                                Some(right_below) =>{
                                    match right_below.1{
                                        Tile::Rock | Tile::Sand =>{
                                            // we cannot go left, as this is already filled
                                        },
                                        Tile::Air =>{
                                            sand_location = right_below.0;
                                            continue;
                                        }
                                    }
                                },
                                None => {
                                    // we cannot go left, as this does not exist
                                }
                            }

                            // below, left below and right below are full, so just store here
                            self.tiles[sand_location.0][sand_location.1] = Tile::Sand;
                            return true;
                        },
                        Tile::Air =>{
                            sand_location = below.0;
                            continue;
                        }
                    }
                },
                None =>{
                    return false
                }
            }
        }

        // TODO return true if stored, false if into void
        unimplemented!()
    }

    fn get_value(&self, coordinates: (usize, usize)) -> Option<Tile> {
        if coordinates.0 < self.size().0 && coordinates.1 < self.size().1
        // greater than 0 check usuless due to type limitations
        {
            return Some(
                *self
                    .tiles
                    .get(coordinates.0)
                    .unwrap()
                    .get(coordinates.1)
                    .unwrap()
            );
        }
        None
    }


    // Asumes coordinates is valid
    fn get_left(&self, coordinates: (usize, usize)) -> Option<((usize, usize), Tile)> {
        if coordinates.0 > 0 {
            let xy = (coordinates.0 - 1, coordinates.1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    // Asumes coordinates is valid
    fn get_right(&self, coordinates: (usize, usize)) -> Option<((usize, usize), Tile)> {
        if coordinates.0 < self.size().0 - 1 {
            let xy = (coordinates.0 + 1, coordinates.1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    // Asumes coordinates is valid
    fn get_below(&self, coordinates: (usize, usize)) -> Option<((usize, usize), Tile)> {
        if coordinates.1 < self.size().1 - 1 {
            let xy = (coordinates.0, coordinates.1 + 1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    // Asumes coordinates is valid
    fn get_above(&self, coordinates: (usize, usize)) -> Option<((usize, usize), Tile)> {
        if coordinates.1 > 0 {
            let xy = (coordinates.0, coordinates.1 - 1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    
}

fn absolute_range(a: usize, b: usize) -> RangeInclusive<usize>{
    if a<b{
        a..=b
    }else{
        b..=a
    }
}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut cave = Cave::new((600,200), (500,0));

    for line in input{
        cave.draw_rock(line);
    }

    
    let mut count = 0;
    while cave.put_sand(){
        count += 1;
        println!("Put {} sand", count);
    }


    // println!("{}", cave.tiles.iter().map(|x| format!("{:?}", x)).collect::<String>());
    
    count.to_string()
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
       
    }

    String::from("Placeholder part 2")
}