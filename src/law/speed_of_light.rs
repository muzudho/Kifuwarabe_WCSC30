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
use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::smart::features::HAND_ADDRESS_LEN;
use crate::cosmic::smart::features::HAND_ADDRESS_TYPE_LEN;
use crate::cosmic::smart::features::PIECE_MEANING_LEN;
use crate::cosmic::smart::features::PIECE_TYPE_LEN;
use crate::cosmic::smart::features::{HandAddress, HandAddressType, PieceMeaning, PieceType};
use crate::cosmic::smart::square::{Angle, RelAdr, ANGLE_LEN};
use num_traits::FromPrimitive;
// use std::sync::Mutex;

// グローバル定数
//
// 使い方（lazy_static!マクロ）
// ============================
// 定数の値を実行時に決めることができる。
//
// Cargo.toml に１行追記
// > [dependencies]
// > lazy_static = "1.0.0"
//
// main.rs の冒頭あたりに次の２行を記述
// > #[macro_use]
// > extern crate lazy_static;
//
// 「How can I use mutable lazy_static?」
// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
lazy_static! {
    /// ９桁の有効数字☆（＾～＾）
    static ref NINE_299792458: SpeedOfLight = {
        SpeedOfLight::default()
    };
    /*
    static ref NINE_299792458: Mutex<SpeedOfLight> = {
        Mutex::new(SpeedOfLight::default())
    };
    */
}

