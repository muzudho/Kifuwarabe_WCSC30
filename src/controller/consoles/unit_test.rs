//!
//! ユニットテストだぜ☆（＾～＾）
//!
//! test コマンドで実行しろだぜ☆（＾～＾）
//!
use super::super::super::controller::boardmetries::mapping::sasite_element::*;
use super::super::super::controller::boardmetries::proposition::math_meidai::*;
use super::super::super::controller::communication::usi::*;
use super::super::super::controller::geometries::geo_teigi::*;
use super::super::super::controller::thinking::randommove;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::Phase;
use super::super::super::model::master::piece_type::PieceType;
use super::super::super::model::master::square::*;
use super::super::super::model::universe::*;
use super::visuals::dumps::*;
use std::collections::HashSet;

/**
 * test 2
 * といったコマンドに対応☆（＾～＾）
 */
pub fn test(line: &String, starts: &mut usize, len: usize, universe: &mut Universe) {
    // いろいろな動作テスト
    g_writeln(&format!("test starts={} len={}", *starts, len));

    if 4 < (len - *starts) && &line[*starts..*starts + 5] == "mvsrc" {
        *starts += 5;
        g_writeln("4<len mvsrc");
        // 駒の移動元升
        g_writeln("駒の移動元升");
        let kms = randommove::rnd_kms();
        let ps = universe
            .get_application_part()
            .get_piece_struct_master()
            .get_piece_struct_by_phase_and_piece_type(
                &universe.get_search_part().get_phase(&Person::Ji),
                kms,
            );
        let km = ps.piece();
        let sq_dst = randommove::random_square();
        g_writeln(&format!(
            "kms={} km={} ms_dst={}",
            kms,
            km,
            sq_dst.to_umasu()
        ));
        let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
        let mut da_kms_hashset: HashSet<usize> = HashSet::new();
        get_no_promotion_src_by_sq_km(
            &sq_dst,
            &ps,
            &universe.get_search_part(),
            &mut mv_src_hashset,
        );
        get_before_promotion_src_by_sq_km(
            &sq_dst,
            &ps,
            &universe.get_application_part(),
            &universe.get_search_part(),
            &mut mv_src_hashset,
        );
        insert_da_kms_by_sq_km(&sq_dst, &km, &universe, &mut da_kms_hashset);
        hyoji_sq_hashset(&mv_src_hashset);
        hyoji_kms_hashset(&da_kms_hashset);
    } else if 3 < (len - *starts) && &line[*starts..*starts + 4] == "mvkm" {
        *starts += 4;
        // 移動後の駒
        let kms = randommove::rnd_kms();
        let ps = universe
            .get_application_part()
            .get_piece_struct_master()
            .get_piece_struct_by_phase_and_piece_type(
                &universe.get_search_part().get_phase(&Person::Ji),
                &kms,
            );
        // 移動先の升、および　不成駒／成駒
        let sq_dst = randommove::random_square();
        let pro_dst = randommove::rnd_bool();
        let mut ss = Sasite::new();
        // 移動可能な元升
        let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
        //let mut da_kms_hashset : HashSet<usize> = HashSet::new();
        get_no_promotion_src_by_sq_km(
            &sq_dst,
            &ps,
            &universe.get_search_part(),
            &mut mv_src_hashset,
        );
        get_before_promotion_src_by_sq_km(
            &sq_dst,
            &ps,
            &universe.get_application_part(),
            &universe.get_search_part(),
            &mut mv_src_hashset,
        );
        //insert_da_kms_by_sq_km      ( ms_dst, &km, &universe, &mut da_kms_hashset );
        for sq_src in mv_src_hashset {
            ss.src = sq_src.clone();
            g_writeln(&format!("移動可能な駒がある升={}", sq_src.to_umasu()));
            ss.dst = sq_dst;
            ss.pro = pro_dst;
            ss.drop = PieceType::Kara;
            break;
        }
        g_writeln(&format!("指し手にすると={}", ss));
    } else if 0 < (len - *starts) && &line[*starts..*starts + 1] == "1" {
        *starts += 1;
        // 駒の移動元升
        {
            g_writeln("利きテスト1");
            let kms = PieceType::PH; // ぱわーあっぷひよこ
            let ps = universe
                .get_application_part()
                .get_piece_struct_master()
                .get_piece_struct_by_phase_and_piece_type(&Phase::Go, &kms);
            let km = ps.piece(); // △ph
            let sq_dst = Square::from_umasu(79);
            g_writeln(&format!(
                "kms={} km={} ms_dst={}",
                kms,
                km,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_kms_hashset: HashSet<usize> = HashSet::new();
            get_no_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            get_before_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_application_part(),
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            insert_da_kms_by_sq_km(&sq_dst, &km, &universe, &mut da_kms_hashset);
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_kms_hashset(&da_kms_hashset);
        }
        {
            g_writeln("利きテスト2");
            let kms = PieceType::PH; // ぱわーあっぷひよこ
            let ps = universe
                .get_application_part()
                .get_piece_struct_master()
                .get_piece_struct_by_phase_and_piece_type(&Phase::Go, &kms);
            let km = ps.piece(); // △ph
            let sq_dst = Square::from_umasu(68);
            g_writeln(&format!(
                "kms={} km={} ms_dst={}",
                kms,
                km,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_kms_hashset: HashSet<usize> = HashSet::new();
            get_no_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            get_before_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_application_part(),
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            insert_da_kms_by_sq_km(&sq_dst, &km, &universe, &mut da_kms_hashset);
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_kms_hashset(&da_kms_hashset);
        }
        {
            g_writeln("利きテスト3");
            let kms = PieceType::PH; // ぱわーあっぷひよこ
            let ps = universe
                .get_application_part()
                .get_piece_struct_master()
                .get_piece_struct_by_phase_and_piece_type(&Phase::Go, &kms);
            let km = ps.piece(); // △ph
            let sq_dst = Square::from_umasu(77);
            g_writeln(&format!(
                "kms={} km={} ms_dst={}",
                kms,
                km,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_kms_hashset: HashSet<usize> = HashSet::new();
            get_no_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            get_before_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_application_part(),
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            insert_da_kms_by_sq_km(&sq_dst, &km, &universe, &mut da_kms_hashset);
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_kms_hashset(&da_kms_hashset);
        }
        {
            g_writeln("利きテスト2");
            let kms = PieceType::R; // らいおん
            let ps = universe
                .get_application_part()
                .get_piece_struct_master()
                .get_piece_struct_by_phase_and_piece_type(&Phase::Sen, &kms);
            let km = ps.piece(); // ▼ら
            let sq_dst = Square::from_umasu(58);
            g_writeln(&format!(
                "kms={} km={} ms_dst={}",
                kms,
                km,
                sq_dst.to_umasu()
            ));
            let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
            let mut da_kms_hashset: HashSet<usize> = HashSet::new();
            get_no_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            get_before_promotion_src_by_sq_km(
                &sq_dst,
                &ps,
                &universe.get_application_part(),
                &universe.get_search_part(),
                &mut mv_src_hashset,
            );
            insert_da_kms_by_sq_km(&sq_dst, &km, &universe, &mut da_kms_hashset);
            hyoji_sq_hashset(&mv_src_hashset);
            hyoji_kms_hashset(&da_kms_hashset);
        }
    } else if 0 < (len - *starts) && &line[*starts..*starts + 1] == "2" {
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
    } else if 0 < (len - *starts) && &line[*starts..*starts + 1] == "3" {
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
    } else if 0 < (len - *starts) && &line[*starts..*starts + 1] == "4" {
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
    } else if 0 < (len - *starts) && &line[*starts..*starts + 1] == "5" {
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
    } else if 0 < (len - *starts) && &line[*starts..*starts + 1] == "6" {
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
        universe.push_command( &"position startpos moves 6i5h 8c8d 9i9h 8d8e 3g3f 8e8f 5h4h 8f8g+ 1i1h 8g9h 2g2f 9h8h 9g9f 8h7i 2i3g 8b8i+ 2f2e 7i7h".to_string() );
        universe.push_command(&"position1".to_string());
        //g_writeln( &universe.pop_command() );
    }

    // positionコマンドの読取を丸投げ
    // tusin::usi::read_position(&KY593.to_string(), &mut universe);
    // tusin::usi::read_position(&KY2.to_string(), &mut universe);
}
