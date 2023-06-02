#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate libc;

#[link(name = "fmopl", kind = "static")]
extern "C" {
    // FM sound generator
    pub fn YM3812Init(numChips: i32, clock: i32, rate: i32) -> i32;
    pub fn YM3812Write(which: i32, a: i32, v: i32);
    pub fn YM3812UpdateOne(which: i32, stream: *const i16, length: i32);
    pub fn YM3812Shutdown();
}

#[allow(improper_ctypes)]
extern "C" {
    // SDL mixer missing wrappers in Rust-SDL

    pub fn Mix_HookMusic(
        mix_func: ::core::option::Option<
            unsafe extern "C" fn(udata: &mut modules, stream: *mut u8, len: libc::c_int),
        >,
        arg: &mut modules,
    );

    pub fn Mix_SetPostMix(
        mix_func: ::core::option::Option<
            unsafe extern "C" fn(udata: &mut modules, stream: *mut u8, len: libc::c_int),
        >,
        arg: &mut modules,
    );
}

extern crate sdl2;
use bincode::ErrorKind;
use libm;
use sdl2::event::*;
use sdl2::hint;
use sdl2::joystick::*;
use sdl2::keyboard::*;
use sdl2::mixer::*;
use sdl2::mouse::*;
use sdl2::pixels::*;
use sdl2::render::*;
use sdl2::surface::*;
use sdl2::video::*;
use sdl2::IntegerOrSdlError;
use sdl2::JoystickSubsystem;
use sdl2::Sdl;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use std::env;
use std::fs::*;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::process::exit;
use std::ptr;
use std::str::Split;

/////////////////////////////////

mod audiowl6;
mod foreign;
#[cfg(feature = "UPLOAD")]
mod gfxv_apo;
#[cfg(feature = "GOODTIMES")]
mod gfxv_wl6;
mod sdl_wrap;
mod signon;
mod wolfpal;

mod id_ca;
mod id_in;
mod id_pm;
mod id_sd;
mod id_us;
mod id_vh;
mod id_vl;

mod wl_act1;
mod wl_act2;
mod wl_agent;
mod wl_debug;
mod wl_def;
mod wl_draw;
mod wl_game;
mod wl_inter;
mod wl_main;
mod wl_menu;
mod wl_play;
mod wl_scale;
mod wl_state;
mod wl_text;
mod wl_utils;
/////////////////////////////////
mod prelude {
    pub use crate::audiowl6::*;
    pub use crate::foreign::*;
    #[cfg(feature = "UPLOAD")]
    pub use crate::gfxv_apo::*;
    #[cfg(feature = "GOODTIMES")]
    pub use crate::gfxv_wl6::*;
    pub use crate::sdl_wrap::*;
    pub use crate::signon::*;
    pub use crate::wolfpal::*;

    pub use crate::id_ca::*;
    pub use crate::id_in::*;
    pub use crate::id_pm::*;
    pub use crate::id_sd::*;
    pub use crate::id_us::*;
    pub use crate::id_vh::*;
    pub use crate::id_vl::*;

    pub use crate::wl_act1::*;
    pub use crate::wl_act2::*;
    pub use crate::wl_agent::*;
    pub use crate::wl_debug::*;
    pub use crate::wl_def::*;
    pub use crate::wl_draw::*;
    pub use crate::wl_game::*;
    pub use crate::wl_inter::*;
    pub use crate::wl_main::*;
    pub use crate::wl_menu::*;
    pub use crate::wl_play::*;
    pub use crate::wl_scale::*;
    pub use crate::wl_state::*;
    pub use crate::wl_text::*;
    pub use crate::wl_utils::*;
}

use prelude::*;
/////////////////////////////////

pub struct modules<'a> {
    //managers
    id_ca: id_ca::id_ca,
    id_in: id_in::id_in,
    id_pm: id_pm::id_pm,
    id_sd: id_sd::id_sd,
    id_us: id_us::id_us,
    id_vh: id_vh::id_vh,
    id_vl: id_vl::id_vl<'a>,
    //engines
    wl_act1: wl_act1::wl_act1,
    wl_act2: wl_act2::wl_act2,
    wl_agent: wl_agent::wl_agent,
    wl_draw: wl_draw::wl_draw,
    wl_game: wl_game::wl_game,
    wl_inter: wl_inter::wl_inter,
    wl_main: wl_main::wl_main,
    wl_menu: wl_menu::wl_menu,
    wl_play: wl_play::wl_play,
    wl_state: wl_state::wl_state,
    wl_text: wl_text::wl_text,
}

impl<'a> modules<'a> {
    pub fn new() -> Self {
        Self {
            //init managers id_*
            id_ca: id_ca::id_ca::new(),
            id_in: id_in::id_in::new(),
            id_pm: id_pm::id_pm::new(),
            id_sd: id_sd::id_sd::new(),
            id_us: id_us::id_us::new(),
            id_vh: id_vh::id_vh::new(),
            id_vl: id_vl::id_vl::new(),
            //init engines wl_*
            wl_act1: wl_act1::wl_act1::new(),
            wl_act2: wl_act2::wl_act2::new(),
            wl_agent: wl_agent::wl_agent::new(),
            wl_draw: wl_draw::wl_draw::new(),
            wl_game: wl_game::wl_game::new(),
            wl_inter: wl_inter::wl_inter::new(),
            wl_main: wl_main::wl_main::new(),
            wl_menu: wl_menu::wl_menu::new(),
            wl_play: wl_play::wl_play::new(),
            wl_state: wl_state::wl_state::new(),
            wl_text: wl_text::wl_text::new(),
        }
    }
}

fn main() {
    ////////////////////////////////////////////////////////
    // Init Modules
    ////////////////////////////////////////////////////////

    let mut w3d = modules::new();

    ////////////////////////////////////////////////////////
    // Init Object Struct (defined in wl_play)
    ////////////////////////////////////////////////////////

    let mut ob = wl_play::object::new();

    ////////////////////////////////////////////////////////

    CheckParameters(&mut w3d);

    CheckForEpisodes(&mut w3d);

    InitGame(&mut w3d);

    DemoLoop(&mut w3d, &mut ob);

    Quit("Demo loop exited???");
}
