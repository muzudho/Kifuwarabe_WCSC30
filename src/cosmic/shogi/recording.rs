//!
//! History (棋譜),
//! Movement (指し手).
//!
use crate::cosmic::shogi::state::{Person, Phase};
use crate::cosmic::smart::features::{
    pop_piece_type_from_hash, push_piece_type_to_hash, PieceType,
};
use crate::cosmic::smart::square::{AbsoluteAddress, Address};
use crate::cosmic::toy_box::Piece;
use crate::law::cryptographic::{
    num_to_lower_case, pop_bool_from_hash, pop_sq_from_hash, push_bool_to_hash, push_sq_to_hash,
};
use crate::law::diagnostic::*;
use crate::law::speed_of_light::SpeedOfLight;
use std::fmt;

/// 手目数。何手目まで指せるか。
/// 棋譜を残す配列のサイズでもある。
/// 大会ルールで 320手が上限なので、終端子として投了を１個入れておけるように +1 する。
pub const PLY_LN: usize = 321;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: i8 = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    pub ply: i16,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_LN],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_LN],
    /// 取った駒
    pub captured_pieces: [Option<Piece>; PLY_LN],
}
impl Default for History {
    fn default() -> History {
        History {
            ply: 0,
            movements: [Movement::default(); PLY_LN],
            position_hashs: [0; PLY_LN],
            /// 取った駒
            captured_pieces: [None; PLY_LN],
        }
    }
}
impl History {
    /// 手番
    pub fn get_phase(&self, person: Person) -> Phase {
        use crate::cosmic::shogi::state::Person::*;
        match person {
            Friend => {
                // 手番
                if self.ply % 2 == 0 {
                    Phase::First
                } else {
                    Phase::Second
                }
            }
            _Opponent => {
                // 相手番
                if self.ply % 2 == 0 {
                    Phase::Second
                } else {
                    Phase::First
                }
            }
        }
    }
}

/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy)]
pub struct Movement {
    // 移動元升。打った場合は 0。
    pub source: AbsoluteAddress,
    // 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
    pub destination: AbsoluteAddress,
    // 移動後に成るなら真
    pub promote: bool,
    // 打の場合、打った駒種類
    pub drop: Option<PieceType>,
}
impl Default for Movement {
    fn default() -> Self {
        Movement {
            source: Address::default().abs(),
            destination: Address::default().abs(),
            promote: false,
            drop: None,
        }
    }
}
impl Movement {
    pub fn from_hash(hash: u64) -> Movement {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, src52) = pop_sq_from_hash(hash);
        let (hash, dst53) = pop_sq_from_hash(hash);
        let (hash, pro54) = pop_bool_from_hash(hash);
        let (_hash, drop55) = pop_piece_type_from_hash(hash);
        Movement {
            source: src52,
            destination: dst53,
            promote: pro54,
            drop: drop55,
        }
    }

    pub fn to_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_piece_type_to_hash(hash, self.drop, speed_of_light);
        hash = push_bool_to_hash(hash, self.promote);
        hash = push_sq_to_hash(hash, &self.destination);
        push_sq_to_hash(hash, &self.source)
    }

    pub fn resign(&self) -> bool {
        self.destination.is_none()
    }
}
impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if self.resign() {
            return write!(f, "resign");
        }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_in_board_as_absolute(&self.destination, "Movement-display");
        let (dx, dy) = self.destination.to_file_rank();

        if self.source.is_drop() {
            use crate::cosmic::smart::features::PieceType::*;
            write!(
                f,
                "{}*{}{}{}",
                if let Some(drp) = self.drop {
                    match drp {
                        Rook => "R",
                        Bishop => "B",
                        Gold => "G",
                        Silver => "S",
                        Knight => "N",
                        Lance => "L",
                        Pawn => "P",
                        _ => "?",
                    }
                } else {
                    "?"
                },
                dx,
                num_to_lower_case(dy),
                if self.promote { "+" } else { "" }
            )
        } else {
            let (sx, sy) = if self.source.is_none() {
                // エラー・データも表示したい
                (0, 0)
            } else {
                assert_in_board_as_absolute(&self.source, "Movement-display-2");
                self.source.to_file_rank()
            };
            write!(
                f,
                "{}{}{}{}{}",
                sx,
                num_to_lower_case(sy),
                dx,
                num_to_lower_case(dy),
                if self.promote { "+" } else { "" }
            )
        }
    }
}
impl fmt::Debug for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Movement({}{}{}{})",
            self.source.address(),
            self.destination.address(),
            self.promote,
            if let Some(drp) = self.drop {
                format!("{}", drp)
            } else {
                "-".to_string()
            }
        )
    }
}
