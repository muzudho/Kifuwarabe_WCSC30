//!
//! USIプロトコル
//!
use crate::cosmic::recording::Movement;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::*;
use crate::cosmic::toy_box::Piece;
use crate::cosmic::universe::*;
use crate::law::speed_of_light::*;
use crate::white_hole::io::*;

/*
/// USIプロトコル表記: 最多合法手５９３手局面
/// https://ameblo.jp/professionalhearts/entry-10001031814.html
pub const POS_593: &str = "position sfen R8/2K1S1SSk/4B4/9/9/9/9/9/1L1L1L3 w RBGSNLP3g3n17p 1";
*/

/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
pub const POS_1: &str = "position startpos";

/*
/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
pub const POS_2: &str =
  "position sfen lnsgkgsnl/9/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1 moves 5a6b 7g7f 3a3b";
*/

/// USIプロトコル表記: 平手初期局面（の盤上の駒配置部分のみ）
pub const STARTPOS_LN: usize = 57;
pub const STARTPOS: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL";

/// 指し手読取
/// 例: 7g7f
///
/// 読み取った指し手は、棋譜に入れる。
/// 現在の手目のところに入れ、手目のカウントアップも行う。
pub fn read_sasite(line: &str, starts: &mut usize, len: usize, universe: &mut Universe) -> bool {
    // 4文字か5文字あるはず。
    if (len - *starts) < 4 {
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return false;
    }

    let mut buffer = Movement::default();

    // 移動元とドロップ。
    // 1文字目と2文字目
    match &line[*starts..=*starts] {
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Rook);
        }
        "B" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Bishop);
        }
        "G" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Gold);
        }
        "S" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Silver);
        }
        "N" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Knight);
        }
        "L" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Lance);
        }
        "P" => {
            *starts += 2;
            buffer.source = Address::default().abs();
            buffer.drop = Some(PieceType::Pawn);
        }
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            let file = match &line[*starts..=*starts] {
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {
                    panic!(IO::panicing(&format!(
                        "(1) '{}' だった。",
                        &line[*starts..=*starts]
                    )));
                }
            };
            *starts += 1;

            let rank = match &line[*starts..=*starts] {
                "a" => 1,
                "b" => 2,
                "c" => 3,
                "d" => 4,
                "e" => 5,
                "f" => 6,
                "g" => 7,
                "h" => 8,
                "i" => 9,
                _ => {
                    panic!(IO::panicing(&format!(
                        "(2) '{}' だった。",
                        &line[*starts..=*starts]
                    )));
                }
            };
            *starts += 1;

            buffer.source = Address::new(file, rank).abs();
            buffer.drop = None;
        }
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。

    // 3文字目
    let file = match &line[*starts..=*starts] {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => {
            panic!(IO::panicing(&format!(
                "(3) '{}' だった。",
                &line[*starts..=*starts]
            )));
        }
    };
    *starts += 1;
    // 4文字目
    let rank = match &line[*starts..=*starts] {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        _ => {
            panic!(IO::panicing(&format!(
                "(4) '{}' だった。",
                &line[*starts..=*starts]
            )));
        }
    };
    *starts += 1;

    // 行き先。
    buffer.destination = Address::new(file, rank).abs();

    // 5文字に「+」があれば成り。
    if 0 < (len - *starts) && &line[*starts..=*starts] == "+" {
        buffer.promote = true;
        *starts += 1;
    } else {
        buffer.promote = false;
    }

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (len - *starts) && &line[*starts..=*starts] == " " {
        *starts += 1;
    }

    // 確定。
    universe.game.set_move(&buffer);

    universe.game.history.ply += 1;
    true
}

