
const INIT_BOARD1 :u64 = 0u64 | (1 << (3*8+4)) | (1 << (4*8+3));
const INIT_BOARD2 :u64 = 0u64 | (1 << (3*8+3)) | (1 << (4*8+4));

#[derive(Debug, Clone, Copy)]
pub struct BitBoard {
    pub board: [u64; 3]
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {board: [0,INIT_BOARD1, INIT_BOARD2]}
    }

    pub fn get(&self, y: i32, x: i32) -> i32 {
        match y {
            0..=7 => match x {
                0..=7 => (( (self.board[1] & (1 << (y*8+x))) >> (y*8+x) ) + ( (self.board[2] & (1 << (y*8+x))) >> (y*8+x) ) * 2) as i32 ,
                _ => 0
            },
            _ => 0
        }
    }

    pub fn set(&mut self, player: i32, y: i32, x: i32) {
        if y < 0 || y >= 8 || x < 0 || x >= 8 { panic!{"Error"} }
        self.board[1] &= !(0u64 | 1 << (y*8+x));
        self.board[2] &= !(0u64 | 1 << (y*8+x));
        match player {
            1 => self.board[1] |= 1u64 << (y*8+x),
            2 => self.board[2] |= 1u64 << (y*8+x),
            _ => panic!("Error")
        }
    }

    pub fn dump(&self, player: i32) {
        let mut tmp = [[' '; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                tmp[i][j] = match self.get(i as i32,j as i32) {
                    0 => ' ',
                    1 => '●',
                    2 => '○',
                    _ => '.'
                }
            }
        }

        let legal_board = self.make_legal_board(player);

        for i in 0..8 {
            for j in 0..8 {
                if (legal_board >> (i*8+j)) & 1 != 0 {
                    tmp[i as usize][j as usize] = '*'
                }
            }
        }
        
        
        println!("  0 1 2 3 4 5 6 7");
        for i in 0..8 {
            println!("{} {} {} {} {} {} {} {} {}", i , tmp[i][0], tmp[i][1], tmp[i][2], tmp[i][3],
                                                        tmp[i][4], tmp[i][5], tmp[i][6], tmp[i][7]);
        }
    }

    pub fn make_legal_board(&self, player: i32) -> u64 {
        let horizontal_sentinel = self.board[3-player as usize] & 0x7e7e7e7e7e7e7e7eu64;
        let vertical_sentinel = self.board[3-player as usize] & 0x00FFFFFFFFFFFF00u64;
        let all_sentinel = self.board[3-player as usize]  & 0x007e7e7e7e7e7e00u64;
        let blank = !(self.board[player as usize] | self.board[3-player as usize]);
        let mut tmp = 0u64;
        let mut legal_board = 0u64;

        // 左
        tmp = horizontal_sentinel & (self.board[player as usize] << 1);
        for _ in 0..5 {tmp |= horizontal_sentinel & (tmp << 1);}
        legal_board |= blank & (tmp << 1);

        // 右
        tmp = horizontal_sentinel & (self.board[player as usize] >> 1);
        for _ in 0..5 {tmp |= horizontal_sentinel & (tmp >> 1);}
        legal_board |= blank & (tmp >> 1);

        // 上
        tmp = vertical_sentinel & (self.board[player as usize] << 8);
        for _ in 0..5 {tmp |= vertical_sentinel & (tmp << 8);}
        legal_board |= blank & (tmp << 8);

        // 下
        tmp = vertical_sentinel & (self.board[player as usize] >> 8);
        for _ in 0..5 {tmp |= vertical_sentinel & (tmp >> 8);}
        legal_board |= blank & (tmp >> 8);

        // 右斜め上
        tmp = all_sentinel & (self.board[player as usize] << 7);
        for _ in 0..5 {tmp |= all_sentinel & (tmp << 7);}
        legal_board |= blank & (tmp << 7);
        
        // 左斜め下
        tmp = all_sentinel & (self.board[player as usize] >> 7);
        for _ in 0..5 {tmp |= all_sentinel & (tmp >> 7);}
        legal_board |= blank & (tmp >> 7);

        // 左斜め上
        tmp = all_sentinel & (self.board[player as usize] << 9);
        for _ in 0..5 {tmp |= all_sentinel & (tmp << 9);}
        legal_board |= blank & (tmp << 9);
                
        // 右斜め下
        tmp = all_sentinel & (self.board[player as usize] >> 9);
        for _ in 0..5 {tmp |= all_sentinel & (tmp >> 9);}
        legal_board |= blank & (tmp >> 9);

        return legal_board
    }

    pub fn transfer(&self, put: u64, k: i32) -> u64 {
        match k {
            0 => (put << 8) & 0xffffffffffffff00u64,
            1 => (put << 7) & 0x7f7f7f7f7f7f7f00u64,
            2 => (put >> 1) & 0x7f7f7f7f7f7f7f7fu64,
            3 => (put >> 9) & 0x007f7f7f7f7f7f7fu64,
            4 => (put >> 8) & 0x00ffffffffffffffu64,
            5 => (put >> 7) & 0x00fefefefefefefeu64,
            6 => (put << 1) & 0xfefefefefefefefeu64,
            7 => (put << 9) & 0xfefefefefefefe00u64,
            _ => 0
        }
    }

    pub fn reverse(&mut self, player: i32, put: u64) {
        let mut rev = 0u64;
        for k in 0..8 {
            let mut rev_ = 0u64;
            let mut mask = self.transfer(put, k);
            while (mask != 0) && ((mask & self.board[3-player as usize]) != 0) {
                rev_ |= mask;
                mask = self.transfer(mask, k);
            }
            if (mask & self.board[player as usize]) != 0 {
                rev |= rev_;
            }
        }
        self.board[player as usize] ^= put | rev;
        self.board[3-player as usize] ^= rev;
    }

    pub fn put(&mut self, player: i32, y: i32, x: i32) {
        if (self.make_legal_board(player) >> (y*8+x)) & 1 != 0 {
            self.reverse(player, 1u64 << (y*8+x))
        }
    }

    pub fn is_pass(&self, player: i32) -> bool {
        return self.make_legal_board(player) == 0 && self.make_legal_board(3-player) != 0;
    }
    
    pub fn is_end(&self, player: i32) -> i32 {
            if self.make_legal_board(player) == 0 && self.make_legal_board(3-player) == 0 {
                if self.board[1].count_ones() > self.board[2].count_ones() { 1 }
                else if self.board[1].count_ones() < self.board[2].count_ones() { 2 }
                else { 3 }
            }
            else { 0 }
    }
}