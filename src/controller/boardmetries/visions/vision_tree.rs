//!
//! 思考部だぜ☆（＾～＾）
//!

/*
use super::super::super::super::controller::common::conv::*;
use super::super::super::super::controller::communication::usi::*;
use super::super::super::super::controller::movement_generation::mg_sub_part::*;
use super::super::super::super::model::master::phase::*;
use super::super::super::super::model::master::piece::*;
use super::super::super::super::model::master::piece_type::*;
*/
use super::super::super::super::model::vo::square::*;
/*
use super::super::super::super::model::universe::*;
use super::super::super::super::model::vo::piece_vo::PieceVo;
*/
use std::collections::HashSet;

/**
 * 狙いは、この木にぶら下げていくぜ☆（*＾～＾*）
 * 思考で、内容はどんどん変わっていくぜ☆（＾～＾）
 */
pub struct VisionTree {
    // 相手玉の位置
    pub sq_ai_r: Square,
    // 相手玉を取る楽観筋
    pub ss_tume_hashset: HashSet<u64>,
}
impl VisionTree {
    pub fn new() -> VisionTree {
        VisionTree {
            sq_ai_r: Square::from_umasu(0),
            ss_tume_hashset: HashSet::new(),
        }
    }
    pub fn clear(&mut self) {
        self.ss_tume_hashset.clear();
    }
    pub fn set_ai_r(&mut self, sq: &Square) {
        self.sq_ai_r = sq.clone();
    }
}

/*
 * 楽観筋
 */
/* 使ってない？
pub fn insert_rakkansuji(universe: &mut Universe) {
    for sn in SN_ARRAY.iter() {
        let ai_sn = hanten_sn(sn);

        // 相手の　らいおん　の位置を覚える
        universe
            .get_search_part_mut()
            .memory_opponent_king(sn, &ai_sn);
        // 盤上に相手の　らいおん１枚　しかないと想定して、アタックする手
        let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
        //let mut da_kms_hashset : HashSet<usize> = HashSet::new();

        for kms_dst in KMS_ARRAY.iter() {
            // TODO universe.speed_of_light.piece_vo_master.get_piece_vo
            let ps_dst = PieceVo::from_piece(Piece::from_phase_piece_type(&sn, &kms_dst));
            for x in SUJI_1..SUJI_10 {
                // 9..0 みたいに降順に書いても動かない？
                for y in DAN_1..DAN_10 {
                    let sq_dst = Square::from_file_rank(x, y);

                    mv_src_hashset.clear();
                    //da_kms_hashset.clear();
                    make_no_promotion_source_by_square_and_piece(
                        &sq_dst,
                        &ps_dst,
                        &universe.get_search_part(),
                        &universe.speed_of_light,
                        |square| {
                            mv_src_hashset.insert(square);
                        },
                    );
                    make_before_promotion_source_by_square_piece(
                        &sq_dst,
                        &ps_dst,
                        &universe.get_search_part(),
                        &universe.speed_of_light,
                        |square| {
                            mv_src_hashset.insert(square);
                        },
                    );
                    // TODO 王手になるところに打ちたい
                    //insert_da_kms_by_sq_km      ( &ms_dst, &km_dst, &universe, &mut da_kms_hashset );

                    // 盤上
                    for sq_src in mv_src_hashset.iter() {
                        // 成り
                        let hash_ss = Sasite {
                            src: sq_src.clone(),
                            dst: sq_dst.clone(),
                            pro: *&universe.get_search_part().get_current_position().is_natta(
                                sq_src,
                                &sq_dst,
                                &universe.speed_of_light,
                            ),
                            drop: PieceType::Kara,
                        }
                        .to_hash();
                        &universe.get_search_part_mut().vision_tree_by_phase[sn_to_num(sn)]
                            .ss_tume_hashset
                            .insert(hash_ss);
                    }

                    /*
                    // 打
                    for kms_da in da_kms_hashset.iter() {
                        let km_da = sn_kms_to_km( &sn, &kms_da );
                        let hash_ss = Sasite{
                            src:SS_SRC_DA,
                            dst:ms_dst,
                            pro:false,
                            drop:km_da,
                        }.to_hash();
                        &universe.vision_tree_by_sn[sn].ss_tume_hashset.insert( hash_ss );
                    }
                    */
                }
            }
        }
    } //sn
}
*/
