use crate::entities::cosmic::playing::{Game, PosNums};
use crate::entities::cosmic::universe::Universe;
use crate::entities::law::cryptographic::*;
use crate::entities::law::usi::*;
use crate::entities::spaceship::equipment::Beam;
use crate::entities::spaceship::facility::{CommandRoom, GameRoom};
use crate::movegen::PseudoLegalMoves;
use crate::position::Square;
use crate::position::FILE_1;
use crate::usi::Chiyuri;
use crate::view::print_move_list;
use rand::Rng;

impl Chiyuri {
    pub fn do_(universe: &mut Universe, move_code: &str) {
        // コマンド読取。棋譜に追加され、手目も増える
        if read_move_code(&mut universe.game, move_code) {
            // 次の do_move で増えるので、手目をいったん戻す
            universe.game.history.decrease_moves_num();
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = universe.game.history.moves_num();
            let move_ = universe.game.history.moves[ply as usize];
            universe.game.do_move(move_);
        }
    }
    pub fn genmove(game: &Game) {
        // Generation move for debug
        let move_list = PseudoLegalMoves::generate(game.history.get_phase(), &game.position, true);
        print_move_list("genmove", &game.position, &move_list);
    }
    pub fn hash(universe: &Universe) {
        Beam::shoot("局面ハッシュ表示");
        let s = universe.game.get_positions_hash_text();
        Beam::shoot(&s);
    }
    pub fn how_much(tokens: &Vec<&str>) {
        // Example: how-much 7g7f
        let bestmove = tokens[1];
        Beam::shoot(&format!("Debug   | bestmove=|{}|", bestmove));
    }
    /// デバッグ用に棋譜（指し手の一覧）表示
    pub fn record(universe: &Universe) {
        Beam::shoot("棋譜表示");
        let s = universe.game.get_moves_history_debug_text();
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
            .position
            .for_all_pieces_on_board(&mut |i, sq, pc_ex| {
                Beam::shoot(&format!(
                    "[{}]{}{}",
                    i,
                    if let Some(sq) = sq {
                        format!(" {:?}", sq)
                    } else {
                        " --".to_string()
                    },
                    if let Some(piece_val) = pc_ex {
                        format!(" {} {:?}", piece_val.piece, piece_val.num)
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
        let secret_number = rand::thread_rng().gen_range(1..101); //1~100
        Beam::shoot(&format!("乱数={}", secret_number));
    }
    pub fn same(universe: &Universe) {
        let count = universe.game.count_same_position();
        Beam::shoot(&format!("同一局面調べ count={}", count));
    }
    pub fn startpos(universe: &mut Universe) {
        // 平手初期局面
        let tokens: Vec<&str> = POS_1.split(' ').collect();
        set_position(&mut universe.game, &tokens);
    }
    pub fn teigi_conv() {
        Beam::shoot("teigi::convのテスト");

        for ms in 1..9 {
            for hash in 0..10 {
                let sq = Square::from(FILE_1, ms);
                let next = push_sq_to_hash(hash, sq);
                let (hash_orig, sq_orig) = pop_sq_from_hash(next);
                Beam::shoot( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,sq_orig.number()
                ));
            }
        }
    }
    pub fn undo(universe: &mut Universe) {
        if !universe.game.undo_move() {
            Beam::shoot(&format!(
                "ply={} を、これより戻せません",
                universe.game.history.moves_num()
            ));
        }
    }
}
