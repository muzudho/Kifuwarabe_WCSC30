//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 要は早引きのデータベースみたいなもんだな☆（＾～＾）
//!
//! 駒早見表 (PieceChart),
//! 駒種類早見表 (PieceTypeChart).
//!
use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::recording::PHASE_LEN;
use crate::entities::cosmic::smart::features::HAND_ADDRESS_LEN;
use crate::entities::cosmic::smart::features::HAND_ADDRESS_TYPE_LEN;
use crate::entities::cosmic::smart::features::PIECE_TYPE_LEN;
use crate::entities::cosmic::smart::features::{HandPiece, HandType, PieceType};
use crate::entities::cosmic::smart::square::{Angle, RelAdr, ANGLE_LEN};
use crate::movegen::{Mobility, MoveRange};
use crate::position::position::PieceNum;
use crate::position::Square;
use crate::search::CentiPawn;
use crate::take1base::{Piece, PIECE_MEANING_LEN};
//use num_traits::FromPrimitive;
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
    static ref NINE_299792458: SpeedOfLight = SpeedOfLight::default();
}

/// こいつが早引き表なわけだぜ☆（＾～＾）
struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    piece_numbers: Vec<PieceNum>,

    /// 先後付きの駒☆（＾～＾）
    piece_to_phase_table: [Phase; PIECE_MEANING_LEN],
    piece_type_table: [PieceType; PIECE_MEANING_LEN],
    /// 駒→成駒　（成れない駒は、そのまま）
    piece_promoted_table: [Piece; PIECE_MEANING_LEN],
    /// 成駒→駒　（成っていない駒は、そのまま）
    piece_demoted_table: [Piece; PIECE_MEANING_LEN],
    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    piece_captured_table: [Piece; PIECE_MEANING_LEN],
    piece_to_hand_piece_table: [HandPiece; PIECE_MEANING_LEN],
    piece_to_hand_type_table: [HandType; PIECE_MEANING_LEN],

    // 駒種類☆（＾～＾）
    //piece_type_to_promoted_table: [bool; PIECE_TYPE_LEN],
    piece_type_to_mobility_table: [Vec<Mobility>; PIECE_TYPE_LEN],
    //piece_type_to_movility_table: [Vec<Movility>; PIECE_TYPE_LEN],
    //piece_type_to_see_order_table: [usize; PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）
    /// 玉２枚引く☆（＾～＾）
    hand_pieces_legal_all: [HandPiece; HAND_ADDRESS_LEN - 2],
    hand_types: [[HandPiece; HAND_ADDRESS_TYPE_LEN]; PHASE_LEN],
    hand_piece_to_square_table: [Square; HAND_ADDRESS_LEN],
    hand_piece_to_type_table: [HandType; HAND_ADDRESS_LEN],
    hand_piece_to_phase_table: [Phase; HAND_ADDRESS_LEN],
    hand_type_to_captured_value: [CentiPawn; HAND_ADDRESS_TYPE_LEN],

    // 相対番地と角度☆（＾～＾）
    west_ccw: [RelAdr; ANGLE_LEN],
    west_ccw_double_rank: [RelAdr; ANGLE_LEN],

    // 時計回り(Clockwise)☆（＾～＾）
    //rotate90cw: [Angle; ANGLE_LEN],
    // 時計回り(Clockwise)☆（＾～＾）
    //rotate45cw: [Angle; ANGLE_LEN],
    // 反時計回り(Counterclockwise)☆（＾～＾）
    //rotate45ccw: [Angle; ANGLE_LEN],
    // 反時計回り(Counterclockwise)☆（＾～＾）
    //rotate90ccw: [Angle; ANGLE_LEN],
    /// 点対称☆（＾～＾）
    rotate180: [Angle; ANGLE_LEN],

    // 評価値☆（＾～＾）
    // 成らないよりは、成った方がお得という、それだけの差を付けるだけの加点だぜ☆（＾～＾）
    // 大きくすると、歩と交換に角が成り込むぜ☆（＾～＾）
    //promotion_value: [CentiPawn; HAND_ADDRESS_TYPE_LEN],
    west: RelAdr,
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::entities::cosmic::recording::Phase::*;
        use crate::entities::cosmic::smart::features::PieceType::*;
        use crate::take1base::Piece::*;
        SpeedOfLight {
            /// ピースの早見表の生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
            piece_numbers: [
                PieceNum::King1,    // 1 先手玉
                PieceNum::King2,    // 2 後手玉
                PieceNum::Gold3,    // 3 金
                PieceNum::Gold4,    // 4 金
                PieceNum::Gold5,    // 5 金
                PieceNum::Gold6,    // 6 金
                PieceNum::Silver7,  // 7 銀
                PieceNum::Silver8,  // 8 銀
                PieceNum::Silver9,  // 9 銀
                PieceNum::Silver10, // 10 銀
                PieceNum::Knight11, // 11 桂
                PieceNum::Knight12, // 12 桂
                PieceNum::Knight13, // 13 桂
                PieceNum::Knight14, // 14 桂
                PieceNum::Lance15,  // 15 香
                PieceNum::Lance16,  // 16 香
                PieceNum::Lance17,  // 17 香
                PieceNum::Lance18,  // 18 香
                PieceNum::Bishop19, // 19 角
                PieceNum::Bishop20, // 20 角
                PieceNum::Rook21,   // 21 飛
                PieceNum::Rook22,   // 22 飛
                PieceNum::Pawn23,   // 23 歩
                PieceNum::Pawn24,   // 24 歩
                PieceNum::Pawn25,   // 25 歩
                PieceNum::Pawn26,   // 26 歩
                PieceNum::Pawn27,   // 27 歩
                PieceNum::Pawn28,   // 28 歩
                PieceNum::Pawn29,   // 29 歩
                PieceNum::Pawn30,   // 30 歩
                PieceNum::Pawn31,   // 31 歩
                PieceNum::Pawn32,   // 32 歩
                PieceNum::Pawn33,   // 33 歩
                PieceNum::Pawn34,   // 34 歩
                PieceNum::Pawn35,   // 35 歩
                PieceNum::Pawn36,   // 36 歩
                PieceNum::Pawn37,   // 37 歩
                PieceNum::Pawn38,   // 38 歩
                PieceNum::Pawn39,   // 39 歩
                PieceNum::Pawn40,   // 40 歩
            ]
            .to_vec(),

            /// 先後付きの駒☆（＾～＾）
            piece_to_phase_table: [
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
            piece_type_table: [
                K,  // King1
                R,  // Rook1
                B,  // Bishop1
                G,  // Gold1
                S,  // Silver1
                N,  // Knight1
                L,  // Lance1
                P,  // Pawn1
                PR, // Dragon1
                PB, // Horse1
                PS, // PromotedSilver1
                PN, // PromotedKnight1
                PL, // PromotedLance1
                PP, // PromotedPawn1
                K,  // King2
                R,  // Rook2
                B,  // Bishop2
                G,  // Gold2
                S,  // Silver2
                N,  // Knight2
                L,  // Lance2
                P,  // Pawn2
                PR, // Dragon2
                PB, // Horse2
                PS, // PromotedSilver2
                PN, // PromotedKnight2
                PL, // PromotedLance2
                PP, // PromotedPawn2
            ],
            piece_promoted_table: [
                K1,  // King1
                PR1, // Rook1
                PB1, // Bishop1
                G1,  // Gold1
                PS1, // Silver1
                PN1, // Knight1
                PL1, // Lance1
                PP1, // Pawn1
                PR1, // Dragon1
                PB1, // Horse1
                PS1, // PromotedSilver1
                PN1, // PromotedKnight1
                PL1, // PromotedLance1
                PP1, // PromotedPawn1
                K2,  // King2
                PR2, // Rook2
                PB2, // Bishop2
                G2,  // Gold2
                PS2, // Silver2
                PN2, // Knight2
                PL2, // Lance2
                PP2, // Pawn2
                PR2, // Dragon2
                PB2, // Horse2
                PS2, // PromotedSilver2
                PN2, // PromotedKnight2
                PL2, // PromotedLance2
                PP2, // PromotedPawn2
            ],
            piece_demoted_table: [
                K1, // King1
                R1, // Rook1
                B1, // Bishop1
                G1, // Gold1
                S1, // Silver1
                N1, // Knight1
                L1, // Lance1
                P1, // Pawn1
                R1, // Dragon1
                B1, // Horse1
                S1, // PromotedSilver1
                N1, // PromotedKnight1
                L1, // PromotedLance1
                P1, // PromotedPawn1
                K2, // King2
                R2, // Rook2
                B2, // Bishop2
                G2, // Gold2
                S2, // Silver2
                N2, // Knight2
                L2, // Lance2
                P2, // Pawn2
                R2, // Dragon2
                B2, // Horse2
                S2, // PromotedSilver2
                N2, // PromotedKnight2
                L2, // PromotedLance2
                P2, // PromotedPawn2
            ],
            piece_captured_table: [
                K2, // King1
                R2, // Rook1
                B2, // Bishop1
                G2, // Gold1
                S2, // Silver1
                N2, // Knight1
                L2, // Lance1
                P2, // Pawn1
                R2, // Dragon1
                B2, // Horse1
                S2, // PromotedSilver1
                N2, // PromotedKnight1
                L2, // PromotedLance1
                P2, // PromotedPawn1
                K1, // King2
                R1, // Rook2
                B1, // Bishop2
                G1, // Gold2
                S1, // Silver2
                N1, // Knight2
                L1, // Lance2
                P1, // Pawn2
                R1, // Dragon2
                B1, // Horse2
                S1, // PromotedSilver2
                N1, // PromotedKnight2
                L1, // PromotedLance2
                P1, // PromotedPawn2
            ],
            piece_to_hand_piece_table: [
                HandPiece::King1,   // King1
                HandPiece::Rook1,   // Rook1
                HandPiece::Bishop1, // Bishop1
                HandPiece::Gold1,   // Gold1
                HandPiece::Silver1, // Silver1
                HandPiece::Knight1, // Knight1
                HandPiece::Lance1,  // Lance1
                HandPiece::Pawn1,   // Pawn1
                HandPiece::Rook1,   // Dragon1
                HandPiece::Bishop1, // Horse1
                HandPiece::Silver1, // PromotedSilver1
                HandPiece::Knight1, // PromotedKnight1
                HandPiece::Lance1,  // PromotedLance1
                HandPiece::Pawn1,   // PromotedPawn1
                HandPiece::King2,   // King2
                HandPiece::Rook2,   // Rook2
                HandPiece::Bishop2, // Bishop2
                HandPiece::Gold2,   // Gold2
                HandPiece::Silver2, // Silver2
                HandPiece::Knight2, // Knight2
                HandPiece::Lance2,  // Lance2
                HandPiece::Pawn2,   // Pawn2
                HandPiece::Rook2,   // Dragon2
                HandPiece::Bishop2, // Horse2
                HandPiece::Silver2, // PromotedSilver2
                HandPiece::Knight2, // PromotedKnight2
                HandPiece::Lance2,  // PromotedLance2
                HandPiece::Pawn2,   // PromotedPawn2
            ],
            piece_to_hand_type_table: [
                HandType::King,   // King1
                HandType::Rook,   // Rook1
                HandType::Bishop, // Bishop1
                HandType::Gold,   // Gold1
                HandType::Silver, // Silver1
                HandType::Knight, // Knight1
                HandType::Lance,  // Lance1
                HandType::Pawn,   // Pawn1
                HandType::Rook,   // Dragon1
                HandType::Bishop, // Horse1
                HandType::Silver, // PromotedSilver1
                HandType::Knight, // PromotedKnight1
                HandType::Lance,  // PromotedLance1
                HandType::Pawn,   // PromotedPawn1
                HandType::King,   // King2
                HandType::Rook,   // Rook2
                HandType::Bishop, // Bishop2
                HandType::Gold,   // Gold2
                HandType::Silver, // Silver2
                HandType::Knight, // Knight2
                HandType::Lance,  // Lance2
                HandType::Pawn,   // Pawn2
                HandType::Rook,   // Dragon2
                HandType::Bishop, // Horse2
                HandType::Silver, // PromotedSilver2
                HandType::Knight, // PromotedKnight2
                HandType::Lance,  // PromotedLance2
                HandType::Pawn,   // PromotedPawn2
            ],

            // 成り駒か☆（＾～＾）？
            // piece_type_to_promoted_table: [
            //     false, // King
            //     false, // Rook
            //     false, // Bishop
            //     false, // Gold
            //     false, // Silver
            //     false, // Knight
            //     false, // Lance
            //     false, // Pawn
            //     true,  // Dragon
            //     true,  // Horse
            //     true,  // PromotedSilver
            //     true,  // PromotedKnight
            //     true,  // PromotedLance
            //     true,  // PromotedPawn
            // ],
            piece_type_to_mobility_table: [
                vec![
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw45, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw135, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                ], // King
                vec![
                    Mobility::new(Angle::Ccw0, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw90, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw180, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw270, MoveRange::Sliding),
                ], // Rook
                vec![
                    Mobility::new(Angle::Ccw45, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw135, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw225, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw315, MoveRange::Sliding),
                ], // Bishop
                vec![
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                ], // Gold
                vec![
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw45, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw135, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                ], // Silver
                vec![
                    Mobility::new(Angle::Ccw225, MoveRange::Knight),
                    Mobility::new(Angle::Ccw315, MoveRange::Knight),
                ], // Knight
                vec![Mobility::new(Angle::Ccw270, MoveRange::Sliding)], // Lance
                vec![Mobility::new(Angle::Ccw270, MoveRange::Adjacent)], // Pawn
                vec![
                    Mobility::new(Angle::Ccw0, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw90, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw180, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw270, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw45, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw135, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                ], // Dragon
                vec![
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw45, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw135, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw225, MoveRange::Sliding),
                    Mobility::new(Angle::Ccw315, MoveRange::Sliding),
                ], // Horse
                vec![
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                ], // PromotedSilver (Same gold)
                vec![
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                ], // PromotedKnight
                vec![
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                ], // PromotedLance
                vec![
                    Mobility::new(Angle::Ccw270, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw315, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw0, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw90, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw180, MoveRange::Adjacent),
                    Mobility::new(Angle::Ccw225, MoveRange::Adjacent),
                ], // PromotedPawn
            ],
            // piece_type_to_movility_table: [
            //     vec![
            //         Movility::BackDiagonally,
            //         Movility::FrontDiagonally,
            //         Movility::SideBackSlider,
            //         Movility::FrontDiagonally,
            //     ], // King
            //     vec![
            //         Movility::SideBackSlider,
            //         Movility::FrontSlider,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // Rook
            //     vec![
            //         Movility::SlideDiagonally,
            //         Movility::BackDiagonally,
            //         Movility::FrontDiagonally,
            //     ], // Bishop
            //     vec![
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // Gold
            //     vec![
            //         Movility::BackDiagonally,
            //         Movility::FrontDiagonally,
            //         Movility::Front,
            //     ], // Silver
            //     vec![Movility::Knight],                       // Knight
            //     vec![Movility::FrontSlider, Movility::Front], // Lance
            //     vec![Movility::Front],                        // Pawn
            //     vec![
            //         Movility::SideBackSlider,
            //         Movility::FrontSlider,
            //         Movility::BackDiagonally,
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // Dragon
            //     vec![
            //         Movility::SlideDiagonally,
            //         Movility::BackDiagonally,
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // Horse
            //     vec![
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // PromotedSilver
            //     vec![
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // PromotedKnight
            //     vec![
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // PromotedLance
            //     vec![
            //         Movility::FrontDiagonally,
            //         Movility::SideBack,
            //         Movility::Front,
            //     ], // PromotedPawn
            // ],
            // 駒の取り合いになったときに、先に捨てていく順だぜ☆（＾～＾）
            // piece_type_to_see_order_table: [
            //     8, // King
            //     5, // Rook
            //     4, // Bishop
            //     3, // Gold
            //     3, // Silver
            //     2, // Knight
            //     2, // Lance
            //     0, // Pawn
            //     7, // Dragon
            //     6, // Horse
            //     3, // PromotedSilver
            //     2, // PromotedKnight
            //     2, // PromotedLance
            //     1, // PromotedPawn
            // ],
            // 持ち駒☆（＾～＾）
            hand_pieces_legal_all: [
                HandPiece::Rook1,
                HandPiece::Bishop1,
                HandPiece::Gold1,
                HandPiece::Silver1,
                HandPiece::Knight1,
                HandPiece::Lance1,
                HandPiece::Pawn1,
                HandPiece::Rook2,
                HandPiece::Bishop2,
                HandPiece::Gold2,
                HandPiece::Silver2,
                HandPiece::Knight2,
                HandPiece::Lance2,
                HandPiece::Pawn2,
            ],
            hand_types: [
                [
                    HandPiece::King1,
                    HandPiece::Rook1,
                    HandPiece::Bishop1,
                    HandPiece::Gold1,
                    HandPiece::Silver1,
                    HandPiece::Knight1,
                    HandPiece::Lance1,
                    HandPiece::Pawn1,
                ],
                [
                    HandPiece::King2,
                    HandPiece::Rook2,
                    HandPiece::Bishop2,
                    HandPiece::Gold2,
                    HandPiece::Silver2,
                    HandPiece::Knight2,
                    HandPiece::Lance2,
                    HandPiece::Pawn2,
                ],
            ],

            hand_piece_to_square_table: [
                Square::new(100), // K1
                Square::new(101), // R1
                Square::new(102), // B1
                Square::new(103), // G1
                Square::new(104), // S1
                Square::new(105), // N1
                Square::new(106), // L1
                Square::new(107), // P1
                Square::new(108), // K2
                Square::new(109), // R2
                Square::new(110), // B2
                Square::new(111), // G2
                Square::new(112), // S2
                Square::new(113), // N2
                Square::new(114), // L2
                Square::new(115), // P2
            ],
            hand_piece_to_type_table: [
                HandType::King,
                HandType::Rook,
                HandType::Bishop,
                HandType::Gold,
                HandType::Silver,
                HandType::Knight,
                HandType::Lance,
                HandType::Pawn,
                HandType::King,
                HandType::Rook,
                HandType::Bishop,
                HandType::Gold,
                HandType::Silver,
                HandType::Knight,
                HandType::Lance,
                HandType::Pawn,
            ],
            hand_piece_to_phase_table: [
                Phase::First,
                Phase::First,
                Phase::First,
                Phase::First,
                Phase::First,
                Phase::First,
                Phase::First,
                Phase::First,
                Phase::Second,
                Phase::Second,
                Phase::Second,
                Phase::Second,
                Phase::Second,
                Phase::Second,
                Phase::Second,
                Phase::Second,
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
            /// 回転してからダブル・ランクしろだぜ☆（＾～＾）逆だと結果が違う☆（＾～＾）非可換の群、知ってるだろ☆ｍ９（＾～＾）ルービック・キューブと同じだぜ☆（＾～＾）
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

            // 時計回り(Clockwise)☆（＾～＾）
            // rotate90cw: [
            //     Angle::Ccw270,
            //     Angle::Ccw315,
            //     Angle::Ccw0,
            //     Angle::Ccw45,
            //     Angle::Ccw90,
            //     Angle::Ccw135,
            //     Angle::Ccw180,
            //     Angle::Ccw225,
            // ],
            // 時計回り(Clockwise)☆（＾～＾）
            // rotate45cw: [
            //     Angle::Ccw315,
            //     Angle::Ccw0,
            //     Angle::Ccw45,
            //     Angle::Ccw90,
            //     Angle::Ccw135,
            //     Angle::Ccw180,
            //     Angle::Ccw225,
            //     Angle::Ccw270,
            // ],
            // 反時計回り(Counterclockwise)☆（＾～＾）
            // rotate45ccw: [
            //     Angle::Ccw45,
            //     Angle::Ccw90,
            //     Angle::Ccw135,
            //     Angle::Ccw180,
            //     Angle::Ccw225,
            //     Angle::Ccw270,
            //     Angle::Ccw315,
            //     Angle::Ccw0,
            // ],
            // 反時計回り(Counterclockwise)☆（＾～＾）
            // rotate90ccw: [
            //     Angle::Ccw90,
            //     Angle::Ccw135,
            //     Angle::Ccw180,
            //     Angle::Ccw225,
            //     Angle::Ccw270,
            //     Angle::Ccw315,
            //     Angle::Ccw0,
            //     Angle::Ccw45,
            // ],
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
            //promotion_value: [0, 1, 1, 0, 0, 1, 1, 1],
            /// 駒割評価値（＾～＾） 成り駒を特別視しないので、 PieceType ではなく HandPiece を使うぜ（＾～＾）
            hand_type_to_captured_value: [
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                15000, // TODO 玉は 0 にしたい,
                // 駒割は取ったときにカウントしているので、成りを考慮しないぜ☆（＾～＾）
                1000, 900, 600, 500, 300, 200, 100,
            ],
            // 座標☆（＾～＾）
            west: RelAdr::new(1, 0),
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
pub struct Nine299792458 {}
impl Nine299792458 {
    pub fn piece_numbers() -> &'static Vec<PieceNum> {
        &NINE_299792458.piece_numbers
    }
    pub fn west() -> RelAdr {
        NINE_299792458.west
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Piece {
    pub fn phase(self) -> Phase {
        NINE_299792458.piece_to_phase_table[self as usize]
    }

    pub fn type_(self) -> PieceType {
        NINE_299792458.piece_type_table[self as usize]
    }

    pub fn promoted(self) -> Piece {
        NINE_299792458.piece_promoted_table[self as usize]
    }

    pub fn demoted(self) -> Piece {
        NINE_299792458.piece_demoted_table[self as usize]
    }

    pub fn captured(self) -> Piece {
        NINE_299792458.piece_captured_table[self as usize]
    }

    pub fn hand_piece(self) -> HandPiece {
        NINE_299792458.piece_to_hand_piece_table[self as usize]
    }

    pub fn hand_type(self) -> HandType {
        NINE_299792458.piece_to_hand_type_table[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceType {
    // pub fn promoted(self) -> bool {
    //     NINE_299792458.piece_type_to_promoted_table[self as usize]
    // }
    pub fn mobility(self) -> &'static Vec<Mobility> {
        &NINE_299792458.piece_type_to_mobility_table[self as usize]
    }
    // pub fn movility(self) -> &'static Vec<Movility> {
    //     &NINE_299792458.piece_type_to_movility_table[self as usize]
    // }
    /*
    pub fn see_order(self) -> usize {
        NINE_299792458.piece_type_to_see_order_table[self as usize]
    }
    */
}

/// 持駒種類
pub struct HandPieces {}
impl HandPieces {
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(HandPiece),
    {
        for hand_pc in &NINE_299792458.hand_pieces_legal_all {
            callback(*hand_pc);
        }
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl HandPiece {
    pub fn from_phase_and_type(phase: Phase, adr: HandType) -> Self {
        NINE_299792458.hand_types[phase as usize][adr as usize]
    }
    pub fn square(self) -> Square {
        NINE_299792458.hand_piece_to_square_table[self as usize]
    }
    pub fn type_(self) -> HandType {
        NINE_299792458.hand_piece_to_type_table[self as usize]
    }
    pub fn phase(self) -> Phase {
        NINE_299792458.hand_piece_to_phase_table[self as usize]
    }
}

/*
/// ハッシュ値を作る
pub fn push_drop_to_hash(hash: u64, piece_type_o: Option<HandType>) -> u64 {
    let num = if let Some(piece_type) = piece_type_o {
        // 持ち駒の型は 7つ ＋ 持ち駒無しの 1つ なんで、8(=2^3) で OK
        piece_type as u64
    } else {
        // None の変わりに 玉を使うぜ☆（＾～＾）
        HandType::King as u64
    };
    (hash << 3) + num
}
*/

/*
/// ハッシュ値から作る
pub fn pop_drop_from_hash(hash: u64) -> (u64, Option<HandType>) {
    // 使ってるのは8種類なんで、8(=2^3) で OK
    (hash >> 3, HandType::from_u64(hash & 0b111))
}
*/

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl HandType {
    // pub fn promotion_value(self) -> CentiPawn {
    //     NINE_299792458.promotion_value[self as usize]
    // }
    pub fn captured_value(self) -> CentiPawn {
        NINE_299792458.hand_type_to_captured_value[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Angle {
    /*
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
    */
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

/*
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
*/

// ミーシーな駒の機動性☆（＾～＾）
// #[derive(PartialEq)]
// pub enum Movility {
//     Knight,
//     SlideDiagonally,
//     SideBackSlider,
//     FrontSlider,
//     BackDiagonally,
//     FrontDiagonally,
//     SideBack,
//     Front,
// }
