#![doc=include_str!("../README.md")]

// region: use statements
// use lazy_static::lazy_static;
use std::{env, path::Path, vec};
use unwrap::unwrap;
// endregion

fn main() {
    // region: Environment Variables
    // COMP_LINE - the full text that the user has entered
    // COMP_POINT - the cursor position (a numeric index into COMP_LINE)¸
    let comp_line = unwrap!(env::var("COMP_LINE"));
    let _comp_point = unwrap!(unwrap!(env::var("COMP_POINT")).parse::<usize>());
    let vec_comp_line: Vec<&str> = comp_line.split_whitespace().collect();
    // endregion: Environment Variable

    // region: CLI arguments
    let args: Vec<String> = env::args().collect();
    // args[2] - the word we are completing
    let word_being_completed = &args[2];
    // args[3] - the last word before that
    let last_word = &args[3];
    // endregion: CLI arguments

    // first word after `cargo`
    if vec_comp_line.len() <= 2 && last_word == "cargo" {
        let sub_commands_after_cargo = vec![
            "auto", "build", "check", "new", "doc", "test", "fmt", "install",
        ];
        for sub_command in sub_commands_after_cargo {
            // list all for `tab tab` or list only one starting with the word
            if vec_comp_line.len() == 1 || sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
            }
        }
    }
    // the first word after `cargo build`
    else if vec_comp_line.len() <= 3
        && last_word == "build"
        && comp_line.starts_with("cargo build")
    {
        let sub_commands = vec!["--release"];

        for sub_command in sub_commands {
            // list all for `tab tab` or list only one starting with the word
            if vec_comp_line.len() == 2 || sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
            }
        }
    } else if comp_line.starts_with("cargo auto") {
        // words after `cargo auto` execute the appropriate binary, that responds with println
        // 1st argument in "completion"
        // 2rd argument is the `word_being_completed`
        // 3nd argument is the `last_word`
        let path_to_automation = "automation_tasks_rs/target/debug/automation_tasks_rs";
        if Path::new(path_to_automation).exists() {
            std::process::Command::new(path_to_automation)
                .arg("completion")
                .arg(word_being_completed)
                .arg(last_word)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        } else {
            std::process::Command::new("cargo-auto")
                .arg("completion")
                .arg(word_being_completed)
                .arg(last_word)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }
}
