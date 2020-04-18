use crate::controller::io::*;
use crate::controller::main_loop::ml_usi_controller::*;
use crate::model::univ::game::*;
use crate::model::univ::speed_of_light::SpeedOfLight;
use crate::model::universe::Universe;
use crate::view::game_view::*;

pub struct Commands {}
impl Commands {
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
