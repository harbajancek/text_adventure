use std::io;

fn main() {
    let title = r#"
 _____                _          _          _                          _                         
|_   _|   ___   ___  | |_       / \      __| | __   __   ___   _ __   | |_   _   _   _ __    ___ 
  | |    / _ \ / __| | __|     / _ \    / _` | \ \ / /  / _ \ | '_ \  | __| | | | | | '__|  / _ \
  | |   |  __/ \__ \ | |_     / ___ \  | (_| |  \ V /  |  __/ | | | | | |_  | |_| | | |    |  __/
  |_|    \___| |___/  \__|   /_/   \_\  \__,_|   \_/    \___| |_| |_|  \__|  \__,_| |_|     \___|

"#
    .trim_matches('\n');

    let empty_scene: Scene = Scene {id: String::from("empty"), choices: Option::None, text: String::new()};
    let mut choices = vec![
        Choice {display_name: String::from("Start"), ..Default::default()},
        Choice {display_name: String::from("Load"), ..Default::default()},
        Choice {display_name: String::from("Options"), ..Default::default()},
        Choice {display_name: String::from("End"), ..Default::default()}];
    let intro_scene = Scene { id: String::from("intro"), choices: Option::Some(choices), text: String::from(title)};

    let mut global_vars = GlobalVars {
        ..Default::default()
    };

    let mut current_scene = &intro_scene;

    loop {
        process_scene(&mut current_scene);
    }
}

fn get_choice_input(max: usize) -> usize {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        if (1..=max).contains(&input) {
            return input;
        }

        println!("Please input a number between 1 and {max}");
    }
}

fn process_scene(current_scene : &mut Scene) {
    println!("{}", current_scene.text);

    if let Some(choices) = &current_scene.choices {
        for (i, choice) in choices.iter().enumerate() {
            println!("{i} - {}", choice.display_name);
        }

        let choice = choices[get_choice_input(choices.len())];
        
        if let Some(text) = choice.text {
            println!("{text}");
        }

        current_scene = choice.next_scene;
    } else {
        println!("1 - Continue");
        get_choice_input(1);
        current_scene = current_scene.next_scene.expect("Can't change scene - next scene variable is empty.")
    }
}

#[derive(Default)]
struct GlobalVars {
    player_name: String,
    points: i32,
}

#[derive(Default)]
struct Choice {
    next_scene: Option<Scene>,
    display_name: String,
    text: Option<String>,
    changes: Option<fn() -> ()>,
}

#[derive(Default)]
struct Scene {
    id: String,
    choices: Option<Vec<Choice>>,
    text: String,
    next_scene: Option<Scene>,
}
