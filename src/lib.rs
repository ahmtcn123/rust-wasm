use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn confirm(s: &str) -> bool;
}

#[wasm_bindgen]
pub fn merhaba() {
    //Kullanıcıya sor
    let onay = confirm("İyi miyiz?");
    if onay {
        //Kullanıcı onayladı
        alert(":)");
    } else {
        //Kullanıcı onaylamadı
        alert(":(");
    }
}

/// Herhangi bir hata olursa konsola yazıyoruz
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}