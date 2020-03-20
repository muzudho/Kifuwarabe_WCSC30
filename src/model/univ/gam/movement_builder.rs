//!
//! USIプロトコル
//!
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::common_use::cu_conv_controller::*;
use crate::model::univ::gam::piece_type::GPPieceTypeVo;
use crate::model::univ::gam::piece_type::*;
use crate::model::univ::gam::square::*;
use crate::model::vo::main_loop::ml_speed_of_light_vo::MLSpeedOfLightVo;
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
    pub drop: GPPieceTypeVo,
}
impl Default for MovementBuilder {
    fn default() -> MovementBuilder {
        MovementBuilder {
            src: Square::from_usquare(0),
            dst: Square::from_usquare(0),
            pro: false,
            drop: GPPieceTypeVo::KaraPieceType,
        }
    }
}
impl MovementBuilder {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.src = Square::from_usquare(0);
        self.dst = Square::from_usquare(0);
        self.pro = false;
        self.drop = GPPieceTypeVo::KaraPieceType;
    }
    pub fn to_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
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

    /**
     * 考えた結果、指し手が考え付いていれば真。
     */
    pub fn exists(&self) -> bool {
        self.dst.to_usquare() != SQUARE_NONE
    }
}
impl fmt::Display for MovementBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() {
            return write!(f, "resign");
        }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_banjo_sq(&self.dst, "Ｓasite Ｄisplay");
        let (dx, dy) = self.dst.to_file_rank();

        if self.src.to_usquare() == SQUARE_DROP {
            use crate::model::univ::gam::piece_type::GPPieceTypeVo::*;
            write!(
                f,
                "{}*{}{}{}",
                match self.drop {
                    Rook => {
                        "R"
                    }
                    Bishop => {
                        "B"
                    }
                    Gold => {
                        "G"
                    }
                    Silver => {
                        "S"
                    }
                    Knight => {
                        "N"
                    }
                    Lance => {
                        "L"
                    }
                    Pawn => {
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
            let (sx, sy) = if self.src.to_usquare() == SQUARE_NONE {
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
impl fmt::Debug for MovementBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MLMovementDto({}{}{}{})",
            self.src.to_usquare(),
            self.dst.to_usquare(),
            self.pro,
            self.drop
        )
    }
}