/// こいつが早引き表なわけだぜ☆（＾～＾）
pub struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）

    /// 先後付きの駒☆（＾～＾）
    pub piece_meaning_table: [PieceMeaningChart; PIECE_MEANING_LEN],

    /// 駒種類☆（＾～＾）
    pub piece_type_table: [PieceTypeChart; PIECE_TYPE_LEN],

    /// 持ち駒☆（＾～＾）
    pub hand_addresses: [[HandAddress; HAND_ADDRESS_TYPE_LEN]; PHASE_LEN],
    pub hand_address_table: [HandAddressChart; HAND_ADDRESS_LEN],

    // 相対番地と角度☆（＾～＾）
    pub west_ccw: [RelAdr; ANGLE_LEN],
    pub west_ccw_double_rank: [RelAdr; ANGLE_LEN],

    /// 時計回り(Clockwise)☆（＾～＾）
    pub rotate90cw: [Angle; ANGLE_LEN],
    /// 時計回り(Clockwise)☆（＾～＾）
    pub rotate45cw: [Angle; ANGLE_LEN],
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub rotate45ccw: [Angle; ANGLE_LEN],
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub rotate90ccw: [Angle; ANGLE_LEN],
    /// 点対称☆（＾～＾）
    pub rotate180: [Angle; ANGLE_LEN],

    /// 評価値☆（＾～＾）
    /// 成らないよりは、成った方がお得という、それだけの差を付けるだけの加点だぜ☆（＾～＾）
    /// 大きくすると、歩と交換に角が成り込むぜ☆（＾～＾）
    pub promotion_value: [isize; HAND_ADDRESS_TYPE_LEN],
    pub caputured_piece_value: [isize; HAND_ADDRESS_TYPE_LEN],
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::cosmic::recording::Phase::*;
        use crate::cosmic::smart::features::PieceMeaning::*;
        use crate::cosmic::smart::features::PieceType::*;
        SpeedOfLight {
            /// ピースの早見表の生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
            /// 先後付きの駒☆（＾～＾）
            piece_meaning_table: [
                PieceMeaningChart {
                    piece: King1,
                    phase: First,
                    piece_type: King,
                    promoted: King1,
                    demoted: King1,
                    captured: King2,
                    hand_address: HandAddress::King1,
                },
                PieceMeaningChart {
                    piece: Rook1,
                    phase: First,
                    piece_type: Rook,
                    promoted: Dragon1,
                    demoted: Rook1,
                    captured: Rook2,
                    hand_address: HandAddress::Rook1,
                },
                PieceMeaningChart {
                    piece: Bishop1,
                    phase: First,
                    piece_type: Bishop,
                    promoted: Horse1,
                    demoted: Bishop1,
                    captured: Bishop2,
                    hand_address: HandAddress::Bishop1,
                },
                PieceMeaningChart {
                    piece: Gold1,
                    phase: First,
                    piece_type: Gold,
                    promoted: Gold1,
                    demoted: Gold1,
                    captured: Gold2,
                    hand_address: HandAddress::Gold1,
                },
                PieceMeaningChart {
                    piece: Silver1,
                    phase: First,
                    piece_type: Silver,
                    promoted: PromotedSilver1,
                    demoted: Silver1,
                    captured: Silver2,
                    hand_address: HandAddress::Silver1,
                },
                PieceMeaningChart {
                    piece: Knight1,
                    phase: First,
                    piece_type: Knight,
                    promoted: PromotedKnight1,
                    demoted: Knight1,
                    captured: Knight2,
                    hand_address: HandAddress::Knight1,
                },
                PieceMeaningChart {
                    piece: Lance1,
                    phase: First,
                    piece_type: Lance,
                    promoted: PromotedLance1,
                    demoted: Lance1,
                    captured: Lance2,
                    hand_address: HandAddress::Lance1,
                },
                PieceMeaningChart {
                    piece: Pawn1,
                    phase: First,
                    piece_type: Pawn,
                    promoted: PromotedPawn1,
                    demoted: Pawn1,
                    captured: Pawn2,
                    hand_address: HandAddress::Pawn1,
                },
                PieceMeaningChart {
                    piece: Dragon1,
                    phase: First,
                    piece_type: Dragon,
                    promoted: Dragon1,
                    demoted: Rook1,
                    captured: Rook2,
                    hand_address: HandAddress::Rook1,
                },
                PieceMeaningChart {
                    piece: Horse1,
                    phase: First,
                    piece_type: Horse,
                    promoted: Horse1,
                    demoted: Bishop1,
                    captured: Bishop2,
                    hand_address: HandAddress::Bishop1,
                },
                PieceMeaningChart {
                    piece: PromotedSilver1,
                    phase: First,
                    piece_type: PromotedSilver,
                    promoted: PromotedSilver1,
                    demoted: Silver1,
                    captured: Silver2,
                    hand_address: HandAddress::Silver1,
                },
                PieceMeaningChart {
                    piece: PromotedKnight1,
                    phase: First,
                    piece_type: PromotedKnight,
                    promoted: PromotedKnight1,
                    demoted: Knight1,
                    captured: Knight2,
                    hand_address: HandAddress::Knight1,
                },
                PieceMeaningChart {
                    piece: PromotedLance1,
                    phase: First,
                    piece_type: PromotedLance,
                    promoted: PromotedLance1,
                    demoted: Lance1,
                    captured: Lance2,
                    hand_address: HandAddress::Lance1,
                },
                PieceMeaningChart {
                    piece: PromotedPawn1,
                    phase: First,
                    piece_type: PromotedPawn,
                    promoted: PromotedPawn1,
                    demoted: Pawn1,
                    captured: Pawn2,
                    hand_address: HandAddress::Pawn1,
                },
                PieceMeaningChart {
                    piece: King2,
                    phase: Second,
                    piece_type: King,
                    promoted: King2,
                    demoted: King2,
                    captured: King1,
                    hand_address: HandAddress::King2,
                },
                PieceMeaningChart {
                    piece: Rook2,
                    phase: Second,
                    piece_type: Rook,
                    promoted: Dragon2,
                    demoted: Rook2,
                    captured: Rook1,
                    hand_address: HandAddress::Rook2,
                },
                PieceMeaningChart {
                    piece: Bishop2,
                    phase: Second,
                    piece_type: Bishop,
                    promoted: Horse2,
                    demoted: Bishop2,
                    captured: Bishop1,
                    hand_address: HandAddress::Bishop2,
                },
                PieceMeaningChart {
                    piece: Gold2,
                    phase: Second,
                    piece_type: Gold,
                    promoted: Gold2,
                    demoted: Gold2,
                    captured: Gold1,
                    hand_address: HandAddress::Gold2,
                },
                PieceMeaningChart {
                    piece: Silver2,
                    phase: Second,
                    piece_type: Silver,
                    promoted: PromotedSilver2,
                    demoted: Silver2,
                    captured: Silver1,
                    hand_address: HandAddress::Silver2,
                },
                PieceMeaningChart {
                    piece: Knight2,
                    phase: Second,
                    piece_type: Knight,
                    promoted: PromotedKnight2,
                    demoted: Knight2,
                    captured: Knight1,
                    hand_address: HandAddress::Knight2,
                },
                PieceMeaningChart {
                    piece: Lance2,
                    phase: Second,
                    piece_type: Lance,
                    promoted: PromotedLance2,
                    demoted: Lance2,
                    captured: Lance1,
                    hand_address: HandAddress::Lance2,
                },
                PieceMeaningChart {
                    piece: Pawn2,
                    phase: Second,
                    piece_type: Pawn,
                    promoted: PromotedPawn2,
                    demoted: Pawn2,
                    captured: Pawn1,
                    hand_address: HandAddress::Pawn2,
                },
                PieceMeaningChart {
                    piece: Dragon2,
                    phase: Second,
                    piece_type: Dragon,
                    promoted: Dragon2,
                    demoted: Rook2,
                    captured: Rook1,
                    hand_address: HandAddress::Rook2,
                },
                PieceMeaningChart {
                    piece: Horse2,
                    phase: Second,
                    piece_type: Horse,
                    promoted: Horse2,
                    demoted: Bishop2,
                    captured: Bishop1,
                    hand_address: HandAddress::Bishop2,
                },
                PieceMeaningChart {
                    piece: PromotedSilver2,
                    phase: Second,
                    piece_type: PromotedSilver,
                    promoted: PromotedSilver2,
                    demoted: Silver2,
                    captured: Silver1,
                    hand_address: HandAddress::Silver2,
                },
                PieceMeaningChart {
                    piece: PromotedKnight2,
                    phase: Second,
                    piece_type: PromotedKnight,
                    promoted: PromotedKnight2,
                    demoted: Knight2,
                    captured: Knight1,
                    hand_address: HandAddress::Knight2,
                },
                PieceMeaningChart {
                    piece: PromotedLance2,
                    phase: Second,
                    piece_type: PromotedLance,
                    promoted: PromotedLance2,
                    demoted: Lance2,
                    captured: Lance1,
                    hand_address: HandAddress::Lance2,
                },
                PieceMeaningChart {
                    piece: PromotedPawn2,
                    phase: Second,
                    piece_type: PromotedPawn,
                    promoted: PromotedPawn2,
                    demoted: Pawn2,
                    captured: Pawn1,
                    hand_address: HandAddress::Pawn2,
                },
            ],

            // 駒種類☆（＾～＾）
            piece_type_table: [
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: false },
                PieceTypeChart { promoted: true },
                PieceTypeChart { promoted: true },
                PieceTypeChart { promoted: true },
                PieceTypeChart { promoted: true },
                PieceTypeChart { promoted: true },
                PieceTypeChart { promoted: true },
            ],

            // 持ち駒☆（＾～＾）
            hand_addresses: [
                [
                    HandAddress::King1,
                    HandAddress::Rook1,
                    HandAddress::Bishop1,
                    HandAddress::Gold1,
                    HandAddress::Silver1,
                    HandAddress::Knight1,
                    HandAddress::Lance1,
                    HandAddress::Pawn1,
                ],
                [
                    HandAddress::King2,
                    HandAddress::Rook2,
                    HandAddress::Bishop2,
                    HandAddress::Gold2,
                    HandAddress::Silver2,
                    HandAddress::Knight2,
                    HandAddress::Lance2,
                    HandAddress::Pawn2,
                ],
            ],

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

            /// 時計回り(Clockwise)☆（＾～＾）
            rotate90cw: [
                Angle::Ccw270,
                Angle::Ccw315,
                Angle::Ccw0,
                Angle::Ccw45,
                Angle::Ccw90,
                Angle::Ccw135,
                Angle::Ccw180,
                Angle::Ccw225,
            ],
            /// 時計回り(Clockwise)☆（＾～＾）
            rotate45cw: [
                Angle::Ccw315,
                Angle::Ccw0,
                Angle::Ccw45,
                Angle::Ccw90,
                Angle::Ccw135,
                Angle::Ccw180,
                Angle::Ccw225,
                Angle::Ccw270,
            ],
            /// 反時計回り(Counterclockwise)☆（＾～＾）
            rotate45ccw: [
                Angle::Ccw45,
                Angle::Ccw90,
                Angle::Ccw135,
                Angle::Ccw180,
                Angle::Ccw225,
                Angle::Ccw270,
                Angle::Ccw315,
                Angle::Ccw0,
            ],
            /// 反時計回り(Counterclockwise)☆（＾～＾）
            rotate90ccw: [
                Angle::Ccw90,
                Angle::Ccw135,
                Angle::Ccw180,
                Angle::Ccw225,
                Angle::Ccw270,
                Angle::Ccw315,
                Angle::Ccw0,
                Angle::Ccw45,
            ],
            rotate180: [
                Angle::Ccw180,
                Angle::Ccw225,
                Angle::Ccw270,
                Angle::Ccw315,
                Angle::Ccw0,
                Angle::Ccw45,
                Angle::Ccw90,
                Angle::Ccw135,
            ],

            // 評価値☆（＾～＾）
            promotion_value: [0, 1, 1, 0, 0, 1, 1, 1],
            caputured_piece_value: [
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                0,
                // 駒割は取ったときにカウントしているので、成りを考慮しないぜ☆（＾～＾）
                1000, 900, 600, 500, 300, 200, 100,
            ],
        }
    }
}

