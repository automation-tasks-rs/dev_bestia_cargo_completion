// region: auto_md_to_doc_comments include README.md A //!
//! # dev_bestia_cargo_completion  
//!
//! **dev_bestia_cargo_completion - automation tasks written in Rust language for the build process of rust projects**  
//! ***[repository](https://github.com/LucianoBestia/dev_bestia_cargo_completion); version: 2021.816.1738  date: 2021-08-16 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-424-green.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-267-blue.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-47-purple.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
//!
//! [![crates.io](https://img.shields.io/crates/v/dev_bestia_cargo_completion.svg)](https://crates.io/crates/dev_bestia_cargo_completion) [![Documentation](https://docs.rs/dev_bestia_cargo_completion/badge.svg)](https://docs.rs/dev_bestia_cargo_completion/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_cargo_completion.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_cargo_completion/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_cargo_completion/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/dev_bestia_cargo_completion/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)  
//!
//! ## bash auto completion
//!
//! Auto-completion in Debian bash is a great tool. You type the first letters, press tab and the command is auto-completed.  
//! Bash can call an executable binary to return the available words. So it can be written in Rust. This can produce even better results as `dynamic auto-completion`.  
//! For my knowledge `cargo` does not have auto-completion yet. There are many plans. It can take some time.  
//! I will build what I need now. Something simple. It doesn't need to be perfect.  
//! This is a great bog:  
//! <https://www.joshmcguigan.com/blog/shell-completions-pure-rust/>
//!
//! This command will save the auto-completion definition into bash:
//!
//! ```bash
//! complete -C "/home/luciano/rustprojects/dev_bestia_cargo_completion/target/debug/dev_bestia_cargo_completion" cargo
//! ```
//!
//! Then try it, type `cargo` and mandatory `space` and then press `tab tab`.  
//! Something should happen.  
//!
//! ## complete
//!
//! The Linux command complete adds auto_completion definitions to bash.  
//!
//! ```bash
//! # to list the definitions
//! complete
//! # to delete a definition
//! complete -r cargo
//! # define binary to auto-complete the command
//! complete -C "binary" command
//! ```
//!
//! ## development
//!
//! Install `cargo install cargo-auto` and then list the user-defined automation tasks with `cargo auto`.  
//!
//! ## cargo crev reviews and advisory
//!
//! We leave in times of danger with `supply chain attacks`.  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web. Example for the crate `num-traits`:  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
//!
//! ## open-source free and free as a beer
//!
//! My open-source projects are free and free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !
//!
// endregion: auto_md_to_doc_comments include README.md A //!

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
        let sub_commands_after_cargo = vec!["auto", "build", "check", "new", "doc", "test", "fmt"];
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
    }
    // the first word after `cargo auto`
    else if vec_comp_line.len() <= 3 && last_word == "auto" && comp_line.starts_with("cargo auto")
    {
        let path_to_automation = "automation_tasks_rs/target/debug/automation_tasks_rs";
        if Path::new(path_to_automation).exists() {
            std::process::Command::new(path_to_automation)
                .arg("completion")
                .arg(word_being_completed)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        } else {
            println!("new");
            println!("new_with_lib");
        }
    }
}
