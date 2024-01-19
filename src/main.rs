use std::io;

fn main() {
    // 
    let intro = r#"
     _____                _          _          _                          _                         
    |_   _|   ___   ___  | |_       / \      __| | __   __   ___   _ __   | |_   _   _   _ __    ___ 
      | |    / _ \ / __| | __|     / _ \    / _` | \ \ / /  / _ \ | '_ \  | __| | | | | | '__|  / _ \
      | |   |  __/ \__ \ | |_     / ___ \  | (_| |  \ V /  |  __/ | | | | | |_  | |_| | | |    |  __/
      |_|    \___| |___/  \__|   /_/   \_\  \__,_|   \_/    \___| |_| |_|  \__|  \__,_| |_|     \___|
                                                                                                     
    
    1 - Start
    2 - Options
    3 - Exit
    "#;
    println!("{intro}");

    let input = get_input_number(3);

    match input {
        1 => println!("Start Adventure!"),
        2 => println!("Options"),
        3 => println!("Are you sure you want to exit?"),
        _ => panic!("Program encountered an error: 1"),
    };
}

fn get_input_number(max:u32) -> u32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };

        if (1..=max).contains(&input) {
            return input
        }

        println!("Please input a number between 1 and {max}");
    }
}