/// いろいろありそうに見えるが、結局のところ３０種類ぐらいしか存在しない☆（＾～＾）
#[derive(Clone)]
pub struct PieceMeaningChart {
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
/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceMeaning {
    pub fn phase(self) -> Phase {
        NINE_299792458.piece_meaning_table[self as usize].phase
    }

    pub fn r#type(self) -> PieceType {
        NINE_299792458.piece_meaning_table[self as usize].piece_type
    }

    pub fn promoted(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_table[self as usize].promoted
    }

    pub fn demoted(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_table[self as usize].demoted
    }

    pub fn captured(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_table[self as usize].captured
    }

    pub fn hand_address(self) -> HandAddress {
        NINE_299792458.piece_meaning_table[self as usize].hand_address
    }
}

pub struct PieceTypeChart {
    /// 成り駒か。
    promoted: bool,
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceType {
    pub fn promoted(self) -> bool {
        NINE_299792458.piece_type_table[self as usize].promoted
    }
}

impl HandAddress {
    pub fn from_phase_and_type(phase: Phase, adr: HandAddressType) -> Self {
        NINE_299792458.hand_addresses[phase as usize][adr as usize]
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
    pub fn r#type(self) -> HandAddressType {
        NINE_299792458.hand_address_table[self as usize].r#type
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

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl HandAddressType {
    pub fn promotion_value(self) -> isize {
        NINE_299792458.promotion_value[self as usize]
    }
    pub fn caputured_piece_value(self) -> isize {
        NINE_299792458.caputured_piece_value[self as usize]
    }
}

impl Angle {
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate90cw(self) -> Angle {
        NINE_299792458.rotate90cw[self as usize]
    }
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate45cw(self) -> Angle {
        NINE_299792458.rotate45cw[self as usize]
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate45ccw(self) -> Angle {
        NINE_299792458.rotate45ccw[self as usize]
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate90ccw(self) -> Angle {
        NINE_299792458.rotate90ccw[self as usize]
    }
    /// 点対称☆（＾～＾）
    pub fn rotate180(self) -> Angle {
        NINE_299792458.rotate180[self as usize]
    }
    pub fn west_ccw_double_rank(self) -> RelAdr {
        NINE_299792458.west_ccw_double_rank[self as usize]
    }
    pub fn west_ccw(self) -> RelAdr {
        NINE_299792458.west_ccw[self as usize]
    }
}
