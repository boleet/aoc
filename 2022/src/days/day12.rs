use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}
struct Map {
    size: (usize, usize),  // width, height
    start: (usize, usize), // x, y position. top left is 0,0
    goal: (usize, usize),  // x, y position. top left is 0,0
    grids: Vec<Vec<usize>>,
    valid_moves: Vec<Vec<Vec<Move>>>,
    // valid_routes: Vec<usize>,
}

fn get_minimum(values: &Vec<Vec<usize>>, unvisited: &HashSet<(usize, usize)>) -> (usize, usize){
    let mut min_value = usize::MAX;
    let mut min_coordinates = (0,0);
    let mut uninitialized = true;

    for c in unvisited{
        if uninitialized || values[c.0][c.1] < min_value{
            min_value = values[c.0][c.1];
            min_coordinates = (c.0,c.1);
            uninitialized = false;
        }
    }
    return min_coordinates
}

impl Map {

    fn dijkstra_shortest_path(&self, starting_point: (usize, usize)) -> usize{
        let size = self.size;
        let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
        for x in 0..size.0{
            for y in 0..size.1{
                unvisited.insert((x,y));
            }
        }

        let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; size.1]; size.0]; // NOTE: flipped compared to other coordinate lists
        distances[starting_point.0][starting_point.1] = 0;
        
        let mut current = starting_point;
        'outer: while unvisited.len() > 0{
            
            let current_value = distances[current.0][current.1];
            // println!("Working on current node {:?} with {} unvisited left", current, unvisited.len());
            for m in self.get_valid_moves(current){
                let next = self.get_next(m, current).unwrap().0;                
                if !unvisited.contains(&next){ // already visisted, skip
                    continue;
                }

                let mut new_value = current_value;
                if new_value < usize::MAX{
                    new_value += 1;
                }

                if new_value < distances[next.0][next.1]{
                    // println!("Updating next {:?} to value {}", next,new_value);
                    distances[next.0][next.1] = new_value
                }
                if next == self.goal{
                    println!("Found goal! Value {}", distances[self.goal.0][self.goal.1]);
                    break 'outer;
                }
                
            }
            unvisited.remove(&current);

            current = get_minimum(&distances, &unvisited);
        }
        
        // distances.iter().for_each(|x| println!("{:?}",x));
        
        return distances[self.goal.0][self.goal.1];
    }


    fn initialize_valid_moves(&mut self) {
        for y in 0..self.size.1 {
            self.valid_moves.push(vec![]);
            for x in 0..self.size.0 {
                let valid_ms = self.get_dynamic_valid_moves((x, y));
                self.valid_moves[y].push(valid_ms);
            }
        }
    }

    fn get_valid_moves(&self, coordinates: (usize, usize)) -> Vec<Move> {
        return self
            .valid_moves
            .get(coordinates.1)
            .unwrap()
            .get(coordinates.0)
            .unwrap()
            .to_vec();
    }

    // Asumes coordinates is valid
    fn get_dynamic_valid_moves(&self, coordinates: (usize, usize)) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];
        let l = self.get_left(coordinates);
        let r = self.get_right(coordinates);
        let a = self.get_above(coordinates);
        let b = self.get_below(coordinates);
        let current_value = self.get_value(coordinates).unwrap();

        if l.is_some() && l.unwrap().1 <= current_value + 1{
            result.push(Move::Left);
        }
        if r.is_some() && r.unwrap().1 <= current_value + 1 {
            result.push(Move::Right);
        }
        if a.is_some() && a.unwrap().1 <= current_value + 1{
            result.push(Move::Up);
        }
        if b.is_some() && b.unwrap().1 <= current_value + 1{
            result.push(Move::Down);
        }
        result
    }

    fn get_value(&self, coordinates: (usize, usize)) -> Option<usize> {
        if coordinates.0 < self.size.0 && coordinates.1 < self.size.1
        // greater than 0 check usuless due to type limitations
        {
            return Some(
                *self
                    .grids
                    .get(coordinates.1)
                    .unwrap()
                    .get(coordinates.0)
                    .unwrap(),
            );
        }
        None
    }

    fn get_next(&self, m: Move, coordinates: (usize, usize)) -> Option<((usize, usize), usize)> {
        match m {
            Move::Left => self.get_left(coordinates),
            Move::Right => self.get_right(coordinates),
            Move::Up => self.get_above(coordinates),
            Move::Down => self.get_below(coordinates),
        }
    }

    // Asumes coordinates is valid
    fn get_left(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)> {
        if coordinates.0 > 0 {
            let xy = (coordinates.0 - 1, coordinates.1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    // Asumes coordinates is valid
    fn get_right(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)> {
        if coordinates.0 < self.size.0 - 1 {
            let xy = (coordinates.0 + 1, coordinates.1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    // Asumes coordinates is valid
    fn get_below(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)> {
        if coordinates.1 < self.size.1 - 1 {
            let xy = (coordinates.0, coordinates.1 + 1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
    // Asumes coordinates is valid
    fn get_above(&self, coordinates: (usize, usize)) -> Option<((usize, usize), usize)> {
        if coordinates.1 > 0 {
            let xy = (coordinates.0, coordinates.1 - 1);
            Some((xy, self.get_value(xy).unwrap()))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String {
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut grids: Vec<Vec<usize>> = vec![];

    // Build map
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<usize> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
                row.push(0);
            } else if c == 'E' {
                goal = (x, y);
                row.push(('z' as usize) - ('a' as usize));
            } else {
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
        valid_moves: vec![],
        // valid_routes: vec![]
    };
    println!("Loaded map, now initializing valid moves");
    map.initialize_valid_moves();
    

    // Search
    println!("Starting search from start location {:?} to goal {:?}", start, goal);
    let result = map.dijkstra_shortest_path(map.start);
    result.to_string()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String {
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut grids: Vec<Vec<usize>> = vec![];

    // Build map
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<usize> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
                row.push(0);
            } else if c == 'E' {
                goal = (x, y);
                row.push(('z' as usize) - ('a' as usize));
            } else {
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
        valid_moves: vec![],
        // valid_routes: vec![]
    };
    println!("Loaded map, now initializing valid moves");
    map.initialize_valid_moves();
    
    // determine starting points
    let mut starting_points: Vec<(usize, usize)> = vec![];
    for y in 0..map.grids.len(){
        for x in 0..map.grids.first().unwrap().len(){
            if map.grids[y][x] == 0{
                starting_points.push((x,y));
            }
        }
    }
    // Search
    let mut min = usize::MAX;
    for s in starting_points{
        println!("Starting search from start location {:?} to goal {:?}", s, goal);
        let result = map.dijkstra_shortest_path(s);
        if result < min{
            min = result;
        }
    }
   
    min.to_string()
}
