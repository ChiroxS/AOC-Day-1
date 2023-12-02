fn main() {
    let sum : i32 = include_str!("../input.txt")
        .split("\n")
        .map(|l| replace_digit(l.to_string()))
        .map(|l| l.to_string().chars().filter(|c| c.is_numeric()).collect::<String>())
        .filter(|l| l.to_string().len() > 0)
        .map(|l| vec![l.chars().next().unwrap(), 
                      l.chars().last().unwrap()]
                        .into_iter().collect::<String>())
        .map(|l| l.to_string().parse::<i32>().unwrap())
        .sum();
    println!("{}", sum);
}

fn replace_digit(s: String) -> String {
    let mut my_s = s; 
    for n in 1..=9 {
        let str_val = get_str_value(n);
        my_s = my_s.replace(&str_val, &n.to_string());
    };
    return my_s;
}

fn get_str_value(nr : i32 ) -> &'static str {
    match nr {
        1=> return "one",
        2=> return "two",
        3=> return "three",
        4=> return "four",
        5=> return "five",
        6=> return "six",
        7=> return "seven",
        8=> return "eight",
        9=> return "nine",
        _=> return ""
    }
}
