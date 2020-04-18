use crate::controller::io::*;
use crate::controller::main_loop::ml_usi_controller::*;
use crate::controller::movement_generation::movement_generator::*;
use crate::speed_of_light::*;
use crate::universe::game::game::{Game, PosNums};
use crate::universe::universe::Universe;
use crate::view::game_view::*;
use crate::view::unit_test::unit_test_view::print_movement_hashset;
use std::collections::HashSet;

pub struct Commands {}
impl Commands {
    pub fn genmove(speed_of_light: &SpeedOfLight, game: &Game) {
        // Generation move.
        // FIXME 合法手とは限らない
        let mut ss_potential_hashset = HashSet::<u64>::new();
        get_potential_movement(&game, &speed_of_light, &mut |movement_hash| {
            ss_potential_hashset.insert(movement_hash);
        });
        IO::writeln("----指し手生成(合法手とは限らない) ここから----");
        print_movement_hashset(&ss_potential_hashset);
        IO::writeln("----指し手生成(合法手とは限らない) ここまで----");
    }
    pub fn pos(game: &Game) {
        let s = GameView::to_string(&game, &PosNums::Current);
        IO::writeln(&s);
    }
    pub fn position(speed_of_light: &SpeedOfLight, universe: &mut Universe, line: &String) {
        // positionコマンドの読取を丸投げ
        read_position(&line, universe, &speed_of_light);
    }
    pub fn setoption_name(universe: &mut Universe, line: &String) {
        // Example: setoption name USI_Ponder value true
        if let Some(x) = line[15..].find(' ') {
            let name = &line[15..(x + 15)];
            // IO::writeln(&format!("Debug name=|{}|", name));
            let value = &line[(x + 22)..];
            // IO::writeln(&format!("Debug value=|{}|", value));
            if name == "MaxDepth" {
                universe.option_max_depth = value.parse().unwrap();
            }
        };
    }
}
