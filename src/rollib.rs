use command_parser::generic_parser;
use core::f32;
use dicerollerlib::rolltypes::{ModifierOperator, Operators, RollRequest, RollResult};
pub fn generate_modfiers(arr: &Vec<Vec<&str>>) -> Vec<ModifierOperator> {
    let mut modifers_list: Vec<ModifierOperator> = Vec::new();
    for chunk in arr {
        let op = match chunk[0] {
            "+" => Operators::Sum,
            "-" => Operators::Sub,
            "x" => Operators::Mult,
            "/" => Operators::Div,
            &_ => Operators::Sum,
        };
        let number: f32 = chunk[1].parse::<f32>().unwrap();
        let mo = ModifierOperator {
            operator: op,
            number,
        };
        modifers_list.push(mo);
    }
    modifers_list
}
pub fn generate_roll_requests(args: Vec<String>) -> Vec<RollRequest> {
    let mut requests: Vec<RollRequest> = Vec::new();
    for arg in args {
        let mut parsed = generic_parser(&arg, vec!['+', '-', 'x', '/']);
        let dice_args = parsed[0];
        let dice_args = generic_parser(dice_args, vec!['d']);
        parsed.remove(0);

        let modifiers: Option<Vec<ModifierOperator>> = match parsed.len() {
            n if n > 1 => Option::from(generate_modfiers(
                &parsed
                    .into_iter()
                    .array_chunks::<2>()
                    .map(|sub| sub.into())
                    .collect(),
            )),
            _ => None,
        };

        let req = RollRequest {
            dice_qnt: dice_args[0].parse().unwrap(),
            dice_type: dice_args[2].parse().unwrap(),
            modifier: modifiers,
        };
        requests.push(req);
    }
    requests
}

pub fn print_results(results: Vec<RollResult>) {
    for pool in results {
        println!("Rolling {}:", pool.pool);
        println!("==========================");
        for roll in pool.rolls {
            let extra_message: &str = match roll.dice_type {
                20 => match roll.roll {
                    20 => "Critical Success",
                    1 => "Critical Failure",
                    _ => "",
                },
                _ => "",
            };
            println!(
                "Roll {}: {} (D{}) {}",
                roll.roll_number, roll.roll, roll.dice_type, extra_message
            );
        }
        println!("Final result: {}", pool.sum)
    }
}
