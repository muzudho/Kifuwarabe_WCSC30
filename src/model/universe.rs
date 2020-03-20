//!
//! Data transfer object.
//!
extern crate rand;
use crate::model::dto::search_part::sp_info::SPInfo;
use rand::Rng;

use crate::config::*;
use crate::model::univ::gam::board::*;
use crate::model::univ::gam::position::*;
use crate::model::univ::game::*;
use crate::model::vo::game_part::gp_movement_vo::*;
use crate::model::vo::game_part::gp_phase_vo::*;
use crate::model::vo::game_part::gp_piece_struct_vo::GPPieceStructVo;
use crate::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use crate::model::vo::game_part::gp_piece_type_vo::*;
use crate::model::vo::game_part::gp_piece_vo::GPPieceVo;
use crate::model::vo::game_part::gp_piece_vo::*;
use crate::model::vo::game_part::gp_square_vo::*;
use crate::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::model::vo::other_part::op_misc_vo::*;
use crate::model::vo::other_part::op_person_vo::Person;
use crate::model::vo::other_part::op_piece_direction_vo::PieceDirection;
use crate::model::vo::other_part::op_piece_movement_vo::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

// グローバル定数
//
// 使い方（lazy_static!マクロ）
// ============================
// 定数の値を実行時に決めることができる。
//
// Cargo.toml に１行追記
// > [dependencies]
// > lazy_static = "1.0.0"
//
// main.rs の冒頭あたりに次の２行を記述
// > #[macro_use]
// > extern crate lazy_static;
//
// 「How can I use mutable lazy_static?」
// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
lazy_static! {
    /**
      * ログ・ファイル
      */
    pub static ref LOGFILE: Mutex<File> = {
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        Mutex::new(File::create(Path::new(LOG_FILE_PATH)).unwrap())
    };
}

#[allow(dead_code)]
pub fn g_log(s: &str) {
    if LOG_ENABLE {
        // write_allメソッドを使うには use std::io::Write; が必要
        if let Err(_why) = LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            //panic!("couldn't write log. : {}",Error::description(&why)),
        }
    }
}
#[allow(dead_code)]
pub fn g_write(s: &str) {
    println!("{}", s);
    g_log(s)
}
#[allow(dead_code)]
pub fn g_logln(s: &str) {
    if LOG_ENABLE {
        if let Err(_why) = LOGFILE
            .lock()
            .unwrap()
            .write_all(format!("{}\n", s).as_bytes())
        {}
    }
}
#[allow(dead_code)]
pub fn g_writeln(s: &str) {
    println!("{}", s);
    g_logln(s);
}

