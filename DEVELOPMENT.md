# Development details

## CRDE - Containerized Rust Development Environment

I recommend using the CRDE - Containerized Rust Development Environment to write Rust projects.  
Follow the instructions here  
<https://github.com/automation-tasks-rs/docker_rust_development>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the Docker newer alternative: [Podman](https://podman.io/).  
Then you download the prepared container image from DockerHub (3GB).  
And then a little juggling with ssh keys.  
All this is simplified by running a few bash scripts.  
Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs

Automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and push
cargo auto publish_to_crates_io
cargo auto github_new_release
```

Every task finishes with instructions how to proceed.  
The [cargo-auto](https://github.com/automation-tasks-rs/cargo-auto) and [dev_bestia_cargo_completion](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion) are already installed inside the CRDE container.

You can open the automation sub-project in VSCode and then code your own tasks in Rust.

```bash
code automation_tasks_rs
```

## development of dev_bestia_cargo_completion

I choose this long name for my crate, because cargo-completion is a hot topic. Sooner or later someone will build it - officially. This is why I added the prefix `dev_bestia_`, because my web domain is [bestia.dev](https://bestia.dev). This way I have the guarantee of a unique name and leave the short name to the future official version.  

You can change the definition for bash auto-completion to point to the compilation of this project in development. So you can easy test the auto-completion while developing it.

```bash
complete -C "/home/luciano/rustprojects/dev_bestia_cargo_completion/target/debug/dev_bestia_cargo_completion" cargo
```

## Confusion with names in the Rust project

The Rust project is a `[package]`. That is called also a "crate". It can contain one `[lib]` and many `[[bin]]` segments.  
"Lib" stands for library and this is a word anybody can understand.  
"Bin" stands for binary and is very confusing. In this narrow context it is an "executable binary" or "executable" or "runnable program".  
It is confusing because we have many many binary formats today: jpg, mp4,... The term "binary file" is often used as a term meaning "non-text file".
Let's call our bin just bin for clarity.  

On crates.io we can publish "packages" or other name "crates". They can contain a "lib" or "bin" or combinations of these two. But crates.io does not recognize "bin" correctly. It treats them like libraries. It shows the suggestion to add it to your dependencies instead of showing the `cargo install` command. Weird.

There are many many different ways to structure a project in Rust. This is pretty confusing, but there is no medicine.  
I put a `lib.rs` inside the `/src/` folder. Then I make a sub-folder `/src/bin/name-of-bin` and put `main.rs` inside.
In Cargo.toml I define a name of the package/crate in `[package]-name`, the name of the `lib` in `[lib]-name` and the name of the `bin` in `[[bin]]-name`.
Confusing? Yes. But it works somehow until we want to have the documentation on `docs.rs`.

Docs.rs is happy to create documentation for libraries, but not for binaries. But the local command `cargo doc` is happy with both of them. Weird.
