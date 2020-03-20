//!
//! Value object.
//! Movement. (指し手)
//!
use super::gp_square_vo::*;
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::common_use::cu_conv_controller::*;
use crate::model::dto::main_loop::ml_movement_dto::MLMovementDto;
use crate::model::univ::gam::piece_type::GPPieceTypeVo;
use crate::model::univ::gam::piece_type::*;
use crate::model::vo::main_loop::ml_speed_of_light_vo::MLSpeedOfLightVo;
use std::fmt;

/// Movement. (指し手)
/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy)]
pub struct GPMovementVo {
    // 移動元升。打った場合は 0。
    pub source: Square,
    // 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
    pub destination: Square,
    // 移動後に成るなら真
    pub promote: bool,
    // 打の場合、打った駒種類
    pub drop: GPPieceTypeVo,
}
impl Default for GPMovementVo {
    fn default() -> Self {
        GPMovementVo::new(&MLMovementDto::default())
    }
}
impl GPMovementVo {
    pub fn new(movement_dto: &MLMovementDto) -> Self {
        GPMovementVo {
            source: movement_dto.src.clone(),
            destination: movement_dto.dst.clone(),
            promote: movement_dto.pro,
            drop: movement_dto.drop,
        }
    }
    pub fn from_hash(hash: u64) -> GPMovementVo {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, src52) = pop_sq_from_hash(hash);
        let (hash, dst53) = pop_sq_from_hash(hash);
        let (hash, pro54) = pop_bool_from_hash(hash);
        let (_hash, drop55) = pop_piece_type_from_hash(hash);
        GPMovementVo {
            source: src52,
            destination: dst53,
            promote: pro54,
            drop: drop55,
        }
    }
    pub fn to_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_piece_type_to_hash(hash, self.drop, speed_of_light);
        hash = push_bool_to_hash(hash, self.promote);
        hash = push_sq_to_hash(hash, &self.destination);
        push_sq_to_hash(hash, &self.source)
    }

    /// 考えた結果、指し手が考え付いていれば真。
    pub fn exists(&self) -> bool {
        self.destination.to_usquare() != SQUARE_NONE
    }
}
impl fmt::Display for GPMovementVo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() {
            return write!(f, "resign");
        }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_banjo_sq(&self.destination, "Movement-display");
        let (dx, dy) = self.destination.to_file_rank();

        if self.source.to_usquare() == SQUARE_DROP {
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
                if self.promote { "+" } else { "" }
            )
        } else {
            let (sx, sy) = if self.source.to_usquare() == SQUARE_NONE {
                // エラー・データも表示したい
                (0, 0)
            } else {
                assert_banjo_sq(&self.source, "Movement-display-2");
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
impl fmt::Debug for GPMovementVo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GPMovementVo({}{}{}{})",
            self.source.to_usquare(),
            self.destination.to_usquare(),
            self.promote,
            self.drop
        )
    }
}
