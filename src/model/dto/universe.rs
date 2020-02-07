//!
//! 宇宙
//!
//! グローバル変数の作り方が分からないので仕方なく多くの変数を詰め込んでいるぜ☆（＾～＾）
//! 引数で渡しまくれだぜ☆（＾～＾）
//!
extern crate rand;
use rand::Rng;

use super::super::super::config::*;
use super::super::super::controller::common::conv::*;
use super::super::super::controller::communication::usi::*;
use super::super::super::model::vo::misc::*;
use super::super::super::model::vo::person::Person;
use super::super::super::model::vo::phase::*;
use super::super::super::model::vo::piece::Piece;
use super::super::super::model::vo::piece::*;
use super::super::super::model::vo::piece_direction::PieceDirection;
use super::super::super::model::vo::piece_movement::*;
use super::super::super::model::vo::piece_type::PieceType;
use super::super::super::model::vo::piece_type::*;
use super::super::super::model::vo::piece_vo::PieceVo;
use super::super::super::model::vo::speed_of_light::*;
use super::super::super::model::vo::square::*;
use super::super::application::application_part::*;
use super::super::dialogue::dialogue_part::*;
use super::super::search::search_part::*;
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
        match LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            Err(_why) => {} //panic!("couldn't write log. : {}",Error::description(&why)),
            Ok(_) => {}
        }
    }
}
#[allow(dead_code)]
pub fn g_writeln(s: &str) {
    println!("{}", s);
    if LOG_ENABLE {
        match LOGFILE
            .lock()
            .unwrap()
            .write_all(format!("{}\n", s).as_bytes())
        {
            Err(_why) => {}
            Ok(_) => {}
        }
    }
}

/**
 * グローバル変数の作り方が分からないので、
 * ここに全部入れてあるぜ☆（＾～＾）
 */
