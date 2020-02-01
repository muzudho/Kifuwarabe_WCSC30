use super::phase::Phase;
use super::piece::Piece;
use super::piece::Piece::*;
use super::piece_struct::PieceStruct;
use super::piece_type::PieceType;

pub struct PieceStructMaster {
    piece_structs: [PieceStruct; 30],
}
impl PieceStructMaster {
    pub fn new() -> Self {
        PieceStructMaster {
            piece_structs: [
                PieceStruct::from_piece(&King1),
                PieceStruct::from_piece(&Rook1),
                PieceStruct::from_piece(&Bishop1),
                PieceStruct::from_piece(&Gold1),
                PieceStruct::from_piece(&Silver1),
                PieceStruct::from_piece(&Knight1),
                PieceStruct::from_piece(&Lance1),
                PieceStruct::from_piece(&Pawn1),
                PieceStruct::from_piece(&PromotedRook1),
                PieceStruct::from_piece(&PromotedBishop1),
                PieceStruct::from_piece(&PromotedSilver1),
                PieceStruct::from_piece(&PromotedKnight1),
                PieceStruct::from_piece(&PromotedLance1),
                PieceStruct::from_piece(&PromotedPawn1),
                PieceStruct::from_piece(&King2),
                PieceStruct::from_piece(&Rook2),
                PieceStruct::from_piece(&Bishop2),
                PieceStruct::from_piece(&Gold2),
                PieceStruct::from_piece(&Silver2),
                PieceStruct::from_piece(&Knight2),
                PieceStruct::from_piece(&Lance2),
                PieceStruct::from_piece(&Pawn2),
                PieceStruct::from_piece(&PromotedRook2),
                PieceStruct::from_piece(&PromotedBishop2),
                PieceStruct::from_piece(&PromotedSilver2),
                PieceStruct::from_piece(&PromotedKnight2),
                PieceStruct::from_piece(&PromotedLance2),
                PieceStruct::from_piece(&PromotedPawn2),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Owari),
            ],
        }
    }

    pub fn get_piece_structure(&self, piece: &Piece) -> &PieceStruct {
        // 列挙型を定数として使うのに コピーするの　なんか嫌だよな☆（＾～＾）
        // かといって Copyトレイト 付けるのも嫌だし☆（＾～＾）
        let index = (*piece).clone() as i32;
        &self.piece_structs[index as usize]
    }

    /// 先後＆駒種類→先後付き駒
    pub fn get_piece_struct_by_phase_and_piece_type(
        &self,
        sn: &Phase,
        kms: &PieceType,
    ) -> &PieceStruct {
        use super::super::super::model::master::piece::Piece::*;
        use super::super::super::model::master::piece_type::PieceType::*;
        match *sn {
            Phase::Sen => match *kms {
                R => self.get_piece_structure(&King1),
                K => self.get_piece_structure(&Rook1),
                Z => self.get_piece_structure(&Bishop1),
                I => self.get_piece_structure(&Gold1),
                N => self.get_piece_structure(&Silver1),
                U => self.get_piece_structure(&Knight1),
                S => self.get_piece_structure(&Lance1),
                H => self.get_piece_structure(&Pawn1),
                PK => self.get_piece_structure(&PromotedRook1),
                PZ => self.get_piece_structure(&PromotedBishop1),
                PN => self.get_piece_structure(&PromotedSilver1),
                PU => self.get_piece_structure(&PromotedKnight1),
                PS => self.get_piece_structure(&PromotedLance1),
                PH => self.get_piece_structure(&PromotedPawn1),
                _ => self.get_piece_structure(&Piece::Owari),
            },
            Phase::Go => match *kms {
                R => self.get_piece_structure(&King2),
                K => self.get_piece_structure(&Rook2),
                Z => self.get_piece_structure(&Bishop2),
                I => self.get_piece_structure(&Gold2),
                N => self.get_piece_structure(&Silver2),
                U => self.get_piece_structure(&Knight2),
                S => self.get_piece_structure(&Lance2),
                H => self.get_piece_structure(&Pawn2),
                PK => self.get_piece_structure(&PromotedRook2),
                PZ => self.get_piece_structure(&PromotedBishop2),
                PN => self.get_piece_structure(&PromotedSilver2),
                PU => self.get_piece_structure(&PromotedKnight2),
                PS => self.get_piece_structure(&PromotedLance2),
                PH => self.get_piece_structure(&PromotedPawn2),
                _ => self.get_piece_structure(&Piece::Owari),
            },
            Phase::Owari => self.get_piece_structure(&Piece::Owari),
        }
    }
}
