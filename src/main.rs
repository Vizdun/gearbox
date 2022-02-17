use clap::Parser;

use std::collections::HashMap;
use std::{thread, time};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: String,
    #[clap(short, long, default_value_t = 0.01)]
    step_size: f64,
    #[clap(short, long)]
    constant_time: bool,
    #[clap(short, long, default_value_t = 1.0)]
    duration: f64,
    #[clap(short, long, default_value_t = 0.0)]
    rotation: f64,
}

#[derive(Debug)]
enum SyntaxType {
    Gear,
    Number,
    String,
    Symbols,
    Parallel,
}

#[derive(Debug)]
struct SyntaxElement {
    line: usize,
    collumn: usize,
    syntax_type: SyntaxType,
    expression: String,
}

#[derive(Debug, Clone, PartialEq)]
enum GearType {
    Gear,
    Counter,
    Ender,
}

#[derive(Debug)]
struct Gear {
    gear_type: GearType,
    n: u32,
    label: Option<String>,
    symbols: Option<Vec<String>>,
    longest_symbol_len: Option<usize>,
    parallels: Vec<Gear>,
    follower: Option<Box<Gear>>,
    rotation: f64,
}

enum GearCode {
    Continue,
    Break,
}

fn parse_syntax(code: String) -> Vec<SyntaxElement> {
    let mut elements: Vec<SyntaxElement> = Vec::new();

    let mut code_lines = code.lines().enumerate();

    while let Some((line_num, line)) = code_lines.next() {
        let mut code_collumns = line.chars().enumerate();
        while let Some((collumn_num, character)) = code_collumns.next() {
            match character {
                'a'..='z' => elements.push(SyntaxElement {
                    line: line_num,
                    collumn: collumn_num,
                    syntax_type: SyntaxType::Gear,
                    expression: character.to_string(),
                }),
                '0'..='9' => elements.push(SyntaxElement {
                    line: line_num,
                    collumn: collumn_num,
                    syntax_type: SyntaxType::Number,
                    expression: {
                        let mut expression = character.to_string();
                        while let Some((_, '0'..='9')) = code_collumns.clone().peekable().peek() {
                            expression.push(code_collumns.next().unwrap().1)
                        }
                        expression
                    },
                }),
                '{' => elements.push(SyntaxElement {
                    line: line_num,
                    collumn: collumn_num,
                    syntax_type: SyntaxType::String,
                    expression: {
                        let mut expression = "".to_string();
                        while let Some((_, character)) = code_collumns.next() {
                            match character {
                                '}' => break,
                                '\\' => expression.push(code_collumns.next().unwrap().1),
                                _ => expression.push(character),
                            }
                        }
                        expression
                    },
                }),
                '"' => elements.push(SyntaxElement {
                    line: line_num,
                    collumn: collumn_num,
                    syntax_type: SyntaxType::String,
                    expression: {
                        let mut expression = "".to_string();
                        while let Some((_, character)) = code_collumns.next() {
                            match character {
                                '"' => break,
                                '\\' => expression.push(code_collumns.next().unwrap().1),
                                _ => expression.push(character),
                            }
                        }
                        expression
                    },
                }),
                '[' | ']' => elements.push(SyntaxElement {
                    line: line_num,
                    collumn: collumn_num,
                    syntax_type: SyntaxType::Parallel,
                    expression: character.to_string(),
                }),
                character if character.is_whitespace() => {}
                _ => {
                    // TODO: error
                }
            }
        }
    }

    return elements;
}

