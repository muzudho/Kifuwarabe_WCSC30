use crate::cosmic::shogi::playing::{Game, PosNums};
use crate::cosmic::shogi::state::Person;
use crate::cosmic::smart::square::*;
use crate::cosmic::toy_box::{Piece, PIECE_WHITE_SPACE};

pub struct GameView {}
impl GameView {
        fn to_string2(piece_o: Option<Piece>) -> String {
                if let Some(piece) = piece_o {
                        format!("{}", piece)
                } else {
                        PIECE_WHITE_SPACE.to_string()
                }
        }
        /// 表示
        pub fn to_string(game: &Game, pos_nums: &PosNums) -> String {
                let board = game.get_board(pos_nums);
                let ply = game.history.ply;
                let phase = game.history.get_phase(Person::Friend);
                let same_pos_count = game.count_same_position();

                // 局面表示
                format!(
                        "\
[{95} ply. {96} phase. {97} repeats.]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}| a1   ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}| b2   r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}| c3   b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}| d4   g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}| e5   s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}| f6   n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}| g7   l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}| h8   p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}| i9
        +----+----+----+----+----+----+----+----+----+\
",
                        GameView::to_string2(board.piece_at(&Address::new(9, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 1).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 2).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 3).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 4).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 5).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 6).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 7).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 8).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(9, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(8, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(7, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(6, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(5, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(4, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(3, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(2, 9).abs())),
                        GameView::to_string2(board.piece_at(&Address::new(1, 9).abs())),
                        //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
                        board.hand[Piece::Rook1 as usize],
                        board.hand[Piece::Bishop1 as usize],
                        board.hand[Piece::Gold1 as usize],
                        board.hand[Piece::Silver1 as usize],
                        board.hand[Piece::Knight1 as usize],
                        board.hand[Piece::Lance1 as usize],
                        board.hand[Piece::Pawn1 as usize],
                        //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
                        board.hand[Piece::Rook2 as usize],
                        board.hand[Piece::Bishop2 as usize],
                        board.hand[Piece::Gold2 as usize],
                        board.hand[Piece::Silver2 as usize],
                        board.hand[Piece::Knight2 as usize],
                        board.hand[Piece::Lance2 as usize],
                        board.hand[Piece::Pawn2 as usize],
                        ply,
                        phase,
                        same_pos_count
                )
        }
}