/// 局面ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct PositionHashSeed {
    // 盤上の駒
    pub km: [[u64; PIECE_LN]; BOARD_MEMORY_AREA],
    // 持ち駒
    pub mg: [[u64; MG_MAX]; PIECE_LN],
    // 先後
    pub phase: [u64; PHASE_LN],
}

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Universe {
    pub game: Game,
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            vec_command: Vec::new(),
        }
    }
}
impl Universe {
    /**
     * 宇宙誕生
     */
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_ms in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_km in 0..PIECE_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.game.position_hash_seed.km[i_ms][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..PIECE_LN {
            for i_mg in 0..MG_MAX {
                self.game.position_hash_seed.mg[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LN {
            self.game.position_hash_seed.phase[i_phase] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }
    pub fn get_board(&self, num: &PosNums) -> &Board {
        match *num {
            PosNums::Current => self.get_position().get_current_board(),
            PosNums::Start => &self.game.starting_board,
        }
    }
    /**
     * 初期局面、現局面ともにクリアーします。
     * 手目も 0 に戻します。
     */
    pub fn clear_all_positions(&mut self) {
        self.game.starting_board.clear();
        self.get_position_mut().get_current_board_mut().clear();
        self.get_position_mut().set_ply(0);
    }
    /// 開始局面を、現局面にコピーします
    pub fn copy_starting_position_to_current_position(&mut self) {
        // 盤上の駒。
        for i_ms in 0..BOARD_MEMORY_AREA {
            let i_sq = Square::from_usquare(i_ms);
            // TODO 取得→設定　するとエラーになってしまうので、今んとこ 作成→設定　するぜ☆（＾～＾）
            let piece = self.game.starting_board.get_piece_by_square(&i_sq);
            self.game
                .position
                .get_current_board_mut()
                .set_piece_by_square(&i_sq, piece);
        }

        // 持ち駒
        self.game.position.get_current_board_mut().hand[..PIECE_LN]
            .clone_from_slice(&self.game.starting_board.hand[..PIECE_LN]);
        /*
        for i_mg in 0..PIECE_LN {
            self.get_search_part_mut().get_current_position_mut().mg[i_mg] =
                self.get_starting_position().mg[i_mg];
        }
        */
    }

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.game.position
    }
    pub fn get_position(&self) -> &Position {
        &self.game.position
    }

    /* **********************
     * コマンド・バッファー *
     ************************/
    pub fn is_empty_command(&mut self) -> bool {
        self.vec_command.is_empty()
    }
    pub fn push_command(&mut self, line: &str) {
        self.vec_command.push(format!("{}\n", line));
    }
    pub fn pop_command(&mut self) -> String {
        self.vec_command.pop().unwrap()
    }

    /* ******
     * 盤上 *
     ********/

    /// 初期局面の盤上に駒の位置を設定するもの
    pub fn set_piece_to_starting_position(&mut self, suji: i8, dan: i8, piece: GPPieceVo) {
        self.game
            .starting_board
            .set_piece_by_square(&Square::from_file_rank(suji, dan), &piece);
    }
    pub fn set_starting_position_hand_piece(&mut self, km: GPPieceVo, maisu: i8) {
        self.game.starting_board.hand[km as usize] = maisu;
    }
    pub fn get_person_by_piece_vo(&self, piece_vo: &GPPieceStructVo) -> Person {
        if &piece_vo.phase() == &self.game.position.get_phase(&Person::Friend) {
            Person::Friend
        } else {
            Person::Opponent
        }
    }

    /// 局面ハッシュ。
    pub fn get_all_position_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.game.starting_position_hash));

        for ply in 0..self.get_position().get_ply() {
            let hash = &self.get_position().get_position_hash_history()[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /**
     * 自陣
     */
    #[allow(dead_code)]
    pub fn get_ji_jin(&self) -> Vec<Square> {
        if let Phase::First = self.game.position.get_phase(&Person::Friend) {
            crate::model::vo::other_part::op_region_vo::SenteJin::to_elm()
        } else {
            crate::model::vo::other_part::op_region_vo::GoteJin::to_elm()
        }
    }
    /**
     * 相手陣
     */
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::First = self.game.position.get_phase(&Person::Friend) {
            crate::model::vo::other_part::op_region_vo::GoteJin::to_elm()
        } else {
            crate::model::vo::other_part::op_region_vo::SenteJin::to_elm()
        }
    }

    pub fn get_mut_info(&mut self) -> &mut SPInfo {
        &mut self.game.position.info
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
                &self.game.position.control_count_by_piece
                    [speed_of_light.get_piece_struct_vo(pc).serial_piece_number()]
            }
            _ => &self.game.position.control_count_by_phase[phase_to_num(&phase)],
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

    // 駒の動きを出力
    pub fn print_kmugoki(&self, speed_of_light: &MLSpeedOfLightVo) {
        for piece_type in PIECE_TYPE_ARRAY.iter() {
            g_write(&format!("{} ", piece_type));
            self.print_kmugoki_dir(*piece_type, speed_of_light);
            g_writeln(""); //改行
        }
    }
    pub fn print_kmugoki_dir(&self, piece_type: GPPieceTypeVo, speed_of_light: &MLSpeedOfLightVo) {
        for kmdir in KM_UGOKI.back[speed_of_light
            .get_piece_type_struct_vo_from_piece_type(&piece_type)
            .serial_piece_number]
            .iter()
        {
            match *kmdir {
                PieceDirection::Owari => break,
                _ => g_write(&format!("{},", kmdir)),
            }
        }
    }

    // 入れた指し手の通り指すぜ☆（＾～＾）
    pub fn do_move(&mut self, movement: &GPMovementVo, speed_of_light: &MLSpeedOfLightVo) {
        // もう入っているかも知れないが、棋譜に入れる☆
        let ply = self.get_position().get_ply();
        self.game.position.set_current_movement(movement);
        let cap;
        {
            cap = self.game.position.do_move(movement, speed_of_light);
        }
        self.game.position.set_cap(ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_ky1_hash(speed_of_light);
        self.get_position_mut().set_current_position_hash(ky_hash);

        self.get_position_mut().add_ply(1);
    }

    pub fn undo_move(&mut self, speed_of_light: &MLSpeedOfLightVo) -> bool {
        if 0 < self.get_position().get_ply() {
            // 棋譜から読取、手目も減る
            self.get_position_mut().add_ply(-1);
            // let phase = self.sp_earth_dto.get_phase(&Person::Friend);
            let ss = &self.game.position.get_move().clone();
            self.game.position.undo_move(/*&phase,*/ ss, speed_of_light);
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }

    /*
    pub fn remake_visions(&mut self) {
        for phase in PHASE_ARRAY.iter() {
            // 全部忘れる☆（＾～＾）
            self.sp_earth_dto.vision_tree_by_phase[phase_to_num(phase)].clear();
        }
    }
    */

    /**
     * 初期局面ハッシュを作り直す
     */
    pub fn create_starting_position_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self.game.starting_board.create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.game.position_hash_seed.phase[PHASE_SECOND];

        hash
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_ky1_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self
            .get_position()
            .get_current_board()
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use crate::model::vo::game_part::gp_phase_vo::Phase::*;
        match self.game.position.get_phase(&Person::Friend) {
            First => hash ^= self.game.position_hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.game.position_hash_seed.phase[PHASE_SECOND],
            _ => {}
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_ky(&self) -> i8 {
        if self.get_position().get_ply() < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.get_position().get_ply() - 1;
        let new_ply = self.get_position().get_ply();
        // g_writeln( &format!( "Ｃount_same_ky last_ply={} new_ply={}", last_ply ,new_ply ) );
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            // g_writeln( &format!( "i_ply={} t={}", i_ply, t ) );
            if self.get_position().get_position_hash_history()[t as usize]
                == self.get_position().get_position_hash_history()[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.game.starting_position_hash
            == self.get_position().get_position_hash_history()[last_ply as usize]
        {
            count += 1;
        }

        count
    }

    /*
    // 相手の　玉　の位置を覚えます。
    pub fn memory_opponent_king(&mut self, phase: &Phase, opponent_phase: &Phase) {
        self.sp_earth_dto
            .memory_opponent_king(&phase, &opponent_phase);
    }
    */
}