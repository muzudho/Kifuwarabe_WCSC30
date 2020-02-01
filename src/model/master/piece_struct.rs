use super::phase::Phase;
use super::piece::Piece;
use super::piece_type::PieceType;

pub struct PieceStruct {
    piece: Piece,
    /// 先後、駒種類。
    phase_piece_type: (Phase, PieceType),
    /// 駒→成駒　（成れない駒は、そのまま）
    promoted: Piece,
    /// 成駒→駒
    demoted: Piece,
    /// 先後付き駒　を　持ち駒種類　へ変換。
    /// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
    captured: Piece,
    /// 先後付き駒の配列のインデックス
    piece_serial_number: usize,
}
impl PieceStruct {
    pub fn from_piece(p: &Piece) -> Self {
        use super::phase::Phase::*;
        use super::piece::Piece::*;
        use super::piece_type::PieceType::*;
        match *p {
            King1 => PieceStruct {
                piece: King1,
                phase_piece_type: (Sen, R),
                promoted: King1,
                demoted: King1,
                captured: Piece::Owari,
                piece_serial_number: 0,
            },
            Rook1 => PieceStruct {
                piece: Rook1,
                phase_piece_type: (Sen, K),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                piece_serial_number: 1,
            },
            Bishop1 => PieceStruct {
                piece: Bishop1,
                phase_piece_type: (Sen, Z),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                piece_serial_number: 2,
            },
            Gold1 => PieceStruct {
                piece: Gold1,
                phase_piece_type: (Sen, I),
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                piece_serial_number: 3,
            },
            Silver1 => PieceStruct {
                piece: Silver1,
                phase_piece_type: (Sen, N),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                piece_serial_number: 4,
            },
            Knight1 => PieceStruct {
                piece: Knight1,
                phase_piece_type: (Sen, U),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                piece_serial_number: 5,
            },
            Lance1 => PieceStruct {
                piece: Lance1,
                phase_piece_type: (Sen, S),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                piece_serial_number: 6,
            },
            Pawn1 => PieceStruct {
                piece: Pawn1,
                phase_piece_type: (Sen, H),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                piece_serial_number: 7,
            },
            PromotedRook1 => PieceStruct {
                piece: PromotedRook1,
                phase_piece_type: (Sen, PK),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                piece_serial_number: 8,
            },
            PromotedBishop1 => PieceStruct {
                piece: PromotedBishop1,
                phase_piece_type: (Sen, PZ),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                piece_serial_number: 9,
            },
            PromotedSilver1 => PieceStruct {
                piece: PromotedSilver1,
                phase_piece_type: (Sen, PN),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                piece_serial_number: 10,
            },
            PromotedKnight1 => PieceStruct {
                piece: PromotedKnight1,
                phase_piece_type: (Sen, PU),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                piece_serial_number: 11,
            },
            PromotedLance1 => PieceStruct {
                piece: PromotedLance1,
                phase_piece_type: (Sen, PS),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                piece_serial_number: 12,
            },
            PromotedPawn1 => PieceStruct {
                piece: PromotedPawn1,
                phase_piece_type: (Sen, PH),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                piece_serial_number: 13,
            },
            King2 => PieceStruct {
                piece: King2,
                phase_piece_type: (Go, R),
                promoted: King2,
                demoted: King2,
                captured: Piece::Owari,
                piece_serial_number: 14,
            },
            Rook2 => PieceStruct {
                piece: Rook2,
                phase_piece_type: (Go, K),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                piece_serial_number: 15,
            },
            Bishop2 => PieceStruct {
                piece: Bishop2,
                phase_piece_type: (Go, Z),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                piece_serial_number: 16,
            },
            Gold2 => PieceStruct {
                piece: Gold2,
                phase_piece_type: (Go, I),
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                piece_serial_number: 17,
            },
            Silver2 => PieceStruct {
                piece: Silver2,
                phase_piece_type: (Go, N),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                piece_serial_number: 18,
            },
            Knight2 => PieceStruct {
                piece: Knight2,
                phase_piece_type: (Go, U),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                piece_serial_number: 19,
            },
            Lance2 => PieceStruct {
                piece: Lance2,
                phase_piece_type: (Go, S),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                piece_serial_number: 20,
            },
            Pawn2 => PieceStruct {
                piece: Pawn2,
                phase_piece_type: (Go, H),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                piece_serial_number: 21,
            },
            PromotedRook2 => PieceStruct {
                piece: PromotedRook2,
                phase_piece_type: (Go, PK),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                piece_serial_number: 22,
            },
            PromotedBishop2 => PieceStruct {
                piece: PromotedBishop2,
                phase_piece_type: (Go, PZ),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                piece_serial_number: 23,
            },
            PromotedSilver2 => PieceStruct {
                piece: PromotedSilver2,
                phase_piece_type: (Go, PN),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                piece_serial_number: 24,
            },
            PromotedKnight2 => PieceStruct {
                piece: PromotedKnight2,
                phase_piece_type: (Go, PU),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                piece_serial_number: 25,
            },
            PromotedLance2 => PieceStruct {
                piece: PromotedLance2,
                phase_piece_type: (Go, PS),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                piece_serial_number: 26,
            },
            PromotedPawn2 => PieceStruct {
                piece: PromotedPawn2,
                phase_piece_type: (Go, PH),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                piece_serial_number: 27,
            },
            Piece::Kara => PieceStruct {
                piece: Piece::Kara,
                phase_piece_type: (Phase::Owari, PieceType::Kara),
                promoted: Piece::Kara,
                demoted: Piece::Kara,
                captured: Piece::Owari,
                piece_serial_number: 28,
            },
            Piece::Owari => PieceStruct {
                piece: Piece::Owari,
                phase_piece_type: (Phase::Owari, PieceType::Owari),
                promoted: Piece::Owari,
                demoted: Piece::Owari,
                captured: Piece::Owari,
                piece_serial_number: 29,
            },
        }
    }

    pub fn piece(self) -> Piece {
        self.piece
    }

    pub fn phase_piece_type(self) -> (Phase, PieceType) {
        self.phase_piece_type
    }

    pub fn promote(self) -> Piece {
        self.promoted
    }

    pub fn demote(self) -> Piece {
        self.demoted
    }

    /// 持ち駒にするぜ☆（＾～＾）相手の持ち物になるぜ☆（＾～＾）
    pub fn capture(self) -> Piece {
        self.captured
    }

    pub fn piece_serial_number(&self) -> usize {
        self.piece_serial_number
    }

    /**
     * 駒の一致比較
     */
    pub fn equals_piece(self, b: &PieceStruct) -> bool {
        self.piece_serial_number() == b.piece_serial_number()
    }
}
