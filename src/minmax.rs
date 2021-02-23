use crate::board::*;
use std::time::Instant;
use std::collections::HashMap;

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

impl Board {
    fn evaluate(&self, player: i32) -> i32 {
        // 前半は置ける場所が多いほど良い
        // 後半は自分の石の個数を考慮する

        let moves = self.get_all_move(player);
        let mut mystones = 0;
        let turn = (self._b1 | self._b2).count_ones() as i32;

        for y in 0..8 {
            for x in 0..8 {
                if self.get(y,x) == player {
                    mystones += 1;
                }
            }
        }

        let mut evl = 0;
        let corner1: Vec<(i32,i32)> = vec![(0,0),(7,0),(0,7),(7,7)];
        let corner2: Vec<(i32,i32)> = vec![(1,1),(6,1),(1,6),(6,6)];
        let corner3: Vec<(i32,i32)> = vec![(0,1),(1,0),(1,7),(0,6),(7,1),(6,0),(6,7),(7,6)];
        for c in &corner1 {
            if self.get(c.0,c.1) ==     player {evl += EVAL_CORNER1}
            if self.get(c.0,c.1) == 3 - player {evl -= EVAL_CORNER1}
        }
        for c in &corner2 {
            if self.get(c.0,c.1) ==     player {evl -= EVAL_CORNER2}
            if self.get(c.0,c.1) == 3 - player {evl += EVAL_CORNER2}
        }
        for c in &corner3 {
            if self.get(c.0,c.1) ==     player {evl -= EVAL_CORNER3}
            if self.get(c.0,c.1) == 3 - player {evl += EVAL_CORNER3}
        }
        
        if turn <= 40 {
            evl += moves.len() as i32 * 10;
        }
        else {
            evl += moves.len() as i32 * max(0,40 - turn + 10) + mystones * min(10,turn - 40) * 2;
        }
        return evl
    }
}

fn alphabeta_(board: Board, player: i32, player_now: i32, depth: i32, al: i32, bt: i32) -> i32 {
    if board.is_end(player) != 0 || depth == 0 {return board.evaluate(player)}
    let mut alpha = al;
    let mut beta = bt;
    if player == player_now {
        let moves = board.get_all_move(player_now);
        if moves.len() == 0 {
            alpha = max(alpha, alphabeta_(board, player, 3-player_now, depth-1, alpha, beta));
        }
        else {
            for m in moves {
                let mut next = board.clone();
                next.put(player_now, m.0, m.1);
                alpha = max(alpha, alphabeta_(next, player, 3-player_now, depth-1, alpha, beta));
                if alpha >= beta { break }
            }
        }
        return alpha;
    }
    else {
        let moves = board.get_all_move(player_now);
        if moves.len() == 0 {
            beta = min(beta, alphabeta_(board, player, 3-player_now, depth-1, alpha, beta));
        }
        else {
            for m in moves {
                let mut next = board.clone();
                next.put(player_now, m.0, m.1);
                beta = min(beta, alphabeta_(next, player, 3-player_now, depth-1, alpha, beta));
                if alpha >= beta { break }
            }
        }
        return beta;
    }
    
}

pub fn alphabeta(player: i32, board: Board, depth: i32) -> (i32, i32) {
    if board.is_end(player) != 0 {return (-1,-1)}
    let mut alpha = -INF;
    let moves = board.get_all_move(player);
    let mut mv = (moves[0].0,moves[0].1);
    for m in moves {
        let mut next = board.clone();
        next.put(player, m.0, m.1);
        let tmp = alphabeta_(next, player, 3 - player, depth - 1, alpha, INF);
        if alpha < tmp {
            alpha = tmp;
            mv = m;
        }
    }
    return mv;
}


fn alphabeta2_(hm : &mut HashMap::<Board, Vec<(i32,i32)>>, board: Board, player: i32, player_now: i32, depth: i32, al: i32, bt: i32) -> i32 {
    let moves = if hm.contains_key(&board) { hm[&board].clone() }
    else {hm.insert(board, board.get_all_move(player_now)); hm[&board].clone() };

    if depth == 0 || (moves.len() == 0 && board.get_all_move(3-player_now).len() == 0) { return board.evaluate(player) }

    let mut alpha = al;
    let mut beta = bt;

    if player == player_now {
        if moves.len() == 0 {
            alpha = max(alpha, alphabeta2_(hm, board, player, 3-player_now, depth-1, alpha, beta));
        }
        else {
            for m in moves {
                let mut next = board.clone();
                next.put(player_now, m.0, m.1);
                alpha = max(alpha, alphabeta2_(hm, next, player, 3-player_now, depth-1, alpha, beta));
                if alpha >= beta { break }
            }
        }
        return alpha;
    }
    else {
        if moves.len() == 0 {
            beta = min(beta, alphabeta2_(hm, board, player, 3-player_now, depth-1, alpha, beta));
        }
        else {
            for m in moves {
                let mut next = board.clone();
                next.put(player_now, m.0, m.1);
                beta = min(beta, alphabeta2_(hm, next, player, 3-player_now, depth-1, alpha, beta));
                if alpha >= beta { break }
            }
        }
        return beta;
    }
    
}

fn alphabeta2(hm : &mut HashMap::<Board, Vec<(i32,i32)>>, player: i32, board: Board, depth: i32) -> (i32, i32) {
    if board.is_end(player) != 0 {return (-1,-1)}
    let mut alpha = -INF;
    
    let moves = if hm.contains_key(&board) { hm[&board].clone() }
    else {hm.insert(board, board.get_all_move(player)); hm[&board].clone() };

    let mut mv = (moves[0].0,moves[0].1);
    for m in moves {
        let mut next = board.clone();
        next.put(player, m.0, m.1);
        let tmp = alphabeta2_(hm, next, player, 3 - player, depth - 1, alpha, INF);
        if alpha < tmp {
            alpha = tmp;
            mv = m;
        }
    }
    return mv;
}

pub fn alphabeta_time(player: i32, board: Board, time: i32) -> (i32, i32) {
    let mut hm = HashMap::<Board, Vec<(i32,i32)>>::new();

    let start = Instant::now();
    let mut itr = 3;
    let mut mv = (-1,-1);
    loop {
        if start.elapsed().as_millis() > (time / 10 * 5) as u128{ break; }
        mv = alphabeta2(&mut hm, player, board, itr);
        itr += 1;
        if itr > 64 - ((board._b1 | board._b2).count_ones() as i32) { break; }
    }
    println!("{}", itr);
    return mv
}
