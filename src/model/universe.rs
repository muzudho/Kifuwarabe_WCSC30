//!
//! 宇宙
//!
//! グローバル変数の作り方が分からないので仕方なく多くの変数を詰め込んでいるぜ☆（＾～＾）
//! 引数で渡しまくれだぜ☆（＾～＾）
//!
extern crate rand;
use rand::Rng;

use super::super::config::*;
use super::super::controller::common::conv::*;
use super::super::controller::communication::usi::*;
use super::super::controller::status::ky::*;
use super::super::controller::status::number_board::*;
use super::super::controller::thinking::visions::vision_tree::*;
use super::super::model::master::misc::*;
use super::super::model::master::person::Person;
use super::super::model::master::phase::*;
use super::super::model::master::piece::Piece;
use super::super::model::master::piece::*;
use super::super::model::master::piece_direction::PieceDirection;
use super::super::model::master::piece_movement::*;
use super::super::model::master::piece_struct::PieceStruct;
use super::super::model::master::piece_struct_master::PieceStructMaster;
use super::super::model::master::piece_type::PieceType;
use super::super::model::master::piece_type::*;
use super::super::model::master::ply::*;
use super::super::model::master::square::*;
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
 * 局面ハッシュ種
 * ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
 */
pub struct KyHashSeed {
    // 盤上の駒
    pub km: [[u64; KM_LN]; BAN_SIZE],
    // 持ち駒
    pub mg: [[u64; MG_MAX]; KM_LN],
    // 先後
    pub sn: [u64; SN_LN],
}

/**
 * グローバル変数の作り方が分からないので、
 * ここに全部入れてあるぜ☆（＾～＾）
 */
