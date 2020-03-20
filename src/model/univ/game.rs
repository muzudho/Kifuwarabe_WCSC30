use crate::model::dto::search_part::sp_info::SPInfo;
use crate::model::univ::gam::board::Board;
use crate::model::univ::gam::phase::PHASE_LN;
use crate::model::univ::gam::phase::*;
use crate::model::univ::gam::piece::GPPieceVo;
use crate::model::univ::gam::piece::MG_MAX;
use crate::model::univ::gam::piece::PIECE_LN;
use crate::model::univ::gam::piece_struct::GPPieceStructVo;
use crate::model::univ::gam::position::Position;
use crate::model::univ::gam::square::BOARD_MEMORY_AREA;
use crate::model::univ::gam::square::SQUARE_NONE;
use crate::model::univ::gam::square::*;
use crate::model::universe::PositionHashSeed;
use crate::model::vo::main_loop::ml_speed_of_light_vo::MLSpeedOfLightVo;
use crate::model::vo::other_part::op_misc_vo::PosNums;
use crate::model::vo::other_part::op_person_vo::Person;
use rand::Rng;

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
impl Game {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_ms in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_km in 0..PIECE_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.position_hash_seed.km[i_ms][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..PIECE_LN {
            for i_mg in 0..MG_MAX {
                self.position_hash_seed.mg[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LN {
            self.position_hash_seed.phase[i_phase] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    pub fn get_board(&self, num: &PosNums) -> &Board {
        match *num {
            PosNums::Current => self.position.get_current_board(),
            PosNums::Start => &self.starting_board,
        }
    }

    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear_all_positions(&mut self) {
        self.starting_board.clear();
        self.position.get_current_board_mut().clear();
        self.position.set_ply(0);
    }

    /// 開始局面を、現局面にコピーします
    pub fn copy_starting_position_to_current_position(&mut self) {
        // 盤上の駒。
        for i_ms in 0..BOARD_MEMORY_AREA {
            let i_sq = Square::from_usquare(i_ms);
            // TODO 取得→設定　するとエラーになってしまうので、今んとこ 作成→設定　するぜ☆（＾～＾）
            let piece = self.starting_board.get_piece_by_square(&i_sq);
            self.position
                .get_current_board_mut()
                .set_piece_by_square(&i_sq, piece);
        }

        // 持ち駒
        self.position.get_current_board_mut().hand[..PIECE_LN]
            .clone_from_slice(&self.starting_board.hand[..PIECE_LN]);
        /*
        for i_mg in 0..PIECE_LN {
            self.get_search_part_mut().get_current_position_mut().mg[i_mg] =
                self.get_starting_position().mg[i_mg];
        }
        */
    }

    /// 初期局面の盤上に駒の位置を設定するもの
    pub fn set_piece_to_starting_position(&mut self, suji: i8, dan: i8, piece: GPPieceVo) {
        self.starting_board
            .set_piece_by_square(&Square::from_file_rank(suji, dan), &piece);
    }

    pub fn set_starting_position_hand_piece(&mut self, km: GPPieceVo, maisu: i8) {
        self.starting_board.hand[km as usize] = maisu;
    }

    pub fn get_person_by_piece_vo(&self, piece_vo: &GPPieceStructVo) -> Person {
        if &piece_vo.phase() == &self.position.get_phase(&Person::Friend) {
            Person::Friend
        } else {
            Person::Opponent
        }
    }

    /// 局面ハッシュ。
    pub fn get_all_position_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.starting_position_hash));

        for ply in 0..self.position.get_ply() {
            let hash = &self.position.get_position_hash_history()[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /// 自陣
    #[allow(dead_code)]
    pub fn get_ji_jin(&self) -> Vec<Square> {
        if let Phase::First = self.position.get_phase(&Person::Friend) {
            crate::model::vo::other_part::op_region_vo::SenteJin::to_elm()
        } else {
            crate::model::vo::other_part::op_region_vo::GoteJin::to_elm()
        }
    }

    /// 相手陣
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::First = self.position.get_phase(&Person::Friend) {
            crate::model::vo::other_part::op_region_vo::GoteJin::to_elm()
        } else {
            crate::model::vo::other_part::op_region_vo::SenteJin::to_elm()
        }
    }

    pub fn get_mut_info(&mut self) -> &mut SPInfo {
        &mut self.position.info
    }

    /// 初期局面ハッシュを作り直す
    pub fn create_starting_position_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self.starting_board.create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.position_hash_seed.phase[PHASE_SECOND];

        hash
    }

    /// 局面ハッシュを作り直す
    pub fn create_ky1_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self
            .position
            .get_current_board()
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use crate::model::univ::gam::phase::Phase::*;
        match self.position.get_phase(&Person::Friend) {
            First => hash ^= self.position_hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.position_hash_seed.phase[PHASE_SECOND],
            _ => {}
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_ky(&self) -> i8 {
        if self.position.get_ply() < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.position.get_ply() - 1;
        let new_ply = self.position.get_ply();
        // g_writeln( &format!( "Ｃount_same_ky last_ply={} new_ply={}", last_ply ,new_ply ) );
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            // g_writeln( &format!( "i_ply={} t={}", i_ply, t ) );
            if self.position.get_position_hash_history()[t as usize]
                == self.position.get_position_hash_history()[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.starting_position_hash
            == self.position.get_position_hash_history()[last_ply as usize]
        {
            count += 1;
        }

        count
    }

    /// 表示
    pub fn print_number_board(
        &self,
        phase: &Phase,
        pc: &GPPieceVo,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> String {
        let nb = match *phase {
            Phase::None => {
                &self.position.control_count_by_piece
                    [speed_of_light.get_piece_struct_vo(pc).serial_piece_number()]
            }
            _ => &self.position.control_count_by_phase[phase_to_num(&phase)],
        };

        // 数盤表示
        format!(
            "    +----+----+----+----+----+----+----+----+----+
i9  |{0:4}|{1:4}|{2:4}|{3:4}|{4:4}|{5:4}|{6:4}|{7:4}|{8:4}|
    +----+----+----+----+----+----+----+----+----+
h8  |{9:4}|{10:4}|{11:4}|{12:4}|{13:4}|{14:4}|{15:4}|{16:4}|{17:4}|
    +----+----+----+----+----+----+----+----+----+
g7  |{18:4}|{19:4}|{20:4}|{21:4}|{22:4}|{23:4}|{24:4}|{25:4}|{26:4}|
    +----+----+----+----+----+----+----+----+----+
f6  |{27:4}|{28:4}|{29:4}|{30:4}|{31:4}|{32:4}|{33:4}|{34:4}|{35:4}|
    +----+----+----+----+----+----+----+----+----+
e5  |{36:4}|{37:4}|{38:4}|{39:4}|{40:4}|{41:4}|{42:4}|{43:4}|{44:4}|
    +----+----+----+----+----+----+----+----+----+
d4  |{45:4}|{46:4}|{47:4}|{48:4}|{49:4}|{50:4}|{51:4}|{52:4}|{53:4}|
    +----+----+----+----+----+----+----+----+----+
c3  |{54:4}|{55:4}|{56:4}|{57:4}|{58:4}|{59:4}|{60:4}|{61:4}|{62:4}|
    +----+----+----+----+----+----+----+----+----+
b2  |{63:4}|{64:4}|{65:4}|{66:4}|{67:4}|{68:4}|{69:4}|{70:4}|{71:4}|
    +----+----+----+----+----+----+----+----+----+
a1  |{72:4}|{73:4}|{74:4}|{75:4}|{76:4}|{77:4}|{78:4}|{79:4}|{80:4}|
    +----+----+----+----+----+----+----+----+----+
       1    2    3    4    5    6    7    8    9\
",
            nb.get_number_by_square(&Square::from_usquare(19)),
            nb.get_number_by_square(&Square::from_usquare(29)),
            nb.get_number_by_square(&Square::from_usquare(39)),
            nb.get_number_by_square(&Square::from_usquare(49)),
            nb.get_number_by_square(&Square::from_usquare(59)),
            nb.get_number_by_square(&Square::from_usquare(69)),
            nb.get_number_by_square(&Square::from_usquare(79)),
            nb.get_number_by_square(&Square::from_usquare(89)),
            nb.get_number_by_square(&Square::from_usquare(99)),
            nb.get_number_by_square(&Square::from_usquare(18)),
            nb.get_number_by_square(&Square::from_usquare(28)),
            nb.get_number_by_square(&Square::from_usquare(38)),
            nb.get_number_by_square(&Square::from_usquare(48)),
            nb.get_number_by_square(&Square::from_usquare(58)),
            nb.get_number_by_square(&Square::from_usquare(68)),
            nb.get_number_by_square(&Square::from_usquare(78)),
            nb.get_number_by_square(&Square::from_usquare(88)),
            nb.get_number_by_square(&Square::from_usquare(98)),
            nb.get_number_by_square(&Square::from_usquare(17)),
            nb.get_number_by_square(&Square::from_usquare(27)),
            nb.get_number_by_square(&Square::from_usquare(37)),
            nb.get_number_by_square(&Square::from_usquare(47)),
            nb.get_number_by_square(&Square::from_usquare(57)),
            nb.get_number_by_square(&Square::from_usquare(67)),
            nb.get_number_by_square(&Square::from_usquare(77)),
            nb.get_number_by_square(&Square::from_usquare(87)),
            nb.get_number_by_square(&Square::from_usquare(97)),
            nb.get_number_by_square(&Square::from_usquare(16)),
            nb.get_number_by_square(&Square::from_usquare(26)),
            nb.get_number_by_square(&Square::from_usquare(36)),
            nb.get_number_by_square(&Square::from_usquare(46)),
            nb.get_number_by_square(&Square::from_usquare(56)),
            nb.get_number_by_square(&Square::from_usquare(66)),
            nb.get_number_by_square(&Square::from_usquare(76)),
            nb.get_number_by_square(&Square::from_usquare(86)),
            nb.get_number_by_square(&Square::from_usquare(96)),
            nb.get_number_by_square(&Square::from_usquare(15)),
            nb.get_number_by_square(&Square::from_usquare(25)),
            nb.get_number_by_square(&Square::from_usquare(35)),
            nb.get_number_by_square(&Square::from_usquare(45)),
            nb.get_number_by_square(&Square::from_usquare(55)),
            nb.get_number_by_square(&Square::from_usquare(65)),
            nb.get_number_by_square(&Square::from_usquare(75)),
            nb.get_number_by_square(&Square::from_usquare(85)),
            nb.get_number_by_square(&Square::from_usquare(95)),
            nb.get_number_by_square(&Square::from_usquare(14)),
            nb.get_number_by_square(&Square::from_usquare(24)),
            nb.get_number_by_square(&Square::from_usquare(34)),
            nb.get_number_by_square(&Square::from_usquare(44)),
            nb.get_number_by_square(&Square::from_usquare(54)),
            nb.get_number_by_square(&Square::from_usquare(64)),
            nb.get_number_by_square(&Square::from_usquare(74)),
            nb.get_number_by_square(&Square::from_usquare(84)),
            nb.get_number_by_square(&Square::from_usquare(94)),
            nb.get_number_by_square(&Square::from_usquare(13)),
            nb.get_number_by_square(&Square::from_usquare(23)),
            nb.get_number_by_square(&Square::from_usquare(33)),
            nb.get_number_by_square(&Square::from_usquare(43)),
            nb.get_number_by_square(&Square::from_usquare(53)),
            nb.get_number_by_square(&Square::from_usquare(63)),
            nb.get_number_by_square(&Square::from_usquare(73)),
            nb.get_number_by_square(&Square::from_usquare(83)),
            nb.get_number_by_square(&Square::from_usquare(93)),
            nb.get_number_by_square(&Square::from_usquare(12)),
            nb.get_number_by_square(&Square::from_usquare(22)),
            nb.get_number_by_square(&Square::from_usquare(32)),
            nb.get_number_by_square(&Square::from_usquare(42)),
            nb.get_number_by_square(&Square::from_usquare(52)),
            nb.get_number_by_square(&Square::from_usquare(62)),
            nb.get_number_by_square(&Square::from_usquare(72)),
            nb.get_number_by_square(&Square::from_usquare(82)),
            nb.get_number_by_square(&Square::from_usquare(92)),
            nb.get_number_by_square(&Square::from_usquare(11)),
            nb.get_number_by_square(&Square::from_usquare(21)),
            nb.get_number_by_square(&Square::from_usquare(31)),
            nb.get_number_by_square(&Square::from_usquare(41)),
            nb.get_number_by_square(&Square::from_usquare(51)),
            nb.get_number_by_square(&Square::from_usquare(61)),
            nb.get_number_by_square(&Square::from_usquare(71)),
            nb.get_number_by_square(&Square::from_usquare(81)),
            nb.get_number_by_square(&Square::from_usquare(91)),
        )
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

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.game.position
    }
    pub fn get_position(&self) -> &Position {
        &self.game.position
    }
*/
