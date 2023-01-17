#![feature(local_key_cell_methods)]

mod game;
mod random;

use {
    crate::game::*,
    gloo::dialogs::{alert, prompt},
    wasm_bindgen::prelude::*,
    std::cell::RefCell,
    js_sys::Array as JsArray
};

thread_local! {
    static MS: RefCell<Game> = RefCell::new(Game::new({
        let (small, medium, large) = ([9, 9, 10], [16, 16, 40], [30, 16, 99]);
        if let Some(size) = prompt("Choose a board size (`Small`, `Medium`, or `Large`):", Some("Medium")) {
            match size.as_str() {
                "Small"  | "small"  | "S" | "s" => small,
                "Medium" | "medium" | "M" | "m" => medium,
                "Large"  | "large"  | "L" | "l" => large,
                _ => {
                    alert(&format!("`{}` is not a valid board size. Defaulting to Medium.", size));
                    medium
                }
            }
        } else {
            alert("Message was not received. Defualting to Medium");
            medium
        }
    }));
}

#[wasm_bindgen]
pub fn width() -> usize {
    MS.with_borrow(Game::width)
}

#[wasm_bindgen]
pub fn height() -> usize {
    MS.with_borrow(Game::height)
}

#[wasm_bindgen(js_name = boardState)]
pub fn wasm_board_state() -> JsArray {
    MS.with_borrow(Game::wasm_board_state)
}

#[wasm_bindgen(js_name = fieldsOpened)]
pub fn fields_opened() -> usize {
    MS.with_borrow(Game::fields_opened)
}

#[wasm_bindgen(js_name = flagsRemaining)]
pub fn flags_remaining() -> usize {
    MS.with_borrow(Game::flags_remaining)
}

#[wasm_bindgen(js_name = isFinished)]
pub fn is_finished() -> u8 {
    match MS.with_borrow(Game::is_finished) {
        None        => 0,
        Some(true)  => 1,
        Some(false) => 2,
    }
}

#[wasm_bindgen]
pub fn open(x: usize, y: usize) {
    if MS.with_borrow_mut(|ms| ms.open((x, y))) {
        if MS.with_borrow(Game::fields_opened) == 1 {
            MS.with_borrow_mut(Game::board_reset);
            while MS.with_borrow_mut(|ms| ms.open((x, y))) {
                MS.with_borrow_mut(Game::board_reset);
            }
        } else {
            MS.with_borrow_mut(|ms| {
                ms.lost();
                ms.show_loss();
            });
        }
    } else {
        MS.with_borrow_mut(Game::win_check);
    }
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    MS.with_borrow_mut(|ms| {
        ms.toggle_flag((x, y));
        ms.win_check();
    })
}
