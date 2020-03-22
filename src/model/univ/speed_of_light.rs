//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 駒の実体はここだぜ☆（＾～＾）
//! マスター・テーブルみたいなもん☆（＾～＾）
use crate::model::univ::gam::misc::phase::Phase;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece::Piece::*;
use crate::model::univ::gam::misc::piece_struct::PieceStruct;
use crate::model::univ::gam::misc::piece_type::PieceType;
use crate::model::univ::gam::misc::piece_type::PieceType::*;
use crate::model::univ::gam::misc::piece_type_struct::PieceTypeStruct;

pub struct MLSpeedOfLightVo {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    /// イミュータブルなのだから、直接参照してもいい☆（＾～＾）
    /// 先後付きの駒☆（＾～＾）
    pub king1: PieceStruct,
    pub rook1: PieceStruct,
    pub bishop1: PieceStruct,
    pub gold1: PieceStruct,
    pub silver1: PieceStruct,
    pub knight1: PieceStruct,
    pub lance1: PieceStruct,
    pub pawn1: PieceStruct,
    pub promoted_rook1: PieceStruct,
    pub promoted_bishop1: PieceStruct,
    pub promoted_silver1: PieceStruct,
    pub promoted_knight1: PieceStruct,
    pub promoted_lance1: PieceStruct,
    pub promoted_pawn1: PieceStruct,
    pub king2: PieceStruct,
    pub rook2: PieceStruct,
    pub bishop2: PieceStruct,
    pub gold2: PieceStruct,
    pub silver2: PieceStruct,
    pub knight2: PieceStruct,
    pub lance2: PieceStruct,
    pub pawn2: PieceStruct,
    pub promoted_rook2: PieceStruct,
    pub promoted_bishop2: PieceStruct,
    pub promoted_silver2: PieceStruct,
    pub promoted_knight2: PieceStruct,
    pub promoted_lance2: PieceStruct,
    pub promoted_pawn2: PieceStruct,

    /// 駒種類☆（＾～＾）
    pub king: PieceTypeStruct,
    pub rook: PieceTypeStruct,
    pub bishop: PieceTypeStruct,
    pub gold: PieceTypeStruct,
    pub silver: PieceTypeStruct,
    pub knight: PieceTypeStruct,
    pub lance: PieceTypeStruct,
    pub pawn: PieceTypeStruct,
    pub promoted_rook: PieceTypeStruct,
    pub promoted_bishop: PieceTypeStruct,
    pub promoted_silver: PieceTypeStruct,
    pub promoted_knight: PieceTypeStruct,
    pub promoted_lance: PieceTypeStruct,
    pub promoted_pawn: PieceTypeStruct,
}
impl Default for MLSpeedOfLightVo {
    fn default() -> Self {
        MLSpeedOfLightVo {
            king1: PieceStruct::from_piece(King1),
            rook1: PieceStruct::from_piece(Rook1),
            bishop1: PieceStruct::from_piece(Bishop1),
            gold1: PieceStruct::from_piece(Gold1),
            silver1: PieceStruct::from_piece(Silver1),
            knight1: PieceStruct::from_piece(Knight1),
            lance1: PieceStruct::from_piece(Lance1),
            pawn1: PieceStruct::from_piece(Pawn1),
            promoted_rook1: PieceStruct::from_piece(Dragon1),
            promoted_bishop1: PieceStruct::from_piece(Horse1),
            promoted_silver1: PieceStruct::from_piece(PromotedSilver1),
            promoted_knight1: PieceStruct::from_piece(PromotedKnight1),
            promoted_lance1: PieceStruct::from_piece(PromotedLance1),
            promoted_pawn1: PieceStruct::from_piece(PromotedPawn1),
            king2: PieceStruct::from_piece(King2),
            rook2: PieceStruct::from_piece(Rook2),
            bishop2: PieceStruct::from_piece(Bishop2),
            gold2: PieceStruct::from_piece(Gold2),
            silver2: PieceStruct::from_piece(Silver2),
            knight2: PieceStruct::from_piece(Knight2),
            lance2: PieceStruct::from_piece(Lance2),
            pawn2: PieceStruct::from_piece(Pawn2),
            promoted_rook2: PieceStruct::from_piece(Dragon2),
            promoted_bishop2: PieceStruct::from_piece(Horse2),
            promoted_silver2: PieceStruct::from_piece(PromotedSilver2),
            promoted_knight2: PieceStruct::from_piece(PromotedKnight2),
            promoted_lance2: PieceStruct::from_piece(PromotedLance2),
            promoted_pawn2: PieceStruct::from_piece(PromotedPawn2),
            king: PieceTypeStruct::from_piece_type(King),
            rook: PieceTypeStruct::from_piece_type(Rook),
            bishop: PieceTypeStruct::from_piece_type(Bishop),
            gold: PieceTypeStruct::from_piece_type(Gold),
            silver: PieceTypeStruct::from_piece_type(Silver),
            knight: PieceTypeStruct::from_piece_type(Knight),
            lance: PieceTypeStruct::from_piece_type(Lance),
            pawn: PieceTypeStruct::from_piece_type(Pawn),
            promoted_rook: PieceTypeStruct::from_piece_type(Dragon),
            promoted_bishop: PieceTypeStruct::from_piece_type(Horse),
            promoted_silver: PieceTypeStruct::from_piece_type(PromotedSilver),
            promoted_knight: PieceTypeStruct::from_piece_type(PromotedKnight),
            promoted_lance: PieceTypeStruct::from_piece_type(PromotedLance),
            promoted_pawn: PieceTypeStruct::from_piece_type(PromotedPawn),
        }
    }
}

