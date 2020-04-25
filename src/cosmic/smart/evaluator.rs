//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::playing::Game;
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::Person;
use crate::cosmic::smart::features::{PieceMeaning, PieceType};
use crate::cosmic::smart::square::{AbsoluteAddress, RelativeAddress};
use crate::cosmic::toy_box::{Location, PieceNum};
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::Beam;

/// 千日手の価値☆（＾～＾）
pub const REPITITION_VALUE: i16 = -300;

pub struct Evaluation {}
impl Evaluation {
    /// 玉のリスク計算だぜ☆（＾～＾）
    pub fn risk_king(game: &mut Game, control_sign: i16) -> f64 {
        let mut risk_value = 0.0f64;
        let friend_index = game.history.get_phase(Person::Friend) as usize;

        // TODO 玉の位置、計算できてないぜ☆（＾～＾）
        let king_location = game.board.location[if friend_index == 0 {
            PieceNum::King1 as usize
        } else {
            PieceNum::King2 as usize
        }];
        let king_adr = match king_location {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => panic!(Beam::trouble(
                "(Err.30) なんで玉が駒台に乗ってるんだぜ☆（＾～＾）！"
            )),
            Location::Busy => {
                AbsoluteAddress::default()
                /*
                panic!(Beam::trouble(
                    "(Err.32) なんで玉が作業中なんだぜ☆（＾～＾）！"
                ))
                // */
            }
        };
        // 北
        // .xx..
        // ..x..
        // .....
        // .....
        // .....
        let path = &mut Vec::<AbsoluteAddress>::new();
        let cur = &mut AbsoluteAddress::default();
        cur.set(&king_adr);
        let rel = &mut RelativeAddress::new(0, -1);
        cur.offset(rel);
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(0, -1));
            if cur.legal_next() {
                path.push(cur.clone());
                cur.offset(rel.set(1, 0));
                if cur.legal_next() {
                    path.push(cur.clone());
                }
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 北西
        // x....
        // xx...
        // .....
        // .....
        // .....
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(1, -1));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(1, 0));
            if cur.legal_next() {
                path.push(cur.clone());
                cur.offset(rel.set(0, -1));
                if cur.legal_next() {
                    path.push(cur.clone());
                }
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 西
        // .....
        // .....
        // xx...
        // x....
        // x....
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(1, 0));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(1, 0));
            if cur.legal_next() {
                path.push(cur.clone());
                cur.offset(rel.set(0, 1));
                if cur.legal_next() {
                    path.push(cur.clone());
                    cur.offset(rel.set(0, 1));
                    if cur.legal_next() {
                        path.push(cur.clone());
                    }
                }
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 南西
        // .....
        // .....
        // .....
        // .x...
        // .x...
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(1, 1));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(0, 1));
            if cur.legal_next() {
                path.push(cur.clone());
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 南
        // .....
        // .....
        // .....
        // ..x..
        // ..xxx
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(0, 1));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(0, 1));
            if cur.legal_next() {
                path.push(cur.clone());
                cur.offset(rel.set(-1, 0));
                if cur.legal_next() {
                    path.push(cur.clone());
                    cur.offset(rel.set(-1, 0));
                    if cur.legal_next() {
                        path.push(cur.clone());
                    }
                }
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 南東
        // .....
        // .....
        // .....
        // ...xx
        // .....
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(-1, 1));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(-1, 0));
            if cur.legal_next() {
                path.push(cur.clone());
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 東
        // ....x
        // ....x
        // ...xx
        // .....
        // .....
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(-1, 0));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(-1, 0));
            if cur.legal_next() {
                path.push(cur.clone());
                cur.offset(rel.set(0, -1));
                if cur.legal_next() {
                    path.push(cur.clone());
                    cur.offset(rel.set(0, -1));
                    if cur.legal_next() {
                        path.push(cur.clone());
                    }
                }
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        // 北東
        // ...x.
        // ...x.
        // .....
        // .....
        // .....
        cur.set(&king_adr);
        path.clear();
        cur.offset(rel.set(-1, -1));
        if cur.legal_next() {
            path.push(cur.clone());
            cur.offset(rel.set(0, -1));
            if cur.legal_next() {
                path.push(cur.clone());
            }
        }
        risk_value += Evaluation::risk(control_sign, path.to_vec(), &king_adr, game);
        risk_value
    }

    fn risk(
        sign: i16,
        adr_vec: Vec<AbsoluteAddress>,
        king_adr: &AbsoluteAddress,
        game: &mut Game,
    ) -> f64 {
        let mut risk = 0f64;
        for adr in adr_vec {
            if adr.legal_board() {
                // どのマスも、玉から 1マス～16マス 離れている☆（＾～＾）玉に近いものを重くみようぜ☆（＾～＾）
                let weight: f64 = (16 - king_adr.manhattan_distance(&adr)) as f64 / 16.0;
                // println!("sign = {} | a = {}", sign, a);
                let amount =
                    sign as f64 * weight * game.board.control[adr.address() as usize] as f64;
                risk += amount;
            } else {
                break;
            }
        }
        risk
    }

    /// 成ったら評価に加点するぜ☆（＾～＾）
    /// 駒得より 評価は下げた方が良さげ☆（＾～＾）
    pub fn from_promotion(cur_depth: usize, source: PieceType, movement: &Movement) -> i16 {
        if movement.promote {
            (match source {
                PieceType::Bishop => 90,
                PieceType::Knight => 20,
                PieceType::Lance => 10,
                PieceType::Pawn => 50,
                PieceType::Rook => 100,
                PieceType::Silver => 40,
                _ => 0,
            }) / (cur_depth as i16)
        } else {
            0
        }
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    ///
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないんで、
    /// 読みの深い所の駒の価値は減らしてやろうぜ☆（＾～＾）？
    ///
    /// * `cur_depth` - １手指すから葉に進めるわけで、必ず 1 は有るから 0除算エラー は心配しなくていいぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// Centi pawn.
    pub fn from_caputured_piece(
        cur_depth: usize,
        captured_piece: Option<(PieceMeaning, PieceNum)>,
        speed_of_light: &SpeedOfLight,
    ) -> i16 {
        if let Some(captured_piece_val) = captured_piece {
            (match captured_piece_val.0.r#type(speed_of_light) {
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                PieceType::King => 0,
                PieceType::Rook => 1000,
                PieceType::Bishop => 900,
                PieceType::Gold => 600,
                PieceType::Silver => 500,
                PieceType::Knight => 300,
                PieceType::Lance => 200,
                PieceType::Pawn => 100,
                PieceType::Dragon => 2000,
                PieceType::Horse => 1900,
                PieceType::PromotedSilver => 500,
                PieceType::PromotedKnight => 300,
                PieceType::PromotedLance => 200,
                PieceType::PromotedPawn => 100,
            }) / (cur_depth as i16)
        } else {
            0
        }
    }
}
