use std::{str::Chars, vec};
use nom::sequence::{preceded, delimited};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::character::complete::digit1;
use nom::multi::separated_list0;


#[derive(Debug)]
enum Packet{
    Value(i64),
    List(Vec<Packet>),
}

fn parse_packet(input: &str) -> IResult<&str, Packet>{
    // println!("Parsing packet {:?}", input);
    let (rest, val) = separated_list0(
        tag(","),
        alt((
            value,
            delimited(tag("["), parse_packet, tag("]"))
        ))
    )(input)?;
    return Ok((rest, Packet::List(val)));
}

fn value(input: &str) -> IResult<&str, Packet>{
    let (rest, value) = many1(digit1)(input)?;
    Ok((rest, Packet::Value(value.join("").parse().unwrap())))
}



#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut input_iter = input.iter().peekable();
    let mut packet_a: Packet;
    let mut packet_b: Packet; 

    while input_iter.peek().is_some(){
        match parse_packet(input_iter.next().unwrap()){
            Ok(x) => {
                packet_a = x.1;
            }
            Err(e) => {
                println!("error {:?}", e);
                break;
            }
        }
        match parse_packet(input_iter.next().unwrap()){
            Ok(x) => {
                packet_b = x.1;
            }
            Err(e) => {
                println!("error {:?}", e);
                break;
            }
        }

        println!("Packet a {:?} vs packet b {:?}", packet_a, packet_b);
        
        // Skip empty line
        input_iter.next();
        break;
    }

    String::from("Placeholder part 1")
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
       
    }

    String::from("Placeholder part 2")
}