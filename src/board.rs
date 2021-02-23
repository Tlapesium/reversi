
const INIT_BOARD1 :u64 = 0u64 | (1 << (3*8+4)) | (1 << (4*8+3));
const INIT_BOARD2 :u64 = 0u64 | (1 << (3*8+3)) | (1 << (4*8+4));

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Board {
    pub _b1 : u64,
    pub _b2 : u64
}

impl Board {
    pub fn new() -> Self {
        Board {_b1 : INIT_BOARD1, _b2: INIT_BOARD2 }
    }

    pub fn get(&self, y: i32, x: i32) -> i32 {
        match y {
            0..=7 => match x {
                0..=7 => (( (self._b1 & (1 << (y*8+x))) >> (y*8+x) ) + ( (self._b2 & (1 << (y*8+x))) >> (y*8+x) ) * 2) as i32 ,
                _ => 0
            },
            _ => 0
        }
    }

    pub fn set(&mut self, player: i32, y: i32, x: i32) {
        if y < 0 || y >= 8 || x < 0 || x >= 8 { panic!{"Error"} }
        self._b1 &= !(0u64 | 1 << (y*8+x));
        self._b2 &= !(0u64 | 1 << (y*8+x));
        match player {
            1 => self._b1 |= 1u64 << (y*8+x),
            2 => self._b2 |= 1u64 << (y*8+x),
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

        for i in 0..8 {
            for j in 0..8 {
                if self.is_valid(player, i, j) {
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

    pub fn count_turn_over(&self, player: i32, y: i32, x: i32, d: i32, e: i32) -> i32 {
        let mut i = 1;
        while self.get(y+i*d, x+i*e) == 3 - player { i += 1; }
        if self.get(y+i*d, x+i*e) == player { return i-1 }
        else { return 0 }
    }

    pub fn is_valid(&self, player : i32, y: i32 , x: i32) -> bool {
        if y < 0 || x < 0 || x >= 8 || y >= 8 { return false }
        if self.get(y, x) != 0 { return false }
        for i in -1..=1 {
            for j in -1..=1 {
                if self.count_turn_over(player, y, x, i, j) != 0 {
                    return true
                }
            }
        }
        return false
    }
    
    pub fn put(&mut self, player: i32, y: i32, x: i32) {
        if !self.is_valid(player,y,x) {
            println!("{}, {}",y,x);
            panic!("invalid move");
        }
        
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 { continue }
                let cnt = self.count_turn_over(player, y, x, i, j);
                for k in 1..=cnt {
                    self.set(player,y+k*i,x+k*j);
                }
            }
        }
        self.set(player, y, x);
    }

    pub fn get_all_move(&self, player: i32) -> Vec<(i32,i32)> {
        let mut ret: Vec<(i32,i32)> = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if self.is_valid(player, i, j) {
                    ret.push((i,j))
                }
            }
        }
        return ret
    }

    pub fn is_end(&self, player: i32) -> i32 {
        if self.get_all_move(player).len() == 0 && self.get_all_move(3 - player).len() == 0 {
            let mut blk = 0;
            let mut wht = 0;
            for i in 0..8 {
                for j in 0..8 {
                    match self.get(i,j) {
                        1 => blk += 1,
                        2 => wht += 1,
                        _ => ()
                    }
                }
            }
            if blk > wht { return 1 }
            else if blk < wht { return 2 }
            else { return 3 }
        }
        else{
            return 0
        }
    }


}


