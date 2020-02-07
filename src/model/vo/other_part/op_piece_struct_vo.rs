use super::op_phase_vo::Phase;
use super::op_piece_type_vo::PieceType;
use super::op_piece_vo::OPPieceVo;

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
/// アプリ起動時に全種類作って Enum型 で取得するようにした方がよくないか☆（＾～＾）？
pub struct PieceStructVo {
    piece: OPPieceVo,
    /// 先後、駒種類。
    phase_piece_type: (Phase, PieceType),
    /// 駒→成駒　（成れない駒は、そのまま）
    promoted: OPPieceVo,
    /// 成駒→駒　（成っていない駒は、そのまま）
    demoted: OPPieceVo,
    /// 先後付き駒　を　持ち駒種類　へ変換。
    /// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
    captured: OPPieceVo,
    /// 先後付き駒の配列のインデックス
    serial_piece_number: usize,
}
impl PieceStructVo {
    /// ピースの生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
    pub fn from_piece(p: OPPieceVo) -> Self {
        use super::op_phase_vo::Phase::*;
        use super::op_piece_type_vo::PieceType::*;
        use super::op_piece_vo::OPPieceVo::*;
        match p {
            King1 => PieceStructVo {
                piece: King1,
                phase_piece_type: (Sen, R),
                promoted: King1,
                demoted: King1,
                captured: OPPieceVo::Owari,
                serial_piece_number: 0,
            },
            Rook1 => PieceStructVo {
                piece: Rook1,
                phase_piece_type: (Sen, K),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 1,
            },
            Bishop1 => PieceStructVo {
                piece: Bishop1,
                phase_piece_type: (Sen, Z),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 2,
            },
            Gold1 => PieceStructVo {
                piece: Gold1,
                phase_piece_type: (Sen, I),
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                serial_piece_number: 3,
            },
            Silver1 => PieceStructVo {
                piece: Silver1,
                phase_piece_type: (Sen, N),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 4,
            },
            Knight1 => PieceStructVo {
                piece: Knight1,
                phase_piece_type: (Sen, U),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 5,
            },
            Lance1 => PieceStructVo {
                piece: Lance1,
                phase_piece_type: (Sen, S),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 6,
            },
            Pawn1 => PieceStructVo {
                piece: Pawn1,
                phase_piece_type: (Sen, H),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 7,
            },
            PromotedRook1 => PieceStructVo {
                piece: PromotedRook1,
                phase_piece_type: (Sen, PK),
                promoted: PromotedRook1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 8,
            },
            PromotedBishop1 => PieceStructVo {
                piece: PromotedBishop1,
                phase_piece_type: (Sen, PZ),
                promoted: PromotedBishop1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 9,
            },
            PromotedSilver1 => PieceStructVo {
                piece: PromotedSilver1,
                phase_piece_type: (Sen, PN),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 10,
            },
            PromotedKnight1 => PieceStructVo {
                piece: PromotedKnight1,
                phase_piece_type: (Sen, PU),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 11,
            },
            PromotedLance1 => PieceStructVo {
                piece: PromotedLance1,
                phase_piece_type: (Sen, PS),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 12,
            },
            PromotedPawn1 => PieceStructVo {
                piece: PromotedPawn1,
                phase_piece_type: (Sen, PH),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 13,
            },
            King2 => PieceStructVo {
                piece: King2,
                phase_piece_type: (Go, R),
                promoted: King2,
                demoted: King2,
                captured: OPPieceVo::Owari,
                serial_piece_number: 14,
            },
            Rook2 => PieceStructVo {
                piece: Rook2,
                phase_piece_type: (Go, K),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 15,
            },
            Bishop2 => PieceStructVo {
                piece: Bishop2,
                phase_piece_type: (Go, Z),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 16,
            },
            Gold2 => PieceStructVo {
                piece: Gold2,
                phase_piece_type: (Go, I),
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                serial_piece_number: 17,
            },
            Silver2 => PieceStructVo {
                piece: Silver2,
                phase_piece_type: (Go, N),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 18,
            },
            Knight2 => PieceStructVo {
                piece: Knight2,
                phase_piece_type: (Go, U),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 19,
            },
            Lance2 => PieceStructVo {
                piece: Lance2,
                phase_piece_type: (Go, S),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 20,
            },
            Pawn2 => PieceStructVo {
                piece: Pawn2,
                phase_piece_type: (Go, H),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 21,
            },
            PromotedRook2 => PieceStructVo {
                piece: PromotedRook2,
                phase_piece_type: (Go, PK),
                promoted: PromotedRook2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 22,
            },
            PromotedBishop2 => PieceStructVo {
                piece: PromotedBishop2,
                phase_piece_type: (Go, PZ),
                promoted: PromotedBishop2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 23,
            },
            PromotedSilver2 => PieceStructVo {
                piece: PromotedSilver2,
                phase_piece_type: (Go, PN),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 24,
            },
            PromotedKnight2 => PieceStructVo {
                piece: PromotedKnight2,
                phase_piece_type: (Go, PU),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 25,
            },
            PromotedLance2 => PieceStructVo {
                piece: PromotedLance2,
                phase_piece_type: (Go, PS),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 26,
            },
            PromotedPawn2 => PieceStructVo {
                piece: PromotedPawn2,
                phase_piece_type: (Go, PH),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 27,
            },
            OPPieceVo::Kara => PieceStructVo {
                piece: OPPieceVo::Kara,
                phase_piece_type: (Phase::Owari, PieceType::Kara),
                promoted: OPPieceVo::Kara,
                demoted: OPPieceVo::Kara,
                captured: OPPieceVo::Owari,
                serial_piece_number: 28,
            },
            OPPieceVo::Owari => PieceStructVo {
                piece: OPPieceVo::Owari,
                phase_piece_type: (Phase::Owari, PieceType::Owari),
                promoted: OPPieceVo::Owari,
                demoted: OPPieceVo::Owari,
                captured: OPPieceVo::Owari,
                serial_piece_number: 29,
            },
        }
    }

