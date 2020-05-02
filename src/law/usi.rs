//!
//! USIプロトコル
//!
use crate::cosmic::playing::Game;
use crate::cosmic::recording::Movement;
use crate::cosmic::smart::features::HandAddressType;
use crate::cosmic::smart::features::PieceMeaning;
use crate::cosmic::smart::square::{AbsoluteAddress, FILE_9, RANK_1};
use crate::spaceship::equipment::Beam;
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

    let mut buffer = Movement::default();

    // 1文字目と2文字目
    // 移動元とドロップ。
    enum Source {
        Move(usize, usize),
        Drop(HandAddressType),
    }

    let source = match &line[*starts..=*starts] {
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => Source::Drop(HandAddressType::Rook),
        "B" => Source::Drop(HandAddressType::Bishop),
        "G" => Source::Drop(HandAddressType::Gold),
        "S" => Source::Drop(HandAddressType::Silver),
        "N" => Source::Drop(HandAddressType::Knight),
        "L" => Source::Drop(HandAddressType::Lance),
        "P" => Source::Drop(HandAddressType::Pawn),
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            // 数字じゃないものが入ったら強制終了するんじゃないか☆（＾～＾）
            let file = if let Some(num) = atoi::<usize>(line[*starts..=*starts].as_bytes()) {
                num
            } else {
                panic!(Beam::trouble(&format!(
                    "(Err.72)  '{}' だった。",
                    &line[*starts..=*starts]
                )))
            };
            *starts += 1;

            match &line[*starts..=*starts] {
                "a" => Source::Move(file, 1),
                "b" => Source::Move(file, 2),
                "c" => Source::Move(file, 3),
                "d" => Source::Move(file, 4),
                "e" => Source::Move(file, 5),
                "f" => Source::Move(file, 6),
                "g" => Source::Move(file, 7),
                "h" => Source::Move(file, 8),
                "i" => Source::Move(file, 9),
                _ => {
                    panic!(Beam::trouble(&format!(
                        "(Err.90)  '{}' だった。",
                        &line[*starts..=*starts]
                    )));
                }
            }
        }
    };

    match source {
        Source::Move(file, rank) => {
            *starts += 1;
            buffer.source = Some(AbsoluteAddress::new(file, rank));
            buffer.drop = None;
        }
        Source::Drop(hand) => {
            *starts += 2;
            buffer.source = None;
            buffer.drop = Some(hand);
        }
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。

    // 3文字目
    let file = if let Some(num) = atoi::<usize>(line[*starts..=*starts].as_bytes()) {
        num
    } else {
        panic!(Beam::trouble(&format!(
            "(Err.118)  '{}' だった。",
            &line[*starts..=*starts]
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
            panic!(Beam::trouble(&format!(
                "(Err.136)  '{}' だった。",
                &line[*starts..=*starts]
            )));
        }
    };
    *starts += 1;

    // 行き先。
    buffer.destination = AbsoluteAddress::new(file, rank);

    // 5文字に「+」があれば成り。
    buffer.promote = if 0 < (len - *starts) && &line[*starts..=*starts] == "+" {
        *starts += 1;
        true
    } else {
        false
    };

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (len - *starts) && &line[*starts..=*starts] == " " {
        *starts += 1;
    }

    // 確定。
    game.set_move(&buffer);

    game.history.ply += 1;
    true
}

/// position コマンド 盤上部分のみ 読取
/// 初期化は既に終わらせてあります。
pub fn read_board(line: &str, starts: &mut usize, len: usize, game: &mut Game) {
    // 初期盤面
    let board = game.mut_starting();
    let mut file = FILE_9; //９筋から右方向へ読取
    let mut rank = RANK_1;

    // `/` か、`+`か、1桁の数か、1文字のアルファベットのいずれかだぜ☆（＾～＾）それ以外なら盤パート終了☆（＾～＾）
    enum BoardPart {
        /// 改行のようなものだぜ☆（＾～＾）
        NewLine,
        /// スペース数☆（＾～＾）
        Number(usize),
        /// 駒☆（＾～＾）+で始まるものもこっちだぜ☆（＾～＾）
        Alphabet(PieceMeaning),
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
            "K" => BoardPart::Alphabet(PieceMeaning::King1),
            "R" => BoardPart::Alphabet(PieceMeaning::Rook1),
            "B" => BoardPart::Alphabet(PieceMeaning::Bishop1),
            "G" => BoardPart::Alphabet(PieceMeaning::Gold1),
            "S" => BoardPart::Alphabet(PieceMeaning::Silver1),
            "N" => BoardPart::Alphabet(PieceMeaning::Knight1),
            "L" => BoardPart::Alphabet(PieceMeaning::Lance1),
            "P" => BoardPart::Alphabet(PieceMeaning::Pawn1),
            "k" => BoardPart::Alphabet(PieceMeaning::King2),
            "r" => BoardPart::Alphabet(PieceMeaning::Rook2),
            "b" => BoardPart::Alphabet(PieceMeaning::Bishop2),
            "g" => BoardPart::Alphabet(PieceMeaning::Gold2),
            "s" => BoardPart::Alphabet(PieceMeaning::Silver2),
            "n" => BoardPart::Alphabet(PieceMeaning::Knight2),
            "l" => BoardPart::Alphabet(PieceMeaning::Lance2),
            "p" => BoardPart::Alphabet(PieceMeaning::Pawn2),
            "+" => {
                *starts += 1;
                // 次に必ず１文字が来るぜ☆（＾～＾）
                match &line[*starts..=*starts] {
                    "R" => BoardPart::Alphabet(PieceMeaning::Dragon1),
                    "B" => BoardPart::Alphabet(PieceMeaning::Horse1),
                    "S" => BoardPart::Alphabet(PieceMeaning::PromotedSilver1),
                    "N" => BoardPart::Alphabet(PieceMeaning::PromotedKnight1),
                    "L" => BoardPart::Alphabet(PieceMeaning::PromotedLance1),
                    "P" => BoardPart::Alphabet(PieceMeaning::PromotedPawn1),
                    "r" => BoardPart::Alphabet(PieceMeaning::Dragon2),
                    "b" => BoardPart::Alphabet(PieceMeaning::Horse2),
                    "s" => BoardPart::Alphabet(PieceMeaning::PromotedSilver2),
                    "n" => BoardPart::Alphabet(PieceMeaning::PromotedKnight2),
                    "l" => BoardPart::Alphabet(PieceMeaning::PromotedLance2),
                    "p" => BoardPart::Alphabet(PieceMeaning::PromotedPawn2),
                    _ => {
                        panic!(Beam::trouble(&format!(
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
            BoardPart::Alphabet(piece) => {
                *starts += 1;
                board.push_piece_on_init(file, rank, Some(piece));
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
                N1Digit(isize),
                // 2桁の数
                N2Digit(isize),
            }
            'mg: loop {
                if 0 < (len - starts) {
                    // 数字か、数字でないかで大きく分かれるぜ☆（＾～＾）
                    // let mut count = 1;
                    let hand_count = match &line[starts..=starts] {
                        "1" => {
                            // 1枚のときは数字は付かないので、10～18 と確定☆
                            match &line[starts..=starts] {
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
                                    panic!(Beam::trouble(&format!(
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

                    use crate::cosmic::smart::features::PieceMeaning::*;
                    let hand = match &line[starts..=starts] {
                        "R" => Rook1,
                        "B" => Bishop1,
                        "G" => Gold1,
                        "S" => Silver1,
                        "N" => Knight1,
                        "L" => Lance1,
                        "P" => Pawn1,
                        "r" => Rook2,
                        "b" => Bishop2,
                        "g" => Gold2,
                        "s" => Silver2,
                        "n" => Knight2,
                        "l" => Lance2,
                        "p" => Pawn2,
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
    game.board.copy_from(&game.starting_board);
    /* TODO
    // 初期局面の利きを算出しようぜ☆（＾～＾）？
    game.board.init_controls();
    */

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite(line, &mut starts, len, game) {
        // 手目を戻す
        game.history.ply -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let ply = game.history.ply;
        game.do_move(&game.history.movements[ply as usize].clone());
    }
}
