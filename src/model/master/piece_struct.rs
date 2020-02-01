use super::piece::Piece;

pub struct PieceStruct {
    piece: Piece,
    /// 駒→成駒　（成れない駒は、そのまま）
    promoted: Piece,
}
impl PieceStruct {
    pub fn from_piece(p: &Piece) -> Self {
        use super::piece::Piece::*;
        match *p {
            King1 => PieceStruct {
                piece: King1,
                promoted: King1,
            },
            Rook1 => PieceStruct {
                piece: Rook1,
                promoted: PromotedRook1,
            },
            Bishop1 => PieceStruct {
                piece: Bishop1,
                promoted: PromotedBishop1,
            },
            Gold1 => PieceStruct {
                piece: Gold1,
                promoted: Gold1,
            },
            Silver1 => PieceStruct {
                piece: Silver1,
                promoted: PromotedSilver1,
            },
            Knight1 => PieceStruct {
                piece: Knight1,
                promoted: PromotedKnight1,
            },
            Lance1 => PieceStruct {
                piece: Lance1,
                promoted: PromotedLance1,
            },
            Pawn1 => PieceStruct {
                piece: Pawn1,
                promoted: PromotedPawn1,
            },
            PromotedRook1 => PieceStruct {
                piece: PromotedRook1,
                promoted: PromotedRook1,
            },
            PromotedBishop1 => PieceStruct {
                piece: PromotedBishop1,
                promoted: PromotedBishop1,
            },
            PromotedSilver1 => PieceStruct {
                piece: PromotedSilver1,
                promoted: PromotedSilver1,
            },
            PromotedKnight1 => PieceStruct {
                piece: PromotedKnight1,
                promoted: PromotedKnight1,
            },
            PromotedLance1 => PieceStruct {
                piece: PromotedLance1,
                promoted: PromotedLance1,
            },
            PromotedPawn1 => PieceStruct {
                piece: PromotedPawn1,
                promoted: PromotedPawn1,
            },
            King2 => PieceStruct {
                piece: King2,
                promoted: King2,
            },
            Rook2 => PieceStruct {
                piece: Rook2,
                promoted: PromotedRook2,
            },
            Bishop2 => PieceStruct {
                piece: Bishop2,
                promoted: PromotedBishop2,
            },
            Gold2 => PieceStruct {
                piece: Gold2,
                promoted: Gold2,
            },
            Silver2 => PieceStruct {
                piece: Silver2,
                promoted: PromotedSilver2,
            },
            Knight2 => PieceStruct {
                piece: Knight2,
                promoted: PromotedKnight2,
            },
            Lance2 => PieceStruct {
                piece: Lance2,
                promoted: PromotedLance2,
            },
            Pawn2 => PieceStruct {
                piece: Pawn2,
                promoted: PromotedPawn2,
            },
            PromotedRook2 => PieceStruct {
                piece: PromotedRook2,
                promoted: PromotedRook2,
            },
            PromotedBishop2 => PieceStruct {
                piece: PromotedBishop2,
                promoted: PromotedBishop2,
            },
            PromotedSilver2 => PieceStruct {
                piece: PromotedSilver2,
                promoted: PromotedSilver2,
            },
            PromotedKnight2 => PieceStruct {
                piece: PromotedKnight2,
                promoted: PromotedKnight2,
            },
            PromotedLance2 => PieceStruct {
                piece: PromotedLance2,
                promoted: PromotedLance2,
            },
            PromotedPawn2 => PieceStruct {
                piece: PromotedPawn2,
                promoted: PromotedPawn2,
            },
            Kara => PieceStruct {
                piece: Kara,
                promoted: Kara,
            },
            Owari => PieceStruct {
                piece: Owari,
                promoted: Owari,
            },
        }
    }

    pub fn piece(self) -> Piece {
        self.piece
    }

    pub fn promote(self) -> Piece {
        self.promoted
    }
}
