use bevy::prelude::{Commands, Event, ResMut, Resource};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

#[derive(Event, Debug)]
pub struct CmdRunEvent {
    pub cmd: String,
    pub out: String,
}

// pub fn cmdRun() {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameSide {
    pub white: bool,
    pub orange: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeSide {
    pub goat_side: bool,
    pub sloth_side: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub passive_defeat: GameSide,
    pub edge_alignment: EdgeSide,
    pub stalemate: bool,
    pub win: bool,
}

#[derive(Debug, Resource)]
pub struct Instance {
    reader: ChildStdout,
    input: ChildStdin,
}

impl Instance {
    pub fn gtc_startup(mut commands: Commands) {
        let cmd = match Command::new("gtc")
            .arg("--mode")
            .arg("protocol")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => commands.insert_resource(Instance {
                reader: child.stdout.take().unwrap(),
                input: child.stdin.take().unwrap(),
            }),
            Err(err) => {
                panic!("{}", err)
            }
        };
    }

    pub fn cmd(&mut self, cmd: &str) -> String {
        let _ = self
            .input
            .write((cmd.to_owned() + &"\n".to_owned()).as_bytes());
        let _ = self.input.write(b"ping\n");
        let mut buf = String::new();
        for line in BufReader::new(self.reader.by_ref()).lines() {
            if line.as_ref().unwrap() == "ok" {
                break;
            }
            if line.as_ref().unwrap() == "gtc 0.1.0" {
                continue;
            }
            if line.as_ref().unwrap().trim() == "" {
                continue;
            }
            buf.push_str(line.as_ref().unwrap().as_str());
            buf.push_str("\n");
        }

        return buf.trim().to_string();
    }

    pub fn state(&mut self) -> GameState {
        let state = self.cmd("st");

        serde_yaml::from_str::<GameState>(&state).unwrap()
    }
}
