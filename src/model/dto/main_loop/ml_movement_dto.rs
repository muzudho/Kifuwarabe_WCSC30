//!
//! USIプロトコル
//!
use super::super::super::super::controller::common_part::cp_asserts_controller::*;
use super::super::super::super::controller::common_part::cp_conv_controller::*;
use super::super::super::super::model::vo::other_part::op_piece_type_vo::PieceType;
use super::super::super::super::model::vo::other_part::op_square_vo::*;
use std::fmt;

/// 指し手
/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
///
#[derive(Clone)]
pub struct MLMovementDto {
    // 移動元升。打った場合は 0。
    pub src: Square,
    // 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
    pub dst: Square,
    // 移動後に成るなら真
    pub pro: bool,
    // 打の場合、打った駒種類
    pub drop: PieceType,
}
impl MLMovementDto {
    pub fn new() -> MLMovementDto {
        MLMovementDto {
            src: Square::from_umasu(0),
            dst: Square::from_umasu(0),
            pro: false,
            drop: PieceType::Kara,
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.src = Square::from_umasu(0);
        self.dst = Square::from_umasu(0);
        self.pro = false;
        self.drop = PieceType::Kara;
    }
    pub fn to_hash(&self) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_kms_to_hash(hash, &self.drop);
        hash = push_bool_to_hash(hash, self.pro);
        hash = push_sq_to_hash(hash, &self.dst);
        push_sq_to_hash(hash, &self.src)
    }
    pub fn from_hash(hash: u64) -> MLMovementDto {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, src) = pop_sq_from_hash(hash);
        let (hash, dst) = pop_sq_from_hash(hash);
        let (hash, pro) = pop_bool_from_hash(hash);
        let (_hash, drop) = pop_kms_from_hash(hash);
        MLMovementDto {
            src: src,
            dst: dst,
            pro: pro,
            drop: drop,
        }
    }

    /**
     * 考えた結果、指し手が考え付いていれば真。
     */
    pub fn exists(&self) -> bool {
        self.dst.to_umasu() != MASU_0
    }
}
impl fmt::Display for MLMovementDto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() {
            return write!(f, "resign");
        }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_banjo_sq(&self.dst, "Ｓasite Ｄisplay");
        let (dx, dy) = self.dst.to_file_rank();

        if self.src.to_umasu() == SS_SRC_DA {
            use super::super::super::super::model::vo::other_part::op_piece_type_vo::PieceType::*;
            write!(
                f,
                "{}*{}{}{}",
                match self.drop {
                    K => {
                        "R"
                    }
                    Z => {
                        "B"
                    }
                    I => {
                        "G"
                    }
                    N => {
                        "S"
                    }
                    U => {
                        "N"
                    }
                    S => {
                        "L"
                    }
                    H => {
                        "P"
                    }
                    _ => {
                        "?"
                    }
                },
                dx,
                num_to_lower_case(dy),
                if self.pro { "+" } else { "" }
            )
        } else {
            let (sx, sy) = if self.src.to_umasu() == MASU_0 {
                // エラー・データも表示したい
                (0, 0)
            } else {
                assert_banjo_sq(&self.src, "Ｓasite Ｄisplay＜その２＞");
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
impl fmt::Debug for MLMovementDto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MLMovementDto({}{}{}{})",
            self.src.to_umasu(),
            self.dst.to_umasu(),
            self.pro,
            self.drop
        )
    }
}
