//!
//! コレクションの内容をダンプ（全部見る）とかだぜ☆（＾～＾）
//!
use crate::model::univ::gam::movement_builder::*;
use crate::model::univ::gam::piece_type::*;
use crate::model::univ::gam::square::*;
use crate::model::universe::*;
use std::collections::HashSet;
use std::hash::BuildHasher;

/// 指し手
pub fn print_movement_hashset<S: BuildHasher>(ss_hashset: &HashSet<u64, S>) {
    g_writeln(&format!("ss_hashset.len()={}", ss_hashset.len()));
    // 辞書順ソート
    let mut vec_ss_str = Vec::new();
    for ss_hash in ss_hashset {
        let ss = MovementBuilder::from_hash(*ss_hash);
        let ss_str = format!("{}", ss);
        vec_ss_str.push(ss_str);
    }
    //vec_ss_str.sort();
    vec_ss_str.sort_by(|y_str, x_str| {
        let y_arr: Vec<_> = y_str.chars().collect();
        let x_arr: Vec<_> = x_str.chars().collect();
        use std::cmp::min;
        let len = min(y_arr.len(), x_arr.len());

        use std::cmp::Ordering;
        for i in 0..len {
            match x_arr[i].cmp(&y_arr[i]) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }

        // Returns Ordering::Greater, Ordering::Less, Ordering::Equal.
        x_arr.len().cmp(&y_arr.len())
    });
    vec_ss_str.reverse();

    for (i, ss_str) in vec_ss_str.into_iter().enumerate() {
        g_writeln(&format!("[{}] {}", i, ss_str));
    }
}

/// 升を表示
pub fn print_square_hashset<S: BuildHasher>(sq_hashset: &HashSet<Square, S>) {
    g_writeln(&format!("sq_hashset.len()={}", sq_hashset.len()));
    for sq in sq_hashset {
        let ms = (*sq).to_usquare();
        match ms {
            SQUARE_NONE => break,
            _ => g_writeln(&format!("ms({})", ms)),
        }
    }
}

/// 升を表示
pub fn print_square_vec(sq_vec: &[Square]) {
    g_writeln(&format!("sq_vec.len()={}", sq_vec.len()));
    for sq in sq_vec {
        let ms = sq.to_usquare();
        match ms {
            SQUARE_NONE => break,
            _ => g_writeln(&format!("ms({})", ms)),
        }
    }
}

/// 駒種類
pub fn print_piece_type_hashset<S: BuildHasher>(num_piece_type_hashset: &HashSet<usize, S>) {
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
