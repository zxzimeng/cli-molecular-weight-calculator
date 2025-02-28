use std::collections::HashMap;
use std::env;
use periodic_table_rs::PERIODIC_TABLE;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a chemical formula as an argument.");
        println!("Usage: {} <formula>", args[0]);
        println!("Example: {} '(H2O)2'", args[0]);
        std::process::exit(1);
    }

    let formula = &args[1];
    let result = calculate_mass(formula);
    println!("{:.4}", result);
}

fn calculate_mass(formula: &str) -> f64 {
    let elements = create_element_table();
    let parsed = parse_formula(formula);
    
    for (element, _) in &parsed {
        if !elements.contains_key(element) {
            eprintln!("Error: Unknown element '{}'", element);
            std::process::exit(1);
        }
    }
    
    parsed.iter()
        .map(|(element, count)| elements[element] * (*count as f64))
        .sum()
}

fn parse_formula(formula: &str) -> HashMap<String, u32> {
    let mut stack: Vec<HashMap<String, u32>> = vec![HashMap::new()];
    let mut current_element = String::new();
    let mut current_number = String::new();
    let mut chars = formula.chars().peekable();
    let mut in_underscore = false;

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                if !current_element.is_empty() {
                    process_element(&mut stack, &current_element, &current_number);
                    current_element = String::new();
                    current_number = String::new();
                }
                stack.push(HashMap::new());
            },
            ')' => {
                if !current_element.is_empty() {
                    process_element(&mut stack, &current_element, &current_number);
                    current_element = String::new();
                    current_number = String::new();
                }
                process_group(&mut stack, &mut chars);
            },
            '_' => {
                if !in_underscore {
                    if !current_element.is_empty() {
                        process_element(&mut stack, &current_element, &current_number);
                        current_element = String::new();
                        current_number = String::new();
                    }
                    stack.push(HashMap::new());
                    in_underscore = true;
                } else {
                    if !current_element.is_empty() {
                        process_element(&mut stack, &current_element, &current_number);
                        current_element = String::new();
                        current_number = String::new();
                    }
                    process_group(&mut stack, &mut chars);
                    in_underscore = false;
                }
            },
            'A'..='Z' => {
                if !current_element.is_empty() {
                    let count = current_number.parse::<u32>().unwrap_or(1);
                    let current = stack.last_mut().unwrap();
                    *current.entry(current_element).or_insert(0) += count;
                    current_element = String::new();
                    current_number = String::new();
                }
                current_element.push(c);
            },
            'a'..='z' => {
                current_element.push(c);
            },
            '0'..='9' => {
                current_number.push(c);
            },
            _ => {}
        }
    }

    if !current_element.is_empty() {
        let count = current_number.parse::<u32>().unwrap_or(1);
        let current = stack.last_mut().unwrap();
        *current.entry(current_element).or_insert(0) += count;
    }

    stack.pop().unwrap()
}

fn process_group(stack: &mut Vec<HashMap<String, u32>>, chars: &mut std::iter::Peekable<std::str::Chars>) {
    let mut number = String::new();
    while let Some(&next) = chars.peek() {
        if next.is_digit(10) {
            number.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    let multiplier = number.parse::<u32>().unwrap_or(1);
    let top = stack.pop().unwrap();
    let current = stack.last_mut().unwrap();
    
    for (element, count) in top {
        *current.entry(element).or_insert(0) += count * multiplier;
    }
}

fn create_element_table() -> HashMap<String, f64> {
    let mut elements = HashMap::new();
    for element in PERIODIC_TABLE.iter() {
        elements.insert(format!("{:?}", element.symbol), element.atomic_mass);
    }
    elements
}

fn process_element(stack: &mut Vec<HashMap<String, u32>>, element: &str, number: &str) {
    let count = number.parse::<u32>().unwrap_or(1);
    let current = stack.last_mut().unwrap();
    *current.entry(element.to_string()).or_insert(0) += count;
}