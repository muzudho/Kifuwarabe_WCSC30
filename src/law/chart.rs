//!
//! 駒早見表 (PieceChart),
//! 駒種類早見表 (PieceTypeChart).
//!
use crate::cosmic::shogi::state::Phase;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::toy_box::Piece;
use crate::law::speed_of_light::SpeedOfLight;

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
/// アプリ起動時に全種類作って Enum型 で取得するようにした方がよくないか☆（＾～＾）？
#[derive(Clone)]
pub struct PieceChart {
    pub piece: Piece,

    /// 先後
    phase: Phase,

    /// 駒種類
    piece_type: PieceType,

    /// 駒→成駒　（成れない駒は、そのまま）Noneは空升に使っている☆（＾～＾）
    promoted: Piece,

    /// 成駒→駒　（成っていない駒は、そのまま）Noneは空升に使っている☆（＾～＾）
    demoted: Piece,

    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    captured: Piece,

    /// 先後付き駒の配列のインデックス
    serial_number: usize,
}
impl PieceChart {
    /// ピースの生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
    pub fn from_piece(p: Piece) -> Self {
        use crate::cosmic::shogi::state::Phase::*;
        use crate::cosmic::smart::features::PieceType::*;
        use crate::cosmic::toy_box::Piece::*;
        match p {
            King1 => PieceChart {
                piece: King1,
                phase: First,
                piece_type: King,
                promoted: King1,
                demoted: King1,
                captured: King2,
                serial_number: 0,
            },
            Rook1 => PieceChart {
                piece: Rook1,
                phase: First,
                piece_type: Rook,
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                serial_number: 1,
            },
            Bishop1 => PieceChart {
                piece: Bishop1,
                phase: First,
                piece_type: Bishop,
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_number: 2,
            },
            Gold1 => PieceChart {
                piece: Gold1,
                phase: First,
                piece_type: Gold,
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                serial_number: 3,
            },
            Silver1 => PieceChart {
                piece: Silver1,
                phase: First,
                piece_type: Silver,
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_number: 4,
            },
            Knight1 => PieceChart {
                piece: Knight1,
                phase: First,
                piece_type: Knight,
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_number: 5,
            },
            Lance1 => PieceChart {
                piece: Lance1,
                phase: First,
                piece_type: Lance,
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_number: 6,
            },
            Pawn1 => PieceChart {
                piece: Pawn1,
                phase: First,
                piece_type: Pawn,
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_number: 7,
            },
            Dragon1 => PieceChart {
                piece: Dragon1,
                phase: First,
                piece_type: Dragon,
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                serial_number: 8,
            },
            Horse1 => PieceChart {
                piece: Horse1,
                phase: First,
                piece_type: Horse,
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                serial_number: 9,
            },
            PromotedSilver1 => PieceChart {
                piece: PromotedSilver1,
                phase: First,
                piece_type: PromotedSilver,
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                serial_number: 10,
            },
            PromotedKnight1 => PieceChart {
                piece: PromotedKnight1,
                phase: First,
                piece_type: PromotedKnight,
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                serial_number: 11,
            },
            PromotedLance1 => PieceChart {
                piece: PromotedLance1,
                phase: First,
                piece_type: PromotedLance,
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                serial_number: 12,
            },
            PromotedPawn1 => PieceChart {
                piece: PromotedPawn1,
                phase: First,
                piece_type: PromotedPawn,
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                serial_number: 13,
            },
            King2 => PieceChart {
                piece: King2,
                phase: Second,
                piece_type: King,
                promoted: King2,
                demoted: King2,
                captured: King1,
                serial_number: 14,
            },
            Rook2 => PieceChart {
                piece: Rook2,
                phase: Second,
                piece_type: Rook,
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                serial_number: 15,
            },
            Bishop2 => PieceChart {
                piece: Bishop2,
                phase: Second,
                piece_type: Bishop,
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_number: 16,
            },
            Gold2 => PieceChart {
                piece: Gold2,
                phase: Second,
                piece_type: Gold,
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                serial_number: 17,
            },
            Silver2 => PieceChart {
                piece: Silver2,
                phase: Second,
                piece_type: Silver,
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_number: 18,
            },
            Knight2 => PieceChart {
                piece: Knight2,
                phase: Second,
                piece_type: Knight,
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_number: 19,
            },
            Lance2 => PieceChart {
                piece: Lance2,
                phase: Second,
                piece_type: Lance,
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_number: 20,
            },
            Pawn2 => PieceChart {
                piece: Pawn2,
                phase: Second,
                piece_type: Pawn,
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_number: 21,
            },
            Dragon2 => PieceChart {
                piece: Dragon2,
                phase: Second,
                piece_type: Dragon,
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                serial_number: 22,
            },
            Horse2 => PieceChart {
                piece: Horse2,
                phase: Second,
                piece_type: Horse,
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                serial_number: 23,
            },
            PromotedSilver2 => PieceChart {
                piece: PromotedSilver2,
                phase: Second,
                piece_type: PromotedSilver,
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                serial_number: 24,
            },
            PromotedKnight2 => PieceChart {
                piece: PromotedKnight2,
                phase: Second,
                piece_type: PromotedKnight,
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                serial_number: 25,
            },
            PromotedLance2 => PieceChart {
                piece: PromotedLance2,
                phase: Second,
                piece_type: PromotedLance,
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                serial_number: 26,
            },
            PromotedPawn2 => PieceChart {
                piece: PromotedPawn2,
                phase: Second,
                piece_type: PromotedPawn,
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                serial_number: 27,
            },
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Piece {
    pub fn phase(&self, speed_of_light: &SpeedOfLight) -> Phase {
        speed_of_light.piece_chart(self).phase
    }

    pub fn r#type(&self, speed_of_light: &SpeedOfLight) -> PieceType {
        speed_of_light.piece_chart(self).piece_type
    }

    pub fn promoted(&self, speed_of_light: &SpeedOfLight) -> Piece {
        speed_of_light.piece_chart(self).promoted
    }

    pub fn demoted(&self, speed_of_light: &SpeedOfLight) -> Piece {
        speed_of_light.piece_chart(self).demoted
    }

    pub fn captured(&self, speed_of_light: &SpeedOfLight) -> Piece {
        speed_of_light.piece_chart(self).captured
    }

    pub fn serial_number(&self, speed_of_light: &SpeedOfLight) -> usize {
        speed_of_light.piece_chart(self).serial_number
    }
}

pub struct PieceTypeChart {
    /// 配列のインデックス用☆（＾～＾）
    pub serial_number: usize,

    /// 成れる駒種類か。
    pub can_promote: bool,

    /// 打てる駒種類か。
    pub can_drop: bool,

    /// スライダー（長い利きのある駒種類）か☆（＾～＾）
    /// 合い駒で、進路を防ぎえる可能性があれば真
    pub slider: bool,
}
impl PieceTypeChart {
    pub fn from_piece_type(piece_type: PieceType) -> Self {
        use crate::cosmic::smart::features::PieceType::*;
        match piece_type {
            King => PieceTypeChart {
                serial_number: 0,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            Rook => PieceTypeChart {
                serial_number: 1,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Bishop => PieceTypeChart {
                serial_number: 2,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Gold => PieceTypeChart {
                serial_number: 3,
                can_promote: false,
                can_drop: true,
                slider: false,
            },
            Silver => PieceTypeChart {
                serial_number: 4,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Knight => PieceTypeChart {
                serial_number: 5,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Lance => PieceTypeChart {
                serial_number: 6,
                can_promote: true,
                can_drop: true,
                slider: true,
            },
            Pawn => PieceTypeChart {
                serial_number: 7,
                can_promote: true,
                can_drop: true,
                slider: false,
            },
            Dragon => PieceTypeChart {
                serial_number: 8,
                can_promote: false,
                can_drop: false,
                slider: true,
            },
            Horse => PieceTypeChart {
                serial_number: 9,
                can_promote: false,
                can_drop: false,
                slider: true,
            },
            PromotedSilver => PieceTypeChart {
                serial_number: 10,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedKnight => PieceTypeChart {
                serial_number: 11,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedLance => PieceTypeChart {
                serial_number: 12,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
            PromotedPawn => PieceTypeChart {
                serial_number: 13,
                can_promote: false,
                can_drop: false,
                slider: false,
            },
        }
    }
}
