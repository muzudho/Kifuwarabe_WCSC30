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
    serial_piece_number: usize,
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
                serial_piece_number: 0,
            },
            Rook1 => PieceStruct {
                piece: Rook1,
                phase_piece_type: (Sen, K),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 1,
            },
            Bishop1 => PieceStruct {
                piece: Bishop1,
                phase_piece_type: (Sen, Z),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 2,
            },
            Gold1 => PieceStruct {
                piece: Gold1,
                phase_piece_type: (Sen, I),
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                serial_piece_number: 3,
            },
            Silver1 => PieceStruct {
                piece: Silver1,
                phase_piece_type: (Sen, N),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 4,
            },
            Knight1 => PieceStruct {
                piece: Knight1,
                phase_piece_type: (Sen, U),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 5,
            },
            Lance1 => PieceStruct {
                piece: Lance1,
                phase_piece_type: (Sen, S),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 6,
            },
            Pawn1 => PieceStruct {
                piece: Pawn1,
                phase_piece_type: (Sen, H),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 7,
            },
            PromotedRook1 => PieceStruct {
                piece: PromotedRook1,
                phase_piece_type: (Sen, PK),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 8,
            },
            PromotedBishop1 => PieceStruct {
                piece: PromotedBishop1,
                phase_piece_type: (Sen, PZ),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 9,
            },
            PromotedSilver1 => PieceStruct {
                piece: PromotedSilver1,
                phase_piece_type: (Sen, PN),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 10,
            },
            PromotedKnight1 => PieceStruct {
                piece: PromotedKnight1,
                phase_piece_type: (Sen, PU),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 11,
            },
            PromotedLance1 => PieceStruct {
                piece: PromotedLance1,
                phase_piece_type: (Sen, PS),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 12,
            },
            PromotedPawn1 => PieceStruct {
                piece: PromotedPawn1,
                phase_piece_type: (Sen, PH),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 13,
            },
            King2 => PieceStruct {
                piece: King2,
                phase_piece_type: (Go, R),
                promoted: King2,
                demoted: King2,
                captured: Piece::Owari,
                serial_piece_number: 14,
            },
            Rook2 => PieceStruct {
                piece: Rook2,
                phase_piece_type: (Go, K),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 15,
            },
            Bishop2 => PieceStruct {
                piece: Bishop2,
                phase_piece_type: (Go, Z),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 16,
            },
            Gold2 => PieceStruct {
                piece: Gold2,
                phase_piece_type: (Go, I),
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                serial_piece_number: 17,
            },
            Silver2 => PieceStruct {
                piece: Silver2,
                phase_piece_type: (Go, N),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 18,
            },
            Knight2 => PieceStruct {
                piece: Knight2,
                phase_piece_type: (Go, U),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 19,
            },
            Lance2 => PieceStruct {
                piece: Lance2,
                phase_piece_type: (Go, S),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 20,
            },
            Pawn2 => PieceStruct {
                piece: Pawn2,
                phase_piece_type: (Go, H),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 21,
            },
            PromotedRook2 => PieceStruct {
                piece: PromotedRook2,
                phase_piece_type: (Go, PK),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 22,
            },
            PromotedBishop2 => PieceStruct {
                piece: PromotedBishop2,
                phase_piece_type: (Go, PZ),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 23,
            },
            PromotedSilver2 => PieceStruct {
                piece: PromotedSilver2,
                phase_piece_type: (Go, PN),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 24,
            },
            PromotedKnight2 => PieceStruct {
                piece: PromotedKnight2,
                phase_piece_type: (Go, PU),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 25,
            },
            PromotedLance2 => PieceStruct {
                piece: PromotedLance2,
                phase_piece_type: (Go, PS),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 26,
            },
            PromotedPawn2 => PieceStruct {
                piece: PromotedPawn2,
                phase_piece_type: (Go, PH),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 27,
            },
            Piece::Kara => PieceStruct {
                piece: Piece::Kara,
                phase_piece_type: (Phase::Owari, PieceType::Kara),
                promoted: Piece::Kara,
                demoted: Piece::Kara,
                captured: Piece::Owari,
                serial_piece_number: 28,
            },
            Piece::Owari => PieceStruct {
                piece: Piece::Owari,
                phase_piece_type: (Phase::Owari, PieceType::Owari),
                promoted: Piece::Owari,
                demoted: Piece::Owari,
                captured: Piece::Owari,
                serial_piece_number: 29,
            },
        }
    }

    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_piece_type(sn: &Phase, kms: &PieceType) -> Self {
        use super::super::super::model::master::piece::Piece::*;
        use super::super::super::model::master::piece_type::PieceType::*;
        match *sn {
            Phase::Sen => match *kms {
                R => PieceStruct::from_piece(&King1),
                K => PieceStruct::from_piece(&Rook1),
                Z => PieceStruct::from_piece(&Bishop1),
                I => PieceStruct::from_piece(&Gold1),
                N => PieceStruct::from_piece(&Silver1),
                U => PieceStruct::from_piece(&Knight1),
                S => PieceStruct::from_piece(&Lance1),
                H => PieceStruct::from_piece(&Pawn1),
                PK => PieceStruct::from_piece(&PromotedRook1),
                PZ => PieceStruct::from_piece(&PromotedBishop1),
                PN => PieceStruct::from_piece(&PromotedSilver1),
                PU => PieceStruct::from_piece(&PromotedKnight1),
                PS => PieceStruct::from_piece(&PromotedLance1),
                PH => PieceStruct::from_piece(&PromotedPawn1),
                _ => PieceStruct::from_piece(&Piece::Owari),
            },
            Phase::Go => match *kms {
                R => PieceStruct::from_piece(&King2),
                K => PieceStruct::from_piece(&Rook2),
                Z => PieceStruct::from_piece(&Bishop2),
                I => PieceStruct::from_piece(&Gold2),
                N => PieceStruct::from_piece(&Silver2),
                U => PieceStruct::from_piece(&Knight2),
                S => PieceStruct::from_piece(&Lance2),
                H => PieceStruct::from_piece(&Pawn2),
                PK => PieceStruct::from_piece(&PromotedRook2),
                PZ => PieceStruct::from_piece(&PromotedBishop2),
                PN => PieceStruct::from_piece(&PromotedSilver2),
                PU => PieceStruct::from_piece(&PromotedKnight2),
                PS => PieceStruct::from_piece(&PromotedLance2),
                PH => PieceStruct::from_piece(&PromotedPawn2),
                _ => PieceStruct::from_piece(&Piece::Owari),
            },
            Phase::Owari => PieceStruct::from_piece(&Piece::Owari),
        }
    }

    pub fn from_serial_piece_number(km_num: usize) -> Self {
        use super::super::super::model::master::piece::Piece::*;
        match km_num {
            0 => PieceStruct::from_piece(&King1),
            1 => PieceStruct::from_piece(&Rook1),
            2 => PieceStruct::from_piece(&Bishop1),
            3 => PieceStruct::from_piece(&Gold1),
            4 => PieceStruct::from_piece(&Silver1),
            5 => PieceStruct::from_piece(&Knight1),
            6 => PieceStruct::from_piece(&Lance1),
            7 => PieceStruct::from_piece(&Pawn1),
            8 => PieceStruct::from_piece(&PromotedRook1),
            9 => PieceStruct::from_piece(&PromotedBishop1),
            10 => PieceStruct::from_piece(&PromotedSilver1),
            11 => PieceStruct::from_piece(&PromotedKnight1),
            12 => PieceStruct::from_piece(&PromotedLance1),
            13 => PieceStruct::from_piece(&PromotedPawn1),
            14 => PieceStruct::from_piece(&King2),
            15 => PieceStruct::from_piece(&Rook2),
            16 => PieceStruct::from_piece(&Bishop2),
            17 => PieceStruct::from_piece(&Gold2),
            18 => PieceStruct::from_piece(&Silver2),
            19 => PieceStruct::from_piece(&Knight2),
            20 => PieceStruct::from_piece(&Lance2),
            21 => PieceStruct::from_piece(&Pawn2),
            22 => PieceStruct::from_piece(&PromotedRook2),
            23 => PieceStruct::from_piece(&PromotedBishop2),
            24 => PieceStruct::from_piece(&PromotedSilver2),
            25 => PieceStruct::from_piece(&PromotedKnight2),
            26 => PieceStruct::from_piece(&PromotedLance2),
            27 => PieceStruct::from_piece(&PromotedPawn2),
            28 => PieceStruct::from_piece(&Kara),
            _ => PieceStruct::from_piece(&Owari),
        }
    }

    /// ハッシュ値から作る
    pub fn from_hash(hash: u64) -> (u64, Self) {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        let ps = PieceStruct::from_serial_piece_number((hash & 0b11111) as usize);
        (hash >> 5, ps)
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn phase_piece_type(&self) -> (&Phase, &PieceType) {
        (&self.phase_piece_type.0, &self.phase_piece_type.1)
    }

    pub fn phase(&self) -> &Phase {
        &self.phase_piece_type.0
    }

    pub fn piece_type(&self) -> PieceType {
        self.phase_piece_type.1
    }

    pub fn promote(&self) -> Piece {
        self.promoted
    }

    pub fn demote(&self) -> Piece {
        self.demoted
    }

    /// 持ち駒にするぜ☆（＾～＾）相手の持ち物になるぜ☆（＾～＾）
    pub fn capture(&self) -> Piece {
        self.captured
    }

    pub fn serial_piece_number(&self) -> usize {
        self.serial_piece_number
    }

    /**
     * 駒の一致比較
     */
    pub fn equals_piece(&self, b: &PieceStruct) -> bool {
        self.serial_piece_number() == b.serial_piece_number()
    }

    /// ハッシュ値を作る
    pub fn add_hash(self, hash: u64) -> u64 {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        (hash << 5) + self.serial_piece_number() as u64
    }
}
