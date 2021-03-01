use crate::bitboard::*;

const INF: i32 = 100000000;

fn min<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l < r { l }
    else { r }
}
fn max<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l > r { l }
    else { r }
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