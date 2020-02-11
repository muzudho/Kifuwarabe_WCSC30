//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 駒の実体はここだぜ☆（＾～＾）
//! マスター・テーブルみたいなもん☆（＾～＾）
use super::super::game_part::gp_piece_struct_vo::GPPieceStructVo;
use super::super::game_part::gp_piece_type_struct_vo::PieceTypeStructVo;
use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::*;
use super::super::game_part::gp_piece_vo::GPPieceVo;
use super::super::game_part::gp_piece_vo::GPPieceVo::*;
use super::super::other_part::op_phase_vo::Phase;

pub struct MLSpeedOfLightVo {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    /// イミュータブルなのだから、直接参照してもいい☆（＾～＾）
    /// 先後付きの駒☆（＾～＾）
    pub king1: GPPieceStructVo,
    pub rook1: GPPieceStructVo,
    pub bishop1: GPPieceStructVo,
    pub gold1: GPPieceStructVo,
    pub silver1: GPPieceStructVo,
    pub knight1: GPPieceStructVo,
    pub lance1: GPPieceStructVo,
    pub pawn1: GPPieceStructVo,
    pub promoted_rook1: GPPieceStructVo,
    pub promoted_bishop1: GPPieceStructVo,
    pub promoted_silver1: GPPieceStructVo,
    pub promoted_knight1: GPPieceStructVo,
    pub promoted_lance1: GPPieceStructVo,
    pub promoted_pawn1: GPPieceStructVo,
    pub king2: GPPieceStructVo,
    pub rook2: GPPieceStructVo,
    pub bishop2: GPPieceStructVo,
    pub gold2: GPPieceStructVo,
    pub silver2: GPPieceStructVo,
    pub knight2: GPPieceStructVo,
    pub lance2: GPPieceStructVo,
    pub pawn2: GPPieceStructVo,
    pub promoted_rook2: GPPieceStructVo,
    pub promoted_bishop2: GPPieceStructVo,
    pub promoted_silver2: GPPieceStructVo,
    pub promoted_knight2: GPPieceStructVo,
    pub promoted_lance2: GPPieceStructVo,
    pub promoted_pawn2: GPPieceStructVo,
    pub none_piece: GPPieceStructVo,
    pub owari_piece: GPPieceStructVo,

    /// 駒種類☆（＾～＾）
    pub king: PieceTypeStructVo,
    pub rook: PieceTypeStructVo,
    pub bishop: PieceTypeStructVo,
    pub gold: PieceTypeStructVo,
    pub silver: PieceTypeStructVo,
    pub knight: PieceTypeStructVo,
    pub lance: PieceTypeStructVo,
    pub pawn: PieceTypeStructVo,
    pub promoted_rook: PieceTypeStructVo,
    pub promoted_bishop: PieceTypeStructVo,
    pub promoted_silver: PieceTypeStructVo,
    pub promoted_knight: PieceTypeStructVo,
    pub promoted_lance: PieceTypeStructVo,
    pub promoted_pawn: PieceTypeStructVo,
    pub none_piece_type: PieceTypeStructVo,
    pub owari_piece_type: PieceTypeStructVo,
}
impl Default for MLSpeedOfLightVo {
    fn default() -> Self {
        MLSpeedOfLightVo {
            king1: GPPieceStructVo::from_piece(King1),
            rook1: GPPieceStructVo::from_piece(Rook1),
            bishop1: GPPieceStructVo::from_piece(Bishop1),
            gold1: GPPieceStructVo::from_piece(Gold1),
            silver1: GPPieceStructVo::from_piece(Silver1),
            knight1: GPPieceStructVo::from_piece(Knight1),
            lance1: GPPieceStructVo::from_piece(Lance1),
            pawn1: GPPieceStructVo::from_piece(Pawn1),
            promoted_rook1: GPPieceStructVo::from_piece(Dragon1),
            promoted_bishop1: GPPieceStructVo::from_piece(Horse1),
            promoted_silver1: GPPieceStructVo::from_piece(PromotedSilver1),
            promoted_knight1: GPPieceStructVo::from_piece(PromotedKnight1),
            promoted_lance1: GPPieceStructVo::from_piece(PromotedLance1),
            promoted_pawn1: GPPieceStructVo::from_piece(PromotedPawn1),
            king2: GPPieceStructVo::from_piece(King2),
            rook2: GPPieceStructVo::from_piece(Rook2),
            bishop2: GPPieceStructVo::from_piece(Bishop2),
            gold2: GPPieceStructVo::from_piece(Gold2),
            silver2: GPPieceStructVo::from_piece(Silver2),
            knight2: GPPieceStructVo::from_piece(Knight2),
            lance2: GPPieceStructVo::from_piece(Lance2),
            pawn2: GPPieceStructVo::from_piece(Pawn2),
            promoted_rook2: GPPieceStructVo::from_piece(Dragon2),
            promoted_bishop2: GPPieceStructVo::from_piece(Horse2),
            promoted_silver2: GPPieceStructVo::from_piece(PromotedSilver2),
            promoted_knight2: GPPieceStructVo::from_piece(PromotedKnight2),
            promoted_lance2: GPPieceStructVo::from_piece(PromotedLance2),
            promoted_pawn2: GPPieceStructVo::from_piece(PromotedPawn2),
            none_piece: GPPieceStructVo::from_piece(
                super::super::game_part::gp_piece_vo::GPPieceVo::NonePiece,
            ),
            owari_piece: GPPieceStructVo::from_piece(
                super::super::game_part::gp_piece_vo::GPPieceVo::OwariPiece,
            ),
            king: PieceTypeStructVo::from_piece_type(King),
            rook: PieceTypeStructVo::from_piece_type(Rook),
            bishop: PieceTypeStructVo::from_piece_type(Bishop),
            gold: PieceTypeStructVo::from_piece_type(Gold),
            silver: PieceTypeStructVo::from_piece_type(Silver),
            knight: PieceTypeStructVo::from_piece_type(Knight),
            lance: PieceTypeStructVo::from_piece_type(Lance),
            pawn: PieceTypeStructVo::from_piece_type(Pawn),
            promoted_rook: PieceTypeStructVo::from_piece_type(Dragon),
            promoted_bishop: PieceTypeStructVo::from_piece_type(Horse),
            promoted_silver: PieceTypeStructVo::from_piece_type(PromotedSilver),
            promoted_knight: PieceTypeStructVo::from_piece_type(PromotedKnight),
            promoted_lance: PieceTypeStructVo::from_piece_type(PromotedLance),
            promoted_pawn: PieceTypeStructVo::from_piece_type(PromotedPawn),
            none_piece_type: PieceTypeStructVo::from_piece_type(
                super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::KaraPieceType,
            ),
            owari_piece_type: PieceTypeStructVo::from_piece_type(
                super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::OwariPieceType,
            ),
        }
    }
}

