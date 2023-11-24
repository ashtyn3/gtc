use rustyline::{Config, DefaultEditor, Editor};

use crate::{
    game::Game,
    piece::Piece,
    position::{decode_position, Normalizable, Position},
};

struct Instance {
    game: Game,
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
        }
        _ => return,
    }
}
pub fn run() {
    let conf: &mut Instance = &mut Instance {
        game: Game::blank(),
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
