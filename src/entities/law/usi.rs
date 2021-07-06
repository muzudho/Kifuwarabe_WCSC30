//!
//! USIプロトコル
//!
use crate::entities::cosmic::playing::Game;
use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::square::{FILE_9, RANK_1};
use crate::entities::spaceship::equipment::Beam;
use crate::take1base::Move;
use crate::take1base::Piece;
use atoi::atoi;

// 局面の最多合法手５９３手
//pub const MAX_WAYS: usize = 593;

/*
/// USIプロトコル表記: 最多合法手５９３手の局面
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
pub fn read_sasite(line: &str, starts: &mut usize, len: usize, game: &mut Game) -> bool {
    // 4文字か5文字あるはず。
    if (len - *starts) < 4 {
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return false;
    }

    let from = match &line[*starts..=*starts] {
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 101,
                Phase::Second => 109,
            }
        }
        "B" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 102,
                Phase::Second => 110,
            }
        }
        "G" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 103,
                Phase::Second => 111,
            }
        }
        "S" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 104,
                Phase::Second => 112,
            }
        }
        "N" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 105,
                Phase::Second => 113,
            }
        }
        "L" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 106,
                Phase::Second => 114,
            }
        }
        "P" => {
            *starts += 2;
            match game.history.get_phase() {
                Phase::First => 107,
                Phase::Second => 115,
            }
        }
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            // 数字じゃないものが入ったら強制終了するんじゃないか☆（＾～＾）
            let file = if let Some(num) = atoi::<usize>(line[*starts..=*starts].as_bytes()) {
                num
            } else {
                std::panic::panic_any(Beam::trouble(&format!(
                    "(Err.72)  '{}' だった。",
                    &line[*starts..=*starts]
                )))
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
                    std::panic::panic_any(Beam::trouble(&format!(
                        "(Err.90)  '{}' だった。",
                        &line[*starts..=*starts]
                    )));
                }
            };
            *starts += 1;

            file * 10 + rank
        }
    };

    // 残りは「筋の数字」、「段のアルファベット」のはず。
    let to = {
        // 3文字目
        let file = if let Some(num) = atoi::<usize>(line[*starts..=*starts].as_bytes()) {
            num
        } else {
            std::panic::panic_any(Beam::trouble(&format!(
                "(Err.118)  '{}' だった。 line='{}'",
                &line[*starts..=*starts],
                &line
            )));
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
                std::panic::panic_any(Beam::trouble(&format!(
                    "(Err.136)  '{}' だった。",
                    &line[*starts..=*starts]
                )));
            }
        };
        *starts += 1;

        // 行き先。
        file * 10 + rank
    };

    // 5文字に「+」があれば成り。
    let promote = if 0 < (len - *starts) && &line[*starts..=*starts] == "+" {
        *starts += 1;
        1
    } else {
        0
    };

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (len - *starts) && &line[*starts..=*starts] == " " {
        *starts += 1;
    }

    let move_ = ((promote << 14) + (to << 7) + from) as Move;

    // 確定。
    game.set_move(move_);

    game.history.increase_moves_num();
    true
}

/// position コマンド 盤上部分のみ 読取
/// 初期化は既に終わらせてあります。
pub fn read_board(line: &str, starts: &mut usize, len: usize, game: &mut Game) {
    // 初期盤面
    let position = game.mut_starting();
    let mut file = FILE_9; //９筋から右方向へ読取
    let mut rank = RANK_1;

    // `/` か、`+`か、1桁の数か、1文字のアルファベットのいずれかだぜ☆（＾～＾）それ以外なら盤パート終了☆（＾～＾）
    enum BoardPart {
        /// 改行のようなものだぜ☆（＾～＾）
        NewLine,
        /// スペース数☆（＾～＾）
        Number(u8),
        /// 駒☆（＾～＾）+で始まるものもこっちだぜ☆（＾～＾）
        Alphabet(Piece),
    }

    'ban: while 0 < (len - *starts) {
        let board_part = match &line[*starts..=*starts] {
            "/" => BoardPart::NewLine,
            "1" => BoardPart::Number(1),
            "2" => BoardPart::Number(2),
            "3" => BoardPart::Number(3),
            "4" => BoardPart::Number(4),
            "5" => BoardPart::Number(5),
            "6" => BoardPart::Number(6),
            "7" => BoardPart::Number(7),
            "8" => BoardPart::Number(8),
            "9" => BoardPart::Number(9),
            "K" => BoardPart::Alphabet(Piece::K1),
            "R" => BoardPart::Alphabet(Piece::R1),
            "B" => BoardPart::Alphabet(Piece::B1),
            "G" => BoardPart::Alphabet(Piece::G1),
            "S" => BoardPart::Alphabet(Piece::S1),
            "N" => BoardPart::Alphabet(Piece::N1),
            "L" => BoardPart::Alphabet(Piece::L1),
            "P" => BoardPart::Alphabet(Piece::P1),
            "k" => BoardPart::Alphabet(Piece::K2),
            "r" => BoardPart::Alphabet(Piece::R2),
            "b" => BoardPart::Alphabet(Piece::B2),
            "g" => BoardPart::Alphabet(Piece::G2),
            "s" => BoardPart::Alphabet(Piece::S2),
            "n" => BoardPart::Alphabet(Piece::N2),
            "l" => BoardPart::Alphabet(Piece::L2),
            "p" => BoardPart::Alphabet(Piece::P2),
            "+" => {
                *starts += 1;
                // 次に必ず１文字が来るぜ☆（＾～＾）
                match &line[*starts..=*starts] {
                    "R" => BoardPart::Alphabet(Piece::PR1),
                    "B" => BoardPart::Alphabet(Piece::PB1),
                    "S" => BoardPart::Alphabet(Piece::PS1),
                    "N" => BoardPart::Alphabet(Piece::PN1),
                    "L" => BoardPart::Alphabet(Piece::PL1),
                    "P" => BoardPart::Alphabet(Piece::PP1),
                    "r" => BoardPart::Alphabet(Piece::PR2),
                    "b" => BoardPart::Alphabet(Piece::PB2),
                    "s" => BoardPart::Alphabet(Piece::PS2),
                    "n" => BoardPart::Alphabet(Piece::PN2),
                    "l" => BoardPart::Alphabet(Piece::PL2),
                    "p" => BoardPart::Alphabet(Piece::PP2),
                    _ => {
                        std::panic::panic_any(Beam::trouble(&format!(
                            "(Err.235)  盤部(0) '{}' だった。",
                            &line[*starts..=*starts]
                        )));
                    }
                }
            }
            _ => {
                break 'ban;
            } // 盤部正常終了
        };

        match board_part {
            BoardPart::Alphabet(pc) => {
                *starts += 1;
                position.push_piece_on_init(file, rank, Some(pc));
                file -= 1;
            }
            BoardPart::Number(space_num) => {
                *starts += 1;
                // もともと空升なんで、飛ばそうぜ☆（＾～＾）
                file -= space_num;
            }
            BoardPart::NewLine => {
                *starts += 1;
                file = FILE_9;
                rank += 1;
            }
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = game.create_starting_position_hash();
    game.starting_position_hash = ky_hash;
}

/// position コマンド読取
pub fn set_position(line: &str, game: &mut Game) {
    let mut starts = 0;

    // 全体の長さ
    let len = line.chars().count();

    // 局面をクリアー。手目も 0 に戻します。
    game.clear();

    if 16 < (len - starts) && &line[starts..(starts + 17)] == "position startpos" {
        // 'position startpos' を読み飛ばし
        starts += 17;
        // 別途用意した平手初期局面文字列を読取
        let mut local_starts = 0;
        read_board(&STARTPOS.to_string(), &mut local_starts, STARTPOS_LN, game);

        if 0 < (len - starts) && &line[starts..=starts] == " " {
            // ' ' を読み飛ばした。
            starts += 1;
        }
    } else if 13 < (len - starts) && &line[starts..(starts + 14)] == "position sfen " {
        starts += 14; // 'position sfen ' を読み飛ばし
        read_board(line, &mut starts, len, game);

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
            enum HandCount {
                // 数字なし
                N0Digit,
                // 1桁の数
                N1Digit(u8),
                // 2桁の数
                N2Digit(u8),
            }
            'mg: loop {
                if 0 < (len - starts) {
                    // 数字か、数字でないかで大きく分かれるぜ☆（＾～＾）
                    // let mut count = 1;
                    let hand_count = match &line[starts..=starts] {
                        "1" => {
                            // 1枚のときは数字は付かないので、10～18 と確定☆
                            match &line[starts + 1..=starts + 1] {
                                "0" => HandCount::N2Digit(10),
                                "1" => HandCount::N2Digit(11),
                                "2" => HandCount::N2Digit(12),
                                "3" => HandCount::N2Digit(13),
                                "4" => HandCount::N2Digit(14),
                                "5" => HandCount::N2Digit(15),
                                "6" => HandCount::N2Digit(16),
                                "7" => HandCount::N2Digit(17),
                                "8" => HandCount::N2Digit(18),
                                _ => {
                                    std::panic::panic_any(Beam::trouble(&format!(
                                        "(Err.346)  持駒部(0) '{}' だった。",
                                        &line[starts..(starts + 2)]
                                    )));
                                }
                            }
                        }
                        "2" => HandCount::N1Digit(2),
                        "3" => HandCount::N1Digit(3),
                        "4" => HandCount::N1Digit(4),
                        "5" => HandCount::N1Digit(5),
                        "6" => HandCount::N1Digit(6),
                        "7" => HandCount::N1Digit(7),
                        "8" => HandCount::N1Digit(8),
                        "9" => HandCount::N1Digit(9),
                        _ => HandCount::N0Digit, // 駒の名前か、エラーなら次へ
                    };

                    let hand_num = match hand_count {
                        HandCount::N0Digit => {
                            // 持ち駒が１枚のときは数は付かないぜ☆（＾～＾）
                            1
                        }
                        HandCount::N1Digit(hand_num) => {
                            starts += 1;
                            hand_num
                        }
                        HandCount::N2Digit(hand_num) => {
                            starts += 2;
                            hand_num
                        }
                    };

                    use crate::take1base::Piece::*;
                    let hand = match &line[starts..=starts] {
                        "R" => R1,
                        "B" => B1,
                        "G" => G1,
                        "S" => S1,
                        "N" => N1,
                        "L" => L1,
                        "P" => P1,
                        "r" => R2,
                        "b" => B2,
                        "g" => G2,
                        "s" => S2,
                        "n" => N2,
                        "l" => L2,
                        "p" => P2,
                        _ => {
                            break 'mg;
                        } // 持駒部 正常終了
                    };
                    starts += 1;

                    game.mut_starting().push_hand_on_init(hand, hand_num);
                } //if
            } //loop
        } //else

        if 2 < (len - starts) && &line[starts..(starts + 3)] == " 1 " {
            starts += 3;
        }
    } else {
        Beam::shoot("'position startpos' でも、'position sfen ' でも始まらなかった。");
        return;
    }

    if 4 < (len - starts) && &line[starts..(starts + 5)] == "moves" {
        starts += 5;
    }

    if 0 < (len - starts) && &line[starts..=starts] == " " {
        starts += 1;
    }

    // 初期局面を、現局面にコピーします
    game.position.copy_from(&game.starting_board);
    /* TODO
    // 初期局面の利きを算出しようぜ☆（＾～＾）？
    game.position.init_controls();
    */

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite(line, &mut starts, len, game) {
        // 次の do_move で増えるので、手目をいったん戻す
        game.history.decrease_moves_num();
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let ply = game.history.moves_num();
        game.do_move(game.history.moves[ply as usize]);
    }
}
