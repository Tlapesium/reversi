import init, {alphabeta_js, mcts_js} from './pkg/reversi.js';

let canvas = document.getElementById("canvas");
let context = canvas.getContext('2d');


let player = 1;
let waiting = 0;
let board = [
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,2,1,0,0,0],
    [0,0,0,1,2,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0]
];

let offsetX = 50;
let offsetY = 50;
let W = 400;
let H = 400;

function draw() {
    AI();
    context.clearRect(0,0,500,500);
    context.fillStyle = "green";
    context.fillRect(offsetX,offsetY,400,400);

    for (let i = 0; i <= 8; i++) {
        context.beginPath();
        context.moveTo(offsetX,offsetY+i*50);
        context.lineTo(offsetX+W,offsetY+i*50);
        context.closePath();
        context.stroke();
        context.beginPath();
        context.moveTo(offsetX+i*50,offsetY);
        context.lineTo(offsetX+i*50,offsetY+H);
        context.closePath();
        context.stroke();
    }
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            if (board[i][j] == 1) {
                context.beginPath();
                context.fillStyle = "black";
                context.arc(offsetX + j*50 + 25, offsetY + i*50 + 25, 15, 0, Math.PI * 2, true);
                context.closePath();
                context.fill();
            }
            if (board[i][j] == 2) {
                context.beginPath();
                context.fillStyle = "white";
                context.arc(offsetX + j*50 + 25, offsetY + i*50 + 25, 15, 0, Math.PI * 2, true);
                context.closePath();
                context.fill();
            }
            if (board_is_valid(player, i, j)) {
                context.beginPath();
                context.fillStyle = "yellow";
                context.arc(offsetX + j*50 + 25, offsetY + i*50 + 25, 15, 0, Math.PI * 2, true);
                context.closePath();
                context.fill();
            }
        }
    }

    context.fillStyle = "black";
    context.font = "24px sans-serif"
    if (board_is_end() == 1) { context.fillText("黒の勝利", 200, 35, 100); }
    if (board_is_end() == 2) { context.fillText("白の勝利", 200, 35, 100); }
    if (board_is_end() == 3) { context.fillText("引き分け", 200, 35, 100); }
    window.requestAnimationFrame(draw);
}

function board_get(y, x) {
    if (y < 0 || y >= 8 || x < 0 || x >= 8) {
        return 0;
    }
    return board[y][x];
}

function board_init() {
    player = 1;
    board = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,2,1,0,0,0],
        [0,0,0,1,2,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]
    ];
}

function board_get_turn_over(pler, y, x, d, e) {
    let i = 1;
    while ( board_get(y+i*d, x+i*e) == 3 - pler ) { i += 1; }
    if (board_get(y+i*d, x+i*e) == pler) { return i-1; }
    else { return 0; }
}

function board_is_valid(pler, y, x) {
    if (y < 0 || y >= 8 || x < 0 || x >= 8) { return false; }
    if (board[y][x] != 0) { return false; }
    for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
            if (board_get_turn_over(pler, y, x, i, j) != 0) {
                return true;
            }
        }
    }
    return false;
}

function board_put(y, x) {
    if (!board_is_valid(player, y,x)){ return; }
    for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
            let cnt = board_get_turn_over(player, y, x, i, j);
            for (let k = 1; k <= cnt; k++) {
                board[y+k*i][x+k*j] = player;
            }
        }
    }
    board[y][x] = player;
    player = 3-player;
    if (board_get_all_moves(player).length == 0) {
        player = 3 - player;
        return;
    }
}

function board_get_all_moves(pler) {
    let moves = [];
    for(let i = 0; i < 8; i++) {
        for(let j = 0; j < 8; j++) {
            if (board_is_valid(pler, i, j)) {
                moves.push([i,j]);
            }
        }
    }
    return moves;
}

function board_random_move() {
    let moves = board_get_all_moves(player);
    return moves[Math.floor(Math.random()*moves.length)];
}

function board_is_end() {
    if (board_get_all_moves(player).length == 0 && board_get_all_moves(3 - player) == 0) {
        let blk = 0; let wht = 0;
        for(let i = 0; i < 8; i++) {
            for(let j = 0; j < 8; j++) {
                if (board[i][j] == 1) { blk += 1; }
                if (board[i][j] == 2) { wht += 1; }
            }
        }
        if (blk > wht) { return 1; }
        if (blk < wht) { return 2; }
        if (blk == wht) { return 3; }
    }
    return 0;
}

function AI() {
    let sente = document.getElementById("sente").value;
    let gote = document.getElementById("gote").value;

    if (board_is_end() != 0) { return; }

    if (player == 1 && sente == "random") {
        let move = board_random_move();
        board_put(move[0],move[1]);
    }
    if (player == 2 && gote == "random") {
        let move = board_random_move();
        board_put(move[0],move[1]);
    }

    if (player == 1 && sente == "alphabeta" && waiting == 0) {
        async function run() {
            waiting = 1;
            await init();
            let move = alphabeta_js(player, board, 7);
            board_put(move[0],move[1]);
            waiting = 0;
        };
        run();
    }
    if (player == 2 && gote == "alphabeta" && waiting == 0) {
        async function run() {
            waiting = 1;
            await init();
            let move = alphabeta_js(player, board, 7);
            board_put(move[0],move[1]);
            waiting = 0;
        };
        run();
    }

    if (player == 1 && sente == "mcts" && waiting == 0) {
        async function run() {
            waiting = 1;
            await init();
            let move = mcts_js(player, board, 3000);
            board_put(move[0],move[1]);
            waiting = 0;
        };
        run();
    }
    if (player == 2 && gote == "mcts" && waiting == 0) {
        async function run() {
            waiting = 1;
            await init();
            let move = mcts_js(player, board, 3000);
            board_put(move[0],move[1]);
            waiting = 0;
        };
        run();
    }
}

function onClick(e) {
    let sente = document.getElementById("sente").value;
    let gote = document.getElementById("gote").value;

    if (board_is_end() != 0) { board_init(); return; }

    let x = e.clientX - canvas.offsetLeft;
    let y = e.clientY - canvas.offsetTop;
    if (0 <= x - offsetX && x-offsetX <= 400 && 0 <= y - offsetY && y-offsetY <= 400) {
        let x_ = Math.floor((x - offsetX)/50); let y_ = Math.floor((y - offsetY)/50);
        if (player == 1 && sente == "human") {
            board_put(y_,x_);
        } 
        if (player == 2 && gote == "human") {
            board_put(y_,x_);
        }
    }
}

window.requestAnimationFrame(draw);
canvas.addEventListener('click', onClick, false);