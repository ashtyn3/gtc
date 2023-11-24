use std::fs::{self, File};
use std::io::Write;
use std::rc::Rc;

use chrono::{Local, Utc};
use random_word::Lang;
use rustyline::DefaultEditor;

use crate::position::{decode_position, Normalizable};
use crate::{game::Game, piece::Piece};

struct Instance {
    game: Game,
    states: Rc<String>,
}

pub fn read_state_file(ctx: &mut Instance, name: String) {
    let raw = fs::read_to_string(name).expect("Couldn't read state file.");
    let binding = raw.clone();
    let parts: Vec<&str> = binding.split("\n").collect();
    ctx.states = raw.into();

    ctx.game = Game::decode(parts[0].to_string());

    for m in parts[1].split(",") {
        if m.trim().len() == 0 {
            continue;
        }
        let move_parts: Vec<&str> = m.split(" ").collect();
        let (p, _) = Piece::decode(move_parts[0].to_string());
        let pos = decode_position(move_parts[1].to_string());
        ctx.game.make_move(p, pos)
    }

    println!("From: {}", ctx.game.encode());
}
pub fn cmd(ctx: &mut Instance, s: &str) {
    let s = s.trim().split_whitespace().collect::<Vec<&str>>();
    match s[0] {
        "l" | "load" => {
            if s.len() > 1 {
                ctx.game = Game::decode(s[1].to_string());
                println!("loaded: {}", ctx.game.encode());
            } else {
                ctx.game = Game::new()
            }
            Rc::get_mut(&mut ctx.states)
                .unwrap()
                .push_str(format!("{}\n", ctx.game.encode()).as_str());
        }
        "lf" | "load-file" => {
            if s.len() > 1 {
                read_state_file(ctx, s[1].to_string());
            } else {
                println!("load-file <filename>")
            }
        }
        "b" | "board" => ctx.game.print_board(),
        "fen" => println!("fen: {}", ctx.game.encode()),
        "g" | "generate" => {
            if s.len() < 2 {
                println!("generate <piece id>");
                return;
            }
            let (p, _) = Piece::decode(s[1].to_string());
            let start_pos = ctx.game.pos_from_piece(p);
            println!("moves:");
            for one in ctx.game.move_mask_raw(p).num.iter_ones() {
                let pos = Game::normal_to_pos(one as u64).encode();
                println!("{}{}-{}{}", s[1], start_pos.encode(), s[1], pos);
            }
        }
        "m" | "move" => {
            if s.len() < 3 {
                println!("move <piece id> <position id>");
                return;
            }
            let (p, _) = Piece::decode(s[1].to_string());
            let pos = decode_position(s[2].to_string());
            ctx.game.make_move(p, pos);
            Rc::get_mut(&mut ctx.states)
                .unwrap()
                .push_str(format!("{} {},", s[1], s[2]).as_str());
        }
        "s" | "save" => {
            let time = Local::now().timestamp().to_string();
            let name: [&str; 3] = [
                random_word::gen(Lang::En),
                random_word::gen(Lang::En),
                time.as_str(),
            ];

            let mut f = File::create(name.join("_")).expect("can't write file");
            f.write_all(ctx.states.as_bytes())
                .expect("Failed to write state.");
            println!("saved: {}", name.join("_"));
        }

        _ => return,
    }
}
pub fn run() {
    let conf: &mut Instance = &mut Instance {
        game: Game::blank(),
        states: Rc::new(String::from("")),
    };

    let mut e = DefaultEditor::new().expect("Could not open repl.");
    e.load_history("history.txt").err();
    loop {
        let res = e.readline("(gtc)");
        match res.as_ref().unwrap().trim() {
            "quit" => break,
            _ => cmd(conf, &res.as_ref().unwrap()),
        }
        e.add_history_entry(res.unwrap().as_str())
            .expect("Bad history");
    }
    e.save_history("history.txt").expect("bad save");
}
