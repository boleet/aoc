use std::{vec};
use nom::sequence::{delimited};
use nom::{IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::character::complete::digit1;
use nom::multi::separated_list0;
use std::cmp::Ordering;


#[derive(Debug, PartialEq, Eq, Ord)]
enum Packet{
    Value(i64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //Some(Less) or Equal or Greater
        match self{
            Packet::Value(x) => {
                match other{
                    Packet::Value(y) =>{
                        Some(x.cmp(y))
                    },
                    Packet::List(_) =>{
                        return Packet::List(vec![Packet::Value(*x)]).partial_cmp(other);
                    }
                }
            },
            Packet::List(x) =>{
                match other{
                    Packet::Value(y) =>{
                        return self.partial_cmp(&Packet::List(vec![Packet::Value(*y)]));
                    },
                    Packet::List(y) =>{
                        for i in 0..x.len(){
                            match y.get(i){
                                Some(elem) =>{
                                    match x.get(i).unwrap().partial_cmp(elem){
                                        Some(res) =>{
                                            match res{
                                                Ordering::Equal => continue,
                                                Ordering::Greater => return Some(Ordering::Greater),
                                                Ordering::Less => return Some(Ordering::Less)
                                            }
                                        },
                                        None => return None // should not happen?
                                    }
                                },
                                None => {
                                    return Some(Ordering::Greater)
                                }
                            }
                        }
                        if y.len() > x.len(){ // right list is bigger, thats not good
                            return Some(Ordering::Less);
                        }else{ // < case is handled above, so only this == is left
                            return Some(Ordering::Equal);
                        }
                    }
                }
            }
        }
    }
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

    let mut count = 0;

    let mut i = 1;
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

        // println!("Packet a {:?} vs packet b {:?}", packet_a, packet_b);
        if packet_a <= packet_b{
            count += i;
            // println!("Just added packet that is in order: {}", i);
        }

        

        
        // Skip empty line
        input_iter.next();
        i +=1;
    }

    count.to_string()
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut input_iter = input.iter().peekable();
    let mut packets: Vec<Packet> = vec![];

    while input_iter.peek().is_some(){
        let inp = input_iter.next().unwrap();
        if !inp.is_empty(){
            match parse_packet(inp){
                Ok(x) => {
                    packets.push(x.1);
                }
                Err(e) => {
                    println!("error {:?}", e);
                    break;
                }
            }
        }
    }
    let start_packet = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let end_packet = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    packets.push(start_packet);
    packets.push(end_packet);
    packets.sort();

    // packets.iter().for_each(|x| println!("{:?}", x));

    let mut result = 1;

    for (i,p) in packets.iter().enumerate(){
        if p == &Packet::List(vec![Packet::List(vec![Packet::Value(2)])]) || p == &Packet::List(vec![Packet::List(vec![Packet::Value(6)])]){
            // println!("Found signal at {}", i);
            result *= i+1;
        }
    }
    result.to_string()
}