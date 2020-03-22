//!
//! 駒種類。
//!
use crate::model::univ::gam::misc::piece_type::PieceType;

pub const NONE_SERIAL_PIECE_TYPE_NUMBER: u64 = 14;

pub struct PieceTypeStruct {
    /// 配列のインデックス用☆（＾～＾）
    pub serial_piece_number: usize,

    /// 成れる駒種類か。
    pub can_promote: bool,

    /// 打てる駒種類か。
    pub can_drop: bool,

    /// スライダー（長い利きのある駒種類）か☆（＾～＾）
    /// 合い駒で、進路を防ぎえる可能性があれば真
    pub slider: bool,
}
impl PieceTypeStruct {
    pub fn from_piece_type(piece_type: PieceType) -> Self {
        use crate::model::univ::gam::misc::piece_type::PieceType::*;
        match piece_type {
            King => PieceTypeStruct {
                serial_piece_number: 0,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            Rook => PieceTypeStruct {
                serial_piece_number: 1,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Bishop => PieceTypeStruct {
                serial_piece_number: 2,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Gold => PieceTypeStruct {
                serial_piece_number: 3,
                can_promote: false,
                can_drop: true,
                slider: false,
            },
            Silver => PieceTypeStruct {
                serial_piece_number: 4,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Knight => PieceTypeStruct {
                serial_piece_number: 5,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Lance => PieceTypeStruct {
                serial_piece_number: 6,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Pawn => PieceTypeStruct {
                serial_piece_number: 7,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Dragon => PieceTypeStruct {
                serial_piece_number: 8,
                can_promote: false,
                can_drop: false,
                slider: true,
            },
            Horse => PieceTypeStruct {
                serial_piece_number: 9,
                can_promote: false,
                can_drop: false,
                slider: true,
            },
            PromotedSilver => PieceTypeStruct {
                serial_piece_number: 10,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedKnight => PieceTypeStruct {
                serial_piece_number: 11,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedLance => PieceTypeStruct {
                serial_piece_number: 12,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedPawn => PieceTypeStruct {
                serial_piece_number: 13,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
        }
    }
}
