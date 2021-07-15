use crate::config::*;
use crate::entities::cosmic::universe::Universe;
use crate::entities::law::usi::*;
use crate::entities::spaceship::equipment::{Beam, Telescope};
use crate::position::to_move_code;
use crate::search::Tree;
use crate::view::print_info;
use std::io as std_io;

/// 乗組員：夢美
pub struct Yumemi {}
impl Yumemi {
    /// 望遠鏡を覗き込みましょう。
    pub fn look_into_the_telescope() {
        Telescope::look();
    }
}
