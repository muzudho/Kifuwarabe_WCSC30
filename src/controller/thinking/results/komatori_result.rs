//!
//! 結果：駒を取られる手
//!
use super::super::super::super::controller::boardmetries::mapping::sasite_seisei::*;
use super::super::super::super::controller::boardmetries::mapping::sasite_sentaku::*;
use super::super::super::super::controller::boardmetries::proposition::math_meidai::*;
use super::super::super::super::controller::common::conv::*;
use super::super::super::super::controller::communication::usi::*;
use super::super::super::super::controller::consoles::asserts::*;
use super::super::super::super::controller::geometries::geo_teigi::*;
use super::super::super::super::controller::status::uchu::*;
use super::super::super::super::model::master::phase::Phase;
use super::super::super::super::model::master::piece::Piece;
use super::super::super::super::model::master::piece_struct::PieceStruct;
use super::super::super::super::model::master::piece_type::*;
use super::super::super::super::model::master::place::*;
use super::super::super::super::model::master::square::*;
use std::collections::HashSet;
use std::fmt;

/********************
 * 駒取り結果の結果 *
 ********************/
pub enum KomatoriResultResult {
    // 駒は取られる
    Done,
    // アタッカーを除去したことにより、不発
    NoneAttacker,
    // 合い駒をしたことにより、不発
    NoneAigoma,
    // 移動したことにより、不発
    NoneMoved,
    // それ以外
    #[allow(dead_code)]
    Owari,
}

/**
 * 結果：駒取り
 */
pub struct KomatoriResult {
    // 要因：王手をしてきている駒（１つ）
    km_attacker: Piece,
    // 要因：アタッカーが居る升
    sq_attacker: Square,
    // 要因：狙われている駒が居る升
    sq_target: Square,
}
impl fmt::Display for KomatoriResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ps_attacker = PieceStruct::from_piece(&self.km_attacker);
        write!(
            f,
            "KmTori:{}{}{}{}",
            self.sq_attacker.to_umasu(),
            self.km_attacker,
            if ps_attacker.is_slider() { "-->" } else { "->" },
            self.sq_target.to_umasu()
        )
    }
}
impl KomatoriResult {
    #[allow(dead_code)]
    pub fn get_sq_attacker(&self) -> &Square {
        &self.sq_attacker
    }
    pub fn to_hash(&self) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_sq_to_hash(hash, &self.sq_target);
        hash = push_sq_to_hash(hash, &self.sq_attacker);
        PieceStruct::from_piece(&self.km_attacker).add_hash(hash)
    }
    pub fn from_hash(hash: u64) -> KomatoriResult {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, km_atk) = PieceStruct::from_hash(hash);
        let (hash, sq_atk) = pop_sq_from_hash(hash);
        let (_hash, sq_tgt) = pop_sq_from_hash(hash);
        KomatoriResult {
            km_attacker: km_atk.piece().clone(),
            sq_attacker: sq_atk.clone(),
            sq_target: sq_tgt.clone(),
        }
    }
    ///
    /// この結果を起こさないのに十分かどうか判断
    ///
    /// 解消十分方法：
    ///     (1) アタッカー升に駒を動かす（取ってしまえば解決）
    ///     (2-1) アタッカーがスライダーの場合
    ///         (2-1-1) 狙われている駒以外の駒（合い駒）を、間の升に置く
    ///     (2-2) アタッカーがスライダーではない場合
    ///         (2-2-1) 狙われている駒を、動かせば解決
    ///
    /// ss : 現局面での、駒の動き手の１つ
    pub fn get_result(&self, ss: &Sasite) -> KomatoriResultResult {
        // (1)
        if self.sq_attacker.to_umasu() == ss.dst.to_umasu() {
            return KomatoriResultResult::NoneAttacker;
        }

        // (2-1)
        let ps_attacker = PieceStruct::from_piece(&self.km_attacker);
        if ps_attacker.is_slider() {
            assert_banjo_sq(&ss.dst, "(205b2)Ｇet_result");
            assert_banjo_sq(&self.sq_attacker, "(205b3)Ｇet_result");
            assert_banjo_sq(&self.sq_target, "(205b4)Ｇet_result");

            let p_dst = sq_to_p(&ss.dst);
            let p_atk = sq_to_p(&self.sq_attacker);
            let p_tgt = sq_to_p(&self.sq_target);

            // 合い駒判定
            if
            // これから動かす駒は、狙われている駒ではないとする
            ss.src.to_umasu() != self.sq_target.to_umasu()
                // あるいは打か
                || ss.src.to_umasu() == SS_SRC_DA
            {
                // 利きの線分上に、駒を置いたか？
                if intersect_point_on_line_segment(&p_dst, &p_atk, &p_tgt) {
                    // 合い駒を置いて、指定の駒取りを不成功にした
                    return KomatoriResultResult::NoneAigoma;
                }
            } else {
                // 狙われている駒を動かす場合

                assert_banjo_sq(&ss.src, "(205b1)Ｇet_result");
                let p_src = sq_to_p(&ss.src);

                // スライダー駒との角度
                let argangle4a = get_argangle4_p_p(&p_atk, &p_tgt);
                // これから動かす駒の、逃げた先と、いた場所との角度
                let argangle4b = get_argangle4_p_p(&p_dst, &p_src);

                // スライダーのいる筋の上で動いても、逃げたことにはならないぜ☆（＾～＾）
                match match_argangle4(&argangle4a, &argangle4b) {
                    MatchingResult::Unmatched => {
                        g_writeln(&format!("info ss={} evaluated in slider.", ss));
                        // スライダーから逃げても、ひよこの利きに飛び込むことはあるが……☆
                        return KomatoriResultResult::NoneMoved;
                    }
                    _ => {
                        g_writeln(&format!("info ss={} in slider attack.", ss));
                    }
                }
            }
        } else {
            // (3-2) 狙われている駒を、とりあえず動かす
            if self.sq_target.to_umasu() == ss.src.to_umasu() {
                return KomatoriResultResult::NoneMoved;
            }
        }

        // TODO 逃げた先の自殺手判定

        // 駒が取られてしまう場合
        KomatoriResultResult::Done
    }
}

