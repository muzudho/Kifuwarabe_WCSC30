use crate::config::*;
use crate::cosmic::daydream::Tree;
use crate::cosmic::playing::{Game, PosNums};
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::square::{AbsoluteAddress, FILE_1};
use crate::cosmic::universe::Universe;
use crate::law::cryptographic::*;
use crate::law::generate_move::PseudoLegalMoves;
use crate::law::generate_move::Way;
use crate::law::usi::*;
use crate::spaceship::equipment::{Beam, PvString, Telescope};
use crate::spaceship::facility::{CommandRoom, GameRoom, Kitchen}; //, RestRoom
use rand::Rng;
use std::io as std_io;

/// 船長：きふわらべ
///
/// 対局で許されている命令だけをするぜ☆（＾～＾）
pub struct Kifuwarabe {}
impl Kifuwarabe {
    pub fn catch_the_message() -> (String, usize, usize) {
        let mut line: String = String::new();

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        match std_io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            Err(e) => panic!(Beam::trouble(&format!(
                "(Err.28)  Failed to read line. / {}",
                e
            ))),
        };

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!(Beam::trouble(&format!(
                "(Err.38)  Failed to parse. / {}",
                e
            ))),
        };

        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let starts = 0;

        (line, len, starts)
    }
    /// bestmoveコマンドを送るぜ☆（＾～＾） 思考するのもこの中だぜ☆（＾～＾）
    pub fn go(universe: &mut Universe) {
        // go btime 40000 wtime 50000 binc 10000 winc 10000
        let mut tree = Tree::new(
            universe.option_many_ways_weight,
            universe.option_komawari_weight,
            universe.option_promotion_weight,
            universe.option_depth_not_to_give_up,
        );
        let ts = tree.iteration_deeping(universe);
        // その手を選んだ理由☆（＾～＾）
        universe.game.info.print(
            None,
            Some((tree.state_nodes, tree.nps())),
            Some(ts.bestmove.value),
            ts.bestmove.movement,
            &Some(PvString::String(ts.bestmove.reason.to_string())),
        );
        // 例: bestmove 7g7f
        // 例: bestmove resign
        Beam::shoot(&format!(
            "bestmove {}",
            if let Some(bestmove) = ts.bestmove.movement {
                format!("{}", bestmove)
            } else {
                "resign".to_string()
            }
        ));
    }
    pub fn isready() {
        Beam::shoot("readyok");
    }
    pub fn position(universe: &mut Universe, line: &String) {
        // positionコマンドの読取を丸投げ
        set_position(&line, &mut universe.game);
    }
    pub fn setoption_name(universe: &mut Universe, line: &String) {
        // Example: setoption name USI_Ponder value true
        let label1_width = "setoption name ".len(); // 15
        if let Some(name_width) = line[label1_width..].find(' ') {
            let name = &line[label1_width..(label1_width + name_width)];
            // IO::writeln(&format!("Debug name=|{}|", name));
            let label2_width = " value ".len(); // 7
            let value = &line[(label1_width + name_width + label2_width)..];
            // IO::writeln(&format!("Debug value=|{}|", value));
            match name {
                "ManyWaysPer1000" => {
                    universe.option_many_ways_weight = value.parse().unwrap();
                }
                "DepthNotToGiveUp" => {
                    universe.option_depth_not_to_give_up = value.parse().unwrap();
                }
                "KomawariWeightPer1000" => {
                    universe.option_komawari_weight = value.parse().unwrap();
                }
                "PromotionWeightPer1000" => {
                    universe.option_promotion_weight = value.parse().unwrap();
                }
                "MaxDepth" => {
                    universe.option_max_depth = value.parse().unwrap();
                }
                "MinThinkSec" => {
                    universe.option_min_think_sec = value.parse().unwrap();
                }
                "MaxThinkSec" => {
                    universe.option_max_think_sec = value.parse().unwrap();
                }
                _ => {}
            }
        };
    }
    pub fn usi() {
        Beam::shoot(&format!("id name {}", ENGINE_NAME));
        Beam::shoot(&format!("id author {}", ENGINE_AUTHOR));
        /*
        IO::writeln("option name BookFile type string default public.bin");
        IO::writeln("option name UseBook type check default true");
        IO::writeln("option name Selectivity type spin default 2 min 0 max 4");
        IO::writeln(
            "option name Style type combo default Normal var Solid var Normal var Risky",
        );
        IO::writeln("option name ResetLearning type button");
        IO::writeln("option name LearningFile type filename default <empty>");
        */
        // アルファベット順ではなく、将棋所のダイアログボックスが見やすくなるように並べろだぜ☆（＾～＾）
        // 読みの深さ関連☆（＾～＾）
        Beam::shoot("option name DepthNotToGiveUp type spin default 4 min 1 max 8");
        Beam::shoot("option name MaxDepth type spin default 7 min 1 max 15");
        // 思考時間関連☆（＾～＾）
        Beam::shoot("option name MinThinkSec type spin default 5 min 0 max 599");
        Beam::shoot("option name MaxThinkSec type spin default 17 min 1 max 600");
        // 評価値関連☆（＾～＾）
        Beam::shoot(
            "option name KomawariWeightPer1000 type spin default 1000 min -100000 max 100000",
        );
        Beam::shoot("option name ManyWaysPer1000 type spin default 1000 min -100000 max 100000");
        Beam::shoot(
            "option name PromotionWeightPer1000 type spin default 1000 min -100000 max 100000",
        );
        Beam::shoot("usiok");
    }
    pub fn usinewgame(universe: &mut Universe) {
        universe.game.clear();
    }
}

