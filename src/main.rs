// dev_bestia_cargo_completion/src/main.rs

// logo for docs.rs in png
#![doc(html_logo_url = "https://github.com/automation-tasks-rs/cargo-auto/raw/main/images/logo/logo_cargo_auto.svg")]
// region: auto_md_to_doc_comments include README.md A //!
//! # dev_bestia_cargo_completion  
//!
//! **Auto-completion for cargo-auto and automation_tasks_rs and partial auto-completion for cargo in bash**  
//! ***version: 2024.306.2155 date: 2024-03-06 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![cargo-auto](https://img.shields.io/badge/cargo_auto-orange)
//!
//!  ![logo](https://raw.githubusercontent.com/automation-tasks-rs/cargo-auto/main/images/logo/logo_cargo_auto.svg)
//!  dev_bestia_cargo_completion is part of [automation_tasks_rs](https://github.com/automation-tasks-rs) project
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-55-green.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-0-blue.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-23-purple.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/)
//!
//!  [![crates.io](https://img.shields.io/crates/v/dev_bestia_cargo_completion.svg)](https://crates.io/crates/dev_bestia_cargo_completion)
//!  [![Documentation](https://docs.rs/dev_bestia_cargo_completion/badge.svg)](https://docs.rs/dev_bestia_cargo_completion/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_cargo_completion.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_cargo_completion/)
//!  [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_cargo_completion/)
//!  [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/blob/master/LICENSE)
//!  [![Rust](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/workflows/RustAction/badge.svg)](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/)
//!  ![dev_bestia_cargo_completion](https://bestia.dev/webpage_hit_counter/get_svg_image/710310517.svg)
//!
//! Hashtags: #rustlang #buildtool #developmenttool  
//! My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/automation-tasks-rs/tutorials_rust_wasm).
//!
//! ## Try it
//!
//! Install the binary:
//!
//! ```bash
//! cargo install dev_bestia_cargo_completion
//! ```
//!
//! Save definition for auto_completion in bash:
//!
//! ```bash
//! complete -C "dev_bestia_cargo_completion" cargo
//! ```
//!
//! Start typing `cargo b` and press `tab`.  
//! It should auto-complete to `cargo build`.  
//! Congratulation! You just used auto-completion :-)  
//!
//! ## bash auto completion
//!
//! Auto-completion in Debian bash is a great tool. You type the first letters, press tab and the word is auto-completed.  
//! Bash can call an executable binary to return the available words. So it can be written in Rust. This can produce even better results as known as `dynamic auto-completion`.  
//! For my knowledge `cargo` does not have auto-completion yet. There are many plans. It can take some time.  
//! I will build what I need now. Something simple. It doesn't need to be perfect.  
//! This is a great blog:  
//! <https://www.joshmcguigan.com/blog/shell-completions-pure-rust/>
//!
//! ## complete, the Linux command
//!
//! The Linux command `complete` adds auto_completion definitions to bash.  
//! But it is only for the current session. If you want to make it persistent add it to you `~/.bashrc` file. Or to some other file that runs commands on initialization of the bash.  
//!
//! ```bash
//! # list the definitions
//! complete
//! # delete a definition
//! complete -r cargo
//! # define a binary to auto-complete the command
//! complete -C "binary" command
//! # for example
//! complete -C "dev_bestia_cargo_completion" cargo
//! ```
//!
//! ## Development details
//!
//! Read the development details in a separate md file:  
//! [DEVELOPMENT.md](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/blob/main/DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the changelog in a separate md file:  
//! [RELEASES.md](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion/blob/main/RELEASES.md)
//!
//! ## TODO
//!
//! Nothing big in the near future.
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// region: use statements
// use lazy_static::lazy_static;
use std::{env, path::Path, vec};
// endregion

fn main() {
    // region: Environment Variables
    // COMP_LINE - the full text that the user has entered
    // COMP_POINT - the cursor position (a numeric index into COMP_LINE)¸
    let comp_line = env::var("COMP_LINE").unwrap();
    let _comp_point = env::var("COMP_POINT").unwrap().parse::<usize>().unwrap();
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
        let sub_commands_after_cargo = vec!["auto", "build", "check", "new", "doc", "test", "fmt", "install"];
        for sub_command in sub_commands_after_cargo {
            // list all for `tab tab` or list only one starting with the word
            if vec_comp_line.len() == 1 || sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
            }
        }
    }
    // the first word after `cargo build`
    else if vec_comp_line.len() <= 3 && last_word == "build" && comp_line.starts_with("cargo build") {
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