impl MLSpeedOfLightVo {
    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_struct_vo(&self, piece: &GPPieceVo) -> &GPPieceStructVo {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.piece_vos[(*piece).clone() as usize]

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
            super::super::game_part::gp_piece_vo::GPPieceVo::NonePiece => &self.none_piece,
            super::super::game_part::gp_piece_vo::GPPieceVo::OwariPiece => &self.owari_piece,
        }
    }

    /// 先後＆駒種類→先後付き駒
    pub fn get_piece_struct_vo_by_phase_and_piece_type(
        &self,
        phase: &Phase,
        piece_type: GPPieceTypeVo,
    ) -> &GPPieceStructVo {
        use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::*;
        use super::super::game_part::gp_piece_vo::GPPieceVo::*;
        match *phase {
            Phase::First => match piece_type {
                King => self.get_piece_struct_vo(&King1),
                Rook => self.get_piece_struct_vo(&Rook1),
                Bishop => self.get_piece_struct_vo(&Bishop1),
                Gold => self.get_piece_struct_vo(&Gold1),
                Silver => self.get_piece_struct_vo(&Silver1),
                Knight => self.get_piece_struct_vo(&Knight1),
                Lance => self.get_piece_struct_vo(&Lance1),
                Pawn => self.get_piece_struct_vo(&Pawn1),
                Dragon => self.get_piece_struct_vo(&Dragon1),
                Horse => self.get_piece_struct_vo(&Horse1),
                PromotedSilver => self.get_piece_struct_vo(&PromotedSilver1),
                PromotedKnight => self.get_piece_struct_vo(&PromotedKnight1),
                PromotedLance => self.get_piece_struct_vo(&PromotedLance1),
                PromotedPawn => self.get_piece_struct_vo(&PromotedPawn1),
                _ => self.get_piece_struct_vo(&GPPieceVo::OwariPiece),
            },
            Phase::Second => match piece_type {
                King => self.get_piece_struct_vo(&King2),
                Rook => self.get_piece_struct_vo(&Rook2),
                Bishop => self.get_piece_struct_vo(&Bishop2),
                Gold => self.get_piece_struct_vo(&Gold2),
                Silver => self.get_piece_struct_vo(&Silver2),
                Knight => self.get_piece_struct_vo(&Knight2),
                Lance => self.get_piece_struct_vo(&Lance2),
                Pawn => self.get_piece_struct_vo(&Pawn2),
                Dragon => self.get_piece_struct_vo(&Dragon2),
                Horse => self.get_piece_struct_vo(&Horse2),
                PromotedSilver => self.get_piece_struct_vo(&PromotedSilver2),
                PromotedKnight => self.get_piece_struct_vo(&PromotedKnight2),
                PromotedLance => self.get_piece_struct_vo(&PromotedLance2),
                PromotedPawn => self.get_piece_struct_vo(&PromotedPawn2),
                _ => self.get_piece_struct_vo(&GPPieceVo::OwariPiece),
            },
            Phase::None => self.get_piece_struct_vo(&GPPieceVo::OwariPiece),
        }
    }
    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_type_struct_vo_from_piece_type(
        &self,
        piece_type: &super::super::game_part::gp_piece_type_vo::GPPieceTypeVo,
    ) -> &PieceTypeStructVo {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.piece_vos[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::*;
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
            super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::KaraPieceType => {
                &self.none_piece_type
            }
            super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::OwariPieceType => {
                &self.owari_piece_type
            }
        }
    }

    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_type_struct_vo_from_piece(
        &self,
        piece: &super::super::game_part::gp_piece_vo::GPPieceVo,
    ) -> &PieceTypeStructVo {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.piece_vos[(*piece).clone() as usize]

        // match構文の方がいいのか☆（＾～＾）？ 不便くさいが……☆（＾～＾）
        use super::super::game_part::gp_piece_vo::GPPieceVo::*;
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
            super::super::game_part::gp_piece_vo::GPPieceVo::NonePiece => &self.none_piece_type,
            super::super::game_part::gp_piece_vo::GPPieceVo::OwariPiece => &self.owari_piece_type,
        }
    }
}
