use std::{io};

fn main() {
    let title = r#"
 _____                _          _          _                          _                         
|_   _|   ___   ___  | |_       / \      __| | __   __   ___   _ __   | |_   _   _   _ __    ___ 
  | |    / _ \ / __| | __|     / _ \    / _` | \ \ / /  / _ \ | '_ \  | __| | | | | | '__|  / _ \
  | |   |  __/ \__ \ | |_     / ___ \  | (_| |  \ V /  |  __/ | | | | | |_  | |_| | | |    |  __/
  |_|    \___| |___/  \__|   /_/   \_\  \__,_|   \_/    \___| |_| |_|  \__|  \__,_| |_|     \___|

"#
    .trim_matches('\n');

    //let empty_scene: Scene = Scene {id: String::from("empty"), choices: None, text: String::new(), ..Default::default()};

    let mut ending_choices = vec![
        Choice {display_name: String::from("No"), ..Default::default()},
        Choice {display_name: String::from("Yes"), ..Default::default()},
    ];

    let ending_scene = Scene {id: String::from("end"), choices: Some(ending_choices), text: String::from("Are you sure you want to exit the game?"), ..Default::default()};

    let intro_choices = vec![
        Choice {display_name: String::from("Start"), ..Default::default()},
        Choice {display_name: String::from("Load"), ..Default::default()},
        Choice {display_name: String::from("Options"), ..Default::default()},
        Choice {display_name: String::from("End"), next_scene: Some(Box::from(ending_scene)), ..Default::default()}];

    let intro_scene = Scene { id: String::from("intro"), choices: Some(intro_choices), text: String::from(title), ..Default::default()};

    ending_choices[0].next_scene = Some(Box::from(&intro_scene).);

    let mut global_vars = GlobalVars {
        ..Default::default()
    };

    let mut current_scene = &intro_scene;

    loop {
        if let Some(scene) = process_scene(&current_scene) {
            current_scene = &scene;
        } else {
            break;
        }
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

fn process_scene(current_scene : &Scene) -> Option<&Scene>{
    println!("{}", current_scene.text);

    let next_scene: Option<&Scene>;

    if let Some(choices) = &current_scene.choices {
        for (i, choice) in choices.iter().enumerate() {
            println!("{i} - {}", choice.display_name);
        }

        let choice = choices[get_choice_input(choices.len())];
        
        if let Some(text) = choice.text {
            println!("{text}");
        }
        
        next_scene = choice.next_scene.as_deref();
    } else {
        println!("1 - Continue");

        get_choice_input(1);

        next_scene = current_scene.next_scene.as_deref();
    }

    next_scene
}

#[derive(Default)]
struct GlobalVars {
    player_name: String,
    points: i32,
}

#[derive(Default)]
struct Choice {
    next_scene: Option<Box<Scene>>,
    display_name: String,
    text: Option<String>,
    changes: Option<fn() -> ()>,
}

#[derive(Default)]
struct Scene {
    id: String,
    choices: Option<Vec<Choice>>,
    text: String,
    next_scene: Option<Box<Scene>>,
}
