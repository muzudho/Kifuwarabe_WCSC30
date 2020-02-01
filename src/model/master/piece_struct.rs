use super::piece::Piece;

pub struct PieceStruct {
    piece: Piece,
    /// 駒→成駒　（成れない駒は、そのまま）
    promoted: Piece,
    /// 先後付き駒の配列のインデックス
    piece_serial_number: usize,
}
impl PieceStruct {
    pub fn from_piece(p: &Piece) -> Self {
        use super::piece::Piece::*;
        match *p {
            King1 => PieceStruct {
                piece: King1,
                promoted: King1,
                piece_serial_number: 0,
            },
            Rook1 => PieceStruct {
                piece: Rook1,
                promoted: PromotedRook1,
                piece_serial_number: 1,
            },
            Bishop1 => PieceStruct {
                piece: Bishop1,
                promoted: PromotedBishop1,
                piece_serial_number: 2,
            },
            Gold1 => PieceStruct {
                piece: Gold1,
                promoted: Gold1,
                piece_serial_number: 3,
            },
            Silver1 => PieceStruct {
                piece: Silver1,
                promoted: PromotedSilver1,
                piece_serial_number: 4,
            },
            Knight1 => PieceStruct {
                piece: Knight1,
                promoted: PromotedKnight1,
                piece_serial_number: 5,
            },
            Lance1 => PieceStruct {
                piece: Lance1,
                promoted: PromotedLance1,
                piece_serial_number: 6,
            },
            Pawn1 => PieceStruct {
                piece: Pawn1,
                promoted: PromotedPawn1,
                piece_serial_number: 7,
            },
            PromotedRook1 => PieceStruct {
                piece: PromotedRook1,
                promoted: PromotedRook1,
                piece_serial_number: 8,
            },
            PromotedBishop1 => PieceStruct {
                piece: PromotedBishop1,
                promoted: PromotedBishop1,
                piece_serial_number: 9,
            },
            PromotedSilver1 => PieceStruct {
                piece: PromotedSilver1,
                promoted: PromotedSilver1,
                piece_serial_number: 10,
            },
            PromotedKnight1 => PieceStruct {
                piece: PromotedKnight1,
                promoted: PromotedKnight1,
                piece_serial_number: 11,
            },
            PromotedLance1 => PieceStruct {
                piece: PromotedLance1,
                promoted: PromotedLance1,
                piece_serial_number: 12,
            },
            PromotedPawn1 => PieceStruct {
                piece: PromotedPawn1,
                promoted: PromotedPawn1,
                piece_serial_number: 13,
            },
            King2 => PieceStruct {
                piece: King2,
                promoted: King2,
                piece_serial_number: 14,
            },
            Rook2 => PieceStruct {
                piece: Rook2,
                promoted: PromotedRook2,
                piece_serial_number: 15,
            },
            Bishop2 => PieceStruct {
                piece: Bishop2,
                promoted: PromotedBishop2,
                piece_serial_number: 16,
            },
            Gold2 => PieceStruct {
                piece: Gold2,
                promoted: Gold2,
                piece_serial_number: 17,
            },
            Silver2 => PieceStruct {
                piece: Silver2,
                promoted: PromotedSilver2,
                piece_serial_number: 18,
            },
            Knight2 => PieceStruct {
                piece: Knight2,
                promoted: PromotedKnight2,
                piece_serial_number: 19,
            },
            Lance2 => PieceStruct {
                piece: Lance2,
                promoted: PromotedLance2,
                piece_serial_number: 20,
            },
            Pawn2 => PieceStruct {
                piece: Pawn2,
                promoted: PromotedPawn2,
                piece_serial_number: 21,
            },
            PromotedRook2 => PieceStruct {
                piece: PromotedRook2,
                promoted: PromotedRook2,
                piece_serial_number: 22,
            },
            PromotedBishop2 => PieceStruct {
                piece: PromotedBishop2,
                promoted: PromotedBishop2,
                piece_serial_number: 23,
            },
            PromotedSilver2 => PieceStruct {
                piece: PromotedSilver2,
                promoted: PromotedSilver2,
                piece_serial_number: 24,
            },
            PromotedKnight2 => PieceStruct {
                piece: PromotedKnight2,
                promoted: PromotedKnight2,
                piece_serial_number: 25,
            },
            PromotedLance2 => PieceStruct {
                piece: PromotedLance2,
                promoted: PromotedLance2,
                piece_serial_number: 26,
            },
            PromotedPawn2 => PieceStruct {
                piece: PromotedPawn2,
                promoted: PromotedPawn2,
                piece_serial_number: 27,
            },
            Kara => PieceStruct {
                piece: Kara,
                promoted: Kara,
                piece_serial_number: 28,
            },
            Owari => PieceStruct {
                piece: Owari,
                promoted: Owari,
                piece_serial_number: 29,
            },
        }
    }

    pub fn piece(self) -> Piece {
        self.piece
    }

    pub fn promote(self) -> Piece {
        self.promoted
    }

    pub fn piece_serial_number(self) -> usize {
        self.piece_serial_number
    }
}
