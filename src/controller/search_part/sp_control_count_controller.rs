//!
//! 利き数
//!
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::movement_generation::mg_controller::*;
use crate::controller::movement_generation::mg_square::*;
use crate::model::univ::gam::phase::*;
use crate::model::univ::gam::piece::*;
use crate::model::univ::gam::square::*;
use crate::model::universe::*;
use crate::model::vo::game_part::gp_piece_struct_vo::GPPieceStructVo;
use crate::model::vo::main_loop::ml_speed_of_light_vo::*;
use std::collections::HashSet;

/// 指定局面の利き升調べ。一から再計算☆（＾～＾）
///
/// 用途：自殺手防止他
pub fn recalculate_control_count(universe: &mut Universe, speed_of_light: &MLSpeedOfLightVo) {
    // ゼロ・リセット
    GPPieces::for_all(&mut |any_piece| {
        universe.game.position.control_count_by_piece
            [GPPieceStructVo::from_piece(any_piece).serial_piece_number()]
        .clear();
    });

    for phase in PHASE_ARRAY.iter() {
        universe.game.position.control_count_by_phase[phase_to_num(phase)].clear();
    }

    // カウント
    GPPieces::for_all(&mut |any_piece| {
        let ps_dst = GPPieceStructVo::from_piece(any_piece);

        // 移動先の升☆（＾～＾）
        MGSquares::for_all(&mut |any_square| {
            assert_banjo_sq(&any_square, "think 利き調べ");

            // 移動元の升
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            lookup_no_promotion_source_by_square_and_piece(
                &any_square,
                &ps_dst,
                &universe.game.position.get_current_board(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );

            lookup_before_promotion_source_by_square_piece(
                &any_square,
                &ps_dst,
                &universe.game.position.get_current_board(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            // 打は考えない。盤上の利き数なので
            let control_count = mv_src_hashset.len();

            // 駒別
            universe.game.position.control_count_by_piece[ps_dst.serial_piece_number()]
                .add_count_by_square(&any_square, control_count as i8);

            // 先後別
            universe.game.position.control_count_by_phase[phase_to_num(&ps_dst.phase())]
                .add_count_by_square(&any_square, control_count as i8);
        });
    });
}
