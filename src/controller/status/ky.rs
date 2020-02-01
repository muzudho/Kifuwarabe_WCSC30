//!
//! 局面
//!
//! 後手（上手）から見た盤にすると、
//! 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
//!
//! プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
//! 盤を想像すること☆（＾～＾）！
//!

use super::super::super::controller::communication::usi::*;
use super::super::super::controller::status::uchu::*;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece::*;
use super::super::super::model::master::piece_struct::PieceStruct;
use super::super::super::model::master::piece_type::*;
use super::super::super::model::master::square::*;

/// 局面
/// でかいのでコピーもクローンも不可☆（＾～＾）！
pub struct Kyokumen {
    /**
     * 10の位を筋、1の位を段とする。
     * 0筋、0段は未使用
     */
    board: [PieceStruct; BAN_SIZE],
    /**
     * 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
     * 増減させたいので、u8 ではなく i8。
     */
    pub mg: [i8; KM_LN],
    /**
     * らいおんの位置
     * [先後]
     */
    sq_r: [Square; SN_LN],
}
impl Kyokumen {
    pub fn new() -> Kyokumen {
        use super::super::super::model::master::piece::Piece::Kara;
        Kyokumen {
            // 盤上
            board: [
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
                PieceStruct::from_piece(&Kara),
            ],
            // 持ち駒数
            mg: [
                // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
                0, 0,
            ],
            sq_r: [
                Square::from_umasu(0),
                Square::from_umasu(0),
                Square::from_umasu(0),
            ],
        }
    }
    pub fn clear(&mut self) {
        use super::super::super::model::master::piece::Piece::Kara;
        self.board = [
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
            PieceStruct::from_piece(&Kara),
        ];
        self.mg = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
            0, 0,
        ];
    }

    /// らいおんの位置
    pub fn get_sq_r(&self, phase_number: usize) -> &Square {
        &self.sq_r[phase_number]
    }

    /**
     * 歩が置いてあるか確認
     */
    pub fn exists_fu_by_sn_suji(&self, sn: &Phase, suji: i8) -> bool {
        for dan in DAN_1..DAN_10 {
            let sq = Square::from_file_rank(suji, dan);
            let km = self.get_piece_struct_by_sq(&sq).piece();
            let piece = PieceStruct::from_piece(&km);
            let (sn_km, kms) = piece.phase_piece_type();
            if match_sn(&sn_km, sn) && match_kms(&kms, &PieceType::H) {
                return true;
            }
        }
        false
    }
    /**
     * 升で指定して駒を取る
     */
    pub fn get_piece_struct_by_sq(&self, sq: &Square) -> &PieceStruct {
        &self.board[sq.to_umasu()]
    }
    /**
     * 升で指定して駒を置く
     */
    pub fn set_km_by_sq(&mut self, sq: &Square, km: &Piece) {
        self.board[sq.to_umasu()] = PieceStruct::from_piece(km);
        use super::super::super::model::master::phase::Phase::*;
        match *km {
            Piece::King1 => self.sq_r[Sen as usize] = sq.clone(),
            Piece::King2 => self.sq_r[Go as usize] = sq.clone(),
            _ => {}
        }
    }
    /**
     * 持ち駒の枚数を加算
     */
    pub fn add_mg(&mut self, mg: Piece, maisu: i8) {
        self.mg[PieceStruct::from_piece(&mg).serial_piece_number()] += maisu;
    }
    pub fn get_mg(&self, mg: &Piece) -> i8 {
        self.mg[PieceStruct::from_piece(mg).serial_piece_number()]
    }

    /**
     * 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
     * 手目のカウントが増えたりはしないぜ☆（＾～＾）
     *
     * return : 取った駒
     */
    pub fn do_sasite(&mut self, sn: &Phase, ss: &Sasite) -> Piece {
        // 取った駒
        let cap;

        {
            // 動かす駒
            let km = if ss.src.to_umasu() == SS_SRC_DA {
                // 打なら
                // 自分の持ち駒を減らす
                let ps = PieceStruct::from_phase_piece_type(&sn, &ss.drop);
                self.add_mg(ps.piece().clone(), -1);
                ps.piece().clone()
            } else {
                // 打で無ければ、元の升の駒を消す。
                let km1 = if ss.pro {
                    // 成りなら
                    self.get_piece_struct_by_sq(&ss.src).promote().clone()
                } else {
                    self.get_piece_struct_by_sq(&ss.src).piece().clone()
                };

                self.set_km_by_sq(&ss.src, &Piece::Kara);

                km1
            };

            // 移動先升に駒があるかどうか
            if let Piece::Kara = self.get_piece_struct_by_sq(&ss.dst).piece() {
                cap = Piece::Kara;
            } else {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                cap = self.get_piece_struct_by_sq(&ss.dst).piece().clone();
                self.add_mg(PieceStruct::from_piece(&cap).capture().clone(), 1);
            }

            // 移動先升に駒を置く
            self.set_km_by_sq(&ss.dst, &km);
        }

        cap
    }

    /**
     * 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
     * 手目のカウントが増えたりはしないぜ☆（＾～＾）
     */
    pub fn undo_sasite(&mut self, sn: &Phase, ss: &Sasite, cap: &Piece) {
        // 移動先の駒
        let km = if ss.src.to_umasu() == SS_SRC_DA {
            // 打なら
            let ps = PieceStruct::from_phase_piece_type(sn, &ss.drop);
            // 自分の持ち駒を増やす
            //let mg = km_to_mg(km);
            //self.add_mg(mg,1);
            self.add_mg(ps.piece().clone(), 1);
            ps.piece().clone()
        } else {
            // 打で無ければ
            if ss.pro {
                // 成ったなら、成る前へ
                self.get_piece_struct_by_sq(&ss.dst).demote().clone()
            } else {
                self.get_piece_struct_by_sq(&ss.dst).piece().clone()
            }
        };

        // 移動先の駒を、取った駒（あるいは空）に戻す
        self.set_km_by_sq(&ss.dst, cap);
        match *cap {
            Piece::Kara => {}
            _ => {
                // 自分の持ち駒を減らす
                self.add_mg(PieceStruct::from_piece(cap).capture().clone(), -1);
            }
        }

        // 移動元升に、動かした駒を置く
        self.set_km_by_sq(&ss.src, &km);
    }

    /**
     * 指定の升に駒があれば真
     */
    pub fn exists_km(&self, sq: &Square) -> bool {
        !self
            .get_piece_struct_by_sq(&sq)
            .equals_piece(&PieceStruct::from_piece(&Piece::Kara))
    }

    /**
     * 指定の升に指定の駒があれば真
     */
    pub fn has_sq_km(&self, sq: &Square, km: &Piece) -> bool {
        self.get_piece_struct_by_sq(&sq)
            .equals_piece(&PieceStruct::from_piece(km))
    }

    /**
     * 指定の升にある駒の先後、または空升
     */
    pub fn get_sn_by_sq(&self, sq: &Square) -> &Phase {
        &self.get_piece_struct_by_sq(sq).phase()
    }

    /**
     * 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
     */
    pub fn is_natta(&self, sq_src: &Square, sq_dst: &Square) -> bool {
        let km_src = &self.get_piece_struct_by_sq(&sq_src).piece();

        let ps_src = PieceStruct::from_piece(&km_src);
        let km_dst = &self.get_piece_struct_by_sq(&sq_dst).piece();

        let ps_dst = PieceStruct::from_piece(&km_dst);
        // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
        let pro_dst = ps_dst.is_promoted();
        let pro_src = ps_src.is_promoted();

        // 成り
        pro_dst && !pro_src
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_hash(&self, uchu: &Uchu) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            let i_sq = Square::from_umasu(i_ms as umasu);
            let km = self.get_piece_struct_by_sq(&i_sq).piece();
            let num_km = PieceStruct::from_piece(&km).serial_piece_number();
            hash ^= uchu.ky_hash_seed.km[i_ms][num_km];
        }

        // 持ち駒ハッシュ
        for i_km in 0..KM_ARRAY_LN {
            let km = KM_ARRAY[i_km].clone();
            let num_km = PieceStruct::from_piece(&km).serial_piece_number();

            let maisu = self.get_mg(&km);
            debug_assert!(
                -1 < maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}",
                km,
                maisu,
                MG_MAX
            );

            hash ^= uchu.ky_hash_seed.mg[num_km][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}
