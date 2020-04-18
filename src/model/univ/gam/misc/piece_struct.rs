use crate::model::univ::gam::misc::phase::Phase;
use crate::model::univ::gam::misc::piece_type::PieceType;
use crate::universe::game::piece::piece::Piece;

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
/// アプリ起動時に全種類作って Enum型 で取得するようにした方がよくないか☆（＾～＾）？
#[derive(Clone)]
pub struct PieceStruct {
    pub piece: Piece,
    /// 先後、駒種類。
    pub phase_piece_type: (Phase, PieceType),

    /// 駒→成駒　（成れない駒は、そのまま）Noneは空升に使っている☆（＾～＾）
    pub promoted: Piece,

    /// 成駒→駒　（成っていない駒は、そのまま）Noneは空升に使っている☆（＾～＾）
    pub demoted: Piece,

    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    pub captured: Piece,

    /// 先後付き駒の配列のインデックス
    pub serial_piece_number: usize,
}
impl PieceStruct {
    /// ピースの生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
    pub fn from_piece(p: Piece) -> Self {
        use crate::model::univ::gam::misc::phase::Phase::*;
        use crate::model::univ::gam::misc::piece_type::PieceType::*;
        use crate::universe::game::piece::piece::Piece::*;
        match p {
            King1 => PieceStruct {
                piece: King1,
                phase_piece_type: (First, King),
                promoted: King1,
                demoted: King1,
                captured: King2,
                serial_piece_number: 0,
            },
            Rook1 => PieceStruct {
                piece: Rook1,
                phase_piece_type: (First, Rook),
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 1,
            },
            Bishop1 => PieceStruct {
                piece: Bishop1,
                phase_piece_type: (First, Bishop),
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 2,
            },
            Gold1 => PieceStruct {
                piece: Gold1,
                phase_piece_type: (First, Gold),
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                serial_piece_number: 3,
            },
            Silver1 => PieceStruct {
                piece: Silver1,
                phase_piece_type: (First, Silver),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 4,
            },
            Knight1 => PieceStruct {
                piece: Knight1,
                phase_piece_type: (First, Knight),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 5,
            },
            Lance1 => PieceStruct {
                piece: Lance1,
                phase_piece_type: (First, Lance),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 6,
            },
            Pawn1 => PieceStruct {
                piece: Pawn1,
                phase_piece_type: (First, Pawn),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 7,
            },
            Dragon1 => PieceStruct {
                piece: Dragon1,
                phase_piece_type: (First, Dragon),
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                serial_piece_number: 8,
            },
            Horse1 => PieceStruct {
                piece: Horse1,
                phase_piece_type: (First, Horse),
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_piece_number: 9,
            },
            PromotedSilver1 => PieceStruct {
                piece: PromotedSilver1,
                phase_piece_type: (First, PromotedSilver),
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_piece_number: 10,
            },
            PromotedKnight1 => PieceStruct {
                piece: PromotedKnight1,
                phase_piece_type: (First, PromotedKnight),
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_piece_number: 11,
            },
            PromotedLance1 => PieceStruct {
                piece: PromotedLance1,
                phase_piece_type: (First, PromotedLance),
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_piece_number: 12,
            },
            PromotedPawn1 => PieceStruct {
                piece: PromotedPawn1,
                phase_piece_type: (First, PromotedPawn),
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_piece_number: 13,
            },
            King2 => PieceStruct {
                piece: King2,
                phase_piece_type: (Second, King),
                promoted: King2,
                demoted: King2,
                captured: King1,
                serial_piece_number: 14,
            },
            Rook2 => PieceStruct {
                piece: Rook2,
                phase_piece_type: (Second, Rook),
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 15,
            },
            Bishop2 => PieceStruct {
                piece: Bishop2,
                phase_piece_type: (Second, Bishop),
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 16,
            },
            Gold2 => PieceStruct {
                piece: Gold2,
                phase_piece_type: (Second, Gold),
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                serial_piece_number: 17,
            },
            Silver2 => PieceStruct {
                piece: Silver2,
                phase_piece_type: (Second, Silver),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 18,
            },
            Knight2 => PieceStruct {
                piece: Knight2,
                phase_piece_type: (Second, Knight),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 19,
            },
            Lance2 => PieceStruct {
                piece: Lance2,
                phase_piece_type: (Second, Lance),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 20,
            },
            Pawn2 => PieceStruct {
                piece: Pawn2,
                phase_piece_type: (Second, Pawn),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 21,
            },
            Dragon2 => PieceStruct {
                piece: Dragon2,
                phase_piece_type: (Second, Dragon),
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                serial_piece_number: 22,
            },
            Horse2 => PieceStruct {
                piece: Horse2,
                phase_piece_type: (Second, Horse),
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_piece_number: 23,
            },
            PromotedSilver2 => PieceStruct {
                piece: PromotedSilver2,
                phase_piece_type: (Second, PromotedSilver),
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_piece_number: 24,
            },
            PromotedKnight2 => PieceStruct {
                piece: PromotedKnight2,
                phase_piece_type: (Second, PromotedKnight),
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_piece_number: 25,
            },
            PromotedLance2 => PieceStruct {
                piece: PromotedLance2,
                phase_piece_type: (Second, PromotedLance),
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_piece_number: 26,
            },
            PromotedPawn2 => PieceStruct {
                piece: PromotedPawn2,
                phase_piece_type: (Second, PromotedPawn),
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_piece_number: 27,
            },
        }
    }

