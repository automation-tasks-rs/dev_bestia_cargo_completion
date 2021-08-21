[comment]: # (auto_md_to_doc_comments segment start A)

# dev_bestia_cargo_completion  

[comment]: # (auto_cargo_toml_to_md start)

**Full auto-completion for cargo-auto and automation_tasks_rs and partial auto-completion for cargo  in bash**  
***[repository](https://github.com/LucianoBestia/dev_bestia_cargo_completion); version: 2021.819.909  date: 2021-08-19 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-55-green.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-93-blue.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-21-purple.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)

[comment]: # (auto_lines_of_code end)

[![crates.io](https://img.shields.io/crates/v/dev_bestia_cargo_completion.svg)](https://crates.io/crates/dev_bestia_cargo_completion) [![Documentation](https://docs.rs/dev_bestia_cargo_completion/badge.svg)](https://docs.rs/dev_bestia_cargo_completion/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_cargo_completion.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_cargo_completion/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_cargo_completion/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/dev_bestia_cargo_completion/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/dev_bestia_cargo_completion/)  

## Try it

Install the binary:

```bash
cargo install dev_bestia_cargo_completion
```

Save definition for auto_completion in bash:

```bash
complete -C "dev_bestia_cargo_completion" cargo
```

Start typing `cargo b` and press `tab`.  
It should auto-complete to `cargo build`.  
Congratulation! You just used auto-completion :-)  

## bash auto completion

Auto-completion in Debian bash is a great tool. You type the first letters, press tab and the word is auto-completed.  
Bash can call an executable binary to return the available words. So it can be written in Rust. This can produce even better results as known as `dynamic auto-completion`.  
For my knowledge `cargo` does not have auto-completion yet. There are many plans. It can take some time.  
I will build what I need now. Something simple. It doesn't need to be perfect.  
This is a great blog:  
<https://www.joshmcguigan.com/blog/shell-completions-pure-rust/>

## complete, the Linux command

The Linux command `complete` adds auto_completion definitions to bash.  
But it is only for the current session. If you want to make it persistent add it to you `~/.bashrc` file. Or to some other file that runs commands on initialization of the bash.  

```bash
# list the definitions
complete
# delete a definition
complete -r cargo
# define a binary to auto-complete the command
complete -C "binary" command
# for example
complete -C "dev_bestia_cargo_completion" cargo
```

## development

I choose this long name for my crate, because cargo-completion is a hot topic. Sooner or later someone will build it - officially. This is why I added the prefix `dev_bestia_`, because my web domain is [bestia.dev](https://bestia.dev). This way I have the guarantee of a unique name and leave the short name to the future official version.  
Run

```bash
cargo install cargo-auto
```

and then list the user-defined automation tasks with

```bash
cargo auto
```  

You can change the definition for bash auto-completion to point to the compilation of this project in development. So you can easy test the auto-completion while developing it.

```bash
complete -C "/home/luciano/rustprojects/dev_bestia_cargo_completion/target/debug/dev_bestia_cargo_completion" cargo
```


## cargo crev reviews and advisory

We leave in times of danger with `supply chain attacks`.  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !

[comment]: # (auto_md_to_doc_comments segment end A)