/// position コマンド 盤上部分のみ 読取
pub fn read_banjo(
    line: &str,
    starts: &mut usize,
    len: usize,
    universe: &mut Universe,
    speed_of_light: &SpeedOfLight,
) {
    // 盤部
    let board = universe.game.get_mut_starting_board();
    let mut suji = FILE_9; //９筋から右方向へ読取
    let mut dan = RANK_1;
    'ban: while 0 < (len - *starts) {
        match &line[*starts..=*starts] {
            "/" => {
                *starts += 1;
                suji = FILE_9;
                dan += 1;
            }
            "1" => {
                *starts += 1;
                board.set_piece(suji, dan, None);
                suji -= 1;
            }
            "2" => {
                *starts += 1;
                board.set_piece(suji, dan, None);
                suji -= 1;
                board.set_piece(suji, dan, None);
                suji -= 1;
            }
            "3" => {
                *starts += 1;
                board.set_piece(suji, dan, None);
                suji -= 1;
                board.set_piece(suji, dan, None);
                suji -= 1;
                board.set_piece(suji, dan, None);
                suji -= 1;
            }
            "4" => {
                *starts += 1;
                for _i_kara in 0..4 {
                    board.set_piece(suji, dan, None);
                    suji -= 1;
                }
            }
            "5" => {
                *starts += 1;
                for _i_kara in 0..5 {
                    board.set_piece(suji, dan, None);
                    suji -= 1;
                }
            }
            "6" => {
                *starts += 1;
                for _i_kara in 0..6 {
                    board.set_piece(suji, dan, None);
                    suji -= 1;
                }
            }
            "7" => {
                *starts += 1;
                for _i_kara in 0..7 {
                    board.set_piece(suji, dan, None);
                    suji -= 1;
                }
            }
            "8" => {
                *starts += 1;
                for _i_kara in 0..8 {
                    board.set_piece(suji, dan, None);
                    suji -= 1;
                }
            }
            "9" => {
                *starts += 1;
                for _i_kara in 0..9 {
                    board.set_piece(suji, dan, None);
                    suji -= 1;
                }
            }
            "K" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::King1));
                suji -= 1;
            }
            "R" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Rook1));
                suji -= 1;
            }
            "B" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Bishop1));
                suji -= 1;
            }
            "G" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Gold1));
                suji -= 1;
            }
            "S" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Silver1));
                suji -= 1;
            }
            "N" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Knight1));
                suji -= 1;
            }
            "L" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Lance1));
                suji -= 1;
            }
            "P" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Pawn1));
                suji -= 1;
            }
            "k" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::King2));
                suji -= 1;
            }
            "r" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Rook2));
                suji -= 1;
            }
            "b" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Bishop2));
                suji -= 1;
            }
            "g" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Gold2));
                suji -= 1;
            }
            "s" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Silver2));
                suji -= 1;
            }
            "n" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Knight2));
                suji -= 1;
            }
            "l" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Lance2));
                suji -= 1;
            }
            "p" => {
                *starts += 1;
                board.set_piece(suji, dan, Some(Piece::Pawn2));
                suji -= 1;
            }
            "+" => {
                *starts += 1;
                match &line[*starts..=*starts] {
                    "R" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::Dragon1));
                        suji -= 1;
                    }
                    "B" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::Horse1));
                        suji -= 1;
                    }
                    "S" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedSilver1));
                        suji -= 1;
                    }
                    "N" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedKnight1));
                        suji -= 1;
                    }
                    "L" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedLance1));
                        suji -= 1;
                    }
                    "P" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedPawn1));
                        suji -= 1;
                    }
                    "r" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::Dragon2));
                        suji -= 1;
                    }
                    "b" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::Horse2));
                        suji -= 1;
                    }
                    "s" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedSilver2));
                        suji -= 1;
                    }
                    "n" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedKnight2));
                        suji -= 1;
                    }
                    "l" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedLance2));
                        suji -= 1;
                    }
                    "p" => {
                        *starts += 1;
                        board.set_piece(suji, dan, Some(Piece::PromotedPawn2));
                        suji -= 1;
                    }
                    _ => {
                        panic!(IO::panicing(&format!(
                            "盤部(0) '{}' だった。",
                            &line[*starts..=*starts]
                        )));
                    }
                }
            }
            _ => {
                break 'ban;
            } // 盤部正常終了
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = universe.game.create_starting_position_hash(speed_of_light);
    universe.game.starting_position_hash = ky_hash;
}

