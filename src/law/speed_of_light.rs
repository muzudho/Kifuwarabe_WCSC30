//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 駒の実体はここだぜ☆（＾～＾）
//! マスター・テーブルみたいなもん☆（＾～＾）
use crate::cosmic::shogi::state::Phase;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::toy_box::Piece;
use crate::law::chart::{PieceChart, PieceTypeChart};

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
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::cosmic::smart::features::PieceType::*;
        use crate::cosmic::toy_box::Piece::*;
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
        }
    }
}

impl SpeedOfLight {
    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_chart(&self, piece: &Piece) -> &PieceChart {
        use crate::cosmic::toy_box::Piece::*;

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

    /// 先後＆駒種類→先後付き駒
    pub fn get_piece_struct_by_phase_and_piece_type(
        &self,
        phase: Phase,
        piece_type: PieceType,
    ) -> &PieceChart {
        use crate::cosmic::smart::features::PieceType::*;
        use crate::cosmic::toy_box::Piece::*;
        match phase {
            Phase::First => match piece_type {
                King => self.get_piece_chart(&King1),
                Rook => self.get_piece_chart(&Rook1),
                Bishop => self.get_piece_chart(&Bishop1),
                Gold => self.get_piece_chart(&Gold1),
                Silver => self.get_piece_chart(&Silver1),
                Knight => self.get_piece_chart(&Knight1),
                Lance => self.get_piece_chart(&Lance1),
                Pawn => self.get_piece_chart(&Pawn1),
                Dragon => self.get_piece_chart(&Dragon1),
                Horse => self.get_piece_chart(&Horse1),
                PromotedSilver => self.get_piece_chart(&PromotedSilver1),
                PromotedKnight => self.get_piece_chart(&PromotedKnight1),
                PromotedLance => self.get_piece_chart(&PromotedLance1),
                PromotedPawn => self.get_piece_chart(&PromotedPawn1),
            },
            Phase::Second => match piece_type {
                King => self.get_piece_chart(&King2),
                Rook => self.get_piece_chart(&Rook2),
                Bishop => self.get_piece_chart(&Bishop2),
                Gold => self.get_piece_chart(&Gold2),
                Silver => self.get_piece_chart(&Silver2),
                Knight => self.get_piece_chart(&Knight2),
                Lance => self.get_piece_chart(&Lance2),
                Pawn => self.get_piece_chart(&Pawn2),
                Dragon => self.get_piece_chart(&Dragon2),
                Horse => self.get_piece_chart(&Horse2),
                PromotedSilver => self.get_piece_chart(&PromotedSilver2),
                PromotedKnight => self.get_piece_chart(&PromotedKnight2),
                PromotedLance => self.get_piece_chart(&PromotedLance2),
                PromotedPawn => self.get_piece_chart(&PromotedPawn2),
            },
        }
    }
    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_type_struct_from_piece_type(&self, piece_type: &PieceType) -> &PieceTypeChart {
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
}
