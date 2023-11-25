use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;

use chrono::Local;
use random_word::Lang;
use rustyline::DefaultEditor;

use crate::game::Instance;
use crate::piece::Side;
use crate::position::{decode_position, Normalizable};
use crate::{board::Board, piece::Piece};

pub fn read_state_file(ctx: &mut Instance, name: String) {
    let raw = fs::read_to_string(name).expect("Couldn't read state file.");
    let binding = raw.clone();
    let parts: Vec<&str> = binding.split("\n").collect();
    ctx.states = raw.into();

    let head_parts = parts[0].split_whitespace().collect::<Vec<&str>>();
    ctx.board = Board::decode(head_parts[0].to_string());
    ctx.side = Side::from_str(head_parts[1]).unwrap();

    for m in parts[1].split(",") {
        if m.trim().len() == 0 {
            continue;
        }
        let move_parts: Vec<&str> = m.split(" ").collect();
        let (p, _) = Piece::decode(move_parts[0].to_string());
        let pos = decode_position(move_parts[1].to_string());
        ctx.board.make_move(p, pos)
    }

    println!("From: {}", ctx.board.encode());
}
pub fn cmd(ctx: &mut Instance, s: &str, prot: bool) {
    let s = s.trim().split_whitespace().collect::<Vec<&str>>();
    match s[0] {
        "l" | "load" => {
            if s.len() > 1 {
                ctx.board = Board::decode(s[1].to_string());
                println!("loaded: {}", ctx.board.encode());
            } else {
                ctx.board = Board::new()
            }
            Arc::get_mut(&mut ctx.states)
                .unwrap()
                .push_str(format!("{} {}\n", ctx.board.encode(), ctx.side).as_str());
        }
        "lf" | "load-file" => {
            if s.len() > 1 {
                read_state_file(ctx, s[1].to_string());
            } else {
                if !prot {
                    println!("load-file <filename>")
                }
            }
        }
        "b" | "board" => ctx.board.print_board(),
        "fen" => println!("fen: {}", ctx.board.encode()),
        "g" | "generate" => {
            if s.len() < 2 {
                if !prot {
                    println!("generate <piece id>");
                }
                return;
            }
            let (p, _) = Piece::decode(s[1].to_string());
            let start_pos = ctx.board.pos_from_piece(p);
            if !prot {
                println!("moves:");
            }
            for one in ctx.board.move_mask_raw(p).num.iter_ones() {
                let pos = Board::normal_to_pos(one as u64).encode();
                println!("{}{}-{}{}", s[1], start_pos.encode(), s[1], pos);
            }
        }
        "m" | "move" => {
            if s.len() < 3 {
                if !prot {
                    println!("move <piece id> <position id>");
                }
                return;
            }
            let (p, _) = Piece::decode(s[1].to_string());
            let pos = decode_position(s[2].to_string());
            ctx.board.make_move(p, pos);
            Arc::get_mut(&mut ctx.states)
                .unwrap()
                .push_str(format!("{} {},", s[1], s[2]).as_str());
        }
        "w" | "who" => {
            if !prot {
                println!("Has move: {}", ctx.side)
            } else {
                println!("{}", ctx.side)
            }
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
            if !prot {
                println!("saved: {}", name.join("_"));
            } else {
                println!("{}", name.join("_"));
            }
        }
        "st" | "state" => {
            if !prot {
                println!("Passive defeat:\n{:?}", ctx.has_passiveless());
                println!();
                println!(
                    "Edge alignment ({}):\ngoat side: {:?}\nsloth side: {:?}",
                    ctx.side,
                    ctx.has_alignment().0,
                    ctx.has_alignment().1
                );
                println!();
                println!("win ({}): {}", ctx.side, ctx.has_win())
            } else {
                println!(
                    "Passive_defeat:\n  white: {:?}\n  orange: {:?}",
                    ctx.has_passiveless()[&Side::White],
                    ctx.has_passiveless()[&Side::Orange]
                );
                println!();
                println!(
                    "edge_alignment:\n  goat_side: {:?}\n  sloth_side: {:?}",
                    ctx.has_alignment().0,
                    ctx.has_alignment().1
                );
                println!();
                println!("win: {}", ctx.has_win())
            }
        }
        "ping" => println!("ok"),

        _ => return,
    }
}
pub fn blank_instance() -> Instance {
    Instance {
        board: Board::blank(),
        side: Side::White,
        states: Arc::new(String::from("")),
    }
}
pub fn run() {
    let conf: &mut Instance = &mut blank_instance();
    let mut e = DefaultEditor::new().expect("Could not open repl.");
    e.load_history("history.txt").err();
    loop {
        let res = e.readline("(gtc)");
        match res.as_ref().unwrap().trim() {
            "quit" => break,
            _ => cmd(conf, &res.as_ref().unwrap(), false),
        }
        e.add_history_entry(res.unwrap().as_str())
            .expect("Bad history");
    }
    e.save_history("history.txt").expect("bad save");
}
