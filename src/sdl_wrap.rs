#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

pub fn Present(w3d: &mut modules) {
    //println!("Present");

    let texture_creator = w3d.id_vl.renderer.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            w3d.id_vl.screenWidth as u32,
            w3d.id_vl.screenHeight as u32,
        )
        .map_err(|e| e.to_string())
        .unwrap();

    let pixels = w3d.id_vl.screen.without_lock_mut().unwrap();

    texture
        .update(None, pixels, w3d.id_vl.screenPitch as usize)
        .unwrap();
    w3d.id_vl.renderer.clear();
    w3d.id_vl.renderer.copy(&texture, None, None).unwrap();
    w3d.id_vl.renderer.present();
}