fn compile_gears<'a>(
    syntax: &mut std::iter::Peekable<impl Iterator<Item = &'a SyntaxElement>>,
) -> Option<Box<Gear>> {
    let gear_type: GearType;

    match syntax.peek() {
        Some(gear_type_syntax) => match gear_type_syntax.expression.as_str() {
            "g" => gear_type = GearType::Gear,
            "c" => gear_type = GearType::Counter,
            "e" => gear_type = GearType::Ender,
            _ => {
                panic!(
                    "Invalid gear type at line {}, collumn {}.",
                    gear_type_syntax.line, gear_type_syntax.collumn
                )
            }
        },
        None => return None,
    };
    syntax.next();
    let n: u32 = if let Some(n_syntax) = syntax.next() {
        match n_syntax.expression.parse() {
            Ok(n) => n,
            Err(_) => {
                panic!(
                    "Invalid numeric value at line {}, collumn {}.",
                    n_syntax.line, n_syntax.collumn
                )
            }
        }
    } else {
        panic!(
            "Missing numeric value."
        )
    };
    let symbols = match gear_type {
        GearType::Counter => Some({
            let mut symbols = Vec::new();
            if let Some(symbol_syntax) = syntax.next() {
                let mut chars = symbol_syntax.expression.chars();
                while let Some(character) = chars.next() {
                    match character {
                        '"' => {
                            let mut new_symbol = String::new();
                            while let Some(character) = chars.next() {
                                match character {
                                    '\\' => {
                                        new_symbol
                                            .push(symbol_syntax.expression.chars().next().unwrap());
                                    }
                                    '"' => {
                                        symbol_syntax.expression.chars().next();
                                        break;
                                    }
                                    _ => {
                                        new_symbol.push(character);
                                    }
                                }
                            }
                            symbols.push(new_symbol);
                        }
                        character if character.is_whitespace() => {}
                        _ => {
                            //TODO: Error
                        }
                    }
                }
            }
            symbols
        }),
        GearType::Gear | GearType::Ender => None,
    };
    Some(Box::new(Gear {
        gear_type: gear_type.clone(),
        n,
        symbols: symbols.clone(),
        longest_symbol_len: match symbols {
            Some(symbols) => {
                let mut symbols = symbols;
                symbols.sort_by(|b, a| a.len().partial_cmp(&b.len()).unwrap());
                Some(symbols[0].chars().collect::<Vec<char>>().len())
            }
            None => None,
        },
        label: if gear_type == GearType::Counter {
            if let Some(element) = syntax.peek() {
                match element.expression.as_str() {
                    "l" => {
                        syntax.next();
                        Some(syntax.next().unwrap().expression.clone())
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        },
        parallels: {
            let mut parallels: Vec<Gear> = Vec::new();
            loop {
                if let Some(element) = syntax.peek() {
                    match element.expression.as_str() {
                        "[" => {
                            syntax.next();
                            if let Some(new_gear) = compile_gears(syntax) {
                                parallels.push(*new_gear);
                            }
                        }
                        "]" => {
                            syntax.next();
                        }
                        _ => break,
                    }
                } else {
                    break;
                }
            }
            parallels
        },
        follower: compile_gears(syntax),
        rotation: 0.0,
    }))
}

fn gear_side_effects(gear: &mut Gear, counter_hashmap: &mut HashMap<String, String>) -> GearCode {
    match gear.gear_type {
        GearType::Counter => {
            let symbols = gear.symbols.clone().unwrap();
            let symbol_index =
                (gear.rotation.abs() * symbols.len() as f64 % symbols.len() as f64) as usize;
            let actual_index = if gear.rotation > 0.0 {
                symbol_index
            } else {
                symbols.len() - 1 - symbol_index
            };
            let symbol = &symbols[actual_index];
            let key = gear.label.clone().unwrap_or("".to_string());
            match counter_hashmap.get_mut(&key) {
                Some(counter_hashmap) => {
                    counter_hashmap.push_str(symbol);
                }
                None => {
                    counter_hashmap.insert(key, symbol.to_string());
                }
            }
        }
        GearType::Ender => {
            if gear.rotation.abs() >= 1.0 {
                return GearCode::Break;
            }
        }
        _ => {}
    }
    if let GearCode::Break = turn_follower_gear(gear, counter_hashmap) {
        return GearCode::Break;
    } else if let GearCode::Break = turn_parallel_gear(gear, counter_hashmap) {
        return GearCode::Break;
    }
    GearCode::Continue
}

fn turn_follower_gear(gear: &mut Gear, counter_hashmap: &mut HashMap<String, String>) -> GearCode {
    if let Some(mut follower) = gear.follower.as_mut() {
        follower.rotation = (gear.rotation / (follower.n as f64 / gear.n as f64)) * -1.0;
        if let GearCode::Break = gear_side_effects(follower, counter_hashmap) {
            return GearCode::Break;
        }
    }
    return GearCode::Continue;
}

fn turn_parallel_gear(gear: &mut Gear, counter_hashmap: &mut HashMap<String, String>) -> GearCode {
    for mut parallel in &mut gear.parallels {
        parallel.rotation = gear.rotation;
        if let GearCode::Break = gear_side_effects(&mut parallel, counter_hashmap) {
            return GearCode::Break;
        }
    }
    return GearCode::Continue;
}

fn execute(
    gensis_gear: Box<Gear>,
    start_rotation: f64,
    step: f64,
    step_duration: f64,
    constant_time: bool,
) {
    let mut gear = gensis_gear;
    let step_duration = step_duration * step;
    let step = step / (gear.n as f64 / 1.0);
    gear.rotation = start_rotation;
    loop {
        let mut counter_hashmap: HashMap<String, String> = HashMap::new();
        let start = time::Instant::now();
        if let GearCode::Break = turn_parallel_gear(&mut gear, &mut counter_hashmap) {
            break;
        }
        if let GearCode::Break = turn_follower_gear(&mut gear, &mut counter_hashmap) {
            break;
        }
        gear.rotation += step;
        let mut sorted_map = counter_hashmap.iter().collect::<Vec<(&String, &String)>>();
        sorted_map.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for (label, string) in sorted_map {
            println!("{}{}", label, string);
        }
        if constant_time {
            let sleep_time = step_duration - start.elapsed().as_secs_f64();
            if sleep_time > 0.0 {
                thread::sleep(time::Duration::from_secs_f64(sleep_time));
            }
        }
        for _ in 0..counter_hashmap.len() {
            print!("\x1b[F");
        }
    }
    print!("\n");
}

fn main() {
    let args = Args::parse();

    let file = std::fs::read_to_string(args.file).unwrap();

    let syntax = parse_syntax(file);

    let genesis_gear = compile_gears(&mut syntax.iter().peekable()).unwrap();

    execute(
        genesis_gear,
        args.rotation,
        args.step_size,
        args.duration,
        args.constant_time,
    );
}