pub struct Universe {
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
    /// 初期局面
    pub ky0: Kyokumen,
    /// 現局面
    pub ky: Kyokumen,
    /// 局面ハッシュ種
    pub ky_hash_seed: KyHashSeed,
    /// 手目
    pub teme: usize,
    /// 棋譜
    pub kifu: [Sasite; TEME_LN],
    /// 初期局面ハッシュ
    pub ky0_hash: u64,
    /// 現局面ハッシュ
    pub ky_hash: [u64; TEME_LN],
    /// 取った駒
    pub cap: [Piece; TEME_LN],
    /// 利きの数（先後別）
    pub kiki_su_by_sn: [NumberBoard; SN_LN],
    /// 利きの数（先後付き駒別）
    pub kiki_su_by_km: [NumberBoard; KM_LN],
    /// ビジョン・ツリー
    pub vision_tree_by_sn: [VisionTree; SN_LN],
    /// 駒構造体・マスター
    piece_struct_master: PieceStructMaster,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            dialogue_mode: false,
            vec_command: Vec::new(),
            // 初期局面
            ky0: Kyokumen::new(),
            // 現局面
            ky: Kyokumen::new(),
            ky_hash_seed: KyHashSeed {
                // 盤上の駒
                km: [[0; KM_LN]; BAN_SIZE],
                // 持ち駒
                mg: [[0; MG_MAX]; KM_LN],
                // 先後
                sn: [0; SN_LN],
            },
            teme: 0,
            kifu: [
                // 1行16要素で並べるぜ☆（＾～＾）
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(), //257要素
            ],
            ky0_hash: 0,
            ky_hash: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, //257要素
            ],
            /// 取った駒
            cap: [
                // 1行16要素で並べるぜ☆（＾～＾）
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara, //257要素
            ],
            // 利き数（先後別）
            kiki_su_by_sn: [NumberBoard::new(), NumberBoard::new(), NumberBoard::new()],
            // 利き数（駒別なので３０個ある）
            kiki_su_by_km: [
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
            ],
            vision_tree_by_sn: [VisionTree::new(), VisionTree::new(), VisionTree::new()],
            piece_struct_master: PieceStructMaster::new(),
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
                self.ky_hash_seed.km[i_ms][i_km] =
                    rand::thread_rng().gen_range(0, 18446744073709551615);
            }
        }
        // 持ち駒
        for i_km in 0..KM_LN {
            for i_mg in 0..MG_MAX {
                self.ky_hash_seed.mg[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18446744073709551615);
            }
        }
        // 先後
        for i_sn in 0..SN_LN {
            self.ky_hash_seed.sn[i_sn] = rand::thread_rng().gen_range(0, 18446744073709551615);
        }
    }
    /**
     * 初期局面、現局面ともにクリアーします。
     * 手目も 0 に戻します。
     */
    pub fn clear_ky01(&mut self) {
        self.ky0.clear();
        self.ky.clear();
        self.set_teme(0);
    }
    /**
     * 初期局面を、現局面にコピーします
     */
    pub fn copy_ky0_to_ky1(&mut self) {
        // 盤上
        for i_ms in 0..BAN_SIZE {
            let i_sq = Square::from_umasu(i_ms);
            self.ky
                .set_km_by_sq(&i_sq, self.ky0.get_piece_struct_by_sq(&i_sq).piece());
        }
        // 持ち駒
        for i_mg in 0..KM_LN {
            self.ky.mg[i_mg] = self.ky0.mg[i_mg];
        }
    }

    pub fn position(&self) -> &Kyokumen {
        &self.ky
    }
    pub fn position_mut(&mut self) -> &Kyokumen {
        &self.ky
    }
    /// らいおんの位置
    pub fn get_sq_r(&self, jiai: &Person) -> &Square {
        &self.ky.get_sq_r(sn_to_num(&self.get_teban(jiai)))
    }

    /* **********************
     * コマンド・バッファー *
     ************************/
    pub fn is_empty_command(&mut self) -> bool {
        self.vec_command.len() == 0
    }
    pub fn push_command(&mut self, line: &String) {
        self.vec_command.push(format!("{}\n", line));
    }
    pub fn pop_command(&mut self) -> String {
        self.vec_command.pop().unwrap()
    }

    /* ******
     * 盤上 *
     ********/

    /**
     * 初期局面の盤上に駒の位置を設定するもの
     */
    pub fn set_ky0_ban_km(&mut self, suji: i8, dan: i8, km: &Piece) {
        self.ky0
            .set_km_by_sq(&Square::from_file_rank(suji, dan), km);
    }
    pub fn set_ky0_mg(&mut self, km: Piece, maisu: i8) {
        self.ky0.mg[km as usize] = maisu;
    }
    pub fn get_jiai_by_km(&self, piece_struct: &PieceStruct) -> Person {
        if match_sn(&piece_struct.phase(), &self.get_teban(&Person::Ji)) {
            Person::Ji
        } else {
            Person::Ai
        }
    }

    /* ******
     * 棋譜 *
     ********/

    pub fn set_teme(&mut self, teme: usize) {
        self.teme = teme
    }
    pub fn get_teme(&self) -> usize {
        self.teme
    }
    // 手番
    pub fn get_teban(&self, jiai: &Person) -> Phase {
        use super::master::person::Person::*;
        match *jiai {
            Ji => {
                // 手番
                if self.teme % 2 == 0 {
                    Phase::Sen
                } else {
                    Phase::Go
                }
            }
            Ai => {
                // 相手番
                if self.teme % 2 == 0 {
                    Phase::Go
                } else {
                    Phase::Sen
                }
            }
            _ => Phase::Owari,
        }
    }

    /**
     * 棋譜の作成
     */
    pub fn set_sasite_src(&mut self, src: &Square) {
        self.kifu[self.teme].src = src.clone()
    }
    pub fn set_sasite_dst(&mut self, dst: &Square) {
        self.kifu[self.teme].dst = dst.clone()
    }
    pub fn set_sasite_pro(&mut self, pro: bool) {
        self.kifu[self.teme].pro = pro
    }
    pub fn set_sasite_drop(&mut self, kms: PieceType) {
        self.kifu[self.teme].drop = kms
    }
    pub fn set_ky0_hash(&mut self, hash: u64) {
        self.ky0_hash = hash
    }
    pub fn set_ky1_hash(&mut self, hash: u64) {
        self.ky_hash[self.teme] = hash
    }
    #[allow(dead_code)]
    pub fn set_cap(&mut self, teme: usize, km: Piece) {
        self.cap[teme] = km
    }
    pub fn get_sasite(&self) -> &Sasite {
        &self.kifu[self.teme]
    }
    #[allow(dead_code)]
    pub fn get_ky_hash(&mut self) -> u64 {
        self.ky_hash[self.teme]
    }
    /**
     * 使い方
     * let s = uchu.kaku_kifu();
     * g_writeln( &s );
     */
    pub fn kaku_kifu(&self) -> String {
        let mut s = String::new();
        for teme in 0..self.teme {
            let ss = &self.kifu[teme];
            s.push_str(&format!("[{}] {}", teme, ss));
        }
        s
    }
    pub fn kaku_ky_hash(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.ky0_hash));

        for teme in 0..self.teme {
            let hash = &self.ky_hash[teme];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", teme, hash));
        }
        s
    }

    /**
     * 自陣
     */
    #[allow(dead_code)]
    pub fn get_ji_jin(&self) -> Vec<Square> {
        if let Phase::Sen = self.get_teban(&Person::Ji) {
            super::master::region::SenteJin::to_elm()
        } else {
            super::master::region::GoteJin::to_elm()
        }
    }
    /**
     * 相手陣
     */
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::Sen = self.get_teban(&Person::Ji) {
            super::master::region::GoteJin::to_elm()
        } else {
            super::master::region::SenteJin::to_elm()
        }
    }

    /**
     * 表示
     *
     * 後手から見た盤を表示するぜ☆（＾～＾）
     * デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
     */
    pub fn kaku_ky(&self, num: &KyNums) -> String {
        let ky = match *num {
            KyNums::Current => &self.ky,
            KyNums::Start => &self.ky0,
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
            ky.get_piece_struct_by_sq(&Square::from_umasu(19)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(29)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(39)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(49)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(59)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(69)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(79)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(89)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(99)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(18)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(28)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(38)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(48)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(58)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(68)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(78)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(88)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(98)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(17)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(27)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(37)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(47)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(57)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(67)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(77)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(87)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(97)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(16)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(26)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(36)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(46)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(56)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(66)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(76)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(86)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(96)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(15)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(25)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(35)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(45)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(55)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(65)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(75)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(85)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(95)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(14)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(24)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(34)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(44)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(54)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(64)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(74)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(84)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(94)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(13)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(23)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(33)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(43)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(53)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(63)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(73)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(83)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(93)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(12)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(22)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(32)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(42)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(52)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(62)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(72)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(82)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(92)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(11)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(21)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(31)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(41)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(51)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(61)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(71)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(81)).piece(),
            ky.get_piece_struct_by_sq(&Square::from_umasu(91)).piece(),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            ky.mg[Piece::Rook1 as usize],
            ky.mg[Piece::Bishop1 as usize],
            ky.mg[Piece::Gold1 as usize],
            ky.mg[Piece::Silver1 as usize],
            ky.mg[Piece::Knight1 as usize],
            ky.mg[Piece::Lance1 as usize],
            ky.mg[Piece::Pawn1 as usize],
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            ky.mg[Piece::Rook2 as usize],
            ky.mg[Piece::Bishop2 as usize],
            ky.mg[Piece::Gold2 as usize],
            ky.mg[Piece::Silver2 as usize],
            ky.mg[Piece::Knight2 as usize],
            ky.mg[Piece::Lance2 as usize],
            ky.mg[Piece::Pawn2 as usize],
            self.get_teme(),
            self.get_teban(&Person::Ji),
            self.count_same_ky()
        )
    }

    /**
     * 表示
     */
    pub fn kaku_number_board(&self, sn: &Phase, km: &Piece) -> String {
        let nb = match *sn {
            Phase::Owari => &self.kiki_su_by_km[PieceStruct::from_piece(&km).serial_piece_number()],
            _ => &self.kiki_su_by_sn[sn_to_num(&sn)],
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
    pub fn do_ss(&mut self, ss: &Sasite) {
        // もう入っているかも知れないが、棋譜に入れる☆
        let teme = self.teme;
        self.kifu[teme] = (*ss).clone();
        let cap = self.ky.do_sasite(&self.get_teban(&Person::Ji), ss);
        self.set_cap(teme, cap.clone());

        // 局面ハッシュを作り直す
        let ky_hash = self.create_ky1_hash();
        self.set_ky1_hash(ky_hash);

        self.teme += 1;
    }

    pub fn undo_ss(&mut self) -> bool {
        if 0 < self.teme {
            // 棋譜から読取、手目も減る
            self.teme -= 1;
            let sn = self.get_teban(&Person::Ji);
            let ss = &self.get_sasite().clone();
            self.ky.undo_sasite(&sn, &ss, &self.cap[self.teme]);
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }

    pub fn remake_visions(&mut self) {
        for sn in SN_ARRAY.iter() {
            // 全部忘れる☆（＾～＾）
            self.vision_tree_by_sn[sn_to_num(sn)].clear();
        }
    }

    /**
     * 初期局面ハッシュを作り直す
     */
    pub fn create_ky0_hash(&self) -> u64 {
        let mut hash = self.ky0.create_hash(&self);

        // 手番ハッシュ（後手固定）
        hash ^= self.ky_hash_seed.sn[SN_GO];

        hash
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_ky1_hash(&self) -> u64 {
        let mut hash = self.ky.create_hash(&self);

        // 手番ハッシュ
        use super::master::phase::Phase::*;
        match self.get_teban(&Person::Ji) {
            Sen => hash ^= self.ky_hash_seed.sn[SN_SEN],
            Go => hash ^= self.ky_hash_seed.sn[SN_GO],
            _ => {}
        }

        hash
    }

    /**
     * 千日手を調べるために、
     * 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
     */
    pub fn count_same_ky(&self) -> i8 {
        if self.get_teme() < 1 {
            return 0;
        }

        let mut count = 0;
        let last_teme = self.get_teme() - 1;
        let new_teme = self.get_teme();
        // g_writeln( &format!( "Ｃount_same_ky last_teme={} new_teme={}", last_teme ,new_teme ) );
        for i_teme in 0..new_teme {
            let t = last_teme - i_teme;
            // g_writeln( &format!( "i_teme={} t={}", i_teme, t ) );
            if self.ky_hash[t] == self.ky_hash[last_teme] {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.ky0_hash == self.ky_hash[last_teme] {
            count += 1;
        }

        count
    }

    pub fn piece_struct_master(&self) -> &PieceStructMaster {
        &self.piece_struct_master
    }
}
