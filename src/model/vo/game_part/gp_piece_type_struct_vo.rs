//!
//! 駒種類。
//!
use super::gp_piece_type_vo::GPPieceTypeVo;

pub struct PieceTypeStructVo {
    /// 打てる駒種類か。
    pub can_drop: bool,
}
impl PieceTypeStructVo {
    pub fn from_piece_type(piece_type: GPPieceTypeVo) -> Self {
        use super::gp_piece_type_vo::GPPieceTypeVo::*;
        match piece_type {
            King => PieceTypeStructVo { can_drop: false },
            Rook => PieceTypeStructVo { can_drop: true },
            Bishop => PieceTypeStructVo { can_drop: true },
            Gold => PieceTypeStructVo { can_drop: true },
            Silver => PieceTypeStructVo { can_drop: true },
            Knight => PieceTypeStructVo { can_drop: true },
            Lance => PieceTypeStructVo { can_drop: true },
            Pawn => PieceTypeStructVo { can_drop: true },
            Dragon => PieceTypeStructVo { can_drop: false },
            Horse => PieceTypeStructVo { can_drop: false },
            PromotedSilver => PieceTypeStructVo { can_drop: false },
            PromotedKnight => PieceTypeStructVo { can_drop: false },
            PromotedLance => PieceTypeStructVo { can_drop: false },
            PromotedPawn => PieceTypeStructVo { can_drop: false },
            Kara => PieceTypeStructVo { can_drop: false },
            Owari => PieceTypeStructVo { can_drop: false },
        }
    }
}