    pub fn from_serial_piece_number(km_num: usize) -> Self {
        use super::op_piece_vo::OPPieceVo::*;
        match km_num {
            0 => PieceStructVo::from_piece(King1),
            1 => PieceStructVo::from_piece(Rook1),
            2 => PieceStructVo::from_piece(Bishop1),
            3 => PieceStructVo::from_piece(Gold1),
            4 => PieceStructVo::from_piece(Silver1),
            5 => PieceStructVo::from_piece(Knight1),
            6 => PieceStructVo::from_piece(Lance1),
            7 => PieceStructVo::from_piece(Pawn1),
            8 => PieceStructVo::from_piece(PromotedRook1),
            9 => PieceStructVo::from_piece(PromotedBishop1),
            10 => PieceStructVo::from_piece(PromotedSilver1),
            11 => PieceStructVo::from_piece(PromotedKnight1),
            12 => PieceStructVo::from_piece(PromotedLance1),
            13 => PieceStructVo::from_piece(PromotedPawn1),
            14 => PieceStructVo::from_piece(King2),
            15 => PieceStructVo::from_piece(Rook2),
            16 => PieceStructVo::from_piece(Bishop2),
            17 => PieceStructVo::from_piece(Gold2),
            18 => PieceStructVo::from_piece(Silver2),
            19 => PieceStructVo::from_piece(Knight2),
            20 => PieceStructVo::from_piece(Lance2),
            21 => PieceStructVo::from_piece(Pawn2),
            22 => PieceStructVo::from_piece(PromotedRook2),
            23 => PieceStructVo::from_piece(PromotedBishop2),
            24 => PieceStructVo::from_piece(PromotedSilver2),
            25 => PieceStructVo::from_piece(PromotedKnight2),
            26 => PieceStructVo::from_piece(PromotedLance2),
            27 => PieceStructVo::from_piece(PromotedPawn2),
            28 => PieceStructVo::from_piece(Kara),
            _ => PieceStructVo::from_piece(Owari),
        }
    }

    /// ハッシュ値から作る
    pub fn from_hash(hash: u64) -> (u64, Self) {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        let ps = PieceStructVo::from_serial_piece_number((hash & 0b11111) as usize);
        (hash >> 5, ps)
    }

    pub fn piece(&self) -> &OPPieceVo {
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

    pub fn promote(&self) -> &OPPieceVo {
        &self.promoted
    }

    pub fn demote(&self) -> &OPPieceVo {
        &self.demoted
    }

    // 降格できるか。
    pub fn can_demote(&self) -> bool {
        // 降格後の駒が、今の駒と異なっていれば、降格できるぜ☆（＾～＾）
        self.piece != self.demoted
    }

    /// 持ち駒にするぜ☆（＾～＾）相手の持ち物になるぜ☆（＾～＾）
    pub fn capture(&self) -> &OPPieceVo {
        &self.captured
    }

    pub fn serial_piece_number(&self) -> usize {
        self.serial_piece_number
    }

    /// 駒の一致比較
    pub fn equals_piece(&self, b: &PieceStructVo) -> bool {
        self.serial_piece_number() == b.serial_piece_number()
    }

    /// 駒種類→｛　成駒,（不成駒、それ以外）　｝
    pub fn is_promoted(&self) -> bool {
        use super::op_piece_type_vo::PieceType::*;
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
        use super::op_piece_type_vo::PieceType::*;
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
