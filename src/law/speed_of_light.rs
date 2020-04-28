//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 要は早引きのデータベースみたいなもんだな☆（＾～＾）
//!
//! 駒早見表 (PieceChart),
//! 駒種類早見表 (PieceTypeChart).
//!
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::HAND_ADDRESS_LEN;
use crate::cosmic::smart::features::{HandAddress, HandAddressType, PieceMeaning, PieceType};
use crate::cosmic::smart::square::{Angle, RelAdr, ANGLE_LEN};
use num_traits::FromPrimitive;

pub struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    /// イミュータブルなのだから、直接参照してもいい☆（＾～＾）
    /// 先後付きの駒☆（＾～＾）
    pub king1: PieceChart,
    pub rook1: PieceChart,
    pub bishop1: PieceChart,
    pub gold1: PieceChart,
    pub silver1: PieceChart,
    pub knight1: PieceChart,
    pub lance1: PieceChart,
    pub pawn1: PieceChart,
    pub promoted_rook1: PieceChart,
    pub promoted_bishop1: PieceChart,
    pub promoted_silver1: PieceChart,
    pub promoted_knight1: PieceChart,
    pub promoted_lance1: PieceChart,
    pub promoted_pawn1: PieceChart,
    pub king2: PieceChart,
    pub rook2: PieceChart,
    pub bishop2: PieceChart,
    pub gold2: PieceChart,
    pub silver2: PieceChart,
    pub knight2: PieceChart,
    pub lance2: PieceChart,
    pub pawn2: PieceChart,
    pub promoted_rook2: PieceChart,
    pub promoted_bishop2: PieceChart,
    pub promoted_silver2: PieceChart,
    pub promoted_knight2: PieceChart,
    pub promoted_lance2: PieceChart,
    pub promoted_pawn2: PieceChart,

    /// 駒種類☆（＾～＾）
    pub king: PieceTypeChart,
    pub rook: PieceTypeChart,
    pub bishop: PieceTypeChart,
    pub gold: PieceTypeChart,
    pub silver: PieceTypeChart,
    pub knight: PieceTypeChart,
    pub lance: PieceTypeChart,
    pub pawn: PieceTypeChart,
    pub promoted_rook: PieceTypeChart,
    pub promoted_bishop: PieceTypeChart,
    pub promoted_silver: PieceTypeChart,
    pub promoted_knight: PieceTypeChart,
    pub promoted_lance: PieceTypeChart,
    pub promoted_pawn: PieceTypeChart,

    pub hand_address_table: [HandAddressChart; HAND_ADDRESS_LEN],

    // 相対番地と角度☆（＾～＾）
    pub west_ccw: [RelAdr; ANGLE_LEN],
    pub west_ccw_double_rank: [RelAdr; ANGLE_LEN],
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::cosmic::smart::features::PieceMeaning::*;
        use crate::cosmic::smart::features::PieceType::*;
        SpeedOfLight {
            king1: PieceChart::from_piece(King1),
            rook1: PieceChart::from_piece(Rook1),
            bishop1: PieceChart::from_piece(Bishop1),
            gold1: PieceChart::from_piece(Gold1),
            silver1: PieceChart::from_piece(Silver1),
            knight1: PieceChart::from_piece(Knight1),
            lance1: PieceChart::from_piece(Lance1),
            pawn1: PieceChart::from_piece(Pawn1),
            promoted_rook1: PieceChart::from_piece(Dragon1),
            promoted_bishop1: PieceChart::from_piece(Horse1),
            promoted_silver1: PieceChart::from_piece(PromotedSilver1),
            promoted_knight1: PieceChart::from_piece(PromotedKnight1),
            promoted_lance1: PieceChart::from_piece(PromotedLance1),
            promoted_pawn1: PieceChart::from_piece(PromotedPawn1),
            king2: PieceChart::from_piece(King2),
            rook2: PieceChart::from_piece(Rook2),
            bishop2: PieceChart::from_piece(Bishop2),
            gold2: PieceChart::from_piece(Gold2),
            silver2: PieceChart::from_piece(Silver2),
            knight2: PieceChart::from_piece(Knight2),
            lance2: PieceChart::from_piece(Lance2),
            pawn2: PieceChart::from_piece(Pawn2),
            promoted_rook2: PieceChart::from_piece(Dragon2),
            promoted_bishop2: PieceChart::from_piece(Horse2),
            promoted_silver2: PieceChart::from_piece(PromotedSilver2),
            promoted_knight2: PieceChart::from_piece(PromotedKnight2),
            promoted_lance2: PieceChart::from_piece(PromotedLance2),
            promoted_pawn2: PieceChart::from_piece(PromotedPawn2),
            king: PieceTypeChart::from_piece_type(King),
            rook: PieceTypeChart::from_piece_type(Rook),
            bishop: PieceTypeChart::from_piece_type(Bishop),
            gold: PieceTypeChart::from_piece_type(Gold),
            silver: PieceTypeChart::from_piece_type(Silver),
            knight: PieceTypeChart::from_piece_type(Knight),
            lance: PieceTypeChart::from_piece_type(Lance),
            pawn: PieceTypeChart::from_piece_type(Pawn),
            promoted_rook: PieceTypeChart::from_piece_type(Dragon),
            promoted_bishop: PieceTypeChart::from_piece_type(Horse),
            promoted_silver: PieceTypeChart::from_piece_type(PromotedSilver),
            promoted_knight: PieceTypeChart::from_piece_type(PromotedKnight),
            promoted_lance: PieceTypeChart::from_piece_type(PromotedLance),
            promoted_pawn: PieceTypeChart::from_piece_type(PromotedPawn),

            hand_address_table: [
                HandAddressChart::new(HandAddress::King1),
                HandAddressChart::new(HandAddress::Rook1),
                HandAddressChart::new(HandAddress::Bishop1),
                HandAddressChart::new(HandAddress::Gold1),
                HandAddressChart::new(HandAddress::Silver1),
                HandAddressChart::new(HandAddress::Knight1),
                HandAddressChart::new(HandAddress::Lance1),
                HandAddressChart::new(HandAddress::Pawn1),
                HandAddressChart::new(HandAddress::King2),
                HandAddressChart::new(HandAddress::Rook2),
                HandAddressChart::new(HandAddress::Bishop2),
                HandAddressChart::new(HandAddress::Gold2),
                HandAddressChart::new(HandAddress::Silver2),
                HandAddressChart::new(HandAddress::Knight2),
                HandAddressChart::new(HandAddress::Lance2),
                HandAddressChart::new(HandAddress::Pawn2),
            ],

            // よく使う、角度の付いた相対番地☆（＾～＾）
            west_ccw: [
                RelAdr::new(1, 0),
                RelAdr::new(1, 0).rotate(Angle::Ccw45).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw90).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw135).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw180).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw225).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw270).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw315).clone(),
            ],
            west_ccw_double_rank: [
                RelAdr::new(1, 0).double_rank().clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw45).double_rank().clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw90).double_rank().clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw135)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw180)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw225)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw270)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw315)
                    .double_rank()
                    .clone(),
            ],
        }
    }
}

