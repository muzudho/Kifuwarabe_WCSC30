//!
//! 局面
//!
//! 後手（上手）から見た盤にすると、
//! 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
//!
//! プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
//! 盤を想像すること☆（＾～＾）！
//!

use super::super::super::super::model::dto::main_loop::ml_dto::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::super::model::vo::other_part::op_piece_type_vo::*;
use super::super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::super::model::vo::other_part::op_piece_vo::*;
use super::super::super::super::model::vo::other_part::op_square_vo::*;

/// 局面
/// でかいのでコピーもクローンも不可☆（＾～＾）！
pub struct SPPositionDto {
    /// 10の位を筋、1の位を段とする。
    /// 0筋、0段は未使用
    board: [OPPieceVo; BAN_SIZE],
    /**
     * 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
     * 増減させたいので、u8 ではなく i8。
     */
    pub mg: [i8; KM_LN],
    /**
     * らいおんの位置
     * [先後]
     */
    sq_r: [Square; SN_LN],
}
impl SPPositionDto {
    pub fn new() -> Self {
        use super::super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo::Kara;
        SPPositionDto {
            // 盤上
            board: [
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
                Kara, Kara,
            ],
            // 持ち駒数
            mg: [
                // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
                0, 0,
            ],
            sq_r: [
                Square::from_umasu(0),
                Square::from_umasu(0),
                Square::from_umasu(0),
            ],
        }
    }
    pub fn clear(&mut self) {
        use super::super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo::Kara;
        self.board = [
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara, Kara,
            Kara, Kara,
        ];
        self.mg = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
            0, 0,
        ];
    }

    /// らいおんの位置
    pub fn get_sq_r(&self, phase_number: usize) -> &Square {
        &self.sq_r[phase_number]
    }

    /**
     * 歩が置いてあるか確認
     */
    pub fn exists_fu_by_sn_suji(
        &self,
        sn: &Phase,
        suji: i8,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> bool {
        for dan in DAN_1..DAN_10 {
            let sq = Square::from_file_rank(suji, dan);
            let piece99 = self.get_piece_by_square(&sq);
            let ps100 = speed_of_light
                .ml_piece_struct_master_vo
                .get_piece_vo(piece99);
            let (sn_km, kms) = ps100.phase_piece_type();
            if match_sn(&sn_km, sn) && match_kms(&kms, &PieceType::H) {
                return true;
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn get_piece_by_square(&self, sq: &Square) -> &OPPieceVo {
        &self.board[sq.to_umasu()]
    }
    /// 升で指定して駒を置く
    pub fn set_piece_by_square(&mut self, sq: &Square, piece: &OPPieceVo) {
        self.board[sq.to_umasu()] = piece.clone();

        // 玉の位置を覚え直します。
        use super::super::super::super::model::vo::other_part::op_phase_vo::Phase::*;
        match *piece {
            OPPieceVo::King1 => self.sq_r[Sen as usize] = sq.clone(),
            OPPieceVo::King2 => self.sq_r[Go as usize] = sq.clone(),
            _ => {}
        }
    }
    /**
     * 持ち駒の枚数を加算
     */
    pub fn add_hand(&mut self, hand: &OPPieceVo, maisu: i8, speed_of_light: &MLSpeedOfLightVo) {
        self.mg[speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(hand)
            .serial_piece_number()] += maisu;
    }
    pub fn get_hand(&self, hand: &OPPieceVo, speed_of_light: &MLSpeedOfLightVo) -> i8 {
        self.mg[speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(hand)
            .serial_piece_number()]
    }

    /**
     * 指定の升に駒があれば真
     */
    pub fn exists_km(&self, sq: &Square, speed_of_light: &MLSpeedOfLightVo) -> bool {
        !speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(self.get_piece_by_square(&sq))
            .equals_piece(
                &speed_of_light
                    .ml_piece_struct_master_vo
                    .get_piece_vo(&OPPieceVo::Kara),
            )
    }

    /// 指定の升に指定の駒があれば真
    pub fn has_sq_km(
        &self,
        sq: &Square,
        piece: &OPPieceVo,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> bool {
        speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(self.get_piece_by_square(&sq))
            .equals_piece(&speed_of_light.ml_piece_struct_master_vo.get_piece_vo(piece))
    }

    /// 指定の升にある駒の先後、または空升
    pub fn get_sn_by_sq(&self, sq: &Square, speed_of_light: &MLSpeedOfLightVo) -> Phase {
        speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(self.get_piece_by_square(sq))
            .phase()
    }

    /// 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
    pub fn is_natta(
        &self,
        sq_src: &Square,
        sq_dst: &Square,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> bool {
        let km_src = self.get_piece_by_square(&sq_src);

        let ps_src = speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(km_src);
        let km_dst = self.get_piece_by_square(&sq_dst);

        let ps_dst = speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(km_dst);
        // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
        let pro_dst = ps_dst.is_promoted();
        let pro_src = ps_src.is_promoted();

        // 成り
        pro_dst && !pro_src
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, ml_dto: &MLDto, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            let i_sq = Square::from_umasu(i_ms as umasu);
            let km = self.get_piece_by_square(&i_sq);
            let num_km = speed_of_light
                .ml_piece_struct_master_vo
                .get_piece_vo(km)
                .serial_piece_number();
            hash ^= ml_dto.get_position_hash_seed().km[i_ms][num_km];
        }

        // 持ち駒ハッシュ
        for i_km in 0..KM_ARRAY_LN {
            let km = &KM_ARRAY[i_km];
            let num_km = speed_of_light
                .ml_piece_struct_master_vo
                .get_piece_vo(km)
                .serial_piece_number();

            let maisu = self.get_hand(km, &speed_of_light);
            debug_assert!(
                -1 < maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}",
                km,
                maisu,
                MG_MAX
            );

            hash ^= ml_dto.get_position_hash_seed().mg[num_km][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}
