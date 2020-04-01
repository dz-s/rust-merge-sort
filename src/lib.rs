use js_sys::{JsString, Uint8Array};
use std::str;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

mod pool;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logs(s: &JsString);
}
pub fn merge(vec: &mut Vec<u8>, _begin: usize, _mid: usize, _end: usize) {
    let mut begin: usize = _begin;
    let mut mid: usize = _mid;
    let end: usize = _end;
    
    let mut mid_plus: usize = mid + 1;

    if vec[mid] <= vec[mid_plus] {
        return;
    }
    while begin <= mid && mid_plus <= end {
        if vec[begin] <= vec[mid_plus] {
            begin += 1;
        } else {
            let val: u8 = vec[mid_plus];
            let mut idx: usize = mid_plus;

            // Shift all the elements between element 1
            // element 2, right by 1.
            while idx != begin {
                vec[idx] = vec[idx - 1];
                idx -= 1;
            }
            vec[begin] = val;

            // Update all the pointers
            begin += 1;
            mid += 1;
            mid_plus += 1;
        }
    }
}

pub fn merge_sort2(vec: &mut Vec<u8>, _l: usize, _r: usize) {
    if _l < _r {
        // Same as (l + r) / 2, but avoids overflow
        // for large l and r
        let m = _l + (_r - _l) / 2;

        // Sort first and second halves
        merge_sort2(vec, _l, m);
        merge_sort2(vec, m + 1, _r);

        merge(vec, _l, m, _r);
    }
}

#[wasm_bindgen]
pub fn merge_sort(arr: &JsValue) {
    let u8_arr = Uint8Array::new(arr);
    let mut vec: Vec<u8> = u8_arr.to_vec();

    let _r = vec.len();
    //merge_sort2(&mut vec, 0, _r);

    vec.sort();
    let str_vec = vec.into_iter().map(|i| i.to_string()).collect::<String>();
    // let str = str_vec.join(", ");
    log(&str_vec)
}
