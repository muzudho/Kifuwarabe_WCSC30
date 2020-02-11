use super::gp_phase_vo::Phase;
use super::gp_piece_type_vo::GPPieceTypeVo;
use super::gp_piece_vo::GPPieceVo;

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
/// アプリ起動時に全種類作って Enum型 で取得するようにした方がよくないか☆（＾～＾）？
#[derive(Clone)]
pub struct GPPieceStructVo {
    piece: GPPieceVo,
    /// 先後、駒種類。
    phase_piece_type: (Phase, GPPieceTypeVo),

    /// 駒→成駒　（成れない駒は、そのまま）
    promoted: GPPieceVo,

    /// 成駒→駒　（成っていない駒は、そのまま）
    demoted: GPPieceVo,

    /// 先後付き駒　を　持ち駒種類　へ変換。
    /// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
    captured: GPPieceVo,

    /// 先後付き駒の配列のインデックス
    serial_piece_number: usize,
}
impl GPPieceStructVo {
    /// ピースの生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
    pub fn from_piece(p: GPPieceVo) -> Self {
        use super::super::game_part::gp_phase_vo::Phase::*;
        use super::super::game_part::gp_piece_vo::GPPieceVo::*;
        use super::gp_piece_type_vo::GPPieceTypeVo::*;
        match p {
            King1 => GPPieceStructVo {
                piece: King1,
                phase_piece_type: (First, King),
                promoted: King1,
                demoted: King1,
                captured: GPPieceVo::OwariPiece,
                serial_piece_number: 0,
            },
            Rook1 => GPPieceStructVo {
                piece: Rook1,
                phase_piece_type: (First, Rook),
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 1,
            },
            Bishop1 => GPPieceStructVo {
                piece: Bishop1,
                phase_piece_type: (First, Bishop),
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 2,
            },
            Gold1 => GPPieceStructVo {
                piece: Gold1,
                phase_piece_type: (First, Gold),
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                serial_piece_number: 3,
            },
            Silver1 => GPPieceStructVo {
                piece: Silver1,
                phase_piece_type: (First, Silver),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 4,
            },
            Knight1 => GPPieceStructVo {
                piece: Knight1,
                phase_piece_type: (First, Knight),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 5,
            },
            Lance1 => GPPieceStructVo {
                piece: Lance1,
                phase_piece_type: (First, Lance),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 6,
            },
            Pawn1 => GPPieceStructVo {
                piece: Pawn1,
                phase_piece_type: (First, Pawn),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 7,
            },
            Dragon1 => GPPieceStructVo {
                piece: Dragon1,
                phase_piece_type: (First, Dragon),
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 8,
            },
            Horse1 => GPPieceStructVo {
                piece: Horse1,
                phase_piece_type: (First, Horse),
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 9,
            },
            PromotedSilver1 => GPPieceStructVo {
                piece: PromotedSilver1,
                phase_piece_type: (First, PromotedSilver),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 10,
            },
            PromotedKnight1 => GPPieceStructVo {
                piece: PromotedKnight1,
                phase_piece_type: (First, PromotedKnight),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 11,
            },
            PromotedLance1 => GPPieceStructVo {
                piece: PromotedLance1,
                phase_piece_type: (First, PromotedLance),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 12,
            },
            PromotedPawn1 => GPPieceStructVo {
                piece: PromotedPawn1,
                phase_piece_type: (First, PromotedPawn),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 13,
            },
            King2 => GPPieceStructVo {
                piece: King2,
                phase_piece_type: (Second, King),
                promoted: King2,
                demoted: King2,
                captured: GPPieceVo::OwariPiece,
                serial_piece_number: 14,
            },
            Rook2 => GPPieceStructVo {
                piece: Rook2,
                phase_piece_type: (Second, Rook),
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 15,
            },
            Bishop2 => GPPieceStructVo {
                piece: Bishop2,
                phase_piece_type: (Second, Bishop),
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 16,
            },
            Gold2 => GPPieceStructVo {
                piece: Gold2,
                phase_piece_type: (Second, Gold),
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                serial_piece_number: 17,
            },
            Silver2 => GPPieceStructVo {
                piece: Silver2,
                phase_piece_type: (Second, Silver),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 18,
            },
            Knight2 => GPPieceStructVo {
                piece: Knight2,
                phase_piece_type: (Second, Knight),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 19,
            },
            Lance2 => GPPieceStructVo {
                piece: Lance2,
                phase_piece_type: (Second, Lance),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 20,
            },
            Pawn2 => GPPieceStructVo {
                piece: Pawn2,
                phase_piece_type: (Second, Pawn),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 21,
            },
            Dragon2 => GPPieceStructVo {
                piece: Dragon2,
                phase_piece_type: (Second, Dragon),
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 22,
            },
            Horse2 => GPPieceStructVo {
                piece: Horse2,
                phase_piece_type: (Second, Horse),
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 23,
            },
            PromotedSilver2 => GPPieceStructVo {
                piece: PromotedSilver2,
                phase_piece_type: (Second, PromotedSilver),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 24,
            },
            PromotedKnight2 => GPPieceStructVo {
                piece: PromotedKnight2,
                phase_piece_type: (Second, PromotedKnight),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 25,
            },
            PromotedLance2 => GPPieceStructVo {
                piece: PromotedLance2,
                phase_piece_type: (Second, PromotedLance),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 26,
            },
            PromotedPawn2 => GPPieceStructVo {
                piece: PromotedPawn2,
                phase_piece_type: (Second, PromotedPawn),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 27,
            },
            GPPieceVo::NonePiece => GPPieceStructVo {
                piece: GPPieceVo::NonePiece,
                phase_piece_type: (Phase::None, GPPieceTypeVo::KaraPieceType),
                promoted: GPPieceVo::NonePiece,
                demoted: GPPieceVo::NonePiece,
                captured: GPPieceVo::OwariPiece,
                serial_piece_number: 28,
            },
            GPPieceVo::OwariPiece => GPPieceStructVo {
                piece: GPPieceVo::OwariPiece,
                phase_piece_type: (Phase::None, GPPieceTypeVo::OwariPieceType),
                promoted: GPPieceVo::OwariPiece,
                demoted: GPPieceVo::OwariPiece,
                captured: GPPieceVo::OwariPiece,
                serial_piece_number: 29,
            },
        }
    }

