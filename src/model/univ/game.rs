use crate::model::univ::gam::board::Board;
use crate::model::univ::gam::position::Position;
use crate::model::universe::PositionHashSeed;
use crate::model::vo::game_part::gp_phase_vo::PHASE_LN;
use crate::model::vo::game_part::gp_piece_vo::MG_MAX;
use crate::model::vo::game_part::gp_piece_vo::PIECE_LN;
use crate::model::vo::game_part::gp_square_vo::BOARD_MEMORY_AREA;

pub struct Game {
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
    /// 初期局面
    pub starting_board: Board,
    /// 現局面ハッシュ種☆（＾～＾）
    pub position_hash_seed: PositionHashSeed,
    /// 探索部
    pub position: Position,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            starting_position_hash: 0,
            starting_board: Board::default(),
            position_hash_seed: PositionHashSeed {
                // 盤上の駒
                km: [[0; PIECE_LN]; BOARD_MEMORY_AREA],
                // 持ち駒
                mg: [[0; MG_MAX]; PIECE_LN],
                // 先後
                phase: [0; PHASE_LN],
            },
            position: Position::default(),
        }
    }
}
/*
   pub fn get_starting_position_hash(&self) -> &u64 {
       &self.starting_position_hash
   }
   pub fn get_starting_position_hash_mut(&mut self) -> &mut u64 {
       &mut self.starting_position_hash
   }
   pub fn set_starting_position_hash(&mut self, val: u64) {
       self.starting_position_hash = val;
   }
    pub fn get_starting_board(&self) -> &Board {
        &self.starting_board
    }
    pub fn get_starting_board_mut(&mut self) -> &mut Board {
        &mut self.starting_board
    }
    pub fn get_position_hash_seed(&self) -> &PositionHashSeed {
        &self.position_hash_seed
    }
    pub fn get_position_hash_seed_mut(&mut self) -> &mut PositionHashSeed {
        &mut self.position_hash_seed
    }
*/
