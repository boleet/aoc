pub struct RPCChoice{
    value: RPCValue
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum RPCValue {
    ROCK,
    PAPER,
    SCISSORS,
}


impl RPCChoice{
    fn new(choice: &str) -> RPCChoice{
        let val = match choice{
            "A" | "X" =>{
                RPCValue::ROCK
            },
            "B" | "Y" =>{
                RPCValue::PAPER
            },
            "C" | "Z" =>{
                RPCValue::SCISSORS
            },
            _ =>{
                panic!("could not find choice")
            }
        };
        RPCChoice { value: val }
    }

    // Should rather return RPCGameResult
    fn wins(&self, other: &RPCChoice) -> bool{
        if self.value == RPCValue::ROCK && other.value == RPCValue::SCISSORS
        || self.value == RPCValue::SCISSORS && other.value == RPCValue::PAPER
        || self.value == RPCValue::PAPER && other.value == RPCValue::ROCK{
            return true;
        }
        false
    }

    fn points(&self, other: &RPCChoice) -> i64{
        let mut mypoints: i64 = 0;
        // TODO add choice points
        match self.value{
            RPCValue::ROCK => mypoints +=1,
            RPCValue::PAPER => mypoints +=2,
            RPCValue::SCISSORS => mypoints +=3,
        }
        // draw
        if self.value == other.value {
            mypoints += 3;
        }else if self.wins(other){
            mypoints += 6
        }

        mypoints
    }

    fn new_draw(other: &RPCChoice) -> RPCChoice{
        return RPCChoice { value: other.value }
    }

    fn new_winer(other: &RPCChoice) -> RPCChoice{
        let val = match other.value {
            RPCValue::ROCK => RPCValue::SCISSORS,
            RPCValue::PAPER => RPCValue::ROCK,
            RPCValue::SCISSORS => RPCValue::PAPER,
        };
        return RPCChoice { value: val }
    }

    fn new_winner(other: &RPCChoice) -> RPCChoice{
        let val = match other.value {
            RPCValue::ROCK => RPCValue::PAPER,
            RPCValue::PAPER => RPCValue::SCISSORS,
            RPCValue::SCISSORS => RPCValue::ROCK,
        };
        return RPCChoice { value: val }
    }


}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    // A rock, B paper, C scissors
    // X rock, Y paper, C scissors
    // 1 Rock, 2 Paper, 3 scissors + 0 lost, 3 draw, 6 win

    let mut score: i64 = 0;
    for line in input{
        let parts = line.split(" ").collect::<Vec<&str>>();
        let choice_a = RPCChoice::new(parts[0]);
        let choice_b = RPCChoice::new(parts[1]);
        score += choice_b.points(&choice_a);
    }

    score.to_string()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut score: i64 = 0;
    for line in input{
        let parts = line.split(" ").collect::<Vec<&str>>();
        let choice_a = RPCChoice::new(parts[0]);
        let choice_b = match parts[1]{
            "X" =>{
                RPCChoice::new_winer(&choice_a)
            },
            "Y" =>{
                RPCChoice::new_draw(&choice_a)
            },
            "Z" =>{
                RPCChoice::new_winner(&choice_a)
            },
            _ =>{
                panic!("could not find choice")
            }
        };

        score += choice_b.points(&choice_a);
    }

    score.to_string()
}