use crate::bitboard::*;
use std::collections::HashMap;

const INF: i32 = 100000000;

fn min<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l < r { l }
    else { r }
}
fn max<T: std::cmp::PartialOrd>(l: T, r: T) -> T {
    if l > r { l }
    else { r }
}


fn alphabeta_(board: BitBoard, player: i32, player_now: i32, depth: i32, alpha: i32, beta: i32) -> i32 {
    if board.is_end(player_now) != 0 || depth == 0 {return board.evaluate(player, player_now)}
    let mut a = alpha;
    let mut score;
    let mut score_max = -INF;

    let legal_board = board.make_legal_board(player_now);
    if legal_board == 0 {
        score = -alphabeta_(board, player, 3-player_now, depth-1, -beta,-a);
        score_max = max(score, score_max);
    }
    else {
        for i in 0..64 {
            if (legal_board >> i) & 1 != 0 {
                let mut next = board.clone();
                next.reverse(player_now, 1u64 << i);
                score = -alphabeta_(next, player, 3-player_now, depth-1, -beta, -a);
                if score >= beta { return score }
                if score > score_max {
                    a = max(a,score);
                    score_max = score;
                }
            }
        }
    }
    return score_max;
}

pub fn alphabeta(player: i32, board: BitBoard, depth: i32) -> (i32, i32) {
    if board.is_end(player) != 0 {return (-1,-1)}
    let mut mv = (-1,-1);

    let mut a = -INF;
    let beta = INF;
    let mut score;
    let mut score_max = -INF;

    let legal_board = board.make_legal_board(player);
    for i in 0..64 {
        if (legal_board >> i) & 1 != 0 {
            let mut next = board.clone();
            next.reverse(player, 1u64 << i);
            score = -alphabeta_(next, player, 3-player, depth-1, -beta, -a);
            if score > score_max {
                a = max(a,score);
                score_max = score;
                mv = (i / 8, i % 8);
            }
        }
    }
    println!("{}",score_max);
    return mv;
}