pub struct Universe {
    /// アプリケーション部
    application_part: ApplicationPart,
    /// 対話部
    dialogue_part: DialoguePart,
    /// 探索部
    search_part: SearchPart,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            application_part: ApplicationPart::new(),
            dialogue_part: DialoguePart::new(),
            search_part: SearchPart::new(),
        }
    }
    /**
     * 宇宙誕生
     */
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            for i_km in 0..KM_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.get_application_part_mut()
                    .get_position_hash_seed_mut()
                    .km[i_ms][i_km] = rand::thread_rng().gen_range(0, 18446744073709551615);
            }
        }
        // 持ち駒
        for i_km in 0..KM_LN {
            for i_mg in 0..MG_MAX {
                self.get_application_part_mut()
                    .get_position_hash_seed_mut()
                    .mg[i_km][i_mg] = rand::thread_rng().gen_range(0, 18446744073709551615);
            }
        }
        // 先後
        for i_sn in 0..SN_LN {
            self.get_application_part_mut()
                .get_position_hash_seed_mut()
                .sn[i_sn] = rand::thread_rng().gen_range(0, 18446744073709551615);
        }
    }
    /**
     * 初期局面、現局面ともにクリアーします。
     * 手目も 0 に戻します。
     */
    pub fn clear_all_positions(&mut self) {
        self.get_application_part_mut()
            .get_starting_position_mut()
            .clear();
        self.get_search_part_mut()
            .get_current_position_mut()
            .clear();
        self.get_search_part_mut().set_ply(0);
    }
    /// 開始局面を、現局面にコピーします
    pub fn copy_starting_position_to_current_position(&mut self) {
        // 盤上の駒。
        for i_ms in 0..BAN_SIZE {
            let i_sq = Square::from_umasu(i_ms);
            // TODO 取得→設定　するとエラーになってしまうので、今んとこ 作成→設定　するぜ☆（＾～＾）
            let piece = self
                .application_part
                .get_starting_position()
                .get_piece_by_square(&i_sq);
            self.search_part
                .get_current_position_mut()
                .set_piece_by_square(&i_sq, piece);
        }

        // 持ち駒
        for i_mg in 0..KM_LN {
            self.get_search_part_mut().get_current_position_mut().mg[i_mg] =
                self.get_application_part().get_starting_position().mg[i_mg];
        }
    }

    pub fn get_application_part_mut(&mut self) -> &mut ApplicationPart {
        &mut self.application_part
    }
    pub fn get_application_part(&self) -> &ApplicationPart {
        &self.application_part
    }

    pub fn get_dialogue_part_mut(&mut self) -> &mut DialoguePart {
        &mut self.dialogue_part
    }
    pub fn get_dialogue_part(&self) -> &DialoguePart {
        &self.dialogue_part
    }

    pub fn get_search_part_mut(&mut self) -> &mut SearchPart {
        &mut self.search_part
    }
    pub fn get_search_part(&self) -> &SearchPart {
        &self.search_part
    }

    /* **********************
     * コマンド・バッファー *
     ************************/
    pub fn is_empty_command(&mut self) -> bool {
        self.dialogue_part.vec_command.len() == 0
    }
    pub fn push_command(&mut self, line: &String) {
        self.dialogue_part.vec_command.push(format!("{}\n", line));
    }
    pub fn pop_command(&mut self) -> String {
        self.dialogue_part.vec_command.pop().unwrap()
    }

    /* ******
     * 盤上 *
     ********/

    /// 初期局面の盤上に駒の位置を設定するもの
    pub fn set_piece_to_starting_position(&mut self, suji: i8, dan: i8, piece: Piece) {
        self.get_application_part_mut()
            .get_starting_position_mut()
            .set_piece_by_square(&Square::from_file_rank(suji, dan), &piece);
    }
    pub fn set_starting_position_hand_piece(&mut self, km: Piece, maisu: i8) {
        self.get_application_part_mut()
            .get_starting_position_mut()
            .mg[km as usize] = maisu;
    }
    pub fn get_person_by_piece_vo(&self, piece_vo: &PieceVo) -> Person {
        if match_sn(&piece_vo.phase(), &self.search_part.get_phase(&Person::Ji)) {
            Person::Ji
        } else {
            Person::Ai
        }
    }

    /// 局面ハッシュ。
    pub fn get_all_position_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "[ini] {:20}\n",
            &self.get_application_part().get_starting_position_hash()
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
        if let Phase::Sen = self.search_part.get_phase(&Person::Ji) {
            super::super::vo::region::SenteJin::to_elm()
        } else {
            super::super::vo::region::GoteJin::to_elm()
        }
    }
    /**
     * 相手陣
     */
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::Sen = self.search_part.get_phase(&Person::Ji) {
            super::super::vo::region::GoteJin::to_elm()
        } else {
            super::super::vo::region::SenteJin::to_elm()
        }
    }

    /**
     * 表示
     *
     * 後手から見た盤を表示するぜ☆（＾～＾）
     * デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
     */
    pub fn kaku_ky(&self, num: &KyNums) -> String {
        let cur_pos = match *num {
            KyNums::Current => self.search_part.get_current_position(),
            KyNums::Start => self.application_part.get_starting_position(),
        };

        // 局面表示
        format!(
            "\
表示 {95}手目 {96} 同一局面{97}回目

           +----+----+----+----+----+----+----+----+----+
        i9 |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|
           +----+----+----+----+----+----+----+----+----+
ひx{87:2}   h8 |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|   ヒx{94:2}
           +----+----+----+----+----+----+----+----+----+
しx{86:2}   g7 |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}|   シx{93:2}
           +----+----+----+----+----+----+----+----+----+
うx{85:2}   f6 |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}|   ウx{92:2}
           +----+----+----+----+----+----+----+----+----+
ねx{84:2}   e5 |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}|   ネx{91:2}
           +----+----+----+----+----+----+----+----+----+
いx{83:2}   d4 |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}|   イx{90:2}
           +----+----+----+----+----+----+----+----+----+
ぞx{82:2}   c3 |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}|   ゾx{89:2}
           +----+----+----+----+----+----+----+----+----+
きx{81:2}   b2 |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}|   キx{88:2}
           +----+----+----+----+----+----+----+----+----+
