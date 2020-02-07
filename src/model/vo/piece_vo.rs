use super::super::vo::phase::Phase;
use super::super::vo::piece::Piece;
use super::super::vo::piece_type::PieceType;

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
/// アプリ起動時に全種類作って Enum型 で取得するようにした方がよくないか☆（＾～＾）？
pub struct PieceVo {
    piece: Piece,
    /// 先後、駒種類。
    phase_piece_type: (Phase, PieceType),
    /// 駒→成駒　（成れない駒は、そのまま）
    promoted: Piece,
    /// 成駒→駒　（成っていない駒は、そのまま）
    demoted: Piece,
    /// 先後付き駒　を　持ち駒種類　へ変換。
    /// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
    captured: Piece,
    /// 先後付き駒の配列のインデックス
    serial_piece_number: usize,
}
impl PieceVo {
    /// ピースの生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
    pub fn from_piece(p: Piece) -> Self {
        use super::super::vo::phase::Phase::*;
        use super::super::vo::piece::Piece::*;
        use super::super::vo::piece_type::PieceType::*;
        match p {
            King1 => PieceVo {
                piece: King1,
                phase_piece_type: (Sen, R),
                promoted: King1,
                demoted: King1,
                captured: Piece::Owari,
                serial_piece_number: 0,
            },
            Rook1 => PieceVo {
                piece: Rook1,
                phase_piece_type: (Sen, K),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 1,
            },
            Bishop1 => PieceVo {
                piece: Bishop1,
                phase_piece_type: (Sen, Z),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 2,
            },
            Gold1 => PieceVo {
                piece: Gold1,
                phase_piece_type: (Sen, I),
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                serial_piece_number: 3,
            },
            Silver1 => PieceVo {
                piece: Silver1,
                phase_piece_type: (Sen, N),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 4,
            },
            Knight1 => PieceVo {
                piece: Knight1,
                phase_piece_type: (Sen, U),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 5,
            },
            Lance1 => PieceVo {
                piece: Lance1,
                phase_piece_type: (Sen, S),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 6,
            },
            Pawn1 => PieceVo {
                piece: Pawn1,
                phase_piece_type: (Sen, H),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 7,
            },
            PromotedRook1 => PieceVo {
                piece: PromotedRook1,
                phase_piece_type: (Sen, PK),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 8,
            },
            PromotedBishop1 => PieceVo {
                piece: PromotedBishop1,
                phase_piece_type: (Sen, PZ),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 9,
            },
            PromotedSilver1 => PieceVo {
                piece: PromotedSilver1,
                phase_piece_type: (Sen, PN),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 10,
            },
            PromotedKnight1 => PieceVo {
                piece: PromotedKnight1,
                phase_piece_type: (Sen, PU),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 11,
            },
            PromotedLance1 => PieceVo {
                piece: PromotedLance1,
                phase_piece_type: (Sen, PS),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 12,
            },
            PromotedPawn1 => PieceVo {
                piece: PromotedPawn1,
                phase_piece_type: (Sen, PH),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 13,
            },
            King2 => PieceVo {
                piece: King2,
                phase_piece_type: (Go, R),
                promoted: King2,
                demoted: King2,
                captured: Piece::Owari,
                serial_piece_number: 14,
            },
            Rook2 => PieceVo {
                piece: Rook2,
                phase_piece_type: (Go, K),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 15,
            },
            Bishop2 => PieceVo {
                piece: Bishop2,
                phase_piece_type: (Go, Z),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 16,
            },
            Gold2 => PieceVo {
                piece: Gold2,
                phase_piece_type: (Go, I),
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                serial_piece_number: 17,
            },
            Silver2 => PieceVo {
                piece: Silver2,
                phase_piece_type: (Go, N),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 18,
            },
            Knight2 => PieceVo {
                piece: Knight2,
                phase_piece_type: (Go, U),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 19,
            },
            Lance2 => PieceVo {
                piece: Lance2,
                phase_piece_type: (Go, S),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 20,
            },
            Pawn2 => PieceVo {
                piece: Pawn2,
                phase_piece_type: (Go, H),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 21,
            },
            PromotedRook2 => PieceVo {
                piece: PromotedRook2,
                phase_piece_type: (Go, PK),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 22,
            },
            PromotedBishop2 => PieceVo {
                piece: PromotedBishop2,
                phase_piece_type: (Go, PZ),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 23,
            },
            PromotedSilver2 => PieceVo {
                piece: PromotedSilver2,
                phase_piece_type: (Go, PN),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 24,
            },
            PromotedKnight2 => PieceVo {
                piece: PromotedKnight2,
                phase_piece_type: (Go, PU),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 25,
            },
            PromotedLance2 => PieceVo {
                piece: PromotedLance2,
                phase_piece_type: (Go, PS),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 26,
            },
            PromotedPawn2 => PieceVo {
                piece: PromotedPawn2,
                phase_piece_type: (Go, PH),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 27,
            },
            Piece::Kara => PieceVo {
                piece: Piece::Kara,
                phase_piece_type: (Phase::Owari, PieceType::Kara),
                promoted: Piece::Kara,
                demoted: Piece::Kara,
                captured: Piece::Owari,
                serial_piece_number: 28,
            },
            Piece::Owari => PieceVo {
                piece: Piece::Owari,
                phase_piece_type: (Phase::Owari, PieceType::Owari),
                promoted: Piece::Owari,
                demoted: Piece::Owari,
                captured: Piece::Owari,
                serial_piece_number: 29,
            },
        }
    }

