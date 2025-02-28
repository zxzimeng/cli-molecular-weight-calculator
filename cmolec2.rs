use std::collections::HashMap;
use std::env;

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
    let mut result: HashMap<String, u32> = HashMap::new();
    let mut stack: Vec<HashMap<String, u32>> = vec![HashMap::new()];
    let mut current_element = String::new();
    let mut current_number = String::new();
    let mut chars = formula.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' | '_' => {
                stack.push(HashMap::new());
            },
            ')' | '_' => {
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

fn create_element_table() -> HashMap<String, f64> {
    let mut elements = HashMap::new();
    elements.insert("H".to_string(), 1.00794);
    elements.insert("He".to_string(), 4.002602);
    elements.insert("Li".to_string(), 6.941);
    elements.insert("Be".to_string(), 9.012182);
    elements.insert("B".to_string(), 10.811);
    elements.insert("C".to_string(), 12.0107);
    elements.insert("N".to_string(), 14.0067);
    elements.insert("O".to_string(), 15.9994);
    elements.insert("F".to_string(), 18.9984032);
    elements.insert("Ne".to_string(), 20.1797);
    // ... add more elements as needed
    elements
}