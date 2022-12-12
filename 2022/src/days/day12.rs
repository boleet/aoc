enum Move{
    Left,
    Right,
    Up,
    Down
}

struct Map{
    size: (usize, usize), // width, height
    start: (usize, usize), // x, y position. Bottom left is 0,0
    goal: (usize, usize), // x, y position. Bottom left is 0,0
    grids: Vec<Vec<usize>>,
    valid_moves: Vec<Vec<Vec<Move>>>,
}

impl Map{
    fn get_route(&self) -> Option<Vec<(usize,usize)>>{
        let mut trace: Vec<(usize, usize)> = vec![];
        let current = self.start;
        while current != self.goal{
            let valid_moves = self.get_valid_moves(current);
            if valid_moves.len() == 1{
                // there is only one valid move, namely to go back, so do that
            }
        }
        Some(trace)
    }

    fn initialize_valid_moves(&mut self){
        for y in 0..self.size.0{
            for x in 0..self.size.1{
                self.valid_moves[y][x] = self.get_valid_moves((x,y));
            }
        }
    }

    // Asumes coordinates is valid
    fn get_valid_moves(&self, coordinates: (usize, usize)) -> Vec<Move>{
        let mut result: Vec<Move> = vec![];
        let l = self.get_left(coordinates);
        let r = self.get_right(coordinates);
        let a = self.get_above(coordinates);
        let b = self.get_below(coordinates);
        let current_value = self.get_value(coordinates).unwrap();
        if l.is_some() && l.unwrap().1 > current_value {
            result.push(Move::Left);
        }
        if r.is_some() && r.unwrap().1 > current_value {
            result.push(Move::Right);
        }
        if a.is_some() && a.unwrap().1 > current_value {
            result.push(Move::Up);
        }
        if b.is_some() && b.unwrap().1 > current_value {
            result.push(Move::Down);
        }
        result
    }

    fn get_value(&self, coordinates: (usize, usize)) -> Option<usize>{
        if coordinates.0 > 0 && coordinates.0 < self.size.0 && coordinates.1 > 0 && coordinates.1 < self.size.1{
            return Some(*self.grids.get(coordinates.1).unwrap().get(coordinates.0).unwrap());
        }
        None
    }

    // Asumes coordinates is valid
    fn get_left(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)>{
        if coordinates.0 > 0{
            let xy = (coordinates.0-1, coordinates.1); 
            Some((xy, *self.grids.get(xy.1).unwrap().get(xy.0).unwrap()))
        }else{
            None
        }
        
    }
    // Asumes coordinates is valid
    fn get_right(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)>{
        if coordinates.0 < self.size.0 -1{
            let xy = (coordinates.0+1, coordinates.1);
            Some((xy,*self.grids.get(xy.1).unwrap().get(xy.0).unwrap()))
        }else{
            None
        }
        
    }
    // Asumes coordinates is valid
    fn get_above(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)>{
        if coordinates.1 < self.size.1 -1 {
            let xy = (coordinates.0, coordinates.1+1);
            Some((xy,*self.grids.get(xy.1).unwrap().get(xy.0).unwrap()))
        }else{
            None
        }
        
    }
    // Asumes coordinates is valid
    fn get_below(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)>{
        if coordinates.1 > 0{
            let xy = (coordinates.0-1, coordinates.1-1);
            Some((xy,*self.grids.get(xy.1).unwrap().get(xy.0).unwrap()))
        }else{
            None
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut start = (0,0);
    let mut goal = (0,0);
    let mut grids: Vec<Vec<usize>> = vec![];

    // Build map
    for (i, line) in input.iter().enumerate(){
        let mut row: Vec<usize> = vec![];
        for (j, c) in line.chars().enumerate(){
            if c == 'S'{
                start = (i,j);
                row.push(0);
            }else if c == 'E'{
                goal = (i,j);
                row.push(('z' as usize) - ('a' as usize));
            }else{
                row.push((c as usize) - ('a' as usize));
            }
            
        }
        grids.push(row);
    }
    let mut map: Map = Map { 
        size: (width, height), 
        start: start, 
        goal: goal, 
        grids: grids,
        valid_moves: vec![]
    };

    // Search
    map.initialize_valid_moves();
    
    map.grids.iter().for_each(|x| println!("{:?}", x));

    String::from("Placeholder part 1")
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
       
    }

    String::from("Placeholder part 2")
}