    pub fn from_serial_piece_number(km_num: usize) -> Self {
        use super::super::super::model::vo::piece::Piece::*;
        match km_num {
            0 => PieceVo::from_piece(King1),
            1 => PieceVo::from_piece(Rook1),
            2 => PieceVo::from_piece(Bishop1),
            3 => PieceVo::from_piece(Gold1),
            4 => PieceVo::from_piece(Silver1),
            5 => PieceVo::from_piece(Knight1),
            6 => PieceVo::from_piece(Lance1),
            7 => PieceVo::from_piece(Pawn1),
            8 => PieceVo::from_piece(PromotedRook1),
            9 => PieceVo::from_piece(PromotedBishop1),
            10 => PieceVo::from_piece(PromotedSilver1),
            11 => PieceVo::from_piece(PromotedKnight1),
            12 => PieceVo::from_piece(PromotedLance1),
            13 => PieceVo::from_piece(PromotedPawn1),
            14 => PieceVo::from_piece(King2),
            15 => PieceVo::from_piece(Rook2),
            16 => PieceVo::from_piece(Bishop2),
            17 => PieceVo::from_piece(Gold2),
            18 => PieceVo::from_piece(Silver2),
            19 => PieceVo::from_piece(Knight2),
            20 => PieceVo::from_piece(Lance2),
            21 => PieceVo::from_piece(Pawn2),
            22 => PieceVo::from_piece(PromotedRook2),
            23 => PieceVo::from_piece(PromotedBishop2),
            24 => PieceVo::from_piece(PromotedSilver2),
            25 => PieceVo::from_piece(PromotedKnight2),
            26 => PieceVo::from_piece(PromotedLance2),
            27 => PieceVo::from_piece(PromotedPawn2),
            28 => PieceVo::from_piece(Kara),
            _ => PieceVo::from_piece(Owari),
        }
    }

    /// ハッシュ値から作る
    pub fn from_hash(hash: u64) -> (u64, Self) {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        let ps = PieceVo::from_serial_piece_number((hash & 0b11111) as usize);
        (hash >> 5, ps)
    }

    pub fn piece(&self) -> &Piece {
        &self.piece
    }

    pub fn phase_piece_type(&self) -> (&Phase, &PieceType) {
        (&self.phase_piece_type.0, &self.phase_piece_type.1)
    }

    pub fn phase(&self) -> Phase {
        self.phase_piece_type.0.clone()
    }

    pub fn piece_type(&self) -> PieceType {
        self.phase_piece_type.1
    }

    pub fn promote(&self) -> &Piece {
        &self.promoted
    }

    pub fn demote(&self) -> &Piece {
        &self.demoted
    }

    // 降格できるか。
    pub fn can_demote(&self) -> bool {
        // 降格後の駒が、今の駒と異なっていれば、降格できるぜ☆（＾～＾）
        self.piece != self.demoted
    }

    /// 持ち駒にするぜ☆（＾～＾）相手の持ち物になるぜ☆（＾～＾）
    pub fn capture(&self) -> &Piece {
        &self.captured
    }

    pub fn serial_piece_number(&self) -> usize {
        self.serial_piece_number
    }

    /// 駒の一致比較
    pub fn equals_piece(&self, b: &PieceVo) -> bool {
        self.serial_piece_number() == b.serial_piece_number()
    }

    /// 駒種類→｛　成駒,（不成駒、それ以外）　｝
    pub fn is_promoted(&self) -> bool {
        use super::super::super::model::vo::piece_type::PieceType::*;
        match self.piece_type() {
            R => false,
            K => false,
            Z => false,
            I => false,
            N => false,
            U => false,
            S => false,
            H => false,
            PK => true,
            PZ => true,
            PN => true,
            PU => true,
            PS => true,
            PH => true,
            Kara => false,
            Owari => false,
        }
    }

    /// スライダー（長い利きのある駒）か☆（＾～＾）
    ///
    /// 合い駒で、進路を防ぎえる可能性があれば真
    pub fn is_slider(&self) -> bool {
        use super::super::super::model::vo::piece_type::PieceType::*;
        match &self.piece_type() {
            R => false,
            K => true,
            Z => true,
            I => false,
            N => false,
            U => false,
            S => true,
            H => false,
            PK => true,
            PZ => true,
            PN => false,
            PU => false,
            PS => false,
            PH => false,
            Kara => false,
            Owari => false,
        }
    }

    /// ハッシュ値を作る
    pub fn add_hash(&self, hash: u64) -> u64 {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        (hash << 5) + self.serial_piece_number() as u64
    }
}
