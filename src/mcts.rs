use rand::Rng;
use std::time::Instant;

use crate::bitboard::*;

impl BitBoard {
    fn playout(&self, p: i32) -> i32 {
        let mut tmp_board = self.clone();
        let mut player = p;
        loop {
            if tmp_board.is_end(player) != 0 {break}
            let mut nexts: Vec<BitBoard> = Vec::new();
            let legal_board = tmp_board.make_legal_board(player);
            if legal_board != 0 {
                for i in 0..64 {
                    if (legal_board >> i) & 1 != 0 {
                        let mut next = tmp_board.clone();
                        next.reverse(player, 1u64 << i);
                        nexts.push(next)
                    }
                }
                tmp_board = nexts[rand::thread_rng().gen_range(0,nexts.len())];
            }
            player = match player {
                1 => 2,
                2 => 1,
                _ => 0
            }
        }
        tmp_board.is_end(player)
    }
}

struct MCTNode {
    win : i32, // 勝った回数
    n : i32, // 試行回数
    player : i32, // 手番のプレイヤー
    board : BitBoard, // 盤面
    mov : (i32,i32), // その盤面に至る手
    childs : Vec<MCTNode> // 子
}

impl MCTNode {
    fn new() -> Self {
        MCTNode {win: 0, n: 0, player: 1, board: BitBoard::new(), mov: (-1,-1), childs: Vec::new()}
    }
    fn make_child(&mut self) {
        if self.board.is_end(self.player) != 0 { return }

        let legal_board = self.board.make_legal_board(self.player);
        if legal_board != 0 {
            for i in 0..64 {
                if (legal_board >> i) & 1 != 0 {
                    let mut next = self.board.clone();
                    next.reverse(self.player, 1u64 << i);
                    self.childs.push(MCTNode {win: 0, n: 0,  player: 3-self.player, board: next, mov: (i/8, i%8), childs: Vec::new()})
                }
            }
        }
        else {
            self.childs.push(MCTNode {win: 0, n: 0,  player: 3-self.player, board: self.board, mov: (-1,-1), childs: Vec::new()})
        }
    }

    fn select(&mut self, N: i32) -> i32 {
        if self.childs.len() == 0 {
            return -1
        }
        let mut rnd: Vec<i32> = Vec::new();
        for i in 0..self.childs.len() {
            if self.childs[i].n == 0 { rnd.push(i as i32) }
        }
        if rnd.len() > 0 {
            return rnd[rand::thread_rng().gen_range(0,rnd.len())];
        }
        
        let e = (1.0 as f64).exp();

        let mut max_ucb = -1.0;
        let mut max_idx = 0;
        for i in 0..self.childs.len() {
            let mut tmp: f64 = 0.0;
            tmp += (self.childs[i].win as f64) / (self.childs[i].n as f64);
            tmp += (2.0 * (N as f64).log(e) / (self.childs[i].n as f64) ).sqrt();
            if max_ucb < tmp {
                max_ucb = tmp;
                max_idx = i;
            }
        }
        return max_idx as i32
    }
}
fn MCTS_(node: &mut MCTNode, N: i32) -> i32{
    let tmp = node.select(N);
    let winner = match tmp {
        -1 => {
            if node.n > 1 { node.make_child() }; 
            node.board.playout(node.player)
        },
        _ => MCTS_(&mut node.childs[tmp as usize], N)
    };
    if node.player == 3 - winner { node.win += 1; }
    node.n += 1;
    return winner
}

pub fn MCTS(player : i32, board: BitBoard, maxitr: i32) -> (i32,i32) {
    let mut root = MCTNode::new();
    root.player = player;
    root.board = board;
    root.make_child();

    for i in 1..maxitr {
        MCTS_(&mut root, i);
    }

    let mut dbg : Vec<(f64,(i32,i32))> = Vec::new();
    let mut next = (0,0);
    let mut mx = -1.0;
    for i in 0..root.childs.len() {
        dbg.push((root.childs[i].win as f64 / root.childs[i].n as f64, root.childs[i].mov));
        if mx < root.childs[i].win as f64 / root.childs[i].n as f64 {
            mx = root.childs[i].win as f64 / root.childs[i].n as f64;
            next = root.childs[i].mov;
        }
    }
    return next
}