    pub fn from_serial_piece_number(km_num: usize) -> Option<Self> {
        use crate::universe::game::piece::piece::Piece::*;
        match km_num {
            0 => Some(PieceStruct::from_piece(King1)),
            1 => Some(PieceStruct::from_piece(Rook1)),
            2 => Some(PieceStruct::from_piece(Bishop1)),
            3 => Some(PieceStruct::from_piece(Gold1)),
            4 => Some(PieceStruct::from_piece(Silver1)),
            5 => Some(PieceStruct::from_piece(Knight1)),
            6 => Some(PieceStruct::from_piece(Lance1)),
            7 => Some(PieceStruct::from_piece(Pawn1)),
            8 => Some(PieceStruct::from_piece(Dragon1)),
            9 => Some(PieceStruct::from_piece(Horse1)),
            10 => Some(PieceStruct::from_piece(PromotedSilver1)),
            11 => Some(PieceStruct::from_piece(PromotedKnight1)),
            12 => Some(PieceStruct::from_piece(PromotedLance1)),
            13 => Some(PieceStruct::from_piece(PromotedPawn1)),
            14 => Some(PieceStruct::from_piece(King2)),
            15 => Some(PieceStruct::from_piece(Rook2)),
            16 => Some(PieceStruct::from_piece(Bishop2)),
            17 => Some(PieceStruct::from_piece(Gold2)),
            18 => Some(PieceStruct::from_piece(Silver2)),
            19 => Some(PieceStruct::from_piece(Knight2)),
            20 => Some(PieceStruct::from_piece(Lance2)),
            21 => Some(PieceStruct::from_piece(Pawn2)),
            22 => Some(PieceStruct::from_piece(Dragon2)),
            23 => Some(PieceStruct::from_piece(Horse2)),
            24 => Some(PieceStruct::from_piece(PromotedSilver2)),
            25 => Some(PieceStruct::from_piece(PromotedKnight2)),
            26 => Some(PieceStruct::from_piece(PromotedLance2)),
            27 => Some(PieceStruct::from_piece(PromotedPawn2)),
            _ => None,
        }
    }

    /// ハッシュ値から作る
    pub fn from_hash(hash: u64) -> (u64, Option<Self>) {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        let ps_o = PieceStruct::from_serial_piece_number((hash & 0b11111) as usize);
        (hash >> 5, ps_o)
    }

    pub fn phase(&self) -> Phase {
        self.phase_piece_type.0.clone()
    }

    pub fn piece_type(&self) -> PieceType {
        self.phase_piece_type.1
    }

    // 降格できるか。
    pub fn can_demote(&self) -> bool {
        // 降格後の駒が、今の駒と異なっていれば、降格できるぜ☆（＾～＾）
        self.piece != self.demoted
    }

    /// 駒の一致比較
    pub fn equals_piece(&self, b: &PieceStruct) -> bool {
        self.serial_piece_number == b.serial_piece_number
    }

    /// 駒種類→｛　成駒,（不成駒、それ以外）　｝
    pub fn is_promoted(&self) -> bool {
        use crate::model::univ::gam::misc::piece_type::PieceType::*;
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
        }
    }

    pub fn is_promotable(&self) -> bool {
        self.piece != self.promoted
    }

    /// ハッシュ値を作る
    pub fn add_hash(&self, hash: u64) -> u64 {
        // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
        (hash << 5) + self.serial_piece_number as u64
    }
}
