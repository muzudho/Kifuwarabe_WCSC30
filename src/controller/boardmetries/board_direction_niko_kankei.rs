#![allow(dead_code)]
//!
//! 盤上の二項関係☆（＾～＾）
//!
use super::super::super::controller::common::conv::*;
use super::super::super::controller::consoles::asserts::*;
use super::super::super::controller::geometries::geo_direction_niko_kankei::*;
use super::super::super::controller::geometries::geo_teigi::*;
use super::super::super::model::master::direction::*;
use super::super::super::model::master::piece_struct::PieceStruct;
use super::super::super::model::master::square::*;

/**
 * 狙われている駒から見た、長い利きの駒の居る方向（８方向）
 *
 * 盤の方向は、後手から見た視点
 * 引数には、同じ升を指定しないものとする
 */
pub fn get_dir8_to_slider_from_target(
    sq_slider: &Square,
    slider_piece_struct: &PieceStruct,
    sq_target: &Square,
) -> Dir8 {
    debug_assert!(
        sq_slider.to_umasu() != sq_target.to_umasu(),
        "dosn't ms{}!={}",
        sq_slider.to_umasu(),
        sq_target.to_umasu()
    );

    assert_banjo_ms(
        sq_slider.to_umasu(),
        "(205a1)get_dir8_to_slider_from_target",
    );
    assert_banjo_ms(
        sq_target.to_umasu(),
        "(205a2)get_dir8_to_slider_from_target",
    );
    let p_slider = ms_to_p(sq_slider.to_umasu());
    let p_target = ms_to_p(sq_target.to_umasu());

    let (sn_slider, kms) = slider_piece_struct.phase_piece_type();
    use super::super::super::model::master::phase::Phase::*;
    use super::super::super::model::master::piece_type::PieceType::*;
    match kms {
        K => {
            // 筋か、段かのどちらかが同じ
            if match_argangle0_p_p(&p_slider, &p_target) {
                if match_a_south_of_b(&p_slider, &p_target) {
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle90_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else {
                Dir8::Owari
            }
        }
        Z => {
            // 左上がり筋か、左下がり筋かのどちらかが同じ
            if match_argangle45_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::SW
                } else {
                    Dir8::NE
                }
            } else if match_argangle135_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                Dir8::Owari
            }
        }
        S => {
            // 先後
            match sn_slider {
                Sen => Dir8::N,
                Go => Dir8::S,
                _ => Dir8::Owari,
            }
        }
        PK => {
            // 筋か、段か、
            // 左上がり筋か、左下がり筋かの　いずれかが同じ
            if match_argangle0_p_p(&p_slider, &p_target) {
                if match_a_south_of_b(&p_slider, &p_target) {
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle45_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else if match_argangle90_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else if match_argangle135_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                Dir8::Owari
            }
        }
        PZ => {
            // 筋か、段か、
            // 左上がり筋か、左下がり筋かの　いずれかが同じ
            if match_argangle0_p_p(&p_slider, &p_target) {
                if match_a_south_of_b(&p_slider, &p_target) {
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle45_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else if match_argangle90_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else if match_argangle135_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                Dir8::Owari
            }
        }
        _ => Dir8::Owari,
    }
}
