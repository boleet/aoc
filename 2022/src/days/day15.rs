use nom::character::complete::digit1;
use nom::character::streaming::i128;
use nom::sequence::{preceded, pair};
use nom::{IResult};
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::combinator::map;

#[derive(Debug)]
struct Sensor{
    location: (i128, i128),
    nearest_beacon: (i128, i128),
}

impl Sensor{
    fn distance(&self) -> i128{
        return (self.location.0 - self.nearest_beacon.0).abs() + (self.location.1 - self.nearest_beacon.1).abs();
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
    return Ok((rest,Sensor{location: val.0, nearest_beacon: val.1}));
}





#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{

    for line in input{
        let sensor = parse_packet(line).unwrap().1;
        println!("{:?}", sensor);
    }

    String::from("Placeholder part 1")
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
       
    }

    String::from("Placeholder part 2")
}