//!
//! ユニットテストだぜ☆（＾～＾）
//!
//! unit-test コマンドで実行しろだぜ☆（＾～＾）
//!
use super::super::super::controller::common_use::cu_geo_teigi_controller::*;
use super::super::super::controller::common_use::cu_math_controller::*;
use super::super::super::controller::common_use::cu_random_move_controller;
use super::super::super::controller::movement_generation::mg_sub_part_controller::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::model::vo::game_part::gp_square_and_piece_vo::*;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::model::vo::other_part::op_phase_vo::Phase;
use std::collections::HashSet;
use std::hash::BuildHasher;

/// 升を表示
pub fn hyoji_sq_hashset<S: BuildHasher>(sq_hashset: &HashSet<Square, S>) {
    g_writeln(&format!("sq_hashset.len()={}", sq_hashset.len()));
    for sq in sq_hashset {
        let ms = (*sq).to_umasu();
        match ms {
            MASU_0 => break,
            _ => g_writeln(&format!("ms({})", ms)),
        }
    }
}

/// 升を表示
pub fn hyoji_sq_vec(sq_vec: &[Square]) {
    g_writeln(&format!("sq_vec.len()={}", sq_vec.len()));
    for sq in sq_vec {
        let ms = sq.to_umasu();
        match ms {
            MASU_0 => break,
            _ => g_writeln(&format!("ms({})", ms)),
        }
    }
}

/// 駒種類
pub fn hyoji_piece_type_hashset<S: BuildHasher>(num_piece_type_hashset: &HashSet<usize, S>) {
    g_writeln(&format!(
        "num_piece_type_hashset.len()={}",
        num_piece_type_hashset.len()
    ));
    for num_piece_type in num_piece_type_hashset {
        g_writeln(&format!(
            "piece_type({})",
            num_to_piece_type(*num_piece_type)
        ));
    }
}

