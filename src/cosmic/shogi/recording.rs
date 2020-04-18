//!
//! History (棋譜),
//! Movement (指し手).
//!
use crate::cosmic::shogi::state::{Person, Phase};
use crate::cosmic::smart::piece_type::PieceType;
use crate::cosmic::smart::piece_type::*;
use crate::cosmic::smart::square::*;
use crate::cosmic::toy_box::Piece;
use crate::law::cryptographic::cu_conv_controller::{
    num_to_lower_case, pop_bool_from_hash, pop_sq_from_hash, push_bool_to_hash, push_sq_to_hash,
};
use crate::law::diagnostic::cu_asserts_controller::*;
use crate::law::speed_of_light::*;
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
    pub fn get_phase(&self, person: &Person) -> Phase {
        use crate::cosmic::shogi::state::Person::*;
        match *person {
            // None => Phase::None,
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
    pub source: Square,
    // 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
    pub destination: Square,
    // 移動後に成るなら真
    pub promote: bool,
    // 打の場合、打った駒種類
    pub drop: Option<PieceType>,
}
impl Default for Movement {
    fn default() -> Self {
        Movement::new(&MovementBuilder::default())
    }
}
impl Movement {
    pub fn new(movement_dto: &MovementBuilder) -> Self {
        Movement {
            source: movement_dto.src.clone(),
            destination: movement_dto.dst.clone(),
            promote: movement_dto.pro,
            drop: movement_dto.drop,
        }
    }
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

    /*
    pub fn to_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_piece_type_to_hash(hash, self.drop, speed_of_light);
        hash = push_bool_to_hash(hash, self.promote);
        hash = push_sq_to_hash(hash, &self.destination);
        push_sq_to_hash(hash, &self.source)
    }
    */

    /// 考えた結果、指し手が考え付いていれば真。
    pub fn exists(&self) -> bool {
        self.destination.address != SQUARE_NONE
    }
}
impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() {
            return write!(f, "resign");
        }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_in_board_as_absolute(self.destination.address, "Movement-display");
        let (dx, dy) = self.destination.to_file_rank();

        if self.source.address == SQUARE_DROP {
            use crate::cosmic::smart::piece_type::PieceType::*;
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
            let (sx, sy) = if self.source.address == SQUARE_NONE {
                // エラー・データも表示したい
                (0, 0)
            } else {
                assert_in_board_as_absolute(self.source.address, "Movement-display-2");
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
            "GPMovementVo({}{}{}{})",
            self.source.address,
            self.destination.address,
            self.promote,
            if let Some(drp) = self.drop {
                format!("{}", drp)
            } else {
                "-".to_string()
            }
        )
    }
}

/// Movement. (指し手)
/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
///
#[derive(Clone)]
pub struct MovementBuilder {
    // 移動元升。打った場合は 0。
    pub src: Square,
    // 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
    pub dst: Square,
    // 移動後に成るなら真
    pub pro: bool,
    // 打の場合、打った駒種類
    pub drop: Option<PieceType>,
}
impl Default for MovementBuilder {
    fn default() -> MovementBuilder {
        MovementBuilder {
            src: Square::from_address(0),
            dst: Square::from_address(0),
            pro: false,
            drop: None,
        }
    }
}
impl MovementBuilder {
    /*
    pub fn clear(&mut self) {
        self.src = Square::from_address(0);
        self.dst = Square::from_address(0);
        self.pro = false;
        self.drop = None;
    }
    */
    pub fn to_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_piece_type_to_hash(hash, self.drop, speed_of_light);
        hash = push_bool_to_hash(hash, self.pro);
        hash = push_sq_to_hash(hash, &self.dst);
        push_sq_to_hash(hash, &self.src)
    }
    pub fn from_hash(hash: u64) -> MovementBuilder {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, src52) = pop_sq_from_hash(hash);
        let (hash, dst53) = pop_sq_from_hash(hash);
        let (hash, pro54) = pop_bool_from_hash(hash);
        let (_hash, drop55) = pop_piece_type_from_hash(hash);
        MovementBuilder {
            src: src52,
            dst: dst53,
            pro: pro54,
            drop: drop55,
        }
    }

    pub fn resign(&self) -> bool {
        self.dst.address == SQUARE_NONE
    }
}
impl fmt::Display for MovementBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if self.resign() {
            return write!(f, "resign");
        }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_in_board_as_absolute(self.dst.address, "Ｓasite Ｄisplay1");
        let (dx, dy) = self.dst.to_file_rank();

        if self.src.address == SQUARE_DROP {
            use crate::cosmic::smart::piece_type::PieceType::*;
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
                if self.pro { "+" } else { "" }
            )
        } else {
            let (sx, sy) = if self.src.address == SQUARE_NONE {
                // エラー・データも表示したい
                (0, 0)
            } else {
                assert_in_board_as_absolute(self.src.address, "Ｓasite Ｄisplay＜その２＞");
                self.src.to_file_rank()
            };
            write!(
                f,
                "{}{}{}{}{}",
                sx,
                num_to_lower_case(sy),
                dx,
                num_to_lower_case(dy),
                if self.pro { "+" } else { "" }
            )
        }
    }
}
impl fmt::Debug for MovementBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MLMovementDto({}{}{}{})",
            self.src.address,
            self.dst.address,
            self.pro,
            if let Some(drp) = self.drop {
                format!("{}", drp)
            } else {
                "-".to_string()
            }
        )
    }
}
