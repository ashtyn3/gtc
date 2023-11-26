#![feature(panic_info_message)]
pub mod bitboard;
pub mod board;
pub mod game;
pub mod piece;
pub mod position;
mod repl;

use core::panic;
use std::{
    cell::RefCell,
    env,
    io::{self, BufRead},
    panic::{catch_unwind, set_hook},
    pin::Pin,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

use clap::{arg, value_parser};
use piece::Side;

use crate::{
    game::Instance,
    repl::{blank_instance, run},
};

fn body(ctx: &mut Instance, new: bool) {
    let args: Vec<String> = env::args().collect();

    let mut cmd = clap::Command::new("gtc")
        .bin_name("gtc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ashtyn MB")
        .args([arg!(--mode <MODE> "sets mode of engine")
            .value_parser(["protocol", "engine"])
            .required(true)]);

    let matches = cmd.clone().get_matches();
    let mode = matches.get_one::<String>("mode").unwrap();

    match mode.to_string().as_str() {
        "protocol" => {
            if new {
                println!("gtc {}", env!("CARGO_PKG_VERSION"))
            };
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let data = line.unwrap().clone();
                if data == "quit" {
                    return;
                }
                println!();
                repl::cmd(ctx, data.as_str(), true);
            }
        }
        "engine" => {
            run();
        }
        _ => cmd.print_help().expect("bad"),
    };
}
fn main() {
    let mut new: bool = true;
    set_hook(Box::new(|info| {
        if info.message().is_some() {
            #[cfg(not(debug_assertions))]
            println!("{}", info.message().unwrap());

            #[cfg(debug_assertions)]
            println!("{}", info)
        } else {
            println!("{}", info);
        }
    }));
    thread::scope(|scope| loop {
        let mut inst: Instance = blank_instance();
        let status = scope.spawn(move || {
            body(&mut inst, new);
        });
        let res = status.join();
        if !res.is_err() || res.is_ok() {
            break;
        } else {
            new = false;
        }
    });
}
