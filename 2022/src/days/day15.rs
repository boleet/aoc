use std::collections::HashSet;

use nom::character::complete::digit1;
use nom::character::streaming::i128;
use nom::sequence::{preceded, pair};
use nom::{IResult};
use nom::bytes::complete::tag;
use nom::combinator::map;

#[derive(Debug)]
struct Sensor{
    location: (i128, i128),
    nearest_beacon: (i128, i128),
    distance: usize,
}

impl Sensor{
    fn new(location: (i128, i128), nearest_beacon: (i128, i128)) -> Self{
        Sensor { 
            location: location, 
            nearest_beacon: nearest_beacon, 
            distance: distance(location, nearest_beacon)
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Sensor>{
    let (rest, val) = (
        pair(
            pair(
                preceded(
                    tag("Sensor at x="), 
                    i128
                ),
                preceded(
                    tag(", y="), 
                    i128
                ),
            ),
            pair(
                preceded(
                    tag(": closest beacon is at x="), 
                    i128
                ),
                preceded(
                    tag(", y="), 
                    map(// TODO i128 cannot distinguish end of line error from completed input
                        digit1,
                        |x: &str| x.parse::<i128>().unwrap() 
                    )
                      
                ),
            )   
        )
    )(input)?;
    return Ok((rest,Sensor::new(val.0, val.1)));
}


fn distance(a: (i128, i128), b: (i128, i128)) -> usize{
    let res =((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize;
    // println!("distance between {:?} and {:?} is {}", a, b, res);
    res

}


#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{

    // Parse vectors and determine size of playing field
    let mut sensors: Vec<Sensor> = vec![];
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;
    let mut min_x: isize = 0;
    let mut min_y: isize = 0;
    for line in input{
        let sensor = parse_packet(line).unwrap().1;
        if sensor.location.0 > max_x as i128{
            max_x = sensor.location.0 as isize;
        }
        if sensor.nearest_beacon.0 > max_x  as i128{
            max_x = sensor.nearest_beacon.0 as isize;
        }
        if sensor.location.1 > max_y  as i128{
            max_y = sensor.location.1 as isize;
        }
        if sensor.nearest_beacon.1 > max_y  as i128{
            max_y = sensor.nearest_beacon.1 as isize;
        }
        if sensor.location.0 < min_x  as i128{
            min_x = sensor.location.0 as isize;
        }
        if sensor.nearest_beacon.0 < min_x  as i128{
            min_x = sensor.nearest_beacon.0 as isize;
        }
        if sensor.location.1 < min_y  as i128{
            min_y = sensor.location.1 as isize;
        }
        if sensor.nearest_beacon.1 < min_y  as i128{
            min_y = sensor.nearest_beacon.1 as isize;
        }
        sensors.push(sensor);
    }

    println!("Dimensions of our puzzle are x {} to {} and y {} to {}", min_x, max_x, min_y, max_y);
    let target_y: i128 = 2000000 ; //2000000
    let mut count: i128 = 0;
    let edges = max_x - min_x + 10000;
    for x in (min_x-edges)..(max_x + edges){
        for s in &sensors{
            if distance((x as i128, target_y), s.location) <= s.distance{
                count += 1;
                break;
            }
        }
    }

    let mut intersecting_objects: HashSet<(i128, i128)> = HashSet::new();
    for s in &sensors{
        if s.location.1 == target_y {
            intersecting_objects.insert(s.location);
        }
        if s.nearest_beacon.1 == target_y{
            intersecting_objects.insert(s.nearest_beacon);
        }
    }
    println!("Intersecting objects {:?}", intersecting_objects);
    count -= intersecting_objects.len() as i128;
    
    count.to_string()  //4502208
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{ // Not very efficient sadly, exhaustive search takes a day on average...
    let mut sensors: Vec<Sensor> = vec![];
    let max_x: isize = 4000000; //4000000

    let max_y: isize = 4000000;
    for line in input{
        let sensor = parse_packet(line).unwrap().1;
        sensors.push(sensor);
    }
   


    // let mut x = 4000000;
    let mut x = -1;
    let mut y;
    while x < max_x{
        x -= 1;
        if x % 10000 == 0{
            println!("Working on x {}", x);
        }
        y = -1;
        'yloop: while y < max_y{
            y += 1;
            for s in &sensors{
                if distance((x as i128, y as i128), s.location) <= s.distance{
                    let jump: isize = s.location.1 as isize - y - 1;
                    if jump >0{
                        y += jump;
                    }
                    continue 'yloop;
                }
            }
            let spot = (x,y);
            println!("Found the spot! {:?}", spot);
            return (spot.0 * 4000000 + spot.1).to_string(); //Found the spot! (3446137, 3204480) Answer day 15b 13784551204480
            
        }
    }

    String::from("Placeholder part 2")
}