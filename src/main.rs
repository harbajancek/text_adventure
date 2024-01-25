use std::cell::{RefCell, RefMut};
use std::io::{self};
use std::collections::HashMap;

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

    let mut intro_scene = Scene { id: String::from("intro"), text: String::from(title), ..Default::default()};

    let mut ending_scene = Scene {id: String::from("end"), text: String::from("Are you sure you want to exit the game?"), ..Default::default() };

    let intro_choices = vec![
        Choice {display_name: String::from("Load"), ..Default::default()},
        Choice {display_name: String::from("Start"), ..Default::default()},
        Choice {display_name: String::from("Options"), ..Default::default()},
        Choice {display_name: String::from("End"), next_scene_id: Some(ending_scene.id.to_string()), ..Default::default()},
    ];

    let ending_choices = vec![
        Choice {display_name: String::from("No"), next_scene_id: Some(intro_scene.id.to_string()), ..Default::default()},
        Choice {display_name: String::from("Yes"), ..Default::default()},
    ];

    intro_scene.choices = Some(intro_choices);
    ending_scene.choices = Some(ending_choices);

    let vars = GameVars {..Default::default()};
    let vars = RefCell::new(vars);

    let mut scene_manager = SceneProcessor::new(intro_scene, vars);
    scene_manager.add_scene(ending_scene);

    loop {
        if !scene_manager.process_scene() {
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

#[derive(Default)]
struct GameVars {
    player_name: String,
    points: i32,
}

#[derive(Default)]
struct Choice {
    next_scene_id: Option<String>,
    display_name: String,
    text: Option<String>,
    changes: Option<fn(RefMut<GameVars>) -> ()>,
}

#[derive(Default)]
struct Scene {
    id: String,
    text: String,
    choices: Option<Vec<Choice>>,
    next_scene_id: Option<String>,
}

struct SceneProcessor {
    scenes: HashMap<String, Scene>,
    current_scene_id: String,
    vars: RefCell<GameVars>,
}

impl SceneProcessor {
    fn process_scene(&mut self) -> bool {
        let current_scene = &self.scenes[&self.current_scene_id];
        let next_scene_id: &Option<String>;
        let mut choice: &Choice;

        println!("{}", current_scene.text);
        println!();

        if let Some(choices) = &current_scene.choices {

            for (i, choice) in choices.iter().enumerate() {
                println!("{} - {}", i+1, choice.display_name);
            }

            choice = &choices[get_choice_input(choices.len()) - 1];

            self.process_choice(&mut choice);

            next_scene_id = &choice.next_scene_id;

        } else {
            println!("1 - Continue");
            get_choice_input(1);
            next_scene_id = &current_scene.next_scene_id;
        }

        

        if let Some(next_scene_id) = next_scene_id {
            self.current_scene_id = next_scene_id.to_string();
            return true;
        }
        
        return false;
    }

    fn process_choice(&self, choice: &Choice) {
        if let Some(text) = &choice.text {
            println!("{text}");
        }

        let Some(changes) = choice.changes else { return; };

        changes(self.vars.borrow_mut());
    }

    fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.id.to_string(), scene);
    }

    fn add_intro_scene(&mut self, scene: Scene) {
        self.current_scene_id = scene.id.to_string();
        self.add_scene(scene);
    }

    fn new(init_scene: Scene, vars: RefCell<GameVars>) -> SceneProcessor {
        let mut scenes = HashMap::new();
        let scene_id = init_scene.id.to_string();
        scenes.insert(init_scene.id.to_string(), init_scene);

        SceneProcessor {scenes,
            current_scene_id: scene_id,
            vars}
    }

}