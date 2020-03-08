//!
//! Data transfer object.
//! Main loop.
//!
extern crate rand;
use rand::Rng;

use super::super::super::super::config::*;
use super::super::super::super::model::dto::search_part::sp_position_dto::*;
use super::super::super::super::model::vo::game_part::gp_movement_vo::*;
use super::super::super::super::model::vo::game_part::gp_phase_vo::*;
use super::super::super::super::model::vo::game_part::gp_piece_struct_vo::GPPieceStructVo;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::super::model::vo::game_part::gp_piece_vo::*;
use super::super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection;
use super::super::super::super::model::vo::other_part::op_piece_movement_vo::*;
use super::super::super::dto::search_part::sp_earth_dto::*;
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
pub fn g_write(s: &str) {
    println!("{}", s);
    if LOG_ENABLE {
        // write_allメソッドを使うには use std::io::Write; が必要
        if let Err(_why) = LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            //panic!("couldn't write log. : {}",Error::description(&why)),
        }
    }
}
#[allow(dead_code)]
pub fn g_writeln(s: &str) {
    println!("{}", s);
    if LOG_ENABLE {
        if let Err(_why) = LOGFILE
            .lock()
            .unwrap()
            .write_all(format!("{}\n", s).as_bytes())
        {}
    }
}

/// 局面ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct PositionHashSeed {
    // 盤上の駒
    pub km: [[u64; PIECE_LN]; BOARD_SIZE],
    // 持ち駒
    pub mg: [[u64; MG_MAX]; PIECE_LN],
    // 先後
    pub phase: [u64; PHASE_LN],
}

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct MLUniverseDto {
    /// 局面ハッシュ種☆（＾～＾）
    position_hash_seed: PositionHashSeed,
    /// 初期局面
    starting_position: SPPositionDto,
    /// 初期局面ハッシュ
    starting_position_hash: u64,
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
    /// 探索部
    sp_earth_dto: SPEarthDto,
}

