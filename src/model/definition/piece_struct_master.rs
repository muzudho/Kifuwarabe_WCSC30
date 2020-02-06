//! 駒の実体はここだぜ☆（＾～＾）
//! マスター・テーブルみたいなもん☆（＾～＾）
use super::super::master::phase::Phase;
use super::super::master::piece::Piece;
use super::super::master::piece::Piece::*;
use super::super::master::piece_struct::PieceStruct;
use super::super::master::piece_type::PieceType;

pub struct PieceStructMaster {
    king1: PieceStruct,
    rook1: PieceStruct,
    bishop1: PieceStruct,
    gold1: PieceStruct,
    silver1: PieceStruct,
    knight1: PieceStruct,
    lance1: PieceStruct,
    pawn1: PieceStruct,
    promoted_rook1: PieceStruct,
    promoted_bishop1: PieceStruct,
    promoted_silver1: PieceStruct,
    promoted_knight1: PieceStruct,
    promoted_lance1: PieceStruct,
    promoted_pawn1: PieceStruct,
    king2: PieceStruct,
    rook2: PieceStruct,
    bishop2: PieceStruct,
    gold2: PieceStruct,
    silver2: PieceStruct,
    knight2: PieceStruct,
    lance2: PieceStruct,
    pawn2: PieceStruct,
    promoted_rook2: PieceStruct,
    promoted_bishop2: PieceStruct,
    promoted_silver2: PieceStruct,
    promoted_knight2: PieceStruct,
    promoted_lance2: PieceStruct,
    promoted_pawn2: PieceStruct,
    kara: PieceStruct,
    owari: PieceStruct,
}
impl PieceStructMaster {
    pub fn new() -> Self {
        PieceStructMaster {
            king1: PieceStruct::from_piece(King1),
            rook1: PieceStruct::from_piece(Rook1),
            bishop1: PieceStruct::from_piece(Bishop1),
            gold1: PieceStruct::from_piece(Gold1),
            silver1: PieceStruct::from_piece(Silver1),
            knight1: PieceStruct::from_piece(Knight1),
            lance1: PieceStruct::from_piece(Lance1),
            pawn1: PieceStruct::from_piece(Pawn1),
            promoted_rook1: PieceStruct::from_piece(PromotedRook1),
            promoted_bishop1: PieceStruct::from_piece(PromotedBishop1),
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
            promoted_rook2: PieceStruct::from_piece(PromotedRook2),
            promoted_bishop2: PieceStruct::from_piece(PromotedBishop2),
            promoted_silver2: PieceStruct::from_piece(PromotedSilver2),
            promoted_knight2: PieceStruct::from_piece(PromotedKnight2),
            promoted_lance2: PieceStruct::from_piece(PromotedLance2),
            promoted_pawn2: PieceStruct::from_piece(PromotedPawn2),
            kara: PieceStruct::from_piece(Kara),
            owari: PieceStruct::from_piece(Owari),
        }
    }

    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_struct(&self, piece: &Piece) -> &PieceStruct {
        // 列挙型を配列のインデックスとして使用☆（＾～＾）
        // ここでクローンするの　もったいないが……☆（＾～＾）match構文の方がいいのか☆（＾～＾）？
        // &self.piece_structs[(*piece).clone() as usize]

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
            PromotedRook1 => &self.promoted_rook1,
            PromotedBishop1 => &self.promoted_bishop1,
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
            PromotedRook2 => &self.promoted_rook2,
            PromotedBishop2 => &self.promoted_bishop2,
            PromotedSilver2 => &self.promoted_silver2,
            PromotedKnight2 => &self.promoted_knight2,
            PromotedLance2 => &self.promoted_lance2,
            PromotedPawn2 => &self.promoted_pawn2,
            Kara => &self.kara,
            Owari => &self.owari,
        }
    }

    /// 先後＆駒種類→先後付き駒
    pub fn get_piece_struct_by_phase_and_piece_type(
        &self,
        phase: &Phase,
        piece_type: &PieceType,
    ) -> &PieceStruct {
        use super::super::super::model::master::piece::Piece::*;
        use super::super::super::model::master::piece_type::PieceType::*;
        match *phase {
            Phase::Sen => match *piece_type {
                R => self.get_piece_struct(&King1),
                K => self.get_piece_struct(&Rook1),
                Z => self.get_piece_struct(&Bishop1),
                I => self.get_piece_struct(&Gold1),
                N => self.get_piece_struct(&Silver1),
                U => self.get_piece_struct(&Knight1),
                S => self.get_piece_struct(&Lance1),
                H => self.get_piece_struct(&Pawn1),
                PK => self.get_piece_struct(&PromotedRook1),
                PZ => self.get_piece_struct(&PromotedBishop1),
                PN => self.get_piece_struct(&PromotedSilver1),
                PU => self.get_piece_struct(&PromotedKnight1),
                PS => self.get_piece_struct(&PromotedLance1),
                PH => self.get_piece_struct(&PromotedPawn1),
                _ => self.get_piece_struct(&Piece::Owari),
            },
            Phase::Go => match *piece_type {
                R => self.get_piece_struct(&King2),
                K => self.get_piece_struct(&Rook2),
                Z => self.get_piece_struct(&Bishop2),
                I => self.get_piece_struct(&Gold2),
                N => self.get_piece_struct(&Silver2),
                U => self.get_piece_struct(&Knight2),
                S => self.get_piece_struct(&Lance2),
                H => self.get_piece_struct(&Pawn2),
                PK => self.get_piece_struct(&PromotedRook2),
                PZ => self.get_piece_struct(&PromotedBishop2),
                PN => self.get_piece_struct(&PromotedSilver2),
                PU => self.get_piece_struct(&PromotedKnight2),
                PS => self.get_piece_struct(&PromotedLance2),
                PH => self.get_piece_struct(&PromotedPawn2),
                _ => self.get_piece_struct(&Piece::Owari),
            },
            Phase::Owari => self.get_piece_struct(&Piece::Owari),
        }
    }
}