impl SpeedOfLight {
    /// 駒の属性を参照するぜ☆（＾～＾）
    fn piece_chart(&self, piece: &PieceMeaning) -> &PieceChart {
        use crate::cosmic::smart::features::PieceMeaning::*;

        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.pieces[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        match *piece {
            King1 => &self.king1,
            Rook1 => &self.rook1,
            Bishop1 => &self.bishop1,
            Gold1 => &self.gold1,
            Silver1 => &self.silver1,
            Knight1 => &self.knight1,
            Lance1 => &self.lance1,
            Pawn1 => &self.pawn1,
            Dragon1 => &self.promoted_rook1,
            Horse1 => &self.promoted_bishop1,
            PromotedSilver1 => &self.promoted_silver1,
            PromotedKnight1 => &self.promoted_knight1,
            PromotedLance1 => &self.promoted_lance1,
            PromotedPawn1 => &self.promoted_pawn1,
            King2 => &self.king2,
            Rook2 => &self.rook2,
            Bishop2 => &self.bishop2,
            Gold2 => &self.gold2,
            Silver2 => &self.silver2,
            Knight2 => &self.knight2,
            Lance2 => &self.lance2,
            Pawn2 => &self.pawn2,
            Dragon2 => &self.promoted_rook2,
            Horse2 => &self.promoted_bishop2,
            PromotedSilver2 => &self.promoted_silver2,
            PromotedKnight2 => &self.promoted_knight2,
            PromotedLance2 => &self.promoted_lance2,
            PromotedPawn2 => &self.promoted_pawn2,
        }
    }

    /// 駒の属性を参照するぜ☆（＾～＾）
    fn piece_type_chart(&self, piece_type: &PieceType) -> &PieceTypeChart {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.pieces[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        use crate::cosmic::smart::features::PieceType::*;
        match *piece_type {
            King => &self.king,
            Rook => &self.rook,
            Bishop => &self.bishop,
            Gold => &self.gold,
            Silver => &self.silver,
            Knight => &self.knight,
            Lance => &self.lance,
            Pawn => &self.pawn,
            Dragon => &self.promoted_rook,
            Horse => &self.promoted_bishop,
            PromotedSilver => &self.promoted_silver,
            PromotedKnight => &self.promoted_knight,
            PromotedLance => &self.promoted_lance,
            PromotedPawn => &self.promoted_pawn,
        }
    }

    /// 持ち駒の型☆（＾～＾）
    fn hand_address_chart(&self, adr: HandAddress) -> &HandAddressChart {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.pieces[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        &self.hand_address_table[adr as usize]
    }

    pub fn west_ccw(&self, angle: Angle) -> &RelAdr {
        &self.west_ccw[angle as usize]
    }

    pub fn west_ccw_double_rank(&self, angle: Angle) -> &RelAdr {
        &self.west_ccw_double_rank[angle as usize]
    }
}

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
/// アプリ起動時に全種類作って Enum型 で取得するようにした方がよくないか☆（＾～＾）？
#[derive(Clone)]
pub struct PieceChart {
    pub piece: PieceMeaning,

    /// 先後
    phase: Phase,

    /// 駒種類
    piece_type: PieceType,

    /// 駒→成駒　（成れない駒は、そのまま）Noneは空升に使っている☆（＾～＾）
    promoted: PieceMeaning,

    /// 成駒→駒　（成っていない駒は、そのまま）Noneは空升に使っている☆（＾～＾）
    demoted: PieceMeaning,

    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    captured: PieceMeaning,

    /// 配列のインデックス用☆（＾～＾）
    hand_address: HandAddress,
}
impl PieceChart {
    /// ピースの生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
    fn from_piece(p: PieceMeaning) -> Self {
        use crate::cosmic::recording::Phase::*;
        use crate::cosmic::smart::features::PieceMeaning::*;
        use crate::cosmic::smart::features::PieceType::*;
        match p {
            King1 => PieceChart {
                piece: King1,
                phase: First,
                piece_type: King,
                promoted: King1,
                demoted: King1,
                captured: King2,
                hand_address: HandAddress::King1,
            },
            Rook1 => PieceChart {
                piece: Rook1,
                phase: First,
                piece_type: Rook,
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                hand_address: HandAddress::Rook1,
            },
            Bishop1 => PieceChart {
                piece: Bishop1,
                phase: First,
                piece_type: Bishop,
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                hand_address: HandAddress::Bishop1,
            },
            Gold1 => PieceChart {
                piece: Gold1,
                phase: First,
                piece_type: Gold,
                promoted: Gold1,
                demoted: Gold1,
                captured: Gold2,
                hand_address: HandAddress::Gold1,
            },
            Silver1 => PieceChart {
                piece: Silver1,
                phase: First,
                piece_type: Silver,
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                hand_address: HandAddress::Silver1,
            },
            Knight1 => PieceChart {
                piece: Knight1,
                phase: First,
                piece_type: Knight,
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                hand_address: HandAddress::Knight1,
            },
            Lance1 => PieceChart {
                piece: Lance1,
                phase: First,
                piece_type: Lance,
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                hand_address: HandAddress::Lance1,
            },
            Pawn1 => PieceChart {
                piece: Pawn1,
                phase: First,
                piece_type: Pawn,
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                hand_address: HandAddress::Pawn1,
            },
            Dragon1 => PieceChart {
                piece: Dragon1,
                phase: First,
                piece_type: Dragon,
                promoted: Dragon1,
                demoted: Rook1,
                captured: Rook2,
                hand_address: HandAddress::Rook1,
            },
            Horse1 => PieceChart {
                piece: Horse1,
                phase: First,
                piece_type: Horse,
                promoted: Horse1,
                demoted: Bishop1,
                captured: Bishop2,
                hand_address: HandAddress::Bishop1,
            },
            PromotedSilver1 => PieceChart {
                piece: PromotedSilver1,
                phase: First,
                piece_type: PromotedSilver,
                promoted: PromotedSilver1,
                demoted: Silver1,
                captured: Silver2,
                hand_address: HandAddress::Silver1,
            },
            PromotedKnight1 => PieceChart {
                piece: PromotedKnight1,
                phase: First,
                piece_type: PromotedKnight,
                promoted: PromotedKnight1,
                demoted: Knight1,
                captured: Knight2,
                hand_address: HandAddress::Knight1,
            },
            PromotedLance1 => PieceChart {
                piece: PromotedLance1,
                phase: First,
                piece_type: PromotedLance,
                promoted: PromotedLance1,
                demoted: Lance1,
                captured: Lance2,
                hand_address: HandAddress::Lance1,
            },
            PromotedPawn1 => PieceChart {
                piece: PromotedPawn1,
                phase: First,
                piece_type: PromotedPawn,
                promoted: PromotedPawn1,
                demoted: Pawn1,
                captured: Pawn2,
                hand_address: HandAddress::Pawn1,
            },
            King2 => PieceChart {
                piece: King2,
                phase: Second,
                piece_type: King,
                promoted: King2,
                demoted: King2,
                captured: King1,
                hand_address: HandAddress::King2,
            },
            Rook2 => PieceChart {
                piece: Rook2,
                phase: Second,
                piece_type: Rook,
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                hand_address: HandAddress::Rook2,
            },
            Bishop2 => PieceChart {
                piece: Bishop2,
                phase: Second,
                piece_type: Bishop,
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                hand_address: HandAddress::Bishop2,
            },
            Gold2 => PieceChart {
                piece: Gold2,
                phase: Second,
                piece_type: Gold,
                promoted: Gold2,
                demoted: Gold2,
                captured: Gold1,
                hand_address: HandAddress::Gold2,
            },
            Silver2 => PieceChart {
                piece: Silver2,
                phase: Second,
                piece_type: Silver,
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                hand_address: HandAddress::Silver2,
            },
            Knight2 => PieceChart {
                piece: Knight2,
                phase: Second,
                piece_type: Knight,
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                hand_address: HandAddress::Knight2,
            },
            Lance2 => PieceChart {
                piece: Lance2,
                phase: Second,
                piece_type: Lance,
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                hand_address: HandAddress::Lance2,
            },
            Pawn2 => PieceChart {
                piece: Pawn2,
                phase: Second,
                piece_type: Pawn,
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                hand_address: HandAddress::Pawn2,
            },
            Dragon2 => PieceChart {
                piece: Dragon2,
                phase: Second,
                piece_type: Dragon,
                promoted: Dragon2,
                demoted: Rook2,
                captured: Rook1,
                hand_address: HandAddress::Rook2,
            },
            Horse2 => PieceChart {
                piece: Horse2,
                phase: Second,
                piece_type: Horse,
                promoted: Horse2,
                demoted: Bishop2,
                captured: Bishop1,
                hand_address: HandAddress::Bishop2,
            },
            PromotedSilver2 => PieceChart {
                piece: PromotedSilver2,
                phase: Second,
                piece_type: PromotedSilver,
                promoted: PromotedSilver2,
                demoted: Silver2,
                captured: Silver1,
                hand_address: HandAddress::Silver2,
            },
            PromotedKnight2 => PieceChart {
                piece: PromotedKnight2,
                phase: Second,
                piece_type: PromotedKnight,
                promoted: PromotedKnight2,
                demoted: Knight2,
                captured: Knight1,
                hand_address: HandAddress::Knight2,
            },
            PromotedLance2 => PieceChart {
                piece: PromotedLance2,
                phase: Second,
                piece_type: PromotedLance,
                promoted: PromotedLance2,
                demoted: Lance2,
                captured: Lance1,
                hand_address: HandAddress::Lance2,
            },
            PromotedPawn2 => PieceChart {
                piece: PromotedPawn2,
                phase: Second,
                piece_type: PromotedPawn,
                promoted: PromotedPawn2,
                demoted: Pawn2,
                captured: Pawn1,
                hand_address: HandAddress::Pawn2,
            },
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceMeaning {
    pub fn phase(&self, speed_of_light: &SpeedOfLight) -> Phase {
        speed_of_light.piece_chart(self).phase
    }

    pub fn r#type(&self, speed_of_light: &SpeedOfLight) -> PieceType {
        speed_of_light.piece_chart(self).piece_type
    }

    pub fn promoted(&self, speed_of_light: &SpeedOfLight) -> PieceMeaning {
        speed_of_light.piece_chart(self).promoted
    }

    pub fn demoted(&self, speed_of_light: &SpeedOfLight) -> PieceMeaning {
        speed_of_light.piece_chart(self).demoted
    }

    pub fn captured(&self, speed_of_light: &SpeedOfLight) -> PieceMeaning {
        speed_of_light.piece_chart(self).captured
    }

    pub fn hand_address(&self, speed_of_light: &SpeedOfLight) -> HandAddress {
        speed_of_light.piece_chart(self).hand_address
    }
}

pub struct PieceTypeChart {
    /// 成り駒か。
    promoted: bool,
}
impl PieceTypeChart {
    fn from_piece_type(piece_type: PieceType) -> Self {
        use crate::cosmic::smart::features::PieceType::*;
        match piece_type {
            King => PieceTypeChart { promoted: false },
            Rook => PieceTypeChart { promoted: false },
            Bishop => PieceTypeChart { promoted: false },
            Gold => PieceTypeChart { promoted: false },
            Silver => PieceTypeChart { promoted: false },
            Knight => PieceTypeChart { promoted: false },
            Lance => PieceTypeChart { promoted: false },
            Pawn => PieceTypeChart { promoted: false },
            Dragon => PieceTypeChart { promoted: true },
            Horse => PieceTypeChart { promoted: true },
            PromotedSilver => PieceTypeChart { promoted: true },
            PromotedKnight => PieceTypeChart { promoted: true },
            PromotedLance => PieceTypeChart { promoted: true },
            PromotedPawn => PieceTypeChart { promoted: true },
        }
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceType {
    pub fn promoted(&self, speed_of_light: &SpeedOfLight) -> bool {
        speed_of_light.piece_type_chart(self).promoted
    }
}

pub struct HandAddressChart {
    /// 配列のインデックス用☆（＾～＾）
    r#type: HandAddressType,
}
impl HandAddressChart {
    fn new(adr: HandAddress) -> Self {
        use crate::cosmic::smart::features::HandAddress::*;
        match adr {
            King1 | King2 => HandAddressChart {
                r#type: HandAddressType::King,
            },
            Rook1 | Rook2 => HandAddressChart {
                r#type: HandAddressType::Rook,
            },
            Bishop1 | Bishop2 => HandAddressChart {
                r#type: HandAddressType::Bishop,
            },
            Gold1 | Gold2 => HandAddressChart {
                r#type: HandAddressType::Gold,
            },
            Silver1 | Silver2 => HandAddressChart {
                r#type: HandAddressType::Silver,
            },
            Knight1 | Knight2 => HandAddressChart {
                r#type: HandAddressType::Knight,
            },
            Lance1 | Lance2 => HandAddressChart {
                r#type: HandAddressType::Lance,
            },
            Pawn1 | Pawn2 => HandAddressChart {
                r#type: HandAddressType::Pawn,
            },
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
impl HandAddress {
    pub fn r#type(&self, speed_of_light: &SpeedOfLight) -> HandAddressType {
        speed_of_light.hand_address_chart(*self).r#type
    }
}

/// ハッシュ値を作る
pub fn push_drop_to_hash(hash: u64, piece_type_o: Option<HandAddressType>) -> u64 {
    let num = if let Some(piece_type) = piece_type_o {
        // 持ち駒の型は 7つ ＋ 持ち駒無しの 1つ なんで、8(=2^3) で OK
        piece_type as u64
    } else {
        // None の変わりに 玉を使うぜ☆（＾～＾）
        HandAddressType::King as u64
    };
    (hash << 3) + num
}

/// ハッシュ値から作る
pub fn pop_drop_from_hash(hash: u64) -> (u64, Option<HandAddressType>) {
    // 使ってるのは8種類なんで、8(=2^3) で OK
    (hash >> 3, HandAddressType::from_u64(hash & 0b111))
}
