use crate::bitboard::*;

const EVAL_CORNER1: i32 = 300;
const EVAL_CORNER2: i32 = 50;
const EVAL_CORNER3: i32 = 10;
const INF: i32 = 100000000;

fn min<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l < r { l }
    else { r }
}
fn max<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l > r { l }
    else { r }
}

impl BitBoard {
    fn evaluate(&self, player: i32, player_now: i32) -> i32 {
        // 前半は置ける場所が多いほど良い
        // 後半は自分の石の個数を考慮する

        let moves = self.make_legal_board(player_now).count_ones() as i32;
        let mystones = self.board[player_now as usize].count_ones() as i32;
        let turn = (self.board[1] | self.board[2]).count_ones() as i32;

        let mut evl = 0;
        let corner1: Vec<(i32,i32)> = vec![(0,0),(7,0),(0,7),(7,7)];
        let corner2: Vec<(i32,i32)> = vec![(1,1),(6,1),(1,6),(6,6)];
        let corner3: Vec<(i32,i32)> = vec![(0,1),(1,0),(1,7),(0,6),(7,1),(6,0),(6,7),(7,6)];
        for c in &corner1 {
            if self.get(c.0,c.1) ==     player_now {evl += EVAL_CORNER1}
            if self.get(c.0,c.1) == 3 - player_now {evl -= EVAL_CORNER1}
        }
        for c in &corner2 {
            if self.get(c.0,c.1) ==     player_now {evl -= EVAL_CORNER2}
            if self.get(c.0,c.1) == 3 - player_now {evl += EVAL_CORNER2}
        }
        for c in &corner3 {
            if self.get(c.0,c.1) ==     player_now {evl -= EVAL_CORNER3}
            if self.get(c.0,c.1) == 3 - player_now {evl += EVAL_CORNER3}
        }
        
        if turn <= 40 {
            evl += moves as i32 * 10;
        }
        else {
            evl += moves as i32 * max(0,40 - turn + 10) + mystones * min(10,turn - 40) * 2;
        }

        //if player != player_now {evl = -evl};
        return evl;
    }
}

fn negascout_(board: BitBoard, player: i32, player_now: i32, al: i32, bt: i32, depth: i32) -> i32 {
    if board.is_end(player_now) != 0 || depth == 0 {return board.evaluate(player, player_now)}
    
    let mut b = bt;
    let mut alpha = al;
    let mut beta = bt;
    let mut cnt = 0;
    
    if board.is_pass(player_now) {
        let t = -negascout_(board, player, 3 - player_now, -b, -alpha, depth - 1);
        alpha = max(alpha, t);
    }
    else {
        let legal_board = board.make_legal_board(player_now);
        for i in 0..64 {
            if (legal_board >> i) & 1 != 0 {
                let mut next = board.clone();
                next.reverse(player_now, 1u64 << i);
                let mut t = -negascout_(next, player, 3 - player_now, -b, -alpha, depth - 1);
                if (t > alpha) && (t < beta) && (cnt > 0) {
                    t = -negascout_(next, player, 3 - player_now, -beta, -alpha, depth - 1);
                }
                alpha = max(alpha, t);
                if alpha >= beta {
                    return alpha
                }
                b = alpha + 1;
                cnt += 1;
            }
        }
    }

    return alpha
}

pub fn negascout(player: i32, board: BitBoard, depth: i32) -> (i32,i32) {
    let mut alpha = -INF;
    let mut beta = INF;
    let mut b = INF;
    let mut mv = (0,0);
    let mut cnt = 0;

    let legal_board = board.make_legal_board(player);
    for i in 0..64 {
       if (legal_board >> i) & 1 != 0 {
            let mut next = board.clone();
            next.reverse(player, 1u64 << i);
            let mut t = -negascout_(next, player, 3 - player, -b, -alpha, depth - 1);
            if (t > alpha) && (t < beta) && (cnt > 0) {
                t = -negascout_(next, player, 3 - player, -beta, -alpha, depth - 1);
            }
            if alpha < t {
                alpha = t;
                mv = (i/8,i%8);
            }
            b = alpha + 1;
            cnt += 1;
        }
    }
    
    return mv;

}