/**
 * 王手という原因を作っている関係を、（確率的洗いざらい）調べるぜ☆（＾～＾）
 *
 * sn        : 駒を「動かす」方はどっちだぜ☆（＾～＾）
 * ms_target : 取りたい駒がいる升
 *
 * return u64 : KomatoriResult のハッシュ
 */
pub fn lookup_banjo_catch(uchu: &Uchu, sn: &Phase, sq_target: &Square) -> HashSet<u64> {
    assert_banjo_sq(
        &sq_target,
        &format!(
            "(119)Ｌookup_banjo_catch sn={} sq_target={}",
            sn,
            sq_target.to_umasu()
        ),
    );

    let mut hash = HashSet::new();

    if sq_target.to_umasu() == MASU_0 {
        return hash;
    }

    let mut ss_hashset = HashSet::new();

    for kms_dst in KMS_ARRAY.iter() {
        // 移動した後の相手の駒
        let ps_dst = uchu
            .piece_struct_master()
            .get_piece_struct_by_phase_and_piece_type(&sn, kms_dst);
        let km_dst = ps_dst.piece();
        //let km_dst = sn_kms_to_km( &sn, rnd_kms() );
        // 指定マスに移動できるか
        // 打は除く

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo(&uchu, &sq_target, &km_dst, &mut ss_hashset);

        // g_writeln( &format!("テスト lookup_banjo_catch insert_ss_by_ms_km_on_banjo kms_dst={}.",kms_dst) );
        // use consoles::visuals::dumps::*;
        // hyoji_ss_hashset( &ss_hashset );

        let ss = choice_1ss_by_hashset(&ss_hashset);
        if ss.exists() {
            assert_banjo_sq(
                &ss.src,
                &format!(
                    "(123)Ｌookup_banjo_catch ss.src /  sq_target={} km_dst={} ss={}",
                    sq_target.to_umasu(),
                    km_dst,
                    ss
                ),
            );

            let oute_result = KomatoriResult {
                km_attacker: km_dst.clone(),
                sq_attacker: ss.src.clone(), // FIXME 打だと 0 になるのでは
                sq_target: sq_target.clone(),
            };

            // 重複がいっぱい
            hash.insert(oute_result.to_hash());
        }
    }
    hash
}
