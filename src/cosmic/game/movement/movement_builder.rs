//!
//! USIプロトコル
//!
use crate::cosmic::smart::piece_type::PieceType;
use crate::cosmic::smart::piece_type::*;
use crate::cosmic::smart::square::*;
use crate::law::cryptographic::cu_conv_controller::{
    num_to_lower_case, pop_bool_from_hash, pop_sq_from_hash, push_bool_to_hash, push_sq_to_hash,
};
use crate::law::diagnostic::cu_asserts_controller::*;
use crate::law::speed_of_light::*;
use std::fmt;

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
