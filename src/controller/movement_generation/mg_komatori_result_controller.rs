//!
//! 結果：駒を取られる手
//!
use super::super::super::controller::common_use::cu_asserts_controller::*;
use super::super::super::controller::common_use::cu_conv_controller::*;
use super::super::super::controller::common_use::cu_geo_teigi_controller::*;
use super::super::super::controller::common_use::cu_math_controller::*;
use super::super::super::controller::movement_generation::mg_choicing_controller::*;
use super::super::super::controller::movement_generation::mg_controller::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_phase_vo::Phase;
use super::super::super::model::vo::game_part::gp_piece_struct_vo::GPPieceStructVo;
use super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::model::dto::search_part::position::*;
use crate::model::universe::*;
use std::collections::HashSet;
use std::fmt;

/// 駒取り結果の結果
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

/// 結果：駒取り
pub struct KomatoriResult {
    // 要因：王手をしてきている駒（１つ）
    km_attacker: GPPieceVo,
    // 要因：アタッカーが居る升
    sq_attacker: Square,
    // 要因：狙われている駒が居る升
    sq_target: Square,
}
impl fmt::Display for KomatoriResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let ps_attacker = GPPieceStructVo::from_piece(self.km_attacker.clone());
        write!(
            f,
            "KmTori:{}{}->{}",
            self.sq_attacker.to_usquare(),
            self.km_attacker,
            // if ps_attacker.is_slider() { "-->" } else { "->" },
            self.sq_target.to_usquare()
        )
    }
}
impl KomatoriResult {
    #[allow(dead_code)]
    pub fn get_sq_attacker(&self) -> &Square {
        &self.sq_attacker
    }
    pub fn to_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_sq_to_hash(hash, &self.sq_target);
        hash = push_sq_to_hash(hash, &self.sq_attacker);
        speed_of_light
            .get_piece_struct_vo(&self.km_attacker)
            .add_hash(hash)
    }
    pub fn from_hash(hash: u64) -> KomatoriResult {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, km_atk) = GPPieceStructVo::from_hash(hash);
        let (hash, sq_atk) = pop_sq_from_hash(hash);
        let (_hash, sq_tgt) = pop_sq_from_hash(hash);
        KomatoriResult {
            km_attacker: km_atk.piece().clone(),
            sq_attacker: sq_atk,
            sq_target: sq_tgt,
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
    pub fn get_result(
        &self,
        ss: &MLMovementDto,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> KomatoriResultResult {
        // (1)
        if self.sq_attacker.to_usquare() == ss.dst.to_usquare() {
            return KomatoriResultResult::NoneAttacker;
        }

        // (2-1)
        let ps_attacker1 = speed_of_light.get_piece_type_struct_vo_from_piece(&self.km_attacker);
        if ps_attacker1.slider {
            assert_banjo_sq(&ss.dst, "(205b2)Ｇet_result");
            assert_banjo_sq(&self.sq_attacker, "(205b3)Ｇet_result");
            assert_banjo_sq(&self.sq_target, "(205b4)Ｇet_result");

            let p_dst = ss.dst.to_point();
            let p_atk = self.sq_attacker.to_point();
            let p_tgt = self.sq_target.to_point();

            // 合い駒判定
            if
            // これから動かす駒は、狙われている駒ではないとする
            ss.src.to_usquare() != self.sq_target.to_usquare()
                // あるいは打か
                || ss.src.to_usquare() == SQUARE_DROP
            {
                // 利きの線分上に、駒を置いたか？
                if intersect_point_on_line_segment(&p_dst, &p_atk, &p_tgt) {
                    // 合い駒を置いて、指定の駒取りを不成功にした
                    return KomatoriResultResult::NoneAigoma;
                }
            } else {
                // 狙われている駒を動かす場合

                assert_banjo_sq(&ss.src, "(205b1)Ｇet_result");
                let p_src = ss.src.to_point();

                // スライダー駒との角度
                let argangle4a = get_argangle4_p_p(&p_atk, &p_tgt);
                // これから動かす駒の、逃げた先と、いた場所との角度
                let argangle4b = get_argangle4_p_p(&p_dst, &p_src);

                // スライダーのいる筋の上で動いても、逃げたことにはならないぜ☆（＾～＾）
                match match_argangle4(&argangle4a, &argangle4b) {
                    MatchingResult::Unmatched => {
                        g_writeln(&format!("info string ss={} evaluated in slider.", ss));
                        // スライダーから逃げても、ひよこの利きに飛び込むことはあるが……☆
                        return KomatoriResultResult::NoneMoved;
                    }
                    _ => {
                        g_writeln(&format!("info string ss={} in slider attack.", ss));
                    }
                }
            }
        } else {
            // (3-2) 狙われている駒を、とりあえず動かす
            if self.sq_target.to_usquare() == ss.src.to_usquare() {
                return KomatoriResultResult::NoneMoved;
            }
        }

        // TODO 逃げた先の自殺手判定

        // 駒が取られてしまう場合
        KomatoriResultResult::Done
    }
}

/// 王手という原因を作っている関係を、（確率的洗いざらい）調べるぜ☆（＾～＾）
///
/// phase        : 駒を「動かす」方はどっちだぜ☆（＾～＾）
/// ms_target : 取りたい駒がいる升
///
/// return u64 : KomatoriResult のハッシュ
pub fn lookup_catching_king_on_board(
    phase: &Phase,
    sq_target: &Square,
    position: &Position,
    speed_of_light: &MLSpeedOfLightVo,
) -> HashSet<u64> {
    assert_banjo_sq(
        &sq_target,
        &format!(
            "(119)Ｌookup_banjo_catch phase={} sq_target={}",
            phase,
            sq_target.to_usquare()
        ),
    );

    let mut hash = HashSet::new();

    if sq_target.to_usquare() == SQUARE_NONE {
        return hash;
    }

    let mut multiple_movements_hashset = HashSet::new();

    for piece_type_dst in PIECE_TYPE_ARRAY.iter() {
        // 移動した後の相手の駒
        let ps_dst =
            speed_of_light.get_piece_struct_vo_by_phase_and_piece_type(&phase, *piece_type_dst);
        let km_dst = ps_dst.piece();
        //let km_dst = phase_piece_type_to_km( &phase, random_piece_type() );
        // 指定マスに移動できるか
        // 打は除く

        multiple_movements_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_target,
            km_dst.clone(),
            &position,
            &speed_of_light,
            |movement_hash| {
                multiple_movements_hashset.insert(movement_hash);
            },
        );

        // g_writeln( &format!("テスト lookup_catching_king_on_board get_movement_by_square_and_piece_on_board piece_type_dst={}.",piece_type_dst) );
        // use consoles::visuals::dumps::*;
        // print_movement_hashset( &multiple_movements_hashset );

        let ss = choice_1movement_from_hashset(&multiple_movements_hashset);
        if ss.exists() {
            assert_banjo_sq(
                &ss.src,
                &format!(
                    "(123)Ｌookup_banjo_catch ss.src /  sq_target={} km_dst={} ss={}",
                    sq_target.to_usquare(),
                    km_dst.clone(),
                    ss
                ),
            );

            let oute_result = KomatoriResult {
                km_attacker: km_dst.clone(),
                sq_attacker: ss.src.clone(), // FIXME 打だと 0 になるのでは
                sq_target: sq_target.clone(),
            };

            // 重複がいっぱい
            hash.insert(oute_result.to_hash(speed_of_light));
        }
    }
    hash
}
