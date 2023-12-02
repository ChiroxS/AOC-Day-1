fn main() {
    
    let initial  = include_str!("../input.txt")
        .split("\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let digits : Vec<i32> = include_str!("../input.txt")
        .split("\n")
        .map(|l| vec![find_left(l), find_right(l)].iter().collect::<String>())
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let sum : i32 = digits
        .iter()
        .sum();
    
    println!("Sum is {}", sum);

    for n in 0..1000 {
        println!("initial {}", initial[n]);
        println!("result {}", digits[n]);
        println!("");
    }

}

fn find_left(s: &str) -> char {
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9",
                  "one", "two", "three", "four", "five",
                  "six", "seven", "eight", "nine"];
    let idx_vec : Vec<i32> = digits 
        .iter()
        .map(|d| s.match_indices(d).collect::<Vec<_>>().iter().next().unwrap_or(&(999 as usize,"")).0 as i32)
        .collect();
    let min : i32 = idx_vec    
        .iter()
        .enumerate()
        .min_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
        .unwrap() as i32;
    return char::from_digit(get_numeric_value(digits.iter().nth(min as usize).unwrap()),10).unwrap();
}


fn find_right(s: &str) -> char {
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9",
                  "one", "two", "three", "four", "five",
                  "six", "seven", "eight", "nine"];
    let idx_vec : Vec<i32> = digits 
        .iter()
        .map(|d| s.rmatch_indices(d).collect::<Vec<_>>().iter().next().unwrap_or(&(999 as usize,"")).0)
        .map(|d| {if d == 999 {return -1;} else {return d as i32;}})
        .collect();
    let max : i32 = idx_vec    
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
        .unwrap() as i32;
    return char::from_digit(get_numeric_value(digits.iter().nth(max as usize).unwrap()),10).unwrap();
}

fn get_numeric_value(s : &str) -> u32 {
    match s { 
        "1"|"one" => return 1, 
        "2"|"two" => return 2,
        "3"|"three" => return 3, 
        "4"|"four" => return 4,
        "5"|"five" => return 5, 
        "6"|"six" => return 6,
        "7"|"seven" => return 7,
        "8"|"eight" => return 8, 
        "9"|"nine" => return 9,
        _ => return 0
    }
}