/// 副船長：ちゆり
///
/// 対局でやっちゃいかん命令なら任せろだぜ☆（＾～＾）
pub struct Chiyuri {}
impl Chiyuri {
    pub fn do_(universe: &mut Universe, line: &str, len: usize, mut starts: usize) {
        starts += 3;
        // コマンド読取。棋譜に追加され、手目も増える
        if read_sasite(&line, &mut starts, len, &mut universe.game) {
            // 手目を戻す
            universe.game.history.ply -= 1;
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = universe.game.history.ply;
            let ss = universe.game.history.movements[ply as usize].clone();
            universe.game.do_move(&ss);
        }
    }
    pub fn genmove(game: &Game) {
        // Generation move.
        // FIXME 合法手とは限らない
        let mut ways = Vec::<Way>::new();
        /* TODO
        PseudoLegalMoves::make_move(
            game.history.get_friend(),
            &game.board,
            &mut |way, _destination| {
                if let Some(way_val) = way {
                    ways.push(way_val);
                }
            },
        );
        */
        PseudoLegalMoves::make_move(game.history.get_friend(), &game.board, &mut |way| {
            ways.push(way);
        });
        Beam::shoot("----指し手生成(合法手とは限らない) ここから----");
        Kitchen::print_ways(&ways);
        Beam::shoot("----指し手生成(合法手とは限らない) ここまで----");
    }
    pub fn hash(universe: &Universe) {
        Beam::shoot("局面ハッシュ表示");
        let s = universe.game.get_positions_hash_text();
        Beam::shoot(&s);
    }
    pub fn how_much(line: &str) {
        // Example: how-much 7g7f
        let bestmove = &line[9..];
        Beam::shoot(&format!("Debug   | bestmove=|{}|", bestmove));
    }
    pub fn kifu(universe: &Universe) {
        Beam::shoot("棋譜表示");
        let s = universe.game.get_moves_history_text();
        Beam::shoot(&s);
    }
    /* TODO
    pub fn kiki(universe: &Universe) {
        // 利き数表示
        let s = RestRoom::to_string(&universe.game, Phase::First);
        Beam::shoot(&s);
        let s = RestRoom::to_string(&universe.game, Phase::Second);
        Beam::shoot(&s);
    }
    */
    pub fn list40(universe: &Universe) {
        Beam::shoot("----駒リスト40表示 ここから----");
        universe
            .game
            .board
            .for_all_pieces_on_board(&mut |i, adr, piece| {
                Beam::shoot(&format!(
                    "[{}]{}{}",
                    i,
                    if let Some(adr_val) = adr {
                        format!(" {:?}", adr_val)
                    } else {
                        " --".to_string()
                    },
                    if let Some(piece_val) = piece {
                        format!(" {} {:?}", piece_val.meaning, piece_val.num)
                    } else {
                        " --".to_string()
                    }
                ));
            });
        Beam::shoot("----駒リスト40表示 ここまで----");
    }
    pub fn len0(universe: &mut Universe) {
        Beam::shoot("len==0");
        if !&universe.dialogue_mode {
            // 空打ち１回目なら、対話モードへ☆（＾～＾）
            universe.dialogue_mode = true;
            // タイトル表示
            // １画面は２５行だが、最後の２行は開けておかないと、
            // カーソルが２行分場所を取るんだぜ☆（＾～＾）
            CommandRoom::print_title();
        } else {
            // 局面表示
            let s = GameRoom::to_string(&universe.game, PosNums::Current);
            Beam::shoot(&s);
        }
    }
    pub fn pos(universe: &Universe) {
        // 現局面表示
        let s = GameRoom::to_string(&universe.game, PosNums::Current);
        Beam::shoot(&s);
    }
    pub fn pos0(universe: &Universe) {
        // 初期局面表示
        let s = GameRoom::to_string(&universe.game, PosNums::Start);
        Beam::shoot(&s);
    }
    pub fn rand() {
        Beam::shoot("3<len rand");
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        Beam::shoot(&format!("乱数={}", secret_number));
    }
    pub fn same(universe: &Universe) {
        let count = universe.game.count_same_position();
        Beam::shoot(&format!("同一局面調べ count={}", count));
    }
    pub fn startpos(universe: &mut Universe) {
        // 平手初期局面
        set_position(&POS_1.to_string(), &mut universe.game);
    }
    pub fn teigi_conv() {
        Beam::shoot("teigi::convのテスト");

        for ms in 1..9 {
            for hash in 0..10 {
                let sq = AbsoluteAddress::new(FILE_1, ms);
                let next = push_sq_to_hash(hash, Some(&sq));
                let (hash_orig, square_orig) = pop_sq_from_hash(next);
                Beam::shoot( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,if let Some(square_orig_val) = square_orig{ square_orig_val.address()}else{0}
                ));
            }
        }
    }
    pub fn undo(universe: &mut Universe) {
        if !universe.game.undo_move() {
            Beam::shoot(&format!(
                "ply={} を、これより戻せません",
                universe.game.history.ply
            ));
        }
    }
}

/// 乗組員：夢美
pub struct Yumemi {}
impl Yumemi {
    /// 望遠鏡を覗き込みましょう。
    pub fn look_into_the_telescope() {
        Telescope::look();
    }
}
