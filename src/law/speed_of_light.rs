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
}

/// こいつが早引き表なわけだぜ☆（＾～＾）
pub struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）

    /// 先後付きの駒☆（＾～＾）
    piece_meaning_phase_table: [Phase; PIECE_MEANING_LEN],
    piece_meaning_type_table: [PieceType; PIECE_MEANING_LEN],
    /// 駒→成駒　（成れない駒は、そのまま）
    piece_meaning_promoted_table: [PieceMeaning; PIECE_MEANING_LEN],
    /// 成駒→駒　（成っていない駒は、そのまま）
    piece_meaning_demoted_table: [PieceMeaning; PIECE_MEANING_LEN],
    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    piece_meaning_captured_table: [PieceMeaning; PIECE_MEANING_LEN],
    piece_meaning_hand_address_table: [HandAddress; PIECE_MEANING_LEN],

    /// 駒種類☆（＾～＾）
    piece_type_table: [PieceTypeChart; PIECE_TYPE_LEN],

    /// 持ち駒☆（＾～＾）
    /// 玉２枚引く☆（＾～＾）
    hand_addresses_legal_all: [HandAddress; HAND_ADDRESS_LEN - 2],
    hand_addresses: [[HandAddress; HAND_ADDRESS_TYPE_LEN]; PHASE_LEN],
    hand_address_table: [HandAddressChart; HAND_ADDRESS_LEN],

    // 相対番地と角度☆（＾～＾）
    west_ccw: [RelAdr; ANGLE_LEN],
    west_ccw_double_rank: [RelAdr; ANGLE_LEN],

    /// 時計回り(Clockwise)☆（＾～＾）
    rotate90cw: [Angle; ANGLE_LEN],
    /// 時計回り(Clockwise)☆（＾～＾）
    rotate45cw: [Angle; ANGLE_LEN],
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    rotate45ccw: [Angle; ANGLE_LEN],
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    rotate90ccw: [Angle; ANGLE_LEN],
    /// 点対称☆（＾～＾）
    rotate180: [Angle; ANGLE_LEN],

    /// 評価値☆（＾～＾）
    /// 成らないよりは、成った方がお得という、それだけの差を付けるだけの加点だぜ☆（＾～＾）
    /// 大きくすると、歩と交換に角が成り込むぜ☆（＾～＾）
    promotion_value: [isize; HAND_ADDRESS_TYPE_LEN],
    caputured_piece_value: [isize; HAND_ADDRESS_TYPE_LEN],
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::cosmic::recording::Phase::*;
        use crate::cosmic::smart::features::PieceMeaning::*;
        use crate::cosmic::smart::features::PieceType::*;
        SpeedOfLight {
            /// ピースの早見表の生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
            /// 先後付きの駒☆（＾～＾）
            piece_meaning_phase_table: [
                First,  // King1
                First,  // Rook1
                First,  // Bishop1
                First,  // Gold1
                First,  // Silver1
                First,  // Knight1
                First,  // Lance1
                First,  // Pawn1
                First,  // Dragon1
                First,  // Horse1
                First,  // PromotedSilver1
                First,  // PromotedKnight1
                First,  // PromotedLance1
                First,  // PromotedPawn1
                Second, // King2
                Second, // Rook2
                Second, // Bishop2
                Second, // Gold2
                Second, // Silver2
                Second, // Knight2
                Second, // Lance2
                Second, // Pawn2
                Second, // Dragon2
                Second, // Horse2
                Second, // PromotedSilver2
                Second, // PromotedKnight2
                Second, // PromotedLance2
                Second, // PromotedPawn2
            ],
            piece_meaning_type_table: [
                King,           // King1
                Rook,           // Rook1
                Bishop,         // Bishop1
                Gold,           // Gold1
                Silver,         // Silver1
                Knight,         // Knight1
                Lance,          // Lance1
                Pawn,           // Pawn1
                Dragon,         // Dragon1
                Horse,          // Horse1
                PromotedSilver, // PromotedSilver1
                PromotedKnight, // PromotedKnight1
                PromotedLance,  // PromotedLance1
                PromotedPawn,   // PromotedPawn1
                King,           // King2
                Rook,           // Rook2
                Bishop,         // Bishop2
                Gold,           // Gold2
                Silver,         // Silver2
                Knight,         // Knight2
                Lance,          // Lance2
                Pawn,           // Pawn2
                Dragon,         // Dragon2
                Horse,          // Horse2
                PromotedSilver, // PromotedSilver2
                PromotedKnight, // PromotedKnight2
                PromotedLance,  // PromotedLance2
                PromotedPawn,   // PromotedPawn2
            ],
            piece_meaning_promoted_table: [
                King1,           // King1
                Dragon1,         // Rook1
                Horse1,          // Bishop1
                Gold1,           // Gold1
                PromotedSilver1, // Silver1
                PromotedKnight1, // Knight1
                PromotedLance1,  // Lance1
                PromotedPawn1,   // Pawn1
                Dragon1,         // Dragon1
                Horse1,          // Horse1
                PromotedSilver1, // PromotedSilver1
                PromotedKnight1, // PromotedKnight1
                PromotedLance1,  // PromotedLance1
                PromotedPawn1,   // PromotedPawn1
                King2,           // King2
                Dragon2,         // Rook2
                Horse2,          // Bishop2
                Gold2,           // Gold2
                PromotedSilver2, // Silver2
                PromotedKnight2, // Knight2
                PromotedLance2,  // Lance2
                PromotedPawn2,   // Pawn2
                Dragon2,         // Dragon2
                Horse2,          // Horse2
                PromotedSilver2, // PromotedSilver2
                PromotedKnight2, // PromotedKnight2
                PromotedLance2,  // PromotedLance2
                PromotedPawn2,   // PromotedPawn2
            ],
            piece_meaning_demoted_table: [
                King1,   // King1
                Rook1,   // Rook1
                Bishop1, // Bishop1
                Gold1,   // Gold1
                Silver1, // Silver1
                Knight1, // Knight1
                Lance1,  // Lance1
                Pawn1,   // Pawn1
                Rook1,   // Dragon1
                Bishop1, // Horse1
                Silver1, // PromotedSilver1
                Knight1, // PromotedKnight1
                Lance1,  // PromotedLance1
                Pawn1,   // PromotedPawn1
                King2,   // King2
                Rook2,   // Rook2
                Bishop2, // Bishop2
                Gold2,   // Gold2
                Silver2, // Silver2
                Knight2, // Knight2
                Lance2,  // Lance2
                Pawn2,   // Pawn2
                Rook2,   // Dragon2
                Bishop2, // Horse2
                Silver2, // PromotedSilver2
                Knight2, // PromotedKnight2
                Lance2,  // PromotedLance2
                Pawn2,   // PromotedPawn2
            ],
            piece_meaning_captured_table: [
                King2,   // King1
                Rook2,   // Rook1
                Bishop2, // Bishop1
                Gold2,   // Gold1
                Silver2, // Silver1
                Knight2, // Knight1
                Lance2,  // Lance1
                Pawn2,   // Pawn1
                Rook2,   // Dragon1
                Bishop2, // Horse1
                Silver2, // PromotedSilver1
                Knight2, // PromotedKnight1
                Lance2,  // PromotedLance1
                Pawn2,   // PromotedPawn1
                King1,   // King2
                Rook1,   // Rook2
                Bishop1, // Bishop2
                Gold1,   // Gold2
                Silver1, // Silver2
                Knight1, // Knight2
                Lance1,  // Lance2
                Pawn1,   // Pawn2
                Rook1,   // Dragon2
                Bishop1, // Horse2
                Silver1, // PromotedSilver2
                Knight1, // PromotedKnight2
                Lance1,  // PromotedLance2
                Pawn1,   // PromotedPawn2
            ],
            piece_meaning_hand_address_table: [
                HandAddress::King1,   // King1
                HandAddress::Rook1,   // Rook1
                HandAddress::Bishop1, // Bishop1
                HandAddress::Gold1,   // Gold1
                HandAddress::Silver1, // Silver1
                HandAddress::Knight1, // Knight1
                HandAddress::Lance1,  // Lance1
                HandAddress::Pawn1,   // Pawn1
                HandAddress::Rook1,   // Dragon1
                HandAddress::Bishop1, // Horse1
                HandAddress::Silver1, // PromotedSilver1
                HandAddress::Knight1, // PromotedKnight1
                HandAddress::Lance1,  // PromotedLance1
                HandAddress::Pawn1,   // PromotedPawn1
                HandAddress::King2,   // King2
                HandAddress::Rook2,   // Rook2
                HandAddress::Bishop2, // Bishop2
                HandAddress::Gold2,   // Gold2
                HandAddress::Silver2, // Silver2
                HandAddress::Knight2, // Knight2
                HandAddress::Lance2,  // Lance2
                HandAddress::Pawn2,   // Pawn2
                HandAddress::Rook2,   // Dragon2
                HandAddress::Bishop2, // Horse2
                HandAddress::Silver2, // PromotedSilver2
                HandAddress::Knight2, // PromotedKnight2
                HandAddress::Lance2,  // PromotedLance2
                HandAddress::Pawn2,   // PromotedPawn2
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
            hand_addresses_legal_all: [
                HandAddress::Rook1,
                HandAddress::Bishop1,
                HandAddress::Gold1,
                HandAddress::Silver1,
                HandAddress::Knight1,
                HandAddress::Lance1,
                HandAddress::Pawn1,
                HandAddress::Rook2,
                HandAddress::Bishop2,
                HandAddress::Gold2,
                HandAddress::Silver2,
                HandAddress::Knight2,
                HandAddress::Lance2,
                HandAddress::Pawn2,
            ],
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

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceMeaning {
    pub fn phase(self) -> Phase {
        NINE_299792458.piece_meaning_phase_table[self as usize]
    }

    pub fn r#type(self) -> PieceType {
        NINE_299792458.piece_meaning_type_table[self as usize]
    }

    pub fn promoted(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_promoted_table[self as usize]
    }

    pub fn demoted(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_demoted_table[self as usize]
    }

    pub fn captured(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_captured_table[self as usize]
    }

    pub fn hand_address(self) -> HandAddress {
        NINE_299792458.piece_meaning_hand_address_table[self as usize]
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

/// 持駒種類
pub struct HandAddresses {}
impl HandAddresses {
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(HandAddress),
    {
        for adr in &NINE_299792458.hand_addresses_legal_all {
            callback(*adr);
        }
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

/// 駒の利き☆（＾～＾）
pub enum RelativePieceControl66 {
    West0,
    West1,
    West2,
    West3,
    West4,
    West5,
    West6,
    West7,
    West8,
    SouthWest0,
    SouthWest1,
    SouthWest2,
    SouthWest3,
    SouthWest4,
    SouthWest5,
    SouthWest6,
    SouthWest7,
    South0,
    South1,
    South2,
    South3,
    South4,
    South5,
    South6,
    South7,
    SouthEast0,
    SouthEast1,
    SouthEast2,
    SouthEast3,
    SouthEast4,
    SouthEast5,
    SouthEast6,
    SouthEast7,
    East0,
    East1,
    East2,
    East3,
    East4,
    East5,
    East6,
    East7,
    NorthEast0,
    NorthEast1,
    NorthEast2,
    NorthEast3,
    NorthEast4,
    NorthEast5,
    NorthEast6,
    NorthEast7,
    North0,
    North1,
    North2,
    North3,
    North4,
    North5,
    North6,
    North7,
    NorthWest0,
    NorthWest1,
    NorthWest2,
    NorthWest3,
    NorthWest4,
    NorthWest5,
    NorthWest6,
    NorthWest7,
    Knight0,
    Knight1,
}

/// ミーシーな駒の機動性☆（＾～＾）
pub enum Movility7 {
    Knight,
    SlideDiagonally,
    SideBackSlider,
    FrontSlider,
    BackDiagonally,
    FrontDiagonally,
    SideBack,
    Front,
}