/// unit-test 2
/// といったコマンドに対応☆（＾～＾）
pub fn unit_test(
    line: &str,
    starts: &mut usize,
    len: usize,
    ml_universe_dto: &mut MLDto,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // いろいろな動作テスト
    g_writeln(&format!("unit-test starts={} len={}", *starts, len));

    if 4 < (len - *starts) && &line[*starts..*starts + 5] == "mvsrc" {
        *starts += 5;
        g_writeln("4<len mvsrc");
        // 駒の移動元升
        g_writeln("駒の移動元升");
        let piece_type = cu_random_move_controller::random_piece_type();
        let ps = speed_of_light.get_piece_struct_vo_by_phase_and_piece_type(
            &ml_universe_dto.get_search_part().get_phase(&Person::Friend),
            *piece_type,
        );
        let pc = ps.piece();
        let sq_dst = cu_random_move_controller::random_square();
        g_writeln(&format!(
            "piece_type={} pc={} ms_dst={}",
            piece_type,
            pc,
            sq_dst.to_umasu()
        ));
        let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
        let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
        make_no_promotion_source_by_square_and_piece(
            &sq_dst,
            &ps,
            &ml_universe_dto.get_search_part().get_current_position(),
            &speed_of_light,
            |square| {
                mv_src_hashset.insert(square);
            },
        );
        make_before_promotion_source_by_square_piece(
            &sq_dst,
            &ps,
            &ml_universe_dto.get_search_part().get_current_position(),
            &speed_of_light,
            |square| {
                mv_src_hashset.insert(square);
            },
        );
        make_drop_piece_type_by_square_piece(
            &GPSquareAndPieceVo::new(&sq_dst, pc),
            &ml_universe_dto.get_search_part().get_current_position(),
            &speed_of_light,
            |piece_type_hash| {
                da_piece_type_hashset.insert(piece_type_hash);
            },
        );
        hyoji_sq_hashset(&mv_src_hashset);
        hyoji_piece_type_hashset(&da_piece_type_hashset);
    } else if 3 < (len - *starts) && &line[*starts..*starts + 4] == "mvkm" {
        *starts += 4;
        // 移動後の駒
        let piece_type = cu_random_move_controller::random_piece_type();
        let ps = speed_of_light.get_piece_struct_vo_by_phase_and_piece_type(
            &ml_universe_dto.get_search_part().get_phase(&Person::Friend),
            *piece_type,
        );
        // 移動先の升、および　不成駒／成駒
        let sq_dst = cu_random_move_controller::random_square();
        let pro_dst = cu_random_move_controller::random_bool();
        let mut ss = MLMovementDto::default();
        // 移動可能な元升
        let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
        //let mut da_piece_type_hashset : HashSet<usize> = HashSet::new();
        make_no_promotion_source_by_square_and_piece(
            &sq_dst,
            &ps,
            &ml_universe_dto.get_search_part().get_current_position(),
            &speed_of_light,
            |square| {
                mv_src_hashset.insert(square);
            },
        );
        make_before_promotion_source_by_square_piece(
            &sq_dst,
            &ps,
            &ml_universe_dto.get_search_part().get_current_position(),
            &speed_of_light,
            |square| {
                mv_src_hashset.insert(square);
            },
        );
        //insert_da_piece_type_by_sq_km      ( ms_dst, pc, &ml_universe_dto, &mut da_piece_type_hashset );
        if let Some(sq_src) = mv_src_hashset.iter().next() {
            ss.src = (*sq_src).clone();
            g_writeln(&format!("移動可能な駒がある升={}", sq_src.to_umasu()));
            ss.dst = sq_dst;
            ss.pro = pro_dst;
            ss.drop = GPPieceTypeVo::KaraPieceType;
        }
        /*
        for sq_src in mv_src_hashset {
            ss.src = sq_src.clone();
            g_writeln(&format!("移動可能な駒がある升={}", sq_src.to_umasu()));
            ss.dst = sq_dst;
            ss.pro = pro_dst;
            ss.drop = GPPieceTypeVo::KaraPieceType;
            break;
        }
        */
        g_writeln(&format!("指し手にすると={}", ss));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "1" {
        *starts += 1;
        // 駒の移動元升
        {
            g_writeln("利きテスト1");
            let piece_type = GPPieceTypeVo::PromotedPawn; // ぱわーあっぷひよこ
            let ps = speed_of_light
                .get_piece_struct_vo_by_phase_and_piece_type(&Phase::Second, piece_type);
            let pc = ps.piece(); // △ph
            let sq_dst = Square::from_umasu(79);
            g_writeln(&format!(
                "piece_type={} pc={} ms_dst={}",
                piece_type,
                pc,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
            make_no_promotion_source_by_square_and_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_before_promotion_source_by_square_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_drop_piece_type_by_square_piece(
                &GPSquareAndPieceVo::new(&sq_dst, pc),
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |piece_type_hash| {
                    da_piece_type_hashset.insert(piece_type_hash);
                },
            );
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_piece_type_hashset(&da_piece_type_hashset);
        }
        {
            g_writeln("利きテスト2");
            let piece_type = GPPieceTypeVo::PromotedPawn; // ぱわーあっぷひよこ
            let ps = speed_of_light
                .get_piece_struct_vo_by_phase_and_piece_type(&Phase::Second, piece_type);
            let pc = ps.piece(); // △ph
            let sq_dst = Square::from_umasu(68);
            g_writeln(&format!(
                "piece_type={} pc={} ms_dst={}",
                piece_type,
                pc,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
            make_no_promotion_source_by_square_and_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_before_promotion_source_by_square_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_drop_piece_type_by_square_piece(
                &GPSquareAndPieceVo::new(&sq_dst, pc),
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |piece_type_hash| {
                    da_piece_type_hashset.insert(piece_type_hash);
                },
            );
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_piece_type_hashset(&da_piece_type_hashset);
        }
        {
            g_writeln("利きテスト3");
            let piece_type = GPPieceTypeVo::PromotedPawn; // ぱわーあっぷひよこ
            let ps = speed_of_light
                .get_piece_struct_vo_by_phase_and_piece_type(&Phase::Second, piece_type);
            let pc = ps.piece(); // △ph
            let sq_dst = Square::from_umasu(77);
            g_writeln(&format!(
                "piece_type={} pc={} ms_dst={}",
                piece_type,
                pc,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
            make_no_promotion_source_by_square_and_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_before_promotion_source_by_square_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_drop_piece_type_by_square_piece(
                &GPSquareAndPieceVo::new(&sq_dst, pc),
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |piece_type_hash| {
                    da_piece_type_hashset.insert(piece_type_hash);
                },
            );
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_piece_type_hashset(&da_piece_type_hashset);
        }
        {
            g_writeln("利きテスト2");
            let piece_type = GPPieceTypeVo::King; // らいおん
            let ps = speed_of_light
                .get_piece_struct_vo_by_phase_and_piece_type(&Phase::First, piece_type);
            let pc = ps.piece(); // ▼ら
            let sq_dst = Square::from_umasu(58);
            g_writeln(&format!(
                "piece_type={} pc={} ms_dst={}",
                piece_type,
                pc,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
            make_no_promotion_source_by_square_and_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_before_promotion_source_by_square_piece(
                &sq_dst,
                &ps,
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |square| {
                    mv_src_hashset.insert(square);
                },
            );
            make_drop_piece_type_by_square_piece(
                &GPSquareAndPieceVo::new(&sq_dst, pc),
                &ml_universe_dto.get_search_part().get_current_position(),
                &speed_of_light,
                |piece_type_hash| {
                    da_piece_type_hashset.insert(piece_type_hash);
                },
            );
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_piece_type_hashset(&da_piece_type_hashset);
        }
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "2" {
        *starts += 1;
        g_writeln("順番テスト");
        g_writeln(&format!("0・0・0 = {}", reflexive_ordered3_i8(0, 0, 0)));
        g_writeln(&format!("0・0・1 = {}", reflexive_ordered3_i8(0, 0, 1)));
        g_writeln(&format!("0・0・2 = {}", reflexive_ordered3_i8(0, 0, 2)));
        g_writeln(&format!("0・1・0 = {}", reflexive_ordered3_i8(0, 1, 0)));
        g_writeln(&format!("0・1・1 = {}", reflexive_ordered3_i8(0, 1, 1)));
        g_writeln(&format!("0・1・2 = {}", reflexive_ordered3_i8(0, 1, 2)));
        g_writeln(&format!("0・2・0 = {}", reflexive_ordered3_i8(0, 2, 0)));
        g_writeln(&format!("0・2・1 = {}", reflexive_ordered3_i8(0, 2, 1)));
        g_writeln(&format!("0・2・2 = {}", reflexive_ordered3_i8(0, 2, 2)));

        g_writeln(&format!("1・0・0 = {}", reflexive_ordered3_i8(1, 0, 0)));
        g_writeln(&format!("1・0・1 = {}", reflexive_ordered3_i8(1, 0, 1)));
        g_writeln(&format!("1・0・2 = {}", reflexive_ordered3_i8(1, 0, 2)));
        g_writeln(&format!("1・1・0 = {}", reflexive_ordered3_i8(1, 1, 0)));
        g_writeln(&format!("1・1・1 = {}", reflexive_ordered3_i8(1, 1, 1)));
        g_writeln(&format!("1・1・2 = {}", reflexive_ordered3_i8(1, 1, 2)));
        g_writeln(&format!("1・2・0 = {}", reflexive_ordered3_i8(1, 2, 0)));
        g_writeln(&format!("1・2・1 = {}", reflexive_ordered3_i8(1, 2, 1)));
        g_writeln(&format!("1・2・2 = {}", reflexive_ordered3_i8(1, 2, 2)));

        g_writeln(&format!("2・0・0 = {}", reflexive_ordered3_i8(2, 0, 0)));
        g_writeln(&format!("2・0・1 = {}", reflexive_ordered3_i8(2, 0, 1)));
        g_writeln(&format!("2・0・2 = {}", reflexive_ordered3_i8(2, 0, 2)));
        g_writeln(&format!("2・1・0 = {}", reflexive_ordered3_i8(2, 1, 0)));
        g_writeln(&format!("2・1・1 = {}", reflexive_ordered3_i8(2, 1, 1)));
        g_writeln(&format!("2・1・2 = {}", reflexive_ordered3_i8(2, 1, 2)));
        g_writeln(&format!("2・2・0 = {}", reflexive_ordered3_i8(2, 2, 0)));
        g_writeln(&format!("2・2・1 = {}", reflexive_ordered3_i8(2, 2, 1)));
        g_writeln(&format!("2・2・2 = {}", reflexive_ordered3_i8(2, 2, 2)));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "3" {
        *starts += 1;
        g_writeln("升Pは、点ABで作る平面上にあるか？");
        g_writeln("P・A・B");
        g_writeln("a{0,0} b{1,1} c{2,2}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };

        g_writeln(&format!(
            "a・a・a = {}",
            intersect_point_on_plane(&a, &a, &a)
        ));
        g_writeln(&format!(
            "a・a・b = {}",
            intersect_point_on_plane(&a, &a, &b)
        ));
        g_writeln(&format!(
            "a・a・c = {}",
            intersect_point_on_plane(&a, &a, &c)
        ));
        g_writeln(&format!(
            "a・b・a = {}",
            intersect_point_on_plane(&a, &b, &a)
        ));
        g_writeln(&format!(
            "a・b・b = {}",
            intersect_point_on_plane(&a, &b, &b)
        ));
        g_writeln(&format!(
            "a・b・c = {}",
            intersect_point_on_plane(&a, &b, &c)
        ));
        g_writeln(&format!(
            "a・c・a = {}",
            intersect_point_on_plane(&a, &c, &a)
        ));
        g_writeln(&format!(
            "a・c・b = {}",
            intersect_point_on_plane(&a, &c, &b)
        ));
        g_writeln(&format!(
            "a・c・c = {}",
            intersect_point_on_plane(&a, &c, &c)
        ));

        g_writeln(&format!(
            "b・a・a = {}",
            intersect_point_on_plane(&b, &a, &a)
        ));
        g_writeln(&format!(
            "b・a・b = {}",
            intersect_point_on_plane(&b, &a, &b)
        ));
        g_writeln(&format!(
            "b・a・c = {}",
            intersect_point_on_plane(&b, &a, &c)
        ));
        g_writeln(&format!(
            "b・b・a = {}",
            intersect_point_on_plane(&b, &b, &a)
        ));
        g_writeln(&format!(
            "b・b・b = {}",
            intersect_point_on_plane(&b, &b, &b)
        ));
        g_writeln(&format!(
            "b・b・c = {}",
            intersect_point_on_plane(&b, &b, &c)
        ));
        g_writeln(&format!(
            "b・c・a = {}",
            intersect_point_on_plane(&b, &c, &a)
        ));
        g_writeln(&format!(
            "b・c・b = {}",
            intersect_point_on_plane(&b, &c, &b)
        ));
        g_writeln(&format!(
            "b・c・c = {}",
            intersect_point_on_plane(&b, &c, &c)
        ));

        g_writeln(&format!(
            "c・a・a = {}",
            intersect_point_on_plane(&c, &a, &a)
        ));
        g_writeln(&format!(
            "c・a・b = {}",
            intersect_point_on_plane(&c, &a, &b)
        ));
        g_writeln(&format!(
            "c・a・c = {}",
            intersect_point_on_plane(&c, &a, &c)
        ));
        g_writeln(&format!(
            "c・b・a = {}",
            intersect_point_on_plane(&c, &b, &a)
        ));
        g_writeln(&format!(
            "c・b・b = {}",
            intersect_point_on_plane(&c, &b, &b)
        ));
        g_writeln(&format!(
            "c・b・c = {}",
            intersect_point_on_plane(&c, &b, &c)
        ));
        g_writeln(&format!(
            "c・c・a = {}",
            intersect_point_on_plane(&c, &c, &a)
        ));
        g_writeln(&format!(
            "c・c・b = {}",
            intersect_point_on_plane(&c, &c, &b)
        ));
        g_writeln(&format!(
            "c・c・c = {}",
            intersect_point_on_plane(&c, &c, &c)
        ));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "4" {
        *starts += 1;
        g_writeln("点ABは、同じ段にあるか？");
        g_writeln("A・B");
        g_writeln("a{0,0} b{1,1} c{2,2} d{2,0}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };
        let d = Point { x: 2, y: 0 };
        g_writeln(&format!("a・a = {}", match_argangle0_p_p(&a, &a)));
        g_writeln(&format!("a・b = {}", match_argangle0_p_p(&a, &b)));
        g_writeln(&format!("a・c = {}", match_argangle0_p_p(&a, &c)));
        g_writeln(&format!("a・d = {}", match_argangle0_p_p(&a, &d)));

        g_writeln(&format!("b・a = {}", match_argangle0_p_p(&b, &a)));
        g_writeln(&format!("b・b = {}", match_argangle0_p_p(&b, &b)));
        g_writeln(&format!("b・c = {}", match_argangle0_p_p(&b, &c)));
        g_writeln(&format!("b・d = {}", match_argangle0_p_p(&b, &d)));

        g_writeln(&format!("c・a = {}", match_argangle0_p_p(&c, &a)));
        g_writeln(&format!("c・b = {}", match_argangle0_p_p(&c, &b)));
        g_writeln(&format!("c・c = {}", match_argangle0_p_p(&c, &c)));
        g_writeln(&format!("c・d = {}", match_argangle0_p_p(&c, &d)));

        g_writeln(&format!("d・a = {}", match_argangle0_p_p(&d, &a)));
        g_writeln(&format!("d・b = {}", match_argangle0_p_p(&d, &b)));
        g_writeln(&format!("d・c = {}", match_argangle0_p_p(&d, &c)));
        g_writeln(&format!("d・d = {}", match_argangle0_p_p(&d, &d)));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "5" {
        *starts += 1;
        g_writeln("点ABは、４つの角度の直線上にあるか？");
        g_writeln("A・B");
        g_writeln("a{0,0} b{1,1} c{2,2} d{2,0}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };
        let d = Point { x: 2, y: 0 };
        g_writeln(&format!("a・a = {}", get_argangle4_p_p(&a, &a)));
        g_writeln(&format!("a・b = {}", get_argangle4_p_p(&a, &b)));
        g_writeln(&format!("a・c = {}", get_argangle4_p_p(&a, &c)));
        g_writeln(&format!("a・d = {}", get_argangle4_p_p(&a, &d)));

        g_writeln(&format!("b・a = {}", get_argangle4_p_p(&b, &a)));
        g_writeln(&format!("b・b = {}", get_argangle4_p_p(&b, &b)));
        g_writeln(&format!("b・c = {}", get_argangle4_p_p(&b, &c)));
        g_writeln(&format!("b・d = {}", get_argangle4_p_p(&b, &d)));

        g_writeln(&format!("c・a = {}", get_argangle4_p_p(&c, &a)));
        g_writeln(&format!("c・b = {}", get_argangle4_p_p(&c, &b)));
        g_writeln(&format!("c・c = {}", get_argangle4_p_p(&c, &c)));
        g_writeln(&format!("c・d = {}", get_argangle4_p_p(&c, &d)));

        g_writeln(&format!("d・a = {}", get_argangle4_p_p(&d, &a)));
        g_writeln(&format!("d・b = {}", get_argangle4_p_p(&d, &b)));
        g_writeln(&format!("d・c = {}", get_argangle4_p_p(&d, &c)));
        g_writeln(&format!("d・d = {}", get_argangle4_p_p(&d, &d)));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "6" {
        *starts += 1;
        g_writeln("升Pは、線分AB上にあるか？");
        g_writeln("P・A・B");
        g_writeln("a{0,0} b{1,1} c{2,2} d{2,0}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };
        let d = Point { x: 2, y: 0 };

        g_writeln(&format!(
            "a　　a・a = {}",
            intersect_point_on_line_segment(&a, &a, &a)
        ));
        g_writeln(&format!(
            "a　　a・b = {}",
            intersect_point_on_line_segment(&a, &a, &b)
        ));
        g_writeln(&format!(
            "a　　a・c = {}",
            intersect_point_on_line_segment(&a, &a, &c)
        ));
        g_writeln(&format!(
            "a　　a・d = {}",
            intersect_point_on_line_segment(&a, &a, &d)
        ));
        g_writeln(&format!(
            "a　　b・a = {}",
            intersect_point_on_line_segment(&a, &b, &a)
        ));
        g_writeln(&format!(
            "a　　b・b = {}",
            intersect_point_on_line_segment(&a, &b, &b)
        ));
        g_writeln(&format!(
            "a　　b・c = {}",
            intersect_point_on_line_segment(&a, &b, &c)
        ));
        g_writeln(&format!(
            "a　　b・d = {}",
            intersect_point_on_line_segment(&a, &b, &d)
        ));
        g_writeln(&format!(
            "a　　c・a = {}",
            intersect_point_on_line_segment(&a, &c, &a)
        ));
        g_writeln(&format!(
            "a　　c・b = {}",
            intersect_point_on_line_segment(&a, &c, &b)
        ));
        g_writeln(&format!(
            "a　　c・c = {}",
            intersect_point_on_line_segment(&a, &c, &c)
        ));
        g_writeln(&format!(
            "a　　c・d = {}",
            intersect_point_on_line_segment(&a, &c, &d)
        ));
        g_writeln(&format!(
            "a　　d・a = {}",
            intersect_point_on_line_segment(&a, &d, &a)
        ));
        g_writeln(&format!(
            "a　　d・b = {}",
            intersect_point_on_line_segment(&a, &d, &b)
        ));
        g_writeln(&format!(
            "a　　d・c = {}",
            intersect_point_on_line_segment(&a, &d, &c)
        ));
        g_writeln(&format!(
            "a　　d・d = {}",
            intersect_point_on_line_segment(&a, &d, &d)
        ));

        g_writeln(&format!(
            "b　　a・a = {}",
            intersect_point_on_line_segment(&b, &a, &a)
        ));
        g_writeln(&format!(
            "b　　a・b = {}",
            intersect_point_on_line_segment(&b, &a, &b)
        ));
        g_writeln(&format!(
            "b　　a・c = {}",
            intersect_point_on_line_segment(&b, &a, &c)
        ));
        g_writeln(&format!(
            "b　　a・d = {}",
            intersect_point_on_line_segment(&b, &a, &d)
        ));
        g_writeln(&format!(
            "b　　b・a = {}",
            intersect_point_on_line_segment(&b, &b, &a)
        ));
        g_writeln(&format!(
            "b　　b・b = {}",
            intersect_point_on_line_segment(&b, &b, &b)
        ));
        g_writeln(&format!(
            "b　　b・c = {}",
            intersect_point_on_line_segment(&b, &b, &c)
        ));
        g_writeln(&format!(
            "b　　b・d = {}",
            intersect_point_on_line_segment(&b, &b, &d)
        ));
        g_writeln(&format!(
            "b　　c・a = {}",
            intersect_point_on_line_segment(&b, &c, &a)
        ));
        g_writeln(&format!(
            "b　　c・b = {}",
            intersect_point_on_line_segment(&b, &c, &b)
        ));
        g_writeln(&format!(
            "b　　c・c = {}",
            intersect_point_on_line_segment(&b, &c, &c)
        ));
        g_writeln(&format!(
            "b　　c・d = {}",
            intersect_point_on_line_segment(&b, &c, &d)
        ));
        g_writeln(&format!(
            "b　　d・a = {}",
            intersect_point_on_line_segment(&b, &d, &a)
        ));
        g_writeln(&format!(
            "b　　d・b = {}",
            intersect_point_on_line_segment(&b, &d, &b)
        ));
        g_writeln(&format!(
            "b　　d・c = {}",
            intersect_point_on_line_segment(&b, &d, &c)
        ));
        g_writeln(&format!(
            "b　　d・d = {}",
            intersect_point_on_line_segment(&b, &d, &d)
        ));

        g_writeln(&format!(
            "c　　a・a = {}",
            intersect_point_on_line_segment(&c, &a, &a)
        ));
        g_writeln(&format!(
            "c　　a・b = {}",
            intersect_point_on_line_segment(&c, &a, &b)
        ));
        g_writeln(&format!(
            "c　　a・c = {}",
            intersect_point_on_line_segment(&c, &a, &c)
        ));
        g_writeln(&format!(
            "c　　a・d = {}",
            intersect_point_on_line_segment(&c, &a, &d)
        ));
        g_writeln(&format!(
            "c　　b・a = {}",
            intersect_point_on_line_segment(&c, &b, &a)
        ));
        g_writeln(&format!(
            "c　　b・b = {}",
            intersect_point_on_line_segment(&c, &b, &b)
        ));
        g_writeln(&format!(
            "c　　b・c = {}",
            intersect_point_on_line_segment(&c, &b, &c)
        ));
        g_writeln(&format!(
            "c　　b・d = {}",
            intersect_point_on_line_segment(&c, &b, &d)
        ));
        g_writeln(&format!(
            "c　　c・a = {}",
            intersect_point_on_line_segment(&c, &c, &a)
        ));
        g_writeln(&format!(
            "c　　c・b = {}",
            intersect_point_on_line_segment(&c, &c, &b)
        ));
        g_writeln(&format!(
            "c　　c・c = {}",
            intersect_point_on_line_segment(&c, &c, &c)
        ));
        g_writeln(&format!(
            "c　　c・d = {}",
            intersect_point_on_line_segment(&c, &c, &d)
        ));
        g_writeln(&format!(
            "c　　d・a = {}",
            intersect_point_on_line_segment(&c, &d, &a)
        ));
        g_writeln(&format!(
            "c　　d・b = {}",
            intersect_point_on_line_segment(&c, &d, &b)
        ));
        g_writeln(&format!(
            "c　　d・c = {}",
            intersect_point_on_line_segment(&c, &d, &c)
        ));
        g_writeln(&format!(
            "c　　d・d = {}",
            intersect_point_on_line_segment(&c, &d, &d)
        ));
    } else {
        //g_writeln( &format!( "未定義のテスト「{}」", &line[*starts..len-1] ) );
        ml_universe_dto.push_command( &"position startpos moves 6i5h 8c8d 9i9h 8d8e 3g3f 8e8f 5h4h 8f8g+ 1i1h 8g9h 2g2f 9h8h 9g9f 8h7i 2i3g 8b8i+ 2f2e 7i7h".to_string() );
        ml_universe_dto.push_command(&"position1".to_string());
        //g_writeln( &ml_universe_dto.pop_command() );
    }

    // positionコマンドの読取を丸投げ
    // tusin::usi::read_position(&POS_593.to_string(), &mut ml_universe_dto);
    // tusin::usi::read_position(&POS_2.to_string(), &mut ml_universe_dto);
}
