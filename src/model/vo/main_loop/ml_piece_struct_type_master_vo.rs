//! 駒の実体はここだぜ☆（＾～＾）
//! マスター・テーブルみたいなもん☆（＾～＾）
use super::super::game_part::gp_piece_type_struct_vo::PieceTypeStructVo;
use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::*;

/// イミュータブルなのだから、直接参照してもいい☆（＾～＾）
pub struct MLPieceStructTypeMasterVo {
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
    pub kara: PieceTypeStructVo,
    pub owari: PieceTypeStructVo,
}
impl Default for MLPieceStructTypeMasterVo {
    fn default() -> Self {
        MLPieceStructTypeMasterVo {
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
            kara: PieceTypeStructVo::from_piece_type(
                super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::Kara,
            ),
            owari: PieceTypeStructVo::from_piece_type(
                super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::Owari,
            ),
        }
    }
}
impl MLPieceStructTypeMasterVo {
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
            super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::Kara => &self.kara,
            super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::Owari => &self.owari,
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
            super::super::game_part::gp_piece_vo::GPPieceVo::Kara => &self.kara,
            super::super::game_part::gp_piece_vo::GPPieceVo::Owari => &self.owari,
        }
    }
}
