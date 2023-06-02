#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  id_in
//
//===========================================================================

pub struct id_in {
    pub MousePresent: bool,
    pub forcegrabmouse: bool,
    pub KeyboardState: [bool; 129],
    pub Paused: bool,
    pub LastASCII: Keycode,
    pub LastScan: Scancode,
    pub KbdDefs: KeyboardDef,
    pub btnstate: [bool; buttontype::NUMBUTTONS as usize],
    pub Joystick: bool,
    pub JoyNumButtons: i32,
    pub JoyNumHats: i32,
    pub GrabInput: bool,
    pub IN_Started: bool,
}

impl id_in {
    pub fn new() -> Self {
        let KbdDefs: KeyboardDef = KeyboardDef {
            button0: Scancode::LCtrl,
            button1: Scancode::LAlt,
            upleft: Scancode::Home,
            up: Scancode::Up,
            upright: Scancode::PageUp,
            left: Scancode::Left,
            right: Scancode::Right,
            downleft: Scancode::End,
            down: Scancode::Down,
            downright: Scancode::PageDown,
        };

        Self {
            MousePresent: false,
            forcegrabmouse: false,
            KeyboardState: [false; 129],
            Paused: false,
            LastASCII: Keycode::F24,
            LastScan: Scancode::F24, // BUG should be sc_None
            KbdDefs,
            btnstate: [false; buttontype::NUMBUTTONS as usize],
            Joystick: false,
            JoyNumButtons: 0,
            JoyNumHats: 0,
            GrabInput: false,
            IN_Started: false,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub fn SDL_BUTTON(X: i32) -> i32 {
    1 << ((X) - 1)
}

pub const SDL_BUTTON_MIDDLE: i32 = 2;
pub const SDL_BUTTON_RIGHT: i32 = 3;

#[repr(i8)]
#[derive(Clone, Copy)]
pub enum Motion_x {
    motion_Left = -1, // -1
    motion_None = 0,  // 0
    motion_Right = 1, // 1
}

#[repr(i8)]
#[derive(Clone, Copy)]
pub enum Motion_y {
    motion_Up = -1,  // -1
    motion_None = 0, // 0
    motion_Down = 1, //1
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    dir_North,
    dir_NorthEast,
    dir_East,
    dir_SouthEast,
    dir_South,
    dir_SouthWest,
    dir_West,
    dir_NorthWest,
    dir_None,
}

// Quick lookup for total direction
pub const DirTable: [Direction; 9] = [
    Direction::dir_NorthWest,
    Direction::dir_North,
    Direction::dir_NorthEast,
    Direction::dir_West,
    Direction::dir_None,
    Direction::dir_East,
    Direction::dir_SouthWest,
    Direction::dir_South,
    Direction::dir_SouthEast,
];

pub struct KeyboardDef {
    pub button0: Scancode,
    pub button1: Scancode,
    pub upleft: Scancode,
    pub up: Scancode,
    pub upright: Scancode,
    pub left: Scancode,
    pub right: Scancode,
    pub downleft: Scancode,
    pub down: Scancode,
    pub downright: Scancode,
}

pub struct CursorInfo {
    pub button0: u8,
    pub button1: u8,
    pub button2: u8,
    pub button3: u8,
    pub x: i32,
    pub y: i32,
    pub xaxis: Motion_x,
    pub yaxis: Motion_y,
    pub dir: Direction,
}

impl CursorInfo {
    pub fn new() -> Self {
        Self {
            button0: 0,
            button1: 0,
            button2: 0,
            button3: 0,
            x: 0,
            y: 0,
            xaxis: Motion_x::motion_None,
            yaxis: Motion_y::motion_None,
            dir: Direction::dir_None,
        }
    }
}
pub type ControlInfo = CursorInfo;

pub const UNKNOWN_KEY: i32 = KEYCOUNT;

pub const KEYCOUNT: i32 = 129;

pub fn Keyboard(w3d: &mut modules, key: Scancode) -> bool {
    //println!("Keyboard");

    let keyIndex = KeyboardLookup(Keycode::from_scancode(key).unwrap());

    if keyIndex != UNKNOWN_KEY as i32 {
        return w3d.id_in.KeyboardState[keyIndex as usize];
    } else {
        return false;
    }
}

pub fn KeyboardSet(w3d: &mut modules, sym: Scancode, state: bool) {
    //println!("KeyboardSet");

    let keyIndex: i32;
    let key = Keycode::from_scancode(sym);

    match key {
        Some(key) => {
            keyIndex = KeyboardLookup(key);
        }
        None => {
            keyIndex = UNKNOWN_KEY;
        }
    }

    if keyIndex != UNKNOWN_KEY as i32 {
        w3d.id_in.KeyboardState[keyIndex as usize] = state;
    }
}

pub fn KeyboardLookup(key: Keycode) -> i32 {
    //println!("KeyboardLookup");

    match key {
        //SDLK_UNKNOWN => 0,
        Keycode::Backspace => return 1,
        Keycode::Tab => return 2,
        Keycode::Clear => return 3,
        Keycode::Return => return 4,
        Keycode::Pause => return 5,
        Keycode::Escape => return 6,
        Keycode::Space => return 7,
        Keycode::Exclaim => return 8,
        Keycode::Quotedbl => return 9,
        Keycode::Hash => return 10,
        Keycode::Dollar => return 11,
        Keycode::Ampersand => return 12,
        Keycode::Quote => return 13,
        Keycode::LeftParen => return 14,
        Keycode::RightParen => return 15,
        Keycode::Asterisk => return 16,
        Keycode::Plus => return 17,
        Keycode::Comma => return 18,
        Keycode::Minus => return 19,
        Keycode::Period => return 20,
        Keycode::Slash => return 21,
        Keycode::Num0 => return 22,
        Keycode::Num1 => return 23,
        Keycode::Num2 => return 24,
        Keycode::Num3 => return 25,
        Keycode::Num4 => return 26,
        Keycode::Num5 => return 27,
        Keycode::Num6 => return 28,
        Keycode::Num7 => return 29,
        Keycode::Num8 => return 30,
        Keycode::Num9 => return 31,
        Keycode::Colon => return 32,
        Keycode::Semicolon => return 33,
        Keycode::Less => return 34,
        Keycode::Equals => return 35,
        Keycode::Greater => return 36,
        Keycode::Question => return 37,
        Keycode::At => return 38,
        Keycode::LeftBracket => return 39,
        Keycode::Backslash => return 40,
        Keycode::RightBracket => return 41,
        Keycode::Caret => return 42,
        Keycode::Underscore => return 43,
        Keycode::Backquote => return 44,
        Keycode::A => return 45,
        Keycode::B => return 46,
        Keycode::C => return 47,
        Keycode::D => return 48,
        Keycode::E => return 49,
        Keycode::F => return 50,
        Keycode::G => return 51,
        Keycode::H => return 52,
        Keycode::I => return 53,
        Keycode::J => return 54,
        Keycode::K => return 55,
        Keycode::L => return 56,
        Keycode::M => return 57,
        Keycode::N => return 58,
        Keycode::O => return 59,
        Keycode::P => return 60,
        Keycode::Q => return 61,
        Keycode::R => return 62,
        Keycode::S => return 63,
        Keycode::T => return 64,
        Keycode::U => return 65,
        Keycode::V => return 66,
        Keycode::W => return 67,
        Keycode::X => return 68,
        Keycode::Y => return 69,
        Keycode::Z => return 70,
        Keycode::Delete => return 71,
        Keycode::KpPeriod => return 72,
        Keycode::KpDivide => return 73,
        Keycode::KpMultiply => return 74,
        Keycode::KpMinus => return 75,
        Keycode::KpPlus => return 76,
        Keycode::KpEnter => return 77,
        Keycode::KpEquals => return 78,
        Keycode::Up => return 79,
        Keycode::Down => return 80,
        Keycode::Right => return 81,
        Keycode::Left => return 82,
        Keycode::Insert => return 83,
        Keycode::Home => return 84,
        Keycode::End => return 85,
        Keycode::PageUp => return 86,
        Keycode::PageDown => return 87,
        Keycode::F1 => return 88,
        Keycode::F2 => return 89,
        Keycode::F3 => return 90,
        Keycode::F4 => return 91,
        Keycode::F5 => return 92,
        Keycode::F6 => return 93,
        Keycode::F7 => return 94,
        Keycode::F8 => return 95,
        Keycode::F9 => return 96,
        Keycode::F10 => return 97,
        Keycode::F11 => return 98,
        Keycode::F12 => return 99,
        Keycode::F13 => return 100,
        Keycode::F14 => return 101,
        Keycode::F15 => return 102,
        Keycode::CapsLock => return 103,
        Keycode::RShift => return 104,
        Keycode::LShift => return 105,
        Keycode::RCtrl => return 106,
        Keycode::LCtrl => return 107,
        Keycode::RAlt => return 108,
        Keycode::LAlt => return 109,
        Keycode::Mode => return 110,
        Keycode::Help => return 111,
        Keycode::Sysreq => return 112,
        Keycode::Menu => return 113,
        Keycode::Power => return 114,
        Keycode::Undo => return 115,
        Keycode::Kp0 => return 116,
        Keycode::Kp1 => return 117,
        Keycode::Kp2 => return 118,
        Keycode::Kp3 => return 119,
        Keycode::Kp4 => return 120,
        Keycode::Kp5 => return 121,
        Keycode::Kp6 => return 122,
        Keycode::Kp7 => return 123,
        Keycode::Kp8 => return 124,
        Keycode::Kp9 => return 125,
        Keycode::PrintScreen => return 126,
        Keycode::NumLockClear => return 127,
        Keycode::ScrollLock => return 128,
        _ => return UNKNOWN_KEY as i32,
    }
}

///////////////////////////////////////////////////////////////////////////
//
//	INL_GetMouseButtons() - Gets the status of the mouse buttons from the
//		mouse driver
//
///////////////////////////////////////////////////////////////////////////

pub fn INL_GetMouseButtons(w3d: &mut modules) -> i32 {
    //println!("INL_GetMouseButtons");

    let event_pump = w3d.id_vl.sdl_context.event_pump().unwrap();
    let mut buttons = event_pump.mouse_state().to_sdl_state() as i32;

    let middlePressed = buttons & SDL_BUTTON(SDL_BUTTON_MIDDLE);
    let rightPressed = buttons & SDL_BUTTON(SDL_BUTTON_RIGHT);

    buttons &= !(SDL_BUTTON(SDL_BUTTON_MIDDLE) | SDL_BUTTON(SDL_BUTTON_RIGHT));

    if middlePressed != 0 {
        buttons |= 1 << 2;
    }
    if rightPressed != 0 {
        buttons |= 1 << 1;
    }

    return buttons;
}

///////////////////////////////////////////////////////////////////////////
//
//	IN_GetJoyDelta() - Returns the relative movement of the specified
//		joystick (from +/-127)
//
///////////////////////////////////////////////////////////////////////////

pub fn IN_GetJoyDelta(w3d: &mut modules, dx: &mut i16, dy: &mut i16) {
    //println!("IN_GetJoyDelta");

    if !w3d.id_in.Joystick {
        *dx = 0;
        *dy = 0;
        return;
    }

    w3d.id_vl.joystick.update();

    match &w3d.id_vl.Joystick {
        Ok(joystick) => {
            let mut x = joystick.axis(0).unwrap() >> 8;
            let mut y = joystick.axis(1).unwrap() >> 8;

            if w3d.wl_main.param_joystickhat != -1 {
                let hatState = joystick.hat(w3d.wl_main.param_joystickhat as u32).unwrap();

                if (hatState as u8) & (HatState::Right as u8) != 0 {
                    x += 127;
                } else if (hatState as u8) & (HatState::Left as u8) != 0 {
                    x -= 127;
                }
                if (hatState as u8) & (HatState::Down as u8) != 0 {
                    y += 127;
                } else if (hatState as u8) & (HatState::Up as u8) != 0 {
                    y -= 127;
                }

                if x < -128 {
                    x = -128;
                } else if x > 127 {
                    x = 127;
                }

                if y < -128 {
                    y = -128;
                } else if y > 127 {
                    y = 127;
                }
            }

            if x < -128 {
                x = -128;
            } else if x > 127 {
                x = 127;
            }

            if y < -128 {
                y = -128;
            } else if y > 127 {
                y = 127;
            }

            *dx = x;
            *dy = y;
        }
        Err(_e) => {
            return;
        }
    }
}

/*
===================
=
= IN_JoyButtons
=
===================
*/

pub fn IN_JoyButtons(w3d: &mut modules) -> i32 {
    //println!("IN_JoyButtons");

    if !w3d.id_in.Joystick {
        return 0;
    }

    w3d.id_vl.joystick.update();

    let mut res = 0;

    match &w3d.id_vl.Joystick {
        Ok(joystick) => {
            for i in 0..w3d.id_in.JoyNumButtons {
                res |= (joystick.button(i as u32).unwrap() as u32) << (i as u32);

                if i >= 32 {
                    break;
                }
            }
        }
        Err(_e) => {
            return 0;
        }
    }
    return res as i32;
}

pub fn IN_JoyPresent(w3d: &mut modules) -> bool {
    //println!("IN_JoyPresent");

    let available = w3d.id_vl.joystick.num_joysticks().unwrap();

    if available != 0 {
        w3d.id_in.Joystick = true;
        return true;
    } else {
        w3d.id_in.Joystick = false;
        return false;
    }
}

pub fn processEvent(w3d: &mut modules, event: Event) {
    //println!("processEvent");

    match event {
        // exit if the window is closed
        Event::Quit { .. } => {
            Quit("Achtung !!!");
        }
        // check for keypresses
        Event::KeyDown {
            scancode: Some(Scancode::ScrollLock),
            ..
        } => {
            w3d.id_in.GrabInput = !w3d.id_in.GrabInput;

            if w3d.id_in.GrabInput {
                w3d.id_vl.mouse.set_relative_mouse_mode(true);
            } else {
                w3d.id_vl.mouse.set_relative_mouse_mode(false);
            }

            return;
        }
        Event::KeyDown {
            scancode: Some(Scancode::F12),
            ..
        } => {
            w3d.id_in.GrabInput = !w3d.id_in.GrabInput;

            if w3d.id_in.GrabInput {
                w3d.id_vl.mouse.set_relative_mouse_mode(true);
            } else {
                w3d.id_vl.mouse.set_relative_mouse_mode(false);
            }

            return;
        }

        Event::KeyDown {
            scancode: Some(scancode),
            ..
        } => {
            w3d.id_in.LastScan = scancode;
            let mode = w3d.id_vl.keyboard.mod_state();

            if Keyboard(w3d, Scancode::LAlt) {
                if w3d.id_in.LastScan == Scancode::F4 {
                    Quit("Achtung !!!");
                }
            }

            if w3d.id_in.LastScan == Scancode::KpEnter {
                w3d.id_in.LastScan = Scancode::Return;
            } else if w3d.id_in.LastScan == Scancode::RShift {
                w3d.id_in.LastScan = Scancode::LShift;
            } else if w3d.id_in.LastScan == Scancode::RAlt {
                w3d.id_in.LastScan = Scancode::LAlt;
            } else if w3d.id_in.LastScan == Scancode::RCtrl {
                w3d.id_in.LastScan = Scancode::LCtrl;
            } else {
                if (mode.bits() & Mod::NUMMOD.bits()) == 0 {
                    match w3d.id_in.LastScan {
                        Scancode::Kp2 => {
                            w3d.id_in.LastScan = Scancode::Down;
                        }
                        Scancode::Kp4 => {
                            w3d.id_in.LastScan = Scancode::Left;
                        }
                        Scancode::Kp6 => {
                            w3d.id_in.LastScan = Scancode::Right;
                        }
                        Scancode::Kp8 => {
                            w3d.id_in.LastScan = Scancode::Up;
                        }
                        _ => (),
                    }
                }
            }

            let sym = w3d.id_in.LastScan;

            let key = Keycode::from_scancode(sym);

            match key {
                Some(key) => {
                    if (key as i32) < 128 {
                        w3d.id_in.LastASCII = key;
                    }
                }
                None => {
                    w3d.id_in.LastASCII = Keycode::F24;
                }
            }

            let intLastScan = w3d.id_in.LastScan;
            KeyboardSet(w3d, intLastScan, true);

            if w3d.id_in.LastScan == Scancode::Pause {
                w3d.id_in.Paused = true;
            }
        }
        Event::KeyUp {
            scancode: Some(scancode),
            ..
        } => {
            let mut sym = scancode;
            let mode = w3d.id_vl.keyboard.mod_state();

            if sym == Scancode::KpEnter {
                sym = Scancode::Return;
            } else if sym == Scancode::RShift {
                sym = Scancode::LShift;
            } else if sym == Scancode::RAlt {
                sym = Scancode::LAlt;
            } else if sym == Scancode::RCtrl {
                sym = Scancode::LCtrl;
            } else {
                if (mode.bits() & Mod::NUMMOD.bits()) == 0 {
                    match sym {
                        Scancode::Kp2 => {
                            sym = Scancode::Down;
                        }
                        Scancode::Kp4 => {
                            sym = Scancode::Left;
                        }
                        Scancode::Kp6 => {
                            sym = Scancode::Right;
                        }
                        Scancode::Kp8 => {
                            sym = Scancode::Up;
                        }
                        _ => (),
                    }
                }
            }
            KeyboardSet(w3d, sym, false);
        }
        _ => (),
    }
}

pub fn IN_WaitAndProcessEvents(w3d: &mut modules) {
    //println!("IN_WaitAndProcessEvents");

    let mut event_pump = w3d.id_vl.sdl_context.event_pump().unwrap();
    let event = event_pump.wait_event_timeout(1000);

    match event {
        Some(event) => processEvent(w3d, event),
        None => (),
    }
}

pub fn IN_ProcessEvents(w3d: &mut modules) {
    //println!("IN_ProcessEvents");

    let mut event_pump = w3d.id_vl.sdl_context.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            _ => {
                processEvent(w3d, event);
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//	IN_Startup() - Starts up the Input Mgr
//
///////////////////////////////////////////////////////////////////////////

//pub static mut Joystick: Joystick;

pub fn IN_Startup(w3d: &mut modules) {
    //println!("IN_Startup");

    if w3d.id_in.IN_Started {
        return;
    }

    IN_ClearKeysDown(w3d);

    let numJoysticks = w3d.id_vl.joystick.num_joysticks().unwrap();

    //update Joystick based on param_joystickindex
    let Joystick = w3d
        .id_vl
        .joystick
        .open(w3d.wl_main.param_joystickindex as u32);
    w3d.id_vl.Joystick = Joystick;

    if w3d.wl_main.param_joystickindex >= 0 && w3d.wl_main.param_joystickindex < numJoysticks as i32
    {
        match &w3d.id_vl.Joystick {
            Ok(joystick) => {
                w3d.id_in.Joystick = true;

                w3d.id_in.JoyNumButtons = joystick.num_buttons() as i32;

                if w3d.id_in.JoyNumButtons > 32 {
                    w3d.id_in.JoyNumButtons = 32; // only up to 32 buttons are supported
                }
                w3d.id_in.JoyNumHats = joystick.num_hats() as i32;

                if w3d.wl_main.param_joystickhat < -1
                    || w3d.wl_main.param_joystickhat >= w3d.id_in.JoyNumHats
                {
                    Quit("The joystickhat param must be between 0 and JoyNumHats - 1!");
                }
            }
            Err(_e) => {
                w3d.id_in.Joystick = false;
            }
        }
    }

    let mut event_pump = w3d.id_vl.sdl_context.event_pump().unwrap();
    event_pump.disable_event(EventType::MouseMotion);

    if w3d.id_vl.fullscreen || w3d.id_in.forcegrabmouse {
        w3d.id_in.GrabInput = true;
        w3d.id_vl.mouse.set_relative_mouse_mode(true);
    }

    // I didn't find a way to ask libSDL whether a mouse is present, yet...
    w3d.id_in.MousePresent = true;

    w3d.id_in.IN_Started = true;
}

///////////////////////////////////////////////////////////////////////////
//
//	IN_ClearKeysDown() - Clears the keyboard array
//
///////////////////////////////////////////////////////////////////////////

pub fn IN_ClearKeysDown(w3d: &mut modules) {
    //println!("IN_ClearKeysDown");

    w3d.id_in.LastScan = Scancode::F24; // BUG should be sc_None
    w3d.id_in.LastASCII = Keycode::F24; // BUG should be key_None

    w3d.id_in.KeyboardState = [false; 129];
}

///////////////////////////////////////////////////////////////////////////
//
//	IN_ReadControl() - Reads the device associated with the specified
//		player and fills in the control info struct
//
///////////////////////////////////////////////////////////////////////////

pub fn IN_ReadControl(w3d: &mut modules, _player: i32, info: &mut ControlInfo) {
    //println!("IN_ReadControl");

    let mut buttons: u16;
    let dx: i32;
    let dy: i32;
    let mut mx: Motion_x;
    let mut my: Motion_y;

    mx = Motion_x::motion_None;
    my = Motion_y::motion_None;
    buttons = 0;

    IN_ProcessEvents(w3d);

    if Keyboard(w3d, w3d.id_in.KbdDefs.upleft) {
        mx = Motion_x::motion_Left;
        my = Motion_y::motion_Up;
    } else if Keyboard(w3d, w3d.id_in.KbdDefs.upright) {
        mx = Motion_x::motion_Right;
        my = Motion_y::motion_Up;
    } else if Keyboard(w3d, w3d.id_in.KbdDefs.downleft) {
        mx = Motion_x::motion_Left;
        my = Motion_y::motion_Down;
    } else if Keyboard(w3d, w3d.id_in.KbdDefs.downright) {
        mx = Motion_x::motion_Right;
        my = Motion_y::motion_Down;
    }

    if Keyboard(w3d, w3d.id_in.KbdDefs.up) {
        my = Motion_y::motion_Up;
    } else if Keyboard(w3d, w3d.id_in.KbdDefs.down) {
        my = Motion_y::motion_Down;
    }

    if Keyboard(w3d, w3d.id_in.KbdDefs.left) {
        mx = Motion_x::motion_Left;
    } else if Keyboard(w3d, w3d.id_in.KbdDefs.right) {
        mx = Motion_x::motion_Right;
    }

    if Keyboard(w3d, w3d.id_in.KbdDefs.button0) {
        buttons += 1 << 0;
    }
    if Keyboard(w3d, w3d.id_in.KbdDefs.button1) {
        buttons += 1 << 1;
    }

    dx = mx as i32 * 127;
    dy = my as i32 * 127;

    info.x = dx;
    info.xaxis = mx;
    info.y = dy;
    info.yaxis = my;
    info.button0 = (buttons & (1 << 0)) as u8;
    info.button1 = (buttons & (1 << 1)) as u8;
    info.button2 = (buttons & (1 << 2)) as u8;
    info.button3 = (buttons & (1 << 3)) as u8;
    info.dir = DirTable[(((my as i32 + 1) * 3) + (mx as i32 + 1)) as usize];
}

///////////////////////////////////////////////////////////////////////////
//
//	IN_Ack() - waits for a button or key press.  If a button is down, upon
// calling, it must be released for it to be recognized
//
///////////////////////////////////////////////////////////////////////////

pub fn IN_StartAck(w3d: &mut modules) {
    //println!("IN_StartAck");

    IN_ProcessEvents(w3d);
    //
    // get initial state of everything
    //
    IN_ClearKeysDown(w3d);

    let mut buttons = IN_JoyButtons(w3d) << 4;

    if w3d.id_in.MousePresent {
        buttons |= IN_MouseButtons(w3d);
    }

    for i in 0..buttontype::NUMBUTTONS as usize {
        buttons >>= 1;

        if (buttons & 1) == 1 {
            w3d.id_in.btnstate[i] = true;
        }
    }
}

pub fn IN_CheckAck(w3d: &mut modules) -> bool {
    //println!("IN_CheckAck");

    IN_ProcessEvents(w3d);
    //
    // see if something has been pressed
    //
    if w3d.id_in.LastScan != Scancode::F24 {
        return true;
    }

    let mut buttons = IN_JoyButtons(w3d) << 4;

    if w3d.id_in.MousePresent {
        buttons |= IN_MouseButtons(w3d);
    }

    for i in 0..buttontype::NUMBUTTONS as usize {
        buttons >>= 1;

        if buttons & 1 == 1 {
            if !w3d.id_in.btnstate[i] {
                // Wait until button has been released
                IN_WaitAndProcessEvents(w3d);
                loop {
                    buttons = IN_JoyButtons(w3d) << 4;

                    if w3d.id_in.MousePresent {
                        buttons |= IN_MouseButtons(w3d);
                    }
                    if buttons & (1 << i) != 1 {
                        break;
                    }
                }

                return true;
            }
        } else {
            w3d.id_in.btnstate[i] = false;
        }
    }

    return false;
}

pub fn IN_Ack(w3d: &mut modules) {
    //println!("IN_Ack");

    IN_StartAck(w3d);

    loop {
        IN_WaitAndProcessEvents(w3d);

        if IN_CheckAck(w3d) {
            break;
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//	IN_UserInput() - Waits for the specified delay time (in ticks) or the
//		user pressing a key or a mouse button. If the clear flag is set, it
//		then either clears the key or waits for the user to let the mouse
//		button up.
//
///////////////////////////////////////////////////////////////////////////

pub fn IN_UserInput(w3d: &mut modules, delay: i32) -> bool {
    //println!("IN_UserInput");

    let lasttime = GetTimeCount(w3d);

    IN_StartAck(w3d);
    loop {
        IN_ProcessEvents(w3d);
        if IN_CheckAck(w3d) {
            return true;
        }
        SDL_Delay(w3d, 5);

        if (GetTimeCount(w3d) - lasttime) >= delay {
            break;
        }
    }
    return false;
}

/*
===================
=
= IN_MouseButtons
=
===================
*/

pub fn IN_MouseButtons(w3d: &mut modules) -> i32 {
    //println!("IN_MouseButtons");

    if w3d.id_in.MousePresent {
        return INL_GetMouseButtons(w3d);
    } else {
        return 0;
    }
}

pub fn IN_IsInputGrabbed(w3d: &mut modules) -> bool {
    //println!("IN_IsInputGrabbed");

    return w3d.id_in.GrabInput;
}

pub fn IN_CenterMouse(_w3d: &mut modules) {
    //println!("IN_CenterMouse");
}