▼      a1 |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}|   △
           +----+----+----+----+----+----+----+----+----+
              1    2    3    4    5    6    7    8    9\
",
            cur_pos.get_piece_by_square(&Square::from_umasu(19)),
            cur_pos.get_piece_by_square(&Square::from_umasu(29)),
            cur_pos.get_piece_by_square(&Square::from_umasu(39)),
            cur_pos.get_piece_by_square(&Square::from_umasu(49)),
            cur_pos.get_piece_by_square(&Square::from_umasu(59)),
            cur_pos.get_piece_by_square(&Square::from_umasu(69)),
            cur_pos.get_piece_by_square(&Square::from_umasu(79)),
            cur_pos.get_piece_by_square(&Square::from_umasu(89)),
            cur_pos.get_piece_by_square(&Square::from_umasu(99)),
            cur_pos.get_piece_by_square(&Square::from_umasu(18)),
            cur_pos.get_piece_by_square(&Square::from_umasu(28)),
            cur_pos.get_piece_by_square(&Square::from_umasu(38)),
            cur_pos.get_piece_by_square(&Square::from_umasu(48)),
            cur_pos.get_piece_by_square(&Square::from_umasu(58)),
            cur_pos.get_piece_by_square(&Square::from_umasu(68)),
            cur_pos.get_piece_by_square(&Square::from_umasu(78)),
            cur_pos.get_piece_by_square(&Square::from_umasu(88)),
            cur_pos.get_piece_by_square(&Square::from_umasu(98)),
            cur_pos.get_piece_by_square(&Square::from_umasu(17)),
            cur_pos.get_piece_by_square(&Square::from_umasu(27)),
            cur_pos.get_piece_by_square(&Square::from_umasu(37)),
            cur_pos.get_piece_by_square(&Square::from_umasu(47)),
            cur_pos.get_piece_by_square(&Square::from_umasu(57)),
            cur_pos.get_piece_by_square(&Square::from_umasu(67)),
            cur_pos.get_piece_by_square(&Square::from_umasu(77)),
            cur_pos.get_piece_by_square(&Square::from_umasu(87)),
            cur_pos.get_piece_by_square(&Square::from_umasu(97)),
            cur_pos.get_piece_by_square(&Square::from_umasu(16)),
            cur_pos.get_piece_by_square(&Square::from_umasu(26)),
            cur_pos.get_piece_by_square(&Square::from_umasu(36)),
            cur_pos.get_piece_by_square(&Square::from_umasu(46)),
            cur_pos.get_piece_by_square(&Square::from_umasu(56)),
            cur_pos.get_piece_by_square(&Square::from_umasu(66)),
            cur_pos.get_piece_by_square(&Square::from_umasu(76)),
            cur_pos.get_piece_by_square(&Square::from_umasu(86)),
            cur_pos.get_piece_by_square(&Square::from_umasu(96)),
            cur_pos.get_piece_by_square(&Square::from_umasu(15)),
            cur_pos.get_piece_by_square(&Square::from_umasu(25)),
            cur_pos.get_piece_by_square(&Square::from_umasu(35)),
            cur_pos.get_piece_by_square(&Square::from_umasu(45)),
            cur_pos.get_piece_by_square(&Square::from_umasu(55)),
            cur_pos.get_piece_by_square(&Square::from_umasu(65)),
            cur_pos.get_piece_by_square(&Square::from_umasu(75)),
            cur_pos.get_piece_by_square(&Square::from_umasu(85)),
            cur_pos.get_piece_by_square(&Square::from_umasu(95)),
            cur_pos.get_piece_by_square(&Square::from_umasu(14)),
            cur_pos.get_piece_by_square(&Square::from_umasu(24)),
            cur_pos.get_piece_by_square(&Square::from_umasu(34)),
            cur_pos.get_piece_by_square(&Square::from_umasu(44)),
            cur_pos.get_piece_by_square(&Square::from_umasu(54)),
            cur_pos.get_piece_by_square(&Square::from_umasu(64)),
            cur_pos.get_piece_by_square(&Square::from_umasu(74)),
            cur_pos.get_piece_by_square(&Square::from_umasu(84)),
            cur_pos.get_piece_by_square(&Square::from_umasu(94)),
            cur_pos.get_piece_by_square(&Square::from_umasu(13)),
            cur_pos.get_piece_by_square(&Square::from_umasu(23)),
            cur_pos.get_piece_by_square(&Square::from_umasu(33)),
            cur_pos.get_piece_by_square(&Square::from_umasu(43)),
            cur_pos.get_piece_by_square(&Square::from_umasu(53)),
            cur_pos.get_piece_by_square(&Square::from_umasu(63)),
            cur_pos.get_piece_by_square(&Square::from_umasu(73)),
            cur_pos.get_piece_by_square(&Square::from_umasu(83)),
            cur_pos.get_piece_by_square(&Square::from_umasu(93)),
            cur_pos.get_piece_by_square(&Square::from_umasu(12)),
            cur_pos.get_piece_by_square(&Square::from_umasu(22)),
            cur_pos.get_piece_by_square(&Square::from_umasu(32)),
            cur_pos.get_piece_by_square(&Square::from_umasu(42)),
            cur_pos.get_piece_by_square(&Square::from_umasu(52)),
            cur_pos.get_piece_by_square(&Square::from_umasu(62)),
            cur_pos.get_piece_by_square(&Square::from_umasu(72)),
            cur_pos.get_piece_by_square(&Square::from_umasu(82)),
            cur_pos.get_piece_by_square(&Square::from_umasu(92)),
            cur_pos.get_piece_by_square(&Square::from_umasu(11)),
            cur_pos.get_piece_by_square(&Square::from_umasu(21)),
            cur_pos.get_piece_by_square(&Square::from_umasu(31)),
            cur_pos.get_piece_by_square(&Square::from_umasu(41)),
            cur_pos.get_piece_by_square(&Square::from_umasu(51)),
            cur_pos.get_piece_by_square(&Square::from_umasu(61)),
            cur_pos.get_piece_by_square(&Square::from_umasu(71)),
            cur_pos.get_piece_by_square(&Square::from_umasu(81)),
            cur_pos.get_piece_by_square(&Square::from_umasu(91)),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            cur_pos.mg[Piece::Rook1 as usize],
            cur_pos.mg[Piece::Bishop1 as usize],
            cur_pos.mg[Piece::Gold1 as usize],
            cur_pos.mg[Piece::Silver1 as usize],
            cur_pos.mg[Piece::Knight1 as usize],
            cur_pos.mg[Piece::Lance1 as usize],
            cur_pos.mg[Piece::Pawn1 as usize],
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            cur_pos.mg[Piece::Rook2 as usize],
            cur_pos.mg[Piece::Bishop2 as usize],
            cur_pos.mg[Piece::Gold2 as usize],
            cur_pos.mg[Piece::Silver2 as usize],
            cur_pos.mg[Piece::Knight2 as usize],
            cur_pos.mg[Piece::Lance2 as usize],
            cur_pos.mg[Piece::Pawn2 as usize],
            self.get_search_part().get_ply(),
            self.search_part.get_phase(&Person::Ji),
            self.count_same_ky()
        )
    }

    /**
     * 表示
     */
    pub fn kaku_number_board(
        &self,
        sn: &Phase,
        pc: &Piece,
        speed_of_light: &SpeedOfLight,
    ) -> String {
        let nb = match *sn {
            Phase::Owari => {
                &self.search_part.effect_count_by_piece[speed_of_light
                    .piece_vo_master
                    .get_piece_vo(pc)
                    .serial_piece_number()]
            }
            _ => &self.search_part.effect_count_by_phase[sn_to_num(&sn)],
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
            nb.get_su_by_sq(&Square::from_umasu(19)),
            nb.get_su_by_sq(&Square::from_umasu(29)),
            nb.get_su_by_sq(&Square::from_umasu(39)),
            nb.get_su_by_sq(&Square::from_umasu(49)),
            nb.get_su_by_sq(&Square::from_umasu(59)),
            nb.get_su_by_sq(&Square::from_umasu(69)),
            nb.get_su_by_sq(&Square::from_umasu(79)),
            nb.get_su_by_sq(&Square::from_umasu(89)),
            nb.get_su_by_sq(&Square::from_umasu(99)),
            nb.get_su_by_sq(&Square::from_umasu(18)),
            nb.get_su_by_sq(&Square::from_umasu(28)),
            nb.get_su_by_sq(&Square::from_umasu(38)),
            nb.get_su_by_sq(&Square::from_umasu(48)),
            nb.get_su_by_sq(&Square::from_umasu(58)),
            nb.get_su_by_sq(&Square::from_umasu(68)),
            nb.get_su_by_sq(&Square::from_umasu(78)),
            nb.get_su_by_sq(&Square::from_umasu(88)),
            nb.get_su_by_sq(&Square::from_umasu(98)),
            nb.get_su_by_sq(&Square::from_umasu(17)),
            nb.get_su_by_sq(&Square::from_umasu(27)),
            nb.get_su_by_sq(&Square::from_umasu(37)),
            nb.get_su_by_sq(&Square::from_umasu(47)),
            nb.get_su_by_sq(&Square::from_umasu(57)),
            nb.get_su_by_sq(&Square::from_umasu(67)),
            nb.get_su_by_sq(&Square::from_umasu(77)),
            nb.get_su_by_sq(&Square::from_umasu(87)),
            nb.get_su_by_sq(&Square::from_umasu(97)),
            nb.get_su_by_sq(&Square::from_umasu(16)),
            nb.get_su_by_sq(&Square::from_umasu(26)),
            nb.get_su_by_sq(&Square::from_umasu(36)),
            nb.get_su_by_sq(&Square::from_umasu(46)),
            nb.get_su_by_sq(&Square::from_umasu(56)),
            nb.get_su_by_sq(&Square::from_umasu(66)),
            nb.get_su_by_sq(&Square::from_umasu(76)),
            nb.get_su_by_sq(&Square::from_umasu(86)),
            nb.get_su_by_sq(&Square::from_umasu(96)),
            nb.get_su_by_sq(&Square::from_umasu(15)),
            nb.get_su_by_sq(&Square::from_umasu(25)),
            nb.get_su_by_sq(&Square::from_umasu(35)),
            nb.get_su_by_sq(&Square::from_umasu(45)),
            nb.get_su_by_sq(&Square::from_umasu(55)),
            nb.get_su_by_sq(&Square::from_umasu(65)),
            nb.get_su_by_sq(&Square::from_umasu(75)),
            nb.get_su_by_sq(&Square::from_umasu(85)),
            nb.get_su_by_sq(&Square::from_umasu(95)),
            nb.get_su_by_sq(&Square::from_umasu(14)),
            nb.get_su_by_sq(&Square::from_umasu(24)),
            nb.get_su_by_sq(&Square::from_umasu(34)),
            nb.get_su_by_sq(&Square::from_umasu(44)),
            nb.get_su_by_sq(&Square::from_umasu(54)),
            nb.get_su_by_sq(&Square::from_umasu(64)),
            nb.get_su_by_sq(&Square::from_umasu(74)),
            nb.get_su_by_sq(&Square::from_umasu(84)),
            nb.get_su_by_sq(&Square::from_umasu(94)),
            nb.get_su_by_sq(&Square::from_umasu(13)),
            nb.get_su_by_sq(&Square::from_umasu(23)),
            nb.get_su_by_sq(&Square::from_umasu(33)),
            nb.get_su_by_sq(&Square::from_umasu(43)),
            nb.get_su_by_sq(&Square::from_umasu(53)),
            nb.get_su_by_sq(&Square::from_umasu(63)),
            nb.get_su_by_sq(&Square::from_umasu(73)),
            nb.get_su_by_sq(&Square::from_umasu(83)),
            nb.get_su_by_sq(&Square::from_umasu(93)),
            nb.get_su_by_sq(&Square::from_umasu(12)),
            nb.get_su_by_sq(&Square::from_umasu(22)),
            nb.get_su_by_sq(&Square::from_umasu(32)),
            nb.get_su_by_sq(&Square::from_umasu(42)),
            nb.get_su_by_sq(&Square::from_umasu(52)),
            nb.get_su_by_sq(&Square::from_umasu(62)),
            nb.get_su_by_sq(&Square::from_umasu(72)),
            nb.get_su_by_sq(&Square::from_umasu(82)),
            nb.get_su_by_sq(&Square::from_umasu(92)),
            nb.get_su_by_sq(&Square::from_umasu(11)),
            nb.get_su_by_sq(&Square::from_umasu(21)),
            nb.get_su_by_sq(&Square::from_umasu(31)),
            nb.get_su_by_sq(&Square::from_umasu(41)),
            nb.get_su_by_sq(&Square::from_umasu(51)),
            nb.get_su_by_sq(&Square::from_umasu(61)),
            nb.get_su_by_sq(&Square::from_umasu(71)),
            nb.get_su_by_sq(&Square::from_umasu(81)),
            nb.get_su_by_sq(&Square::from_umasu(91)),
        )
    }

    // 駒の動きを出力
    pub fn hyoji_kmugoki(&self) {
        for kms in KMS_ARRAY.iter() {
            g_write(&format!("{} ", kms));
            self.hyoji_kmugoki_dir(&kms);
            g_writeln(""); //改行
        }
    }
    pub fn hyoji_kmugoki_dir(&self, kms: &PieceType) {
        for kmdir in KM_UGOKI.back[kms_to_num(kms)].iter() {
            match *kmdir {
                PieceDirection::Owari => break,
                _ => g_write(&format!("{},", kmdir)),
            }
        }
    }

    // 入れた指し手の通り指すぜ☆（＾～＾）
    pub fn do_ss(&mut self, move3: &Sasite, speed_of_light: &SpeedOfLight) {
        // もう入っているかも知れないが、棋譜に入れる☆
        let ply = self.get_search_part().get_ply();
        self.search_part.moves_history[ply as usize] = (*move3).clone();
        let cap;
        {
            cap = self.search_part.do_move(move3, speed_of_light).clone();
        }
        self.search_part.set_cap(ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_ky1_hash(speed_of_light);
        self.get_search_part_mut()
            .set_current_position_hash(ky_hash);

        self.get_search_part_mut().add_ply(1);
    }

    pub fn undo_ss(&mut self, speed_of_light: &SpeedOfLight) -> bool {
        if 0 < self.get_search_part().get_ply() {
            // 棋譜から読取、手目も減る
            self.get_search_part_mut().add_ply(-1);
            let sn = self.search_part.get_phase(&Person::Ji);
            let ss = &self.search_part.get_move().clone();
            self.search_part.undo_move(&sn, ss, speed_of_light);
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }

    pub fn remake_visions(&mut self) {
        for sn in SN_ARRAY.iter() {
            // 全部忘れる☆（＾～＾）
            self.search_part.vision_tree_by_phase[sn_to_num(sn)].clear();
        }
    }

    /**
     * 初期局面ハッシュを作り直す
     */
    pub fn create_starting_position_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = self
            .get_application_part()
            .get_starting_position()
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.get_application_part().get_position_hash_seed().sn[SN_GO];

        hash
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_ky1_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = self
            .get_search_part()
            .get_current_position()
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use super::super::vo::phase::Phase::*;
        match self.search_part.get_phase(&Person::Ji) {
            Sen => hash ^= self.get_application_part().get_position_hash_seed().sn[SN_SEN],
            Go => hash ^= self.get_application_part().get_position_hash_seed().sn[SN_GO],
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
        if *self.get_application_part().get_starting_position_hash()
            == self.get_search_part().get_position_hash_history()[last_ply as usize]
        {
            count += 1;
        }

        count
    }

    /// 相手の　玉　の位置を覚えます。
    pub fn memory_opponent_king(&mut self, phase: &Phase, opponent_phase: &Phase) {
        self.search_part
            .memory_opponent_king(&phase, &opponent_phase);
    }
}
