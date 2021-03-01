use wasm_bindgen::prelude::*;
use js_sys::*;
extern crate console_error_panic_hook;
use std::panic;

mod mcts;
use mcts::*;

mod minmax;
use minmax::*;

mod bitboard;
use bitboard::*;

mod negascout;
use negascout::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn alphabeta_js(player: JsValue, board: JsValue, depth: JsValue) -> Vec<i32> {
    let Player = player.as_f64().unwrap() as i32;
    let Depth = depth.as_f64().unwrap() as i32;

    let mut b = BitBoard::new();
    let arr = Array::from(&board);

    for i in 0..arr.length() {
        let ar = Array::from(&arr.get(i));
        for j in 0..ar.length() {
            let tmp = ar.get(j).as_f64().unwrap() as i32;
            if tmp != 0 {
                b.set(tmp, i as i32, j as i32);
            }
        }
    }

    let mv = alphabeta(Player, b, Depth);

    let mut ret = Vec::new();
    ret.push(mv.0);
    ret.push(mv.1);

    return ret;
}

#[wasm_bindgen]
pub fn mcts_js(player: JsValue, board: JsValue, itr: JsValue) -> Vec<i32> {
    let Player = player.as_f64().unwrap() as i32;
    let Itr = itr.as_f64().unwrap() as i32;

    let mut b = BitBoard::new();
    let arr = Array::from(&board);

    for i in 0..arr.length() {
        let ar = Array::from(&arr.get(i));
        for j in 0..ar.length() {
            let tmp = ar.get(j).as_f64().unwrap() as i32;
            if tmp != 0 {
                b.set(tmp, i as i32, j as i32);
            }
        }
    }

    let mv = MCTS(Player, b, Itr);

    let mut ret = Vec::new();
    ret.push(mv.0);
    ret.push(mv.1);

    return ret;
}

#[wasm_bindgen]
pub fn negascout_js(player: JsValue, board: JsValue, depth: JsValue) -> Vec<i32> {
    let Player = player.as_f64().unwrap() as i32;
    let Depth = depth.as_f64().unwrap() as i32;

    let mut b = BitBoard::new();
    let arr = Array::from(&board);

    for i in 0..arr.length() {
        let ar = Array::from(&arr.get(i));
        for j in 0..ar.length() {
            let tmp = ar.get(j).as_f64().unwrap() as i32;
            if tmp != 0 {
                b.set(tmp, i as i32, j as i32);
            }
        }
    }

    let mv = negascout(Player, b, Depth);

    let mut ret = Vec::new();
    ret.push(mv.0);
    ret.push(mv.1);

    return ret;
}