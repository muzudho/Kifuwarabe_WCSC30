//!
//! ユニットテストだぜ☆（＾～＾）
//!
//! unit-test コマンドで実行しろだぜ☆（＾～＾）
//!
use crate::controller::common_use::cu_geo_teigi_controller::*;
use crate::controller::common_use::cu_math_controller::*;
use crate::controller::common_use::cu_random_move_controller;
use crate::controller::io::*;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::speed_of_light::*;
use crate::model::universe::*;

/// unit-test 2
/// といったコマンドに対応☆（＾～＾）
pub fn unit_test(
    line: &str,
    starts: &mut usize,
    len: usize,
    ml_universe_dto: &mut Universe,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // いろいろな動作テスト
    IO::writeln(&format!("unit-test starts={} len={}", *starts, len));

    if 4 < (len - *starts) && &line[*starts..*starts + 5] == "mvsrc" {
        *starts += 5;
        IO::writeln("4<len mvsrc");
        // 駒の移動元升
        IO::writeln("駒の移動元升");
        let piece_type = cu_random_move_controller::random_piece_type();
        let ps = speed_of_light.get_piece_struct_by_phase_and_piece_type(
            ml_universe_dto.game.history.get_phase(&Person::Friend),
            *piece_type,
        );
        let pc = &ps.piece;
        let sq_dst = cu_random_move_controller::random_square();
        IO::writeln(&format!(
            "piece_type={} pc={} ms_dst={}",
            piece_type, pc, sq_dst.address
        ));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "2" {
        *starts += 1;
        IO::writeln("順番テスト");
        IO::writeln(&format!("0・0・0 = {}", reflexive_ordered3_i8(0, 0, 0)));
        IO::writeln(&format!("0・0・1 = {}", reflexive_ordered3_i8(0, 0, 1)));
        IO::writeln(&format!("0・0・2 = {}", reflexive_ordered3_i8(0, 0, 2)));
        IO::writeln(&format!("0・1・0 = {}", reflexive_ordered3_i8(0, 1, 0)));
        IO::writeln(&format!("0・1・1 = {}", reflexive_ordered3_i8(0, 1, 1)));
        IO::writeln(&format!("0・1・2 = {}", reflexive_ordered3_i8(0, 1, 2)));
        IO::writeln(&format!("0・2・0 = {}", reflexive_ordered3_i8(0, 2, 0)));
        IO::writeln(&format!("0・2・1 = {}", reflexive_ordered3_i8(0, 2, 1)));
        IO::writeln(&format!("0・2・2 = {}", reflexive_ordered3_i8(0, 2, 2)));

        IO::writeln(&format!("1・0・0 = {}", reflexive_ordered3_i8(1, 0, 0)));
        IO::writeln(&format!("1・0・1 = {}", reflexive_ordered3_i8(1, 0, 1)));
        IO::writeln(&format!("1・0・2 = {}", reflexive_ordered3_i8(1, 0, 2)));
        IO::writeln(&format!("1・1・0 = {}", reflexive_ordered3_i8(1, 1, 0)));
        IO::writeln(&format!("1・1・1 = {}", reflexive_ordered3_i8(1, 1, 1)));
        IO::writeln(&format!("1・1・2 = {}", reflexive_ordered3_i8(1, 1, 2)));
        IO::writeln(&format!("1・2・0 = {}", reflexive_ordered3_i8(1, 2, 0)));
        IO::writeln(&format!("1・2・1 = {}", reflexive_ordered3_i8(1, 2, 1)));
        IO::writeln(&format!("1・2・2 = {}", reflexive_ordered3_i8(1, 2, 2)));

        IO::writeln(&format!("2・0・0 = {}", reflexive_ordered3_i8(2, 0, 0)));
        IO::writeln(&format!("2・0・1 = {}", reflexive_ordered3_i8(2, 0, 1)));
        IO::writeln(&format!("2・0・2 = {}", reflexive_ordered3_i8(2, 0, 2)));
        IO::writeln(&format!("2・1・0 = {}", reflexive_ordered3_i8(2, 1, 0)));
        IO::writeln(&format!("2・1・1 = {}", reflexive_ordered3_i8(2, 1, 1)));
        IO::writeln(&format!("2・1・2 = {}", reflexive_ordered3_i8(2, 1, 2)));
        IO::writeln(&format!("2・2・0 = {}", reflexive_ordered3_i8(2, 2, 0)));
        IO::writeln(&format!("2・2・1 = {}", reflexive_ordered3_i8(2, 2, 1)));
        IO::writeln(&format!("2・2・2 = {}", reflexive_ordered3_i8(2, 2, 2)));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "3" {
        *starts += 1;
        IO::writeln("升Pは、点ABで作る平面上にあるか？");
        IO::writeln("P・A・B");
        IO::writeln("a{0,0} b{1,1} c{2,2}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };

        IO::writeln(&format!(
            "a・a・a = {}",
            intersect_point_on_plane(&a, &a, &a)
        ));
        IO::writeln(&format!(
            "a・a・b = {}",
            intersect_point_on_plane(&a, &a, &b)
        ));
        IO::writeln(&format!(
            "a・a・c = {}",
            intersect_point_on_plane(&a, &a, &c)
        ));
        IO::writeln(&format!(
            "a・b・a = {}",
            intersect_point_on_plane(&a, &b, &a)
        ));
        IO::writeln(&format!(
            "a・b・b = {}",
            intersect_point_on_plane(&a, &b, &b)
        ));
        IO::writeln(&format!(
            "a・b・c = {}",
            intersect_point_on_plane(&a, &b, &c)
        ));
        IO::writeln(&format!(
            "a・c・a = {}",
            intersect_point_on_plane(&a, &c, &a)
        ));
        IO::writeln(&format!(
            "a・c・b = {}",
            intersect_point_on_plane(&a, &c, &b)
        ));
        IO::writeln(&format!(
            "a・c・c = {}",
            intersect_point_on_plane(&a, &c, &c)
        ));

        IO::writeln(&format!(
            "b・a・a = {}",
            intersect_point_on_plane(&b, &a, &a)
        ));
        IO::writeln(&format!(
            "b・a・b = {}",
            intersect_point_on_plane(&b, &a, &b)
        ));
        IO::writeln(&format!(
            "b・a・c = {}",
            intersect_point_on_plane(&b, &a, &c)
        ));
        IO::writeln(&format!(
            "b・b・a = {}",
            intersect_point_on_plane(&b, &b, &a)
        ));
        IO::writeln(&format!(
            "b・b・b = {}",
            intersect_point_on_plane(&b, &b, &b)
        ));
        IO::writeln(&format!(
            "b・b・c = {}",
            intersect_point_on_plane(&b, &b, &c)
        ));
        IO::writeln(&format!(
            "b・c・a = {}",
            intersect_point_on_plane(&b, &c, &a)
        ));
        IO::writeln(&format!(
            "b・c・b = {}",
            intersect_point_on_plane(&b, &c, &b)
        ));
        IO::writeln(&format!(
            "b・c・c = {}",
            intersect_point_on_plane(&b, &c, &c)
        ));

        IO::writeln(&format!(
            "c・a・a = {}",
            intersect_point_on_plane(&c, &a, &a)
        ));
        IO::writeln(&format!(
            "c・a・b = {}",
            intersect_point_on_plane(&c, &a, &b)
        ));
        IO::writeln(&format!(
            "c・a・c = {}",
            intersect_point_on_plane(&c, &a, &c)
        ));
        IO::writeln(&format!(
            "c・b・a = {}",
            intersect_point_on_plane(&c, &b, &a)
        ));
        IO::writeln(&format!(
            "c・b・b = {}",
            intersect_point_on_plane(&c, &b, &b)
        ));
        IO::writeln(&format!(
            "c・b・c = {}",
            intersect_point_on_plane(&c, &b, &c)
        ));
        IO::writeln(&format!(
            "c・c・a = {}",
            intersect_point_on_plane(&c, &c, &a)
        ));
        IO::writeln(&format!(
            "c・c・b = {}",
            intersect_point_on_plane(&c, &c, &b)
        ));
        IO::writeln(&format!(
            "c・c・c = {}",
            intersect_point_on_plane(&c, &c, &c)
        ));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "4" {
        *starts += 1;
        IO::writeln("点ABは、同じ段にあるか？");
        IO::writeln("A・B");
        IO::writeln("a{0,0} b{1,1} c{2,2} d{2,0}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };
        let d = Point { x: 2, y: 0 };
        IO::writeln(&format!("a・a = {}", match_argangle0_p_p(&a, &a)));
        IO::writeln(&format!("a・b = {}", match_argangle0_p_p(&a, &b)));
        IO::writeln(&format!("a・c = {}", match_argangle0_p_p(&a, &c)));
        IO::writeln(&format!("a・d = {}", match_argangle0_p_p(&a, &d)));

        IO::writeln(&format!("b・a = {}", match_argangle0_p_p(&b, &a)));
        IO::writeln(&format!("b・b = {}", match_argangle0_p_p(&b, &b)));
        IO::writeln(&format!("b・c = {}", match_argangle0_p_p(&b, &c)));
        IO::writeln(&format!("b・d = {}", match_argangle0_p_p(&b, &d)));

        IO::writeln(&format!("c・a = {}", match_argangle0_p_p(&c, &a)));
        IO::writeln(&format!("c・b = {}", match_argangle0_p_p(&c, &b)));
        IO::writeln(&format!("c・c = {}", match_argangle0_p_p(&c, &c)));
        IO::writeln(&format!("c・d = {}", match_argangle0_p_p(&c, &d)));

        IO::writeln(&format!("d・a = {}", match_argangle0_p_p(&d, &a)));
        IO::writeln(&format!("d・b = {}", match_argangle0_p_p(&d, &b)));
        IO::writeln(&format!("d・c = {}", match_argangle0_p_p(&d, &c)));
        IO::writeln(&format!("d・d = {}", match_argangle0_p_p(&d, &d)));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "5" {
        *starts += 1;
        IO::writeln("点ABは、４つの角度の直線上にあるか？");
        IO::writeln("A・B");
        IO::writeln("a{0,0} b{1,1} c{2,2} d{2,0}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };
        let d = Point { x: 2, y: 0 };
        IO::writeln(&format!("a・a = {}", get_argangle4_p_p(&a, &a)));
        IO::writeln(&format!("a・b = {}", get_argangle4_p_p(&a, &b)));
        IO::writeln(&format!("a・c = {}", get_argangle4_p_p(&a, &c)));
        IO::writeln(&format!("a・d = {}", get_argangle4_p_p(&a, &d)));

        IO::writeln(&format!("b・a = {}", get_argangle4_p_p(&b, &a)));
        IO::writeln(&format!("b・b = {}", get_argangle4_p_p(&b, &b)));
        IO::writeln(&format!("b・c = {}", get_argangle4_p_p(&b, &c)));
        IO::writeln(&format!("b・d = {}", get_argangle4_p_p(&b, &d)));

        IO::writeln(&format!("c・a = {}", get_argangle4_p_p(&c, &a)));
        IO::writeln(&format!("c・b = {}", get_argangle4_p_p(&c, &b)));
        IO::writeln(&format!("c・c = {}", get_argangle4_p_p(&c, &c)));
        IO::writeln(&format!("c・d = {}", get_argangle4_p_p(&c, &d)));

        IO::writeln(&format!("d・a = {}", get_argangle4_p_p(&d, &a)));
        IO::writeln(&format!("d・b = {}", get_argangle4_p_p(&d, &b)));
        IO::writeln(&format!("d・c = {}", get_argangle4_p_p(&d, &c)));
        IO::writeln(&format!("d・d = {}", get_argangle4_p_p(&d, &d)));
    } else if 0 < (len - *starts) && &line[*starts..=*starts] == "6" {
        *starts += 1;
        IO::writeln("升Pは、線分AB上にあるか？");
        IO::writeln("P・A・B");
        IO::writeln("a{0,0} b{1,1} c{2,2} d{2,0}");
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 2, y: 2 };
        let d = Point { x: 2, y: 0 };

        IO::writeln(&format!(
            "a　　a・a = {}",
            intersect_point_on_line_segment(&a, &a, &a)
        ));
        IO::writeln(&format!(
            "a　　a・b = {}",
            intersect_point_on_line_segment(&a, &a, &b)
        ));
        IO::writeln(&format!(
            "a　　a・c = {}",
            intersect_point_on_line_segment(&a, &a, &c)
        ));
        IO::writeln(&format!(
            "a　　a・d = {}",
            intersect_point_on_line_segment(&a, &a, &d)
        ));
        IO::writeln(&format!(
            "a　　b・a = {}",
            intersect_point_on_line_segment(&a, &b, &a)
        ));
        IO::writeln(&format!(
            "a　　b・b = {}",
            intersect_point_on_line_segment(&a, &b, &b)
        ));
        IO::writeln(&format!(
            "a　　b・c = {}",
            intersect_point_on_line_segment(&a, &b, &c)
        ));
        IO::writeln(&format!(
            "a　　b・d = {}",
            intersect_point_on_line_segment(&a, &b, &d)
        ));
        IO::writeln(&format!(
            "a　　c・a = {}",
            intersect_point_on_line_segment(&a, &c, &a)
        ));
        IO::writeln(&format!(
            "a　　c・b = {}",
            intersect_point_on_line_segment(&a, &c, &b)
        ));
        IO::writeln(&format!(
            "a　　c・c = {}",
            intersect_point_on_line_segment(&a, &c, &c)
        ));
        IO::writeln(&format!(
            "a　　c・d = {}",
            intersect_point_on_line_segment(&a, &c, &d)
        ));
        IO::writeln(&format!(
            "a　　d・a = {}",
            intersect_point_on_line_segment(&a, &d, &a)
        ));
        IO::writeln(&format!(
            "a　　d・b = {}",
            intersect_point_on_line_segment(&a, &d, &b)
        ));
        IO::writeln(&format!(
            "a　　d・c = {}",
            intersect_point_on_line_segment(&a, &d, &c)
        ));
        IO::writeln(&format!(
            "a　　d・d = {}",
            intersect_point_on_line_segment(&a, &d, &d)
        ));

        IO::writeln(&format!(
            "b　　a・a = {}",
            intersect_point_on_line_segment(&b, &a, &a)
        ));
        IO::writeln(&format!(
            "b　　a・b = {}",
            intersect_point_on_line_segment(&b, &a, &b)
        ));
        IO::writeln(&format!(
            "b　　a・c = {}",
            intersect_point_on_line_segment(&b, &a, &c)
        ));
        IO::writeln(&format!(
            "b　　a・d = {}",
            intersect_point_on_line_segment(&b, &a, &d)
        ));
        IO::writeln(&format!(
            "b　　b・a = {}",
            intersect_point_on_line_segment(&b, &b, &a)
        ));
        IO::writeln(&format!(
            "b　　b・b = {}",
            intersect_point_on_line_segment(&b, &b, &b)
        ));
        IO::writeln(&format!(
            "b　　b・c = {}",
            intersect_point_on_line_segment(&b, &b, &c)
        ));
        IO::writeln(&format!(
            "b　　b・d = {}",
            intersect_point_on_line_segment(&b, &b, &d)
        ));
        IO::writeln(&format!(
            "b　　c・a = {}",
            intersect_point_on_line_segment(&b, &c, &a)
        ));
        IO::writeln(&format!(
            "b　　c・b = {}",
            intersect_point_on_line_segment(&b, &c, &b)
        ));
        IO::writeln(&format!(
            "b　　c・c = {}",
            intersect_point_on_line_segment(&b, &c, &c)
        ));
        IO::writeln(&format!(
            "b　　c・d = {}",
            intersect_point_on_line_segment(&b, &c, &d)
        ));
        IO::writeln(&format!(
            "b　　d・a = {}",
            intersect_point_on_line_segment(&b, &d, &a)
        ));
        IO::writeln(&format!(
            "b　　d・b = {}",
            intersect_point_on_line_segment(&b, &d, &b)
        ));
        IO::writeln(&format!(
            "b　　d・c = {}",
            intersect_point_on_line_segment(&b, &d, &c)
        ));
        IO::writeln(&format!(
            "b　　d・d = {}",
            intersect_point_on_line_segment(&b, &d, &d)
        ));

        IO::writeln(&format!(
            "c　　a・a = {}",
            intersect_point_on_line_segment(&c, &a, &a)
        ));
        IO::writeln(&format!(
            "c　　a・b = {}",
            intersect_point_on_line_segment(&c, &a, &b)
        ));
        IO::writeln(&format!(
            "c　　a・c = {}",
            intersect_point_on_line_segment(&c, &a, &c)
        ));
        IO::writeln(&format!(
            "c　　a・d = {}",
            intersect_point_on_line_segment(&c, &a, &d)
        ));
        IO::writeln(&format!(
            "c　　b・a = {}",
            intersect_point_on_line_segment(&c, &b, &a)
        ));
        IO::writeln(&format!(
            "c　　b・b = {}",
            intersect_point_on_line_segment(&c, &b, &b)
        ));
        IO::writeln(&format!(
            "c　　b・c = {}",
            intersect_point_on_line_segment(&c, &b, &c)
        ));
        IO::writeln(&format!(
            "c　　b・d = {}",
            intersect_point_on_line_segment(&c, &b, &d)
        ));
        IO::writeln(&format!(
            "c　　c・a = {}",
            intersect_point_on_line_segment(&c, &c, &a)
        ));
        IO::writeln(&format!(
            "c　　c・b = {}",
            intersect_point_on_line_segment(&c, &c, &b)
        ));
        IO::writeln(&format!(
            "c　　c・c = {}",
            intersect_point_on_line_segment(&c, &c, &c)
        ));
        IO::writeln(&format!(
            "c　　c・d = {}",
            intersect_point_on_line_segment(&c, &c, &d)
        ));
        IO::writeln(&format!(
            "c　　d・a = {}",
            intersect_point_on_line_segment(&c, &d, &a)
        ));
        IO::writeln(&format!(
            "c　　d・b = {}",
            intersect_point_on_line_segment(&c, &d, &b)
        ));
        IO::writeln(&format!(
            "c　　d・c = {}",
            intersect_point_on_line_segment(&c, &d, &c)
        ));
        IO::writeln(&format!(
            "c　　d・d = {}",
            intersect_point_on_line_segment(&c, &d, &d)
        ));
    } else {
        //IO::writeln( &format!( "未定義のテスト「{}」", &line[*starts..len-1] ) );
        ml_universe_dto.push_command( &"position startpos moves 6i5h 8c8d 9i9h 8d8e 3g3f 8e8f 5h4h 8f8g+ 1i1h 8g9h 2g2f 9h8h 9g9f 8h7i 2i3g 8b8i+ 2f2e 7i7h".to_string() );
        ml_universe_dto.push_command(&"position1".to_string());
        //IO::writeln( &ml_universe_dto.pop_command() );
    }

    // positionコマンドの読取を丸投げ
    // tusin::usi::read_position(&POS_593.to_string(), &mut ml_universe_dto);
    // tusin::usi::read_position(&POS_2.to_string(), &mut ml_universe_dto);
}