    pub fn from_serial_piece_number(km_num: usize) -> Self {
        use super::super::game_part::gp_piece_vo::GPPieceVo::*;
        match km_num {
            0 => GPPieceStructVo::from_piece(King1),
            1 => GPPieceStructVo::from_piece(Rook1),
            2 => GPPieceStructVo::from_piece(Bishop1),
            3 => GPPieceStructVo::from_piece(Gold1),
            4 => GPPieceStructVo::from_piece(Silver1),
            5 => GPPieceStructVo::from_piece(Knight1),
            6 => GPPieceStructVo::from_piece(Lance1),
            7 => GPPieceStructVo::from_piece(Pawn1),
            8 => GPPieceStructVo::from_piece(Dragon1),
            9 => GPPieceStructVo::from_piece(Horse1),
            10 => GPPieceStructVo::from_piece(PromotedSilver1),
            11 => GPPieceStructVo::from_piece(PromotedKnight1),
            12 => GPPieceStructVo::from_piece(PromotedLance1),
            13 => GPPieceStructVo::from_piece(PromotedPawn1),
            14 => GPPieceStructVo::from_piece(King2),
            15 => GPPieceStructVo::from_piece(Rook2),
            16 => GPPieceStructVo::from_piece(Bishop2),
            17 => GPPieceStructVo::from_piece(Gold2),
            18 => GPPieceStructVo::from_piece(Silver2),
            19 => GPPieceStructVo::from_piece(Knight2),
            20 => GPPieceStructVo::from_piece(Lance2),
            21 => GPPieceStructVo::from_piece(Pawn2),
            22 => GPPieceStructVo::from_piece(Dragon2),
            23 => GPPieceStructVo::from_piece(Horse2),
            24 => GPPieceStructVo::from_piece(PromotedSilver2),
            25 => GPPieceStructVo::from_piece(PromotedKnight2),
            26 => GPPieceStructVo::from_piece(PromotedLance2),
            27 => GPPieceStructVo::from_piece(PromotedPawn2),
            28 => GPPieceStructVo::from_piece(NonePiece),
            _ => GPPieceStructVo::from_piece(OwariPiece),
        }
    }

    /// ハッシュ値から作る
    pub fn from_hash(hash: u64) -> (u64, Self) {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        let ps = GPPieceStructVo::from_serial_piece_number((hash & 0b11111) as usize);
        (hash >> 5, ps)
    }

    pub fn piece(&self) -> &GPPieceVo {
        &self.piece
    }

    pub fn phase_piece_type(&self) -> (&Phase, GPPieceTypeVo) {
        (&self.phase_piece_type.0, self.phase_piece_type.1)
    }

    pub fn phase(&self) -> Phase {
        self.phase_piece_type.0.clone()
    }

    pub fn piece_type(&self) -> GPPieceTypeVo {
        self.phase_piece_type.1
    }

    pub fn promote(&self) -> &GPPieceVo {
        &self.promoted
    }

    pub fn demote(&self) -> &GPPieceVo {
        &self.demoted
    }

    // 降格できるか。
    pub fn can_demote(&self) -> bool {
        // 降格後の駒が、今の駒と異なっていれば、降格できるぜ☆（＾～＾）
        self.piece != self.demoted
    }

    /// 持ち駒にするぜ☆（＾～＾）相手の持ち物になるぜ☆（＾～＾）
    pub fn capture(&self) -> &GPPieceVo {
        &self.captured
    }

    pub fn serial_piece_number(&self) -> usize {
        self.serial_piece_number
    }

    /// 駒の一致比較
    pub fn equals_piece(&self, b: &GPPieceStructVo) -> bool {
        self.serial_piece_number() == b.serial_piece_number()
    }

    /// 駒種類→｛　成駒,（不成駒、それ以外）　｝
    pub fn is_promoted(&self) -> bool {
        use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::*;
        match self.piece_type() {
            King => false,
            Rook => false,
            Bishop => false,
            Gold => false,
            Silver => false,
            Knight => false,
            Lance => false,
            Pawn => false,
            Dragon => true,
            Horse => true,
            PromotedSilver => true,
            PromotedKnight => true,
            PromotedLance => true,
            PromotedPawn => true,
            KaraPieceType => false,
            OwariPieceType => false,
        }
    }

    /// ハッシュ値を作る
    pub fn add_hash(&self, hash: u64) -> u64 {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        (hash << 5) + self.serial_piece_number() as u64
    }
}