impl Default for MLUniverseDto {
    fn default() -> Self {
        MLUniverseDto {
            position_hash_seed: PositionHashSeed {
                // 盤上の駒
                km: [[0; PIECE_LN]; BOARD_SIZE],
                // 持ち駒
                mg: [[0; MG_MAX]; PIECE_LN],
                // 先後
                phase: [0; PHASE_LN],
            },
            starting_position: SPPositionDto::default(),
            starting_position_hash: 0,
            dialogue_mode: false,
            vec_command: Vec::new(),
            sp_earth_dto: SPEarthDto::default(),
        }
    }
}
impl MLUniverseDto {
    /**
     * 宇宙誕生
     */
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_ms in MASU_0..BOARD_SIZE {
            for i_km in 0..PIECE_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.get_position_hash_seed_mut().km[i_ms][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..PIECE_LN {
            for i_mg in 0..MG_MAX {
                self.get_position_hash_seed_mut().mg[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LN {
            self.get_position_hash_seed_mut().phase[i_phase] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }
    /**
     * 初期局面、現局面ともにクリアーします。
     * 手目も 0 に戻します。
     */
    pub fn clear_all_positions(&mut self) {
        self.get_starting_position_mut().clear();
        self.get_search_part_mut()
            .get_current_position_mut()
            .clear();
        self.get_search_part_mut().set_ply(0);
    }
    /// 開始局面を、現局面にコピーします
    pub fn copy_starting_position_to_current_position(&mut self) {
        // 盤上の駒。
        for i_ms in 0..BOARD_SIZE {
            let i_sq = Square::from_umasu(i_ms);
            // TODO 取得→設定　するとエラーになってしまうので、今んとこ 作成→設定　するぜ☆（＾～＾）
            let piece = self.starting_position.get_piece_by_square(&i_sq);
            self.sp_earth_dto
                .get_current_position_mut()
                .set_piece_by_square(&i_sq, piece);
        }

        // 持ち駒
        self.sp_earth_dto.get_current_position_mut().hand[..PIECE_LN]
            .clone_from_slice(&self.starting_position.hand[..PIECE_LN]);
        /*
        for i_mg in 0..PIECE_LN {
            self.get_search_part_mut().get_current_position_mut().mg[i_mg] =
                self.get_starting_position().mg[i_mg];
        }
        */
    }

    pub fn get_position_hash_seed(&self) -> &PositionHashSeed {
        &self.position_hash_seed
    }
    pub fn get_position_hash_seed_mut(&mut self) -> &mut PositionHashSeed {
        &mut self.position_hash_seed
    }

    pub fn get_starting_position(&self) -> &SPPositionDto {
        &self.starting_position
    }
    pub fn get_starting_position_mut(&mut self) -> &mut SPPositionDto {
        &mut self.starting_position
    }

    pub fn get_starting_position_hash(&self) -> &u64 {
        &self.starting_position_hash
    }
    pub fn get_starting_position_hash_mut(&mut self) -> &mut u64 {
        &mut self.starting_position_hash
    }
    pub fn set_starting_position_hash(&mut self, val: u64) {
        self.starting_position_hash = val;
    }

    pub fn get_search_part_mut(&mut self) -> &mut SPEarthDto {
        &mut self.sp_earth_dto
    }
    pub fn get_search_part(&self) -> &SPEarthDto {
        &self.sp_earth_dto
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
        self.get_starting_position_mut()
            .set_piece_by_square(&Square::from_file_rank(suji, dan), &piece);
    }
    pub fn set_starting_position_hand_piece(&mut self, km: GPPieceVo, maisu: i8) {
        self.get_starting_position_mut().hand[km as usize] = maisu;
    }
    pub fn get_person_by_piece_vo(&self, piece_vo: &GPPieceStructVo) -> Person {
        if &piece_vo.phase() == &self.sp_earth_dto.get_phase(&Person::Friend) {
            Person::Friend
        } else {
            Person::Opponent
        }
    }

    /// 局面ハッシュ。
    pub fn get_all_position_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "[ini] {:20}\n",
            &self.get_starting_position_hash()
        ));

        for ply in 0..self.get_search_part().get_ply() {
            let hash = &self.get_search_part().get_position_hash_history()[ply as usize];
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
        if let Phase::First = self.sp_earth_dto.get_phase(&Person::Friend) {
            super::super::super::vo::other_part::op_region_vo::SenteJin::to_elm()
        } else {
            super::super::super::vo::other_part::op_region_vo::GoteJin::to_elm()
        }
    }
    /**
     * 相手陣
     */
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::First = self.sp_earth_dto.get_phase(&Person::Friend) {
            super::super::super::vo::other_part::op_region_vo::GoteJin::to_elm()
        } else {
            super::super::super::vo::other_part::op_region_vo::SenteJin::to_elm()
        }
    }

    /**
     * 表示
     */
    pub fn print_number_board(
        &self,
        phase: &Phase,
        pc: &GPPieceVo,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> String {
        let nb = match *phase {
            Phase::None => {
                &self.sp_earth_dto.control_count_by_piece
                    [speed_of_light.get_piece_struct_vo(pc).serial_piece_number()]
            }
            _ => &self.sp_earth_dto.control_count_by_phase[phase_to_num(&phase)],
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
            nb.get_number_by_square(&Square::from_umasu(19)),
            nb.get_number_by_square(&Square::from_umasu(29)),
            nb.get_number_by_square(&Square::from_umasu(39)),
            nb.get_number_by_square(&Square::from_umasu(49)),
            nb.get_number_by_square(&Square::from_umasu(59)),
            nb.get_number_by_square(&Square::from_umasu(69)),
            nb.get_number_by_square(&Square::from_umasu(79)),
            nb.get_number_by_square(&Square::from_umasu(89)),
            nb.get_number_by_square(&Square::from_umasu(99)),
            nb.get_number_by_square(&Square::from_umasu(18)),
            nb.get_number_by_square(&Square::from_umasu(28)),
            nb.get_number_by_square(&Square::from_umasu(38)),
            nb.get_number_by_square(&Square::from_umasu(48)),
            nb.get_number_by_square(&Square::from_umasu(58)),
            nb.get_number_by_square(&Square::from_umasu(68)),
            nb.get_number_by_square(&Square::from_umasu(78)),
            nb.get_number_by_square(&Square::from_umasu(88)),
            nb.get_number_by_square(&Square::from_umasu(98)),
            nb.get_number_by_square(&Square::from_umasu(17)),
            nb.get_number_by_square(&Square::from_umasu(27)),
            nb.get_number_by_square(&Square::from_umasu(37)),
            nb.get_number_by_square(&Square::from_umasu(47)),
            nb.get_number_by_square(&Square::from_umasu(57)),
            nb.get_number_by_square(&Square::from_umasu(67)),
            nb.get_number_by_square(&Square::from_umasu(77)),
            nb.get_number_by_square(&Square::from_umasu(87)),
            nb.get_number_by_square(&Square::from_umasu(97)),
            nb.get_number_by_square(&Square::from_umasu(16)),
            nb.get_number_by_square(&Square::from_umasu(26)),
            nb.get_number_by_square(&Square::from_umasu(36)),
            nb.get_number_by_square(&Square::from_umasu(46)),
            nb.get_number_by_square(&Square::from_umasu(56)),
            nb.get_number_by_square(&Square::from_umasu(66)),
            nb.get_number_by_square(&Square::from_umasu(76)),
            nb.get_number_by_square(&Square::from_umasu(86)),
            nb.get_number_by_square(&Square::from_umasu(96)),
            nb.get_number_by_square(&Square::from_umasu(15)),
            nb.get_number_by_square(&Square::from_umasu(25)),
            nb.get_number_by_square(&Square::from_umasu(35)),
            nb.get_number_by_square(&Square::from_umasu(45)),
            nb.get_number_by_square(&Square::from_umasu(55)),
            nb.get_number_by_square(&Square::from_umasu(65)),
            nb.get_number_by_square(&Square::from_umasu(75)),
            nb.get_number_by_square(&Square::from_umasu(85)),
            nb.get_number_by_square(&Square::from_umasu(95)),
            nb.get_number_by_square(&Square::from_umasu(14)),
            nb.get_number_by_square(&Square::from_umasu(24)),
            nb.get_number_by_square(&Square::from_umasu(34)),
            nb.get_number_by_square(&Square::from_umasu(44)),
            nb.get_number_by_square(&Square::from_umasu(54)),
            nb.get_number_by_square(&Square::from_umasu(64)),
            nb.get_number_by_square(&Square::from_umasu(74)),
            nb.get_number_by_square(&Square::from_umasu(84)),
            nb.get_number_by_square(&Square::from_umasu(94)),
            nb.get_number_by_square(&Square::from_umasu(13)),
            nb.get_number_by_square(&Square::from_umasu(23)),
            nb.get_number_by_square(&Square::from_umasu(33)),
            nb.get_number_by_square(&Square::from_umasu(43)),
            nb.get_number_by_square(&Square::from_umasu(53)),
            nb.get_number_by_square(&Square::from_umasu(63)),
            nb.get_number_by_square(&Square::from_umasu(73)),
            nb.get_number_by_square(&Square::from_umasu(83)),
            nb.get_number_by_square(&Square::from_umasu(93)),
            nb.get_number_by_square(&Square::from_umasu(12)),
            nb.get_number_by_square(&Square::from_umasu(22)),
            nb.get_number_by_square(&Square::from_umasu(32)),
            nb.get_number_by_square(&Square::from_umasu(42)),
            nb.get_number_by_square(&Square::from_umasu(52)),
            nb.get_number_by_square(&Square::from_umasu(62)),
            nb.get_number_by_square(&Square::from_umasu(72)),
            nb.get_number_by_square(&Square::from_umasu(82)),
            nb.get_number_by_square(&Square::from_umasu(92)),
            nb.get_number_by_square(&Square::from_umasu(11)),
            nb.get_number_by_square(&Square::from_umasu(21)),
            nb.get_number_by_square(&Square::from_umasu(31)),
            nb.get_number_by_square(&Square::from_umasu(41)),
            nb.get_number_by_square(&Square::from_umasu(51)),
            nb.get_number_by_square(&Square::from_umasu(61)),
            nb.get_number_by_square(&Square::from_umasu(71)),
            nb.get_number_by_square(&Square::from_umasu(81)),
            nb.get_number_by_square(&Square::from_umasu(91)),
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
        let ply = self.get_search_part().get_ply();
        self.sp_earth_dto.set_current_movement(movement);
        let cap;
        {
            cap = self.sp_earth_dto.do_move(movement, speed_of_light);
        }
        self.sp_earth_dto.set_cap(ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_ky1_hash(speed_of_light);
        self.get_search_part_mut()
            .set_current_position_hash(ky_hash);

        self.get_search_part_mut().add_ply(1);
    }

    pub fn undo_move(&mut self, speed_of_light: &MLSpeedOfLightVo) -> bool {
        if 0 < self.get_search_part().get_ply() {
            // 棋譜から読取、手目も減る
            self.get_search_part_mut().add_ply(-1);
            // let phase = self.sp_earth_dto.get_phase(&Person::Friend);
            let ss = &self.sp_earth_dto.get_move().clone();
            self.sp_earth_dto.undo_move(/*&phase,*/ ss, speed_of_light);
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
        let mut hash = self
            .get_starting_position()
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.get_position_hash_seed().phase[PHASE_SECOND];

        hash
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_ky1_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self
            .get_search_part()
            .get_current_position()
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use super::super::super::vo::game_part::gp_phase_vo::Phase::*;
        match self.sp_earth_dto.get_phase(&Person::Friend) {
            First => hash ^= self.get_position_hash_seed().phase[PHASE_FIRST],
            Second => hash ^= self.get_position_hash_seed().phase[PHASE_SECOND],
            _ => {}
        }

        hash
    }

    /**
     * 千日手を調べるために、
     * 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
     */
    pub fn count_same_ky(&self) -> i8 {
        if self.get_search_part().get_ply() < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.get_search_part().get_ply() - 1;
        let new_ply = self.get_search_part().get_ply();
        // g_writeln( &format!( "Ｃount_same_ky last_ply={} new_ply={}", last_ply ,new_ply ) );
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            // g_writeln( &format!( "i_ply={} t={}", i_ply, t ) );
            if self.get_search_part().get_position_hash_history()[t as usize]
                == self.get_search_part().get_position_hash_history()[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if *self.get_starting_position_hash()
            == self.get_search_part().get_position_hash_history()[last_ply as usize]
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
