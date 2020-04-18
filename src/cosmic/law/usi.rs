//!
//! USIプロトコル
//!
use crate::cosmic::game::board::square::*;
use crate::cosmic::game::piece::piece::Piece;
use crate::cosmic::game::piece::piece_type::PieceType;
use crate::cosmic::law::speed_of_light::*;
use crate::cosmic::universe::*;
use crate::white_hole::io::*;

/// USIプロトコル表記: 最多合法手５９３手局面
/// https://ameblo.jp/professionalhearts/entry-10001031814.html
pub const POS_593: &str = "position sfen R8/2K1S1SSk/4B4/9/9/9/9/9/1L1L1L3 w RBGSNLP3g3n17p 1";

/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
pub const POS_1: &str = "position startpos";

/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
pub const POS_2: &str =
  "position sfen lnsgkgsnl/9/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1 moves 5a6b 7g7f 3a3b";

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

  // 移動元とドロップ。
  // 1文字目と2文字目
  match &line[*starts..=*starts] {
    // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
    "R" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Rook));
    }
    "B" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Bishop));
    }
    "G" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Gold));
    }
    "S" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Silver));
    }
    "N" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Knight));
    }
    "L" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Lance));
    }
    "P" => {
      *starts += 2;
      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_address(0));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(Some(PieceType::Pawn));
    }
    _ => {
      // 残りは「筋の数字」、「段のアルファベット」のはず。
      let suji;
      let dan;
      match &line[*starts..=*starts] {
        "1" => {
          suji = 1;
          *starts += 1;
        }
        "2" => {
          suji = 2;
          *starts += 1;
        }
        "3" => {
          suji = 3;
          *starts += 1;
        }
        "4" => {
          suji = 4;
          *starts += 1;
        }
        "5" => {
          suji = 5;
          *starts += 1;
        }
        "6" => {
          suji = 6;
          *starts += 1;
        }
        "7" => {
          suji = 7;
          *starts += 1;
        }
        "8" => {
          suji = 8;
          *starts += 1;
        }
        "9" => {
          suji = 9;
          *starts += 1;
        }
        _ => {
          IO::writeln(&format!("(1) '{}' だった。", &line[*starts..=*starts]));
          return false;
        }
      }

      match &line[*starts..=*starts] {
        "a" => {
          dan = 1;
          *starts += 1;
        }
        "b" => {
          dan = 2;
          *starts += 1;
        }
        "c" => {
          dan = 3;
          *starts += 1;
        }
        "d" => {
          dan = 4;
          *starts += 1;
        }
        "e" => {
          dan = 5;
          *starts += 1;
        }
        "f" => {
          dan = 6;
          *starts += 1;
        }
        "g" => {
          dan = 7;
          *starts += 1;
        }
        "h" => {
          dan = 8;
          *starts += 1;
        }
        "i" => {
          dan = 9;
          *starts += 1;
        }
        _ => {
          IO::writeln(&format!("(2) '{}' だった。", &line[*starts..=*starts]));
          return false;
        }
      }

      universe
        .game
        .position
        .set_current_movement_source_temporary(&Square::from_file_rank(suji, dan));
      universe
        .game
        .position
        .set_current_movement_drop_temporary(None);
    }
  }

  // 残りは「筋の数字」、「段のアルファベット」のはず。
  let suji;
  let dan;

  // 3文字目
  match &line[*starts..=*starts] {
    "1" => {
      suji = 1;
      *starts += 1;
    }
    "2" => {
      suji = 2;
      *starts += 1;
    }
    "3" => {
      suji = 3;
      *starts += 1;
    }
    "4" => {
      suji = 4;
      *starts += 1;
    }
    "5" => {
      suji = 5;
      *starts += 1;
    }
    "6" => {
      suji = 6;
      *starts += 1;
    }
    "7" => {
      suji = 7;
      *starts += 1;
    }
    "8" => {
      suji = 8;
      *starts += 1;
    }
    "9" => {
      suji = 9;
      *starts += 1;
    }
    _ => {
      IO::writeln(&format!("(3) '{}' だった。", &line[*starts..=*starts]));
      return false;
    }
  }
  // 4文字目
  match &line[*starts..=*starts] {
    "a" => {
      dan = 1;
      *starts += 1;
    }
    "b" => {
      dan = 2;
      *starts += 1;
    }
    "c" => {
      dan = 3;
      *starts += 1;
    }
    "d" => {
      dan = 4;
      *starts += 1;
    }
    "e" => {
      dan = 5;
      *starts += 1;
    }
    "f" => {
      dan = 6;
      *starts += 1;
    }
    "g" => {
      dan = 7;
      *starts += 1;
    }
    "h" => {
      dan = 8;
      *starts += 1;
    }
    "i" => {
      dan = 9;
      *starts += 1;
    }
    _ => {
      IO::writeln(&format!("(4) '{}' だった。", &line[*starts..=*starts]));
      return false;
    }
  }

  // 行き先。
  universe
    .game
    .position
    .set_current_movement_destination_temporary(&Square::from_file_rank(suji, dan));

  // 5文字に「+」があれば成り。
  if 0 < (len - *starts) && &line[*starts..=*starts] == "+" {
    universe
      .game
      .position
      .set_current_movement_promote_temporary(true);
    *starts += 1;
  } else {
    universe
      .game
      .position
      .set_current_movement_promote_temporary(false);
  }

  // 続きにスペース「 」が１つあれば読み飛ばす
  if 0 < (len - *starts) && &line[*starts..=*starts] == " " {
    *starts += 1;
  }

  // 確定。
  universe.game.build_current_movement();

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
        universe
          .game
          .set_piece_to_starting_position(suji, dan, None);
        suji -= 1;
      }
      "2" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, None);
        suji -= 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, None);
        suji -= 1;
      }
      "3" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, None);
        suji -= 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, None);
        suji -= 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, None);
        suji -= 1;
      }
      "4" => {
        *starts += 1;
        for _i_kara in 0..4 {
          universe
            .game
            .set_piece_to_starting_position(suji, dan, None);
          suji -= 1;
        }
      }
      "5" => {
        *starts += 1;
        for _i_kara in 0..5 {
          universe
            .game
            .set_piece_to_starting_position(suji, dan, None);
          suji -= 1;
        }
      }
      "6" => {
        *starts += 1;
        for _i_kara in 0..6 {
          universe
            .game
            .set_piece_to_starting_position(suji, dan, None);
          suji -= 1;
        }
      }
      "7" => {
        *starts += 1;
        for _i_kara in 0..7 {
          universe
            .game
            .set_piece_to_starting_position(suji, dan, None);
          suji -= 1;
        }
      }
      "8" => {
        *starts += 1;
        for _i_kara in 0..8 {
          universe
            .game
            .set_piece_to_starting_position(suji, dan, None);
          suji -= 1;
        }
      }
      "9" => {
        *starts += 1;
        for _i_kara in 0..9 {
          universe
            .game
            .set_piece_to_starting_position(suji, dan, None);
          suji -= 1;
        }
      }
      "K" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::King1));
        suji -= 1;
      }
      "R" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Rook1));
        suji -= 1;
      }
      "B" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Bishop1));
        suji -= 1;
      }
      "G" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Gold1));
        suji -= 1;
      }
      "S" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Silver1));
        suji -= 1;
      }
      "N" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Knight1));
        suji -= 1;
      }
      "L" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Lance1));
        suji -= 1;
      }
      "P" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Pawn1));
        suji -= 1;
      }
      "k" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::King2));
        suji -= 1;
      }
      "r" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Rook2));
        suji -= 1;
      }
      "b" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Bishop2));
        suji -= 1;
      }
      "g" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Gold2));
        suji -= 1;
      }
      "s" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Silver2));
        suji -= 1;
      }
      "n" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Knight2));
        suji -= 1;
      }
      "l" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Lance2));
        suji -= 1;
      }
      "p" => {
        *starts += 1;
        universe
          .game
          .set_piece_to_starting_position(suji, dan, Some(Piece::Pawn2));
        suji -= 1;
      }
      "+" => {
        *starts += 1;
        match &line[*starts..=*starts] {
          "R" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::Dragon1));
            suji -= 1;
          }
          "B" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::Horse1));
            suji -= 1;
          }
          "S" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedSilver1));
            suji -= 1;
          }
          "N" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedKnight1));
            suji -= 1;
          }
          "L" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedLance1));
            suji -= 1;
          }
          "P" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedPawn1));
            suji -= 1;
          }
          "r" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::Dragon2));
            suji -= 1;
          }
          "b" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::Horse2));
            suji -= 1;
          }
          "s" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedSilver2));
            suji -= 1;
          }
          "n" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedKnight2));
            suji -= 1;
          }
          "l" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedLance2));
            suji -= 1;
          }
          "p" => {
            *starts += 1;
            universe
              .game
              .set_piece_to_starting_position(suji, dan, Some(Piece::PromotedPawn2));
            suji -= 1;
          }
          _ => {
            IO::writeln(&format!("盤部(0) '{}' だった。", &line[*starts..=*starts]));
            break 'ban;
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

/**
 * position コマンド読取
 */
pub fn read_position(line: &str, universe: &mut Universe, speed_of_light: &SpeedOfLight) {
  let mut starts = 0;

  // 全体の長さ
  let len = line.chars().count();

  // 局面をクリアー。手目も 0 に戻します。
  universe.game.clear_all_positions();

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
              match &line[starts..=starts] {
                "0" => {
                  maisu = 10;
                  starts += 2;
                }
                "1" => {
                  maisu = 11;
                  starts += 2;
                }
                "2" => {
                  maisu = 12;
                  starts += 2;
                }
                "3" => {
                  maisu = 13;
                  starts += 2;
                }
                "4" => {
                  maisu = 14;
                  starts += 2;
                }
                "5" => {
                  maisu = 15;
                  starts += 2;
                }
                "6" => {
                  maisu = 16;
                  starts += 2;
                }
                "7" => {
                  maisu = 17;
                  starts += 2;
                }
                "8" => {
                  maisu = 18;
                  starts += 2;
                }
                _ => {
                  IO::writeln(&format!(
                    "持駒部(0) '{}' だった。",
                    &line[starts..(starts + 2)]
                  ));
                  return;
                }
              }
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

          use crate::cosmic::game::piece::piece::Piece::*;
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

          universe.game.set_starting_position_hand_piece(km, maisu);
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

    // 現局面表示
    //let s1 = &ml_universe_dto.print_ky( &PosNums::Current );
    //g_writeln( &s1 );
  }
}
