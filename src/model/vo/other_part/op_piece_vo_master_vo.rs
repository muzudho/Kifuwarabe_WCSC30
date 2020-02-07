//! 駒の実体はここだぜ☆（＾～＾）
//! マスター・テーブルみたいなもん☆（＾～＾）
use super::op_phase_vo::Phase;
use super::op_piece_type_vo::PieceType;
use super::op_piece_vo::PieceVo;
use super::piece::Piece;
use super::piece::Piece::*;

/// イミュータブルなのだから、直接参照してもいい☆（＾～＾）
pub struct PieceVoMaster {
    pub king1: PieceVo,
    pub rook1: PieceVo,
    pub bishop1: PieceVo,
    pub gold1: PieceVo,
    pub silver1: PieceVo,
    pub knight1: PieceVo,
    pub lance1: PieceVo,
    pub pawn1: PieceVo,
    pub promoted_rook1: PieceVo,
    pub promoted_bishop1: PieceVo,
    pub promoted_silver1: PieceVo,
    pub promoted_knight1: PieceVo,
    pub promoted_lance1: PieceVo,
    pub promoted_pawn1: PieceVo,
    pub king2: PieceVo,
    pub rook2: PieceVo,
    pub bishop2: PieceVo,
    pub gold2: PieceVo,
    pub silver2: PieceVo,
    pub knight2: PieceVo,
    pub lance2: PieceVo,
    pub pawn2: PieceVo,
    pub promoted_rook2: PieceVo,
    pub promoted_bishop2: PieceVo,
    pub promoted_silver2: PieceVo,
    pub promoted_knight2: PieceVo,
    pub promoted_lance2: PieceVo,
    pub promoted_pawn2: PieceVo,
    pub kara: PieceVo,
    pub owari: PieceVo,
}
impl PieceVoMaster {
    pub fn new() -> Self {
        PieceVoMaster {
            king1: PieceVo::from_piece(King1),
            rook1: PieceVo::from_piece(Rook1),
            bishop1: PieceVo::from_piece(Bishop1),
            gold1: PieceVo::from_piece(Gold1),
            silver1: PieceVo::from_piece(Silver1),
            knight1: PieceVo::from_piece(Knight1),
            lance1: PieceVo::from_piece(Lance1),
            pawn1: PieceVo::from_piece(Pawn1),
            promoted_rook1: PieceVo::from_piece(PromotedRook1),
            promoted_bishop1: PieceVo::from_piece(PromotedBishop1),
            promoted_silver1: PieceVo::from_piece(PromotedSilver1),
            promoted_knight1: PieceVo::from_piece(PromotedKnight1),
            promoted_lance1: PieceVo::from_piece(PromotedLance1),
            promoted_pawn1: PieceVo::from_piece(PromotedPawn1),
            king2: PieceVo::from_piece(King2),
            rook2: PieceVo::from_piece(Rook2),
            bishop2: PieceVo::from_piece(Bishop2),
            gold2: PieceVo::from_piece(Gold2),
            silver2: PieceVo::from_piece(Silver2),
            knight2: PieceVo::from_piece(Knight2),
            lance2: PieceVo::from_piece(Lance2),
            pawn2: PieceVo::from_piece(Pawn2),
            promoted_rook2: PieceVo::from_piece(PromotedRook2),
            promoted_bishop2: PieceVo::from_piece(PromotedBishop2),
            promoted_silver2: PieceVo::from_piece(PromotedSilver2),
            promoted_knight2: PieceVo::from_piece(PromotedKnight2),
            promoted_lance2: PieceVo::from_piece(PromotedLance2),
            promoted_pawn2: PieceVo::from_piece(PromotedPawn2),
            kara: PieceVo::from_piece(Kara),
            owari: PieceVo::from_piece(Owari),
        }
    }

    /// 駒の属性を参照するぜ☆（＾～＾）
    pub fn get_piece_vo(&self, piece: &Piece) -> &PieceVo {
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
    pub fn get_piece_vo_by_phase_and_piece_type(
        &self,
        phase: &Phase,
        piece_type: &PieceType,
    ) -> &PieceVo {
        use super::op_piece_type_vo::PieceType::*;
        use super::piece::Piece::*;
        match *phase {
            Phase::Sen => match *piece_type {
                R => self.get_piece_vo(&King1),
                K => self.get_piece_vo(&Rook1),
                Z => self.get_piece_vo(&Bishop1),
                I => self.get_piece_vo(&Gold1),
                N => self.get_piece_vo(&Silver1),
                U => self.get_piece_vo(&Knight1),
                S => self.get_piece_vo(&Lance1),
                H => self.get_piece_vo(&Pawn1),
                PK => self.get_piece_vo(&PromotedRook1),
                PZ => self.get_piece_vo(&PromotedBishop1),
                PN => self.get_piece_vo(&PromotedSilver1),
                PU => self.get_piece_vo(&PromotedKnight1),
                PS => self.get_piece_vo(&PromotedLance1),
                PH => self.get_piece_vo(&PromotedPawn1),
                _ => self.get_piece_vo(&Piece::Owari),
            },
            Phase::Go => match *piece_type {
                R => self.get_piece_vo(&King2),
                K => self.get_piece_vo(&Rook2),
                Z => self.get_piece_vo(&Bishop2),
                I => self.get_piece_vo(&Gold2),
                N => self.get_piece_vo(&Silver2),
                U => self.get_piece_vo(&Knight2),
                S => self.get_piece_vo(&Lance2),
                H => self.get_piece_vo(&Pawn2),
                PK => self.get_piece_vo(&PromotedRook2),
                PZ => self.get_piece_vo(&PromotedBishop2),
                PN => self.get_piece_vo(&PromotedSilver2),
                PU => self.get_piece_vo(&PromotedKnight2),
                PS => self.get_piece_vo(&PromotedLance2),
                PH => self.get_piece_vo(&PromotedPawn2),
                _ => self.get_piece_vo(&Piece::Owari),
            },
            Phase::Owari => self.get_piece_vo(&Piece::Owari),
        }
    }
}
