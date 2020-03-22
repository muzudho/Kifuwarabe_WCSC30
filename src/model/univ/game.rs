use crate::controller::search_part::sp_number_board_controller::NumberBoard;
use crate::model::univ::gam::board::Board;
use crate::model::univ::gam::history::*;
use crate::model::univ::gam::misc::info::SPInfo;
use crate::model::univ::gam::misc::movement::Movement;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::phase::PHASE_LN;
use crate::model::univ::gam::misc::phase::*;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece::MG_MAX;
use crate::model::univ::gam::misc::piece::PIECE_LN;
use crate::model::univ::gam::misc::piece_struct::PieceStruct;
use crate::model::univ::gam::misc::square::BOARD_MEMORY_AREA;
use crate::model::univ::gam::misc::square::SQUARE_NONE;
use crate::model::univ::gam::misc::square::*;
use crate::model::univ::gam::position::Position;
use crate::model::univ::speed_of_light::MLSpeedOfLightVo;
use rand::Rng;

/// 局面
pub enum PosNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/// 現対局ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct GameHashSeed {
    // 盤上の駒
    pub km: [[u64; PIECE_LN]; BOARD_MEMORY_AREA],
    // 持ち駒
    pub mg: [[u64; MG_MAX]; PIECE_LN],
    // 先後
    pub phase: [u64; PHASE_LN],
}

