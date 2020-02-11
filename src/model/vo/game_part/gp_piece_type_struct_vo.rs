//!
//! 駒種類。
//!
use super::gp_piece_type_vo::GPPieceTypeVo;

pub struct PieceTypeStructVo {
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
impl PieceTypeStructVo {
    pub fn from_piece_type(piece_type: GPPieceTypeVo) -> Self {
        use super::gp_piece_type_vo::GPPieceTypeVo::*;
        match piece_type {
            King => PieceTypeStructVo {
                serial_piece_number: 0,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            Rook => PieceTypeStructVo {
                serial_piece_number: 1,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Bishop => PieceTypeStructVo {
                serial_piece_number: 2,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Gold => PieceTypeStructVo {
                serial_piece_number: 3,
                can_promote: false,
                can_drop: true,
                slider: false,
            },
            Silver => PieceTypeStructVo {
                serial_piece_number: 4,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Knight => PieceTypeStructVo {
                serial_piece_number: 5,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Lance => PieceTypeStructVo {
                serial_piece_number: 6,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Pawn => PieceTypeStructVo {
                serial_piece_number: 7,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Dragon => PieceTypeStructVo {
                serial_piece_number: 8,
                can_promote: false,
                can_drop: false,
                slider: true,
            },
            Horse => PieceTypeStructVo {
                serial_piece_number: 9,
                can_promote: false,
                can_drop: false,
                slider: true,
            },
            PromotedSilver => PieceTypeStructVo {
                serial_piece_number: 10,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedKnight => PieceTypeStructVo {
                serial_piece_number: 11,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedLance => PieceTypeStructVo {
                serial_piece_number: 12,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedPawn => PieceTypeStructVo {
                serial_piece_number: 13,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            KaraPieceType => PieceTypeStructVo {
                serial_piece_number: 14,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            OwariPieceType => PieceTypeStructVo {
                serial_piece_number: 15,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
        }
    }
}