impl MLSpeedOfLightVo {
    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_struct(&self, piece: &Piece) -> &PieceStruct {
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
        phase: &Phase,
        piece_type: PieceType,
    ) -> &PieceStruct {
        use crate::model::univ::gam::misc::piece::Piece::*;
        use crate::model::univ::gam::misc::piece_type::PieceType::*;
        match *phase {
            Phase::First => match piece_type {
                King => self.get_piece_struct(&King1),
                Rook => self.get_piece_struct(&Rook1),
                Bishop => self.get_piece_struct(&Bishop1),
                Gold => self.get_piece_struct(&Gold1),
                Silver => self.get_piece_struct(&Silver1),
                Knight => self.get_piece_struct(&Knight1),
                Lance => self.get_piece_struct(&Lance1),
                Pawn => self.get_piece_struct(&Pawn1),
                Dragon => self.get_piece_struct(&Dragon1),
                Horse => self.get_piece_struct(&Horse1),
                PromotedSilver => self.get_piece_struct(&PromotedSilver1),
                PromotedKnight => self.get_piece_struct(&PromotedKnight1),
                PromotedLance => self.get_piece_struct(&PromotedLance1),
                PromotedPawn => self.get_piece_struct(&PromotedPawn1),
            },
            Phase::Second => match piece_type {
                King => self.get_piece_struct(&King2),
                Rook => self.get_piece_struct(&Rook2),
                Bishop => self.get_piece_struct(&Bishop2),
                Gold => self.get_piece_struct(&Gold2),
                Silver => self.get_piece_struct(&Silver2),
                Knight => self.get_piece_struct(&Knight2),
                Lance => self.get_piece_struct(&Lance2),
                Pawn => self.get_piece_struct(&Pawn2),
                Dragon => self.get_piece_struct(&Dragon2),
                Horse => self.get_piece_struct(&Horse2),
                PromotedSilver => self.get_piece_struct(&PromotedSilver2),
                PromotedKnight => self.get_piece_struct(&PromotedKnight2),
                PromotedLance => self.get_piece_struct(&PromotedLance2),
                PromotedPawn => self.get_piece_struct(&PromotedPawn2),
            },
        }
    }
    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_type_struct_from_piece_type(
        &self,
        piece_type: &crate::model::univ::gam::misc::piece_type::PieceType,
    ) -> &PieceTypeStruct {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.pieces[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        use crate::model::univ::gam::misc::piece_type::PieceType::*;
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

    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_type_struct_from_piece(
        &self,
        piece: &crate::model::univ::gam::misc::piece::Piece,
    ) -> &PieceTypeStruct {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.pieces[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        use crate::model::univ::gam::misc::piece::Piece::*;
        match *piece {
            King1 => &self.king,
            Rook1 => &self.rook,
            Bishop1 => &self.bishop,
            Gold1 => &self.gold,
            Silver1 => &self.silver,
            Knight1 => &self.knight,
            Lance1 => &self.lance,
            Pawn1 => &self.pawn,
            Dragon1 => &self.promoted_rook,
            Horse1 => &self.promoted_bishop,
            PromotedSilver1 => &self.promoted_silver,
            PromotedKnight1 => &self.promoted_knight,
            PromotedLance1 => &self.promoted_lance,
            PromotedPawn1 => &self.promoted_pawn,
            King2 => &self.king,
            Rook2 => &self.rook,
            Bishop2 => &self.bishop,
            Gold2 => &self.gold,
            Silver2 => &self.silver,
            Knight2 => &self.knight,
            Lance2 => &self.lance,
            Pawn2 => &self.pawn,
            Dragon2 => &self.promoted_rook,
            Horse2 => &self.promoted_bishop,
            PromotedSilver2 => &self.promoted_silver,
            PromotedKnight2 => &self.promoted_knight,
            PromotedLance2 => &self.promoted_lance,
            PromotedPawn2 => &self.promoted_pawn,
        }
    }
}