/// position コマンド読取
pub fn set_position(line: &str, universe: &mut Universe, speed_of_light: &SpeedOfLight) {
    let mut starts = 0;

    // 全体の長さ
    let len = line.chars().count();

    // 局面をクリアー。手目も 0 に戻します。
    universe.game.clear();

    if 16 < (len - starts) && &line[starts..(starts + 17)] == "position startpos" {
        // 'position startpos' を読み飛ばし
        starts += 17;
        // 別途用意した平手初期局面文字列を読取
        let mut local_starts = 0;
        read_banjo(
            &STARTPOS.to_string(),
            &mut local_starts,
            STARTPOS_LN,
            universe,
            speed_of_light,
        );

        if 0 < (len - starts) && &line[starts..=starts] == " " {
            // ' ' を読み飛ばした。
            starts += 1;
        }
    } else if 13 < (len - starts) && &line[starts..(starts + 14)] == "position sfen " {
        starts += 14; // 'position sfen ' を読み飛ばし
        read_banjo(line, &mut starts, len, universe, speed_of_light);

        if 0 < (len - starts) && &line[starts..=starts] == " " {
            starts += 1;
        }

        if 0 < (len - starts) && (&line[starts..=starts] == "w" || &line[starts..=starts] == "b") {
            starts += 1;
        }

        if 0 < (len - starts) && &line[starts..=starts] == " " {
            starts += 1;
        }

        // 持ち駒の読取
        if 0 < (len - starts) && &line[starts..=starts] == "-" {
            starts += 1;
        } else {
            'mg: loop {
                if 0 < (len - starts) {
                    let mut maisu = 1;
                    match &line[starts..=starts] {
                        "1" => {
                            // 1枚のときは数字は付かないので、10～18 と確定☆
                            maisu = match &line[starts..=starts] {
                                "0" => 10,
                                "1" => 11,
                                "2" => 12,
                                "3" => 13,
                                "4" => 14,
                                "5" => 15,
                                "6" => 16,
                                "7" => 17,
                                "8" => 18,
                                _ => {
                                    panic!(IO::panicing(&format!(
                                        "持駒部(0) '{}' だった。",
                                        &line[starts..(starts + 2)]
                                    )));
                                }
                            };
                            starts += 2;
                        }
                        "2" => {
                            maisu = 2;
                            starts += 1;
                        }
                        "3" => {
                            maisu = 3;
                            starts += 1;
                        }
                        "4" => {
                            maisu = 4;
                            starts += 1;
                        }
                        "5" => {
                            maisu = 5;
                            starts += 1;
                        }
                        "6" => {
                            maisu = 6;
                            starts += 1;
                        }
                        "7" => {
                            maisu = 7;
                            starts += 1;
                        }
                        "8" => {
                            maisu = 8;
                            starts += 1;
                        }
                        "9" => {
                            maisu = 9;
                            starts += 1;
                        }
                        _ => {} // 駒の名前か、エラーなら次へ
                    }

                    use crate::cosmic::toy_box::Piece::*;
                    let km: Piece;
                    match &line[starts..=starts] {
                        "R" => {
                            km = Rook1;
                            starts += 1;
                        }
                        "B" => {
                            km = Bishop1;
                            starts += 1;
                        }
                        "G" => {
                            km = Gold1;
                            starts += 1;
                        }
                        "S" => {
                            km = Silver1;
                            starts += 1;
                        }
                        "N" => {
                            km = Knight1;
                            starts += 1;
                        }
                        "L" => {
                            km = Lance1;
                            starts += 1;
                        }
                        "P" => {
                            km = Pawn1;
                            starts += 1;
                        }
                        "r" => {
                            km = Rook2;
                            starts += 1;
                        }
                        "b" => {
                            km = Bishop2;
                            starts += 1;
                        }
                        "g" => {
                            km = Gold2;
                            starts += 1;
                        }
                        "s" => {
                            km = Silver2;
                            starts += 1;
                        }
                        "n" => {
                            km = Knight2;
                            starts += 1;
                        }
                        "l" => {
                            km = Lance2;
                            starts += 1;
                        }
                        "p" => {
                            km = Pawn2;
                            starts += 1;
                        }
                        _ => {
                            break 'mg;
                        } // 持駒部 正常終了
                    }

                    universe.game.get_mut_starting_board().set_hand(km, maisu);
                } //if
            } //loop
        } //else

        if 2 < (len - starts) && &line[starts..(starts + 3)] == " 1 " {
            starts += 3;
        }
    } else {
        IO::writeln("'position startpos' でも、'position sfen ' でも始まらなかった。");
        return;
    }

    if 4 < (len - starts) && &line[starts..(starts + 5)] == "moves" {
        starts += 5;
    }

    if 0 < (len - starts) && &line[starts..=starts] == " " {
        starts += 1;
    }

    // 初期局面を、現局面にコピーします
    universe.game.copy_starting_position_to_current_position();

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite(line, &mut starts, len, universe) {
        // 手目を戻す
        universe.game.history.ply -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let ply = universe.game.history.ply;
        universe.game.do_move(
            &universe.game.history.movements[ply as usize].clone(),
            speed_of_light,
        );
    }
}
