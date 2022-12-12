use std::process::exit;

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

impl Map {
    fn initialize_all_routes(&self, trace: &mut Vec<(usize, usize)>) -> Option<&mut Vec<(usize, usize)>> {        
        // println!("Simulate_route now at {:?}", *trace.last().unwrap());
        let valid_moves = self.get_valid_moves(*trace.last().unwrap());
        if valid_moves.len() == 1 {
            // println!("This is a dead end... Trace : {:?}", trace);
            trace.pop();
            return None;
        } else {
            for m in valid_moves {
                let current_position = *trace.last().unwrap();
                let next = self.get_next(m, current_position).unwrap().0;
                if next == self.goal {
                    println!("Done, found goal! Trace length {}", trace.len());
                    continue;
                } else {
                    if trace.contains(&next){ // we already visited, dont enter loop
                        continue;
                    }
                    trace.push(next);

                    match self.initialize_all_routes(trace) {
                        Some(x) => return Some(x),
                        None => {continue}
                    }
                }
            }
            trace.pop();
            // println!("This is a dead end... Trace : {:?}", trace);
            return None
        }
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
        if a.is_some() && a.unwrap().1 <= current_value + 1 {
            result.push(Move::Up);
        }
        if b.is_some() && b.unwrap().1 <= current_value + 1 {
            result.push(Move::Down);
        }
        result
    }

    fn get_value(&self, coordinates: (usize, usize)) -> Option<usize> {
        if coordinates.0 < self.size.0
            && coordinates.1 < self.size.1 // greater than 0 check usuless due to type limitations
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

    // map.grids.iter().for_each(|x| println!("{:?}", x));
    // println!();
    map.initialize_valid_moves();
    // map.valid_moves.iter().for_each(|x| {x.iter().for_each(|y| print!("{:?}", y)); println!()});

    // Search
    println!("Starting search from start location {:?} to goal {:?}", start, goal);
    let mut trace = vec![map.start];
    let result = map.initialize_all_routes(&mut trace);
    println!("Search result is: {:?}", result);
    

    String::from("Placeholder part 1")
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String {
    for line in input {}

    String::from("Placeholder part 2")
}