pub struct Game {
    /// 棋譜
    pub history: History,
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
    /// 初期局面
    pub starting_board: Board,
    /// 現対局ハッシュ種☆（＾～＾）
    pub hash_seed: GameHashSeed,
    /// 現局面
    pub position: Position,
    /// 情報表示担当
    pub info: SPInfo,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            history: History::default(),
            starting_position_hash: 0,
            starting_board: Board::default(),
            hash_seed: GameHashSeed {
                // 盤上の駒
                km: [[0; PIECE_LN]; BOARD_MEMORY_AREA],
                // 持ち駒
                mg: [[0; MG_MAX]; PIECE_LN],
                // 先後
                phase: [0; PHASE_LN],
            },
            position: Position::default(),
            info: SPInfo::default(),
        }
    }
}
impl Game {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_ms in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_km in 0..PIECE_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.hash_seed.km[i_ms][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..PIECE_LN {
            for i_mg in 0..MG_MAX {
                self.hash_seed.mg[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LN {
            self.hash_seed.phase[i_phase] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    /// 棋譜の作成
    pub fn set_current_movement(&mut self, movement: &Movement) {
        self.history.movements[self.history.ply as usize] = movement.clone()
    }
    pub fn build_current_movement(&mut self) {
        self.history.movements[self.history.ply as usize] =
            Movement::new(&self.position.current_movement_builder)
    }
    pub fn get_move(&self) -> &Movement {
        &self.history.movements[self.history.ply as usize]
    }
    /// 棋譜☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.history.ply {
            let movement = &self.history.movements[ply as usize];
            s.push_str(&format!("[{}] {}", ply, movement));
        }
        s
    }

    pub fn get_current_position_hash(&mut self) -> u64 {
        self.history.position_hashs[self.history.ply as usize]
    }
    pub fn set_current_position_hash(&mut self, hash: u64) {
        self.history.position_hashs[self.history.ply as usize] = hash;
    }
    pub fn set_cap(&mut self, ply1: usize, pc: Option<Piece>) {
        self.history.captured_pieces[ply1] = pc
    }

    pub fn get_board(&self, num: &PosNums) -> &Board {
        match *num {
            PosNums::Current => &self.position.current_board,
            PosNums::Start => &self.starting_board,
        }
    }

    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear_all_positions(&mut self) {
        self.starting_board.clear();
        self.position.current_board.clear();
        self.history.ply = 0;
    }

    /// 開始局面を、現局面にコピーします
    pub fn copy_starting_position_to_current_position(&mut self) {
        // 盤上の駒。
        for i_ms in 0..BOARD_MEMORY_AREA {
            let i_sq = Square::from_usquare(i_ms);
            // TODO 取得→設定　するとエラーになってしまうので、今んとこ 作成→設定　するぜ☆（＾～＾）
            let piece = self.starting_board.get_piece_by_square(&i_sq);
            self.position
                .current_board
                .set_piece_by_square(&i_sq, piece);
        }

        // 持ち駒
        self.position.current_board.hand[..PIECE_LN]
            .clone_from_slice(&self.starting_board.hand[..PIECE_LN]);
        /*
        for i_mg in 0..PIECE_LN {
            self.get_search_part_mut().get_current_position_mut().mg[i_mg] =
                self.get_starting_position().mg[i_mg];
        }
        */
    }

    /// 初期局面の盤上に駒の位置を設定するもの
    pub fn set_piece_to_starting_position(&mut self, suji: i8, dan: i8, pc: Option<Piece>) {
        self.starting_board
            .set_piece_by_square(&Square::from_file_rank(suji, dan), pc);
    }

    pub fn set_starting_position_hand_piece(&mut self, km: Piece, maisu: i8) {
        self.starting_board.hand[km as usize] = maisu;
    }

    pub fn get_person_by_piece_struct(&self, piece_struct: &PieceStruct) -> Person {
        if &piece_struct.phase() == &self.history.get_phase(&Person::Friend) {
            Person::Friend
        } else {
            Person::Opponent
        }
    }

    /// 局面ハッシュ。
    pub fn get_all_position_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.starting_position_hash));

        for ply in 0..self.history.ply {
            let hash = &self.history.position_hashs[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /// 自陣
    #[allow(dead_code)]
    pub fn get_ji_jin(&self) -> Vec<Square> {
        if let Phase::First = self.history.get_phase(&Person::Friend) {
            crate::model::univ::gam::misc::region::SenteJin::to_elm()
        } else {
            crate::model::univ::gam::misc::region::GoteJin::to_elm()
        }
    }

    /// 相手陣
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::First = self.history.get_phase(&Person::Friend) {
            crate::model::univ::gam::misc::region::GoteJin::to_elm()
        } else {
            crate::model::univ::gam::misc::region::SenteJin::to_elm()
        }
    }

    /// 初期局面ハッシュを作り直す
    pub fn create_starting_position_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self.starting_board.create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.hash_seed.phase[PHASE_SECOND];

        hash
    }

    /// 局面ハッシュを作り直す
    pub fn create_ky1_hash(&self, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash = self
            .position
            .current_board
            .create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use crate::model::univ::gam::misc::phase::Phase::*;
        match self.history.get_phase(&Person::Friend) {
            First => hash ^= self.hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.hash_seed.phase[PHASE_SECOND],
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_ky(&self) -> i8 {
        if self.history.ply < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.ply - 1;
        let new_ply = self.history.ply;
        // g_writeln( &format!( "Ｃount_same_ky last_ply={} new_ply={}", last_ply ,new_ply ) );
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            // g_writeln( &format!( "i_ply={} t={}", i_ply, t ) );
            if self.history.position_hashs[t as usize]
                == self.history.position_hashs[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.starting_position_hash == self.history.position_hashs[last_ply as usize] {
            count += 1;
        }

        count
    }

    /// らいおんの位置
    pub fn get_king_sq(&self, person: &Person) -> &Square {
        &self
            .position
            .current_board
            .get_sq_r(phase_to_num(&self.history.get_phase(person)))
    }

    /// 入れた指し手の通り指すぜ☆（＾～＾）
    ///
    /// # Returns
    ///
    /// Captured piece.
    pub fn do_move(
        &mut self,
        movement: &Movement,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> Option<Piece> {
        // もう入っているかも知れないが、棋譜に入れる☆
        let ply = self.history.ply;
        self.set_current_movement(movement);
        let phase = self.history.get_phase(&Person::Friend);

        // 取った駒
        let cap: Option<Piece>;
        {
            // 動かす駒
            let piece144_o: Option<Piece> = if movement.source.to_usquare() == SQUARE_DROP {
                // 打なら
                // 自分の持ち駒を減らす
                if let Some(drp) = movement.drop {
                    let piece734 = Piece::from_phase_and_piece_type(&phase, drp);
                    self.position
                        .current_board
                        .add_hand(&piece734, -1, speed_of_light);
                    Some(piece734)
                } else {
                    panic!("打なのに駒を指定してないぜ☆（＾～＾）");
                }
            } else {
                // 打でなければ、元の升に駒はあるので、それを消す。
                let piece152 = if movement.promote {
                    // 成りなら
                    if let Some(pc) = self
                        .position
                        .current_board
                        .get_piece_by_square(&movement.source)
                    {
                        Some(speed_of_light.get_piece_struct(&pc).promote())
                    } else {
                        panic!("成ったのに、元の升に駒がなかった☆（＾～＾）");
                    }
                } else {
                    self.position
                        .current_board
                        .get_piece_by_square(&movement.source)
                        .clone()
                };

                self.position
                    .current_board
                    .set_piece_by_square(&movement.source, None);

                piece152
            };

            // 移動先升に駒があるかどうか
            cap = if let Some(_) = self
                .position
                .current_board
                .get_piece_by_square(&movement.destination)
            {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                let cap_o764 = {
                    self.position
                        .current_board
                        .get_piece_by_square(&movement.destination)
                };

                if let Some(cap764) = cap_o764 {
                    let cap_o773 = { speed_of_light.get_piece_struct(&cap764).capture() };
                    if let Some(cap773) = cap_o773 {
                        self.position
                            .current_board
                            .add_hand(&cap773, 1, speed_of_light);
                    }
                };
                cap_o764
            } else {
                None
            };

            // 移動先升に駒を置く
            self.position
                .current_board
                .set_piece_by_square(&movement.destination, piece144_o);
        }
        self.set_cap(ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_ky1_hash(speed_of_light);
        self.set_current_position_hash(ky_hash);

        self.history.ply += 1;
        cap
    }

    pub fn undo_move(&mut self, speed_of_light: &MLSpeedOfLightVo) -> bool {
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            // let phase = self.sp_earth_dto.get_phase(&Person::Friend);
            let movement = &self.get_move().clone();
            {
                let phase = self.history.get_phase(&Person::Friend);
                let cap_o: Option<Piece> = self.history.captured_pieces[self.history.ply as usize];
                // 移動先の駒
                let piece186_o: Option<Piece> = if movement.source.to_usquare() == SQUARE_DROP {
                    // 打なら
                    if let Some(drp) = movement.drop {
                        let piece679 = Piece::from_phase_and_piece_type(&phase, drp);
                        // 自分の持ち駒を増やす
                        //let mg = km_to_mg(km);
                        //self.add_hand(mg,1);
                        self.position
                            .current_board
                            .add_hand(&piece679, 1, speed_of_light);
                        Some(piece679)
                    } else {
                        panic!("打なのに駒を指定していないぜ☆（＾～＾）！")
                    }
                } else {
                    // 打でなければ
                    if movement.promote {
                        // 成ったなら、成る前へ
                        if let Some(piece411) = self
                            .position
                            .current_board
                            .get_piece_by_square(&movement.destination)
                        {
                            Some(speed_of_light.get_piece_struct(&piece411).demote())
                        } else {
                            panic!("成ったのに移動先に駒が無いぜ☆（＾～＾）！")
                        }
                    } else {
                        self.position
                            .current_board
                            .get_piece_by_square(&movement.destination)
                            .clone()
                    }
                };

                if let Some(cap) = cap_o {
                    if let Some(captured) = speed_of_light.get_piece_struct(&cap).capture() {
                        // 移動先の駒を、取った駒（あるいは空）に戻す
                        self.position
                            .current_board
                            .set_piece_by_square(&movement.destination, cap_o);
                        // 自分の持ち駒を減らす
                        self.position
                            .current_board
                            .add_hand(&captured, -1, speed_of_light);
                    } else {
                        panic!("取った駒は、駒台に置けない駒だぜ☆（＾～＾）！")
                    }
                }
                // 移動元升に、動かした駒を置く
                self.position
                    .current_board
                    .set_piece_by_square(&movement.source, piece186_o);
            }
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }

    pub fn get_number_board_by_phase(&self, phase: &Phase) -> &NumberBoard {
        &self.position.control_count_by_phase[phase_to_num(&phase)]
    }
    pub fn get_number_board_by_piece(
        &self,
        pc: &Piece,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> &NumberBoard {
        &self.position.control_count_by_piece
            [speed_of_light.get_piece_struct(pc).serial_piece_number()]
    }
    /// 表示
    pub fn print_number_board(&self, nb: &NumberBoard) -> String {
        // 数盤表示
        format!(
            "    +----+----+----+----+----+----+----+----+----+
i9  |{0:4}|{1:4}|{2:4}|{3:4}|{4:4}|{5:4}|{6:4}|{7:4}|{8:4}|
    +----+----+----+----+----+----+----+----+----+
h8  |{9:4}|{10:4}|{11:4}|{12:4}|{13:4}|{14:4}|{15:4}|{16:4}|{17:4}|
    +----+----+----+----+----+----+----+----+----+
g7  |{18:4}|{19:4}|{20:4}|{21:4}|{22:4}|{23:4}|{24:4}|{25:4}|{26:4}|
    +----+----+----+----+----+----+----+----+----+
f6  |{27:4}|{28:4}|{29:4}|{30:4}|{31:4}|{32:4}|{33:4}|{34:4}|{35:4}|
    +----+----+----+----+----+----+----+----+----+
e5  |{36:4}|{37:4}|{38:4}|{39:4}|{40:4}|{41:4}|{42:4}|{43:4}|{44:4}|
    +----+----+----+----+----+----+----+----+----+
d4  |{45:4}|{46:4}|{47:4}|{48:4}|{49:4}|{50:4}|{51:4}|{52:4}|{53:4}|
    +----+----+----+----+----+----+----+----+----+
c3  |{54:4}|{55:4}|{56:4}|{57:4}|{58:4}|{59:4}|{60:4}|{61:4}|{62:4}|
    +----+----+----+----+----+----+----+----+----+
b2  |{63:4}|{64:4}|{65:4}|{66:4}|{67:4}|{68:4}|{69:4}|{70:4}|{71:4}|
    +----+----+----+----+----+----+----+----+----+
a1  |{72:4}|{73:4}|{74:4}|{75:4}|{76:4}|{77:4}|{78:4}|{79:4}|{80:4}|
    +----+----+----+----+----+----+----+----+----+
       1    2    3    4    5    6    7    8    9\
",
            nb.get_number_by_square(&Square::from_usquare(19)),
            nb.get_number_by_square(&Square::from_usquare(29)),
            nb.get_number_by_square(&Square::from_usquare(39)),
            nb.get_number_by_square(&Square::from_usquare(49)),
            nb.get_number_by_square(&Square::from_usquare(59)),
            nb.get_number_by_square(&Square::from_usquare(69)),
            nb.get_number_by_square(&Square::from_usquare(79)),
            nb.get_number_by_square(&Square::from_usquare(89)),
            nb.get_number_by_square(&Square::from_usquare(99)),
            nb.get_number_by_square(&Square::from_usquare(18)),
            nb.get_number_by_square(&Square::from_usquare(28)),
            nb.get_number_by_square(&Square::from_usquare(38)),
            nb.get_number_by_square(&Square::from_usquare(48)),
            nb.get_number_by_square(&Square::from_usquare(58)),
            nb.get_number_by_square(&Square::from_usquare(68)),
            nb.get_number_by_square(&Square::from_usquare(78)),
            nb.get_number_by_square(&Square::from_usquare(88)),
            nb.get_number_by_square(&Square::from_usquare(98)),
            nb.get_number_by_square(&Square::from_usquare(17)),
            nb.get_number_by_square(&Square::from_usquare(27)),
            nb.get_number_by_square(&Square::from_usquare(37)),
            nb.get_number_by_square(&Square::from_usquare(47)),
            nb.get_number_by_square(&Square::from_usquare(57)),
            nb.get_number_by_square(&Square::from_usquare(67)),
            nb.get_number_by_square(&Square::from_usquare(77)),
            nb.get_number_by_square(&Square::from_usquare(87)),
            nb.get_number_by_square(&Square::from_usquare(97)),
            nb.get_number_by_square(&Square::from_usquare(16)),
            nb.get_number_by_square(&Square::from_usquare(26)),
            nb.get_number_by_square(&Square::from_usquare(36)),
            nb.get_number_by_square(&Square::from_usquare(46)),
            nb.get_number_by_square(&Square::from_usquare(56)),
            nb.get_number_by_square(&Square::from_usquare(66)),
            nb.get_number_by_square(&Square::from_usquare(76)),
            nb.get_number_by_square(&Square::from_usquare(86)),
            nb.get_number_by_square(&Square::from_usquare(96)),
            nb.get_number_by_square(&Square::from_usquare(15)),
            nb.get_number_by_square(&Square::from_usquare(25)),
            nb.get_number_by_square(&Square::from_usquare(35)),
            nb.get_number_by_square(&Square::from_usquare(45)),
            nb.get_number_by_square(&Square::from_usquare(55)),
            nb.get_number_by_square(&Square::from_usquare(65)),
            nb.get_number_by_square(&Square::from_usquare(75)),
            nb.get_number_by_square(&Square::from_usquare(85)),
            nb.get_number_by_square(&Square::from_usquare(95)),
            nb.get_number_by_square(&Square::from_usquare(14)),
            nb.get_number_by_square(&Square::from_usquare(24)),
            nb.get_number_by_square(&Square::from_usquare(34)),
            nb.get_number_by_square(&Square::from_usquare(44)),
            nb.get_number_by_square(&Square::from_usquare(54)),
            nb.get_number_by_square(&Square::from_usquare(64)),
            nb.get_number_by_square(&Square::from_usquare(74)),
            nb.get_number_by_square(&Square::from_usquare(84)),
            nb.get_number_by_square(&Square::from_usquare(94)),
            nb.get_number_by_square(&Square::from_usquare(13)),
            nb.get_number_by_square(&Square::from_usquare(23)),
            nb.get_number_by_square(&Square::from_usquare(33)),
            nb.get_number_by_square(&Square::from_usquare(43)),
            nb.get_number_by_square(&Square::from_usquare(53)),
            nb.get_number_by_square(&Square::from_usquare(63)),
            nb.get_number_by_square(&Square::from_usquare(73)),
            nb.get_number_by_square(&Square::from_usquare(83)),
            nb.get_number_by_square(&Square::from_usquare(93)),
            nb.get_number_by_square(&Square::from_usquare(12)),
            nb.get_number_by_square(&Square::from_usquare(22)),
            nb.get_number_by_square(&Square::from_usquare(32)),
            nb.get_number_by_square(&Square::from_usquare(42)),
            nb.get_number_by_square(&Square::from_usquare(52)),
            nb.get_number_by_square(&Square::from_usquare(62)),
            nb.get_number_by_square(&Square::from_usquare(72)),
            nb.get_number_by_square(&Square::from_usquare(82)),
            nb.get_number_by_square(&Square::from_usquare(92)),
            nb.get_number_by_square(&Square::from_usquare(11)),
            nb.get_number_by_square(&Square::from_usquare(21)),
            nb.get_number_by_square(&Square::from_usquare(31)),
            nb.get_number_by_square(&Square::from_usquare(41)),
            nb.get_number_by_square(&Square::from_usquare(51)),
            nb.get_number_by_square(&Square::from_usquare(61)),
            nb.get_number_by_square(&Square::from_usquare(71)),
            nb.get_number_by_square(&Square::from_usquare(81)),
            nb.get_number_by_square(&Square::from_usquare(91)),
        )
    }
}
