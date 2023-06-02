#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  id_us
//
//===========================================================================

pub struct id_us {
    pub PrintX: i32,
    pub PrintY: i32,
    pub PrintX_cur: i32,
    pub PrintY_cur: i32,
    pub WindowX: i32,
    pub WindowY: i32,
    pub WindowW: i32,
    pub WindowH: i32,
    pub US_Started: bool,
    pub Games: [SaveGame; MaxSaveGames],
    pub Scores: [HighScore; MaxScores],
    pub rndindex: usize,
    pub status: u8,
}

impl id_us {
    pub fn new() -> Self {
        Self {
            PrintX: 0,
            PrintY: 0,
            PrintX_cur: 0,
            PrintY_cur: 0,
            WindowX: 0,
            WindowY: 0,
            WindowW: 0,
            WindowH: 0,
            US_Started: false,
            Games: [
                SaveGame {
                    signature: "".to_string(),
                    oldtest: 0,
                    present: false,
                    name: "".to_string(),
                },
                SaveGame {
                    signature: "".to_string(),
                    oldtest: 0,
                    present: false,
                    name: "".to_string(),
                },
                SaveGame {
                    signature: "".to_string(),
                    oldtest: 0,
                    present: false,
                    name: "".to_string(),
                },
                SaveGame {
                    signature: "".to_string(),
                    oldtest: 0,
                    present: false,
                    name: "".to_string(),
                },
                SaveGame {
                    signature: "".to_string(),
                    oldtest: 0,
                    present: false,
                    name: "".to_string(),
                },
                SaveGame {
                    signature: "".to_string(),
                    oldtest: 0,
                    present: false,
                    name: "".to_string(),
                },
            ],
            Scores: [
                HighScore {
                    name: "id software-'92".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
                HighScore {
                    name: "Adrian Carmack".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
                HighScore {
                    name: "John Carmack".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
                HighScore {
                    name: "Kevin Cloud".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
                HighScore {
                    name: "Tom Hall".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
                HighScore {
                    name: "John Romero".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
                HighScore {
                    name: "Jay Wilbur".to_string(),
                    score: 10000,
                    completed: 1,
                    episode: 0,
                },
            ],
            rndindex: 0,
            status: 0,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const MaxHighName: usize = 57;
pub const MaxScores: usize = 7;

pub const MaxSaveGames: usize = 6;

pub const MaxString: usize = 128; // Maximum input string size
pub struct HighScore {
    pub name: String,
    pub score: i32,
    pub completed: i32,
    pub episode: i32,
}
impl HighScore {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            score: 0,
            completed: 0,
            episode: 0,
        }
    }
}
pub struct SaveGame {
    pub signature: String,
    pub oldtest: usize,
    pub present: bool,
    pub name: String,
}

pub static rndtable: [i32; 256] = [
    0, 8, 109, 220, 222, 241, 149, 107, 75, 248, 254, 140, 16, 66, 74, 21, 211, 47, 80, 242, 154,
    27, 205, 128, 161, 89, 77, 36, 95, 110, 85, 48, 212, 140, 211, 249, 22, 79, 200, 50, 28, 188,
    52, 140, 202, 120, 68, 145, 62, 70, 184, 190, 91, 197, 152, 224, 149, 104, 25, 178, 252, 182,
    202, 182, 141, 197, 4, 81, 181, 242, 145, 42, 39, 227, 156, 198, 225, 193, 219, 93, 122, 175,
    249, 0, 175, 143, 70, 239, 46, 246, 163, 53, 163, 109, 168, 135, 2, 235, 25, 92, 20, 145, 138,
    77, 69, 166, 78, 176, 173, 212, 166, 113, 94, 161, 41, 50, 239, 49, 111, 164, 70, 60, 2, 37,
    171, 75, 136, 156, 11, 56, 42, 146, 138, 229, 73, 146, 77, 61, 98, 196, 135, 106, 63, 197, 195,
    86, 96, 203, 113, 101, 170, 247, 181, 113, 80, 250, 108, 7, 255, 237, 129, 226, 79, 107, 112,
    166, 103, 241, 24, 223, 239, 120, 198, 58, 60, 82, 128, 3, 184, 66, 143, 224, 145, 224, 81,
    206, 163, 45, 63, 90, 168, 114, 59, 33, 159, 95, 28, 139, 123, 98, 125, 196, 15, 70, 194, 253,
    54, 14, 109, 226, 71, 17, 161, 93, 186, 87, 244, 138, 20, 52, 123, 251, 26, 36, 17, 46, 52,
    231, 232, 76, 31, 221, 84, 37, 216, 165, 212, 106, 197, 242, 98, 43, 39, 175, 254, 145, 190,
    84, 118, 222, 187, 136, 120, 163, 236, 249,
];

pub fn USL_MeasureString(w3d: &mut modules, s: String, w: &mut i32, h: &mut i32) {
    //println!("USL_MeasureString");

    VW_MeasurePropString(w3d, s, w, h);
}

pub fn USL_DrawString(w3d: &mut modules, s: String) {
    //println!("USL_DrawString");

    VWB_DrawPropString(w3d, s.clone());
}

///////////////////////////////////////////////////////////////////////////
//
//	US_Startup() - Starts the User Mgr
//
///////////////////////////////////////////////////////////////////////////

pub fn US_Startup() {
    //println!("US_Startup");
}

///////////////////////////////////////////////////////////////////////////
//
//	US_Print() - Prints a string in the current window. Newlines are
//		supported.
//
///////////////////////////////////////////////////////////////////////////

pub fn US_Print(w3d: &mut modules, s: String) {
    //println!("US_Print");

    let mut w: i32 = 0;
    let mut h: i32 = 0;

    let strings: Split<&str> = s.split("\n");

    for string in strings {
        USL_MeasureString(w3d, string.to_string(), &mut w, &mut h);

        w3d.id_vh.px = w3d.id_us.PrintX;
        w3d.id_vh.py = w3d.id_us.PrintY;

        USL_DrawString(w3d, string.to_string());

        w3d.id_us.PrintY += h;
    }
    //BUG
    w3d.id_us.PrintY -= h;
    //position blining cursor
    w3d.id_us.PrintX_cur = w3d.id_us.PrintX + w;
    w3d.id_us.PrintY_cur = w3d.id_us.PrintY;
}

///////////////////////////////////////////////////////////////////////////
//
//	US_PrintSigned() - Prints a signed long
//
///////////////////////////////////////////////////////////////////////////

pub fn US_PrintSigned(w3d: &mut modules, n: i32) {
    //println!("US_PrintSigned");

    let buffer = n.to_string();

    US_Print(w3d, buffer);
}

///////////////////////////////////////////////////////////////////////////
//
//	US_CPrintLine() - Prints a string centered on the current line and
//		advances to the next line. Newlines are not supported.
//
///////////////////////////////////////////////////////////////////////////

pub fn US_CPrintLine(w3d: &mut modules, s: String) {
    //println!("US_CPrintLine");

    let mut w: i32 = 0;
    let mut h: i32 = 0;

    USL_MeasureString(w3d, s.clone(), &mut w, &mut h);

    w3d.id_vh.px = w3d.id_us.WindowX + ((w3d.id_us.WindowW - w) / 2);
    w3d.id_vh.py = w3d.id_us.PrintY;

    USL_DrawString(w3d, s.clone());

    w3d.id_us.PrintY += h;
}

///////////////////////////////////////////////////////////////////////////
//
//  US_CPrint() - Prints a string centered in the current window.
//      Newlines are supported.
//
///////////////////////////////////////////////////////////////////////////

pub fn US_CPrint(w3d: &mut modules, sorg: String) {
    //println!("US_CPrint");

    US_CPrintLine(w3d, sorg);
}

//	Input routines

///////////////////////////////////////////////////////////////////////////
//
//	USL_XORICursor() - XORs the I-bar text cursor. Used by US_LineInput()
//
///////////////////////////////////////////////////////////////////////////

pub fn USL_XORICursor(w3d: &mut modules, x: i32, y: i32, s: String, _cursor: i32) {
    //println!("USL_XORICursor");

    //static	boolean	status;		// VGA doesn't XOR...
    let mut buf: String;
    let temp: i32;
    let mut w: i32 = 0;
    let mut h: i32 = 0;

    buf = s.clone();
    buf.truncate(MaxString);
    USL_MeasureString(w3d, buf, &mut w, &mut h);

    w3d.id_vh.px = x + w - 1;
    w3d.id_vh.py = y;
    w3d.id_us.status ^= 1;
    if w3d.id_us.status != 0 {
        USL_DrawString(w3d, "\x5f".to_string());
    } else {
        temp = w3d.id_vh.fontcolor;
        w3d.id_vh.fontcolor = w3d.id_vh.backcolor;
        USL_DrawString(w3d, "\x5f".to_string());
        w3d.id_vh.fontcolor = temp;
    }
}

///////////////////////////////////////////////////////////////////////////
//
//	US_LineInput() - Gets a line of user input at (x,y), the string defaults
//		to whatever is pointed at by def. Input is restricted to maxchars
//		chars or maxwidth pixels wide. If the user hits escape (and escok is
//		true), nothing is copied into buf, and false is returned. If the
//		user hits return, the current string is copied into buf, and true is
//		returned
//
///////////////////////////////////////////////////////////////////////////

pub fn US_LineInput(
    w3d: &mut modules,
    x: i32,
    y: i32,
    buf: &mut String,
    def: String,
    escok: bool,
    maxchars: i32,
    maxwidth: i32,
) -> bool {
    //println!("US_LineInput");

    let mut redraw: bool;
    let mut cursorvis: bool;
    let mut cursormoved: bool;
    let mut done: bool;
    let mut result: bool = false;
    let mut checkkey: bool;
    let mut sc: Scancode;
    let mut c: Keycode;
    let mut s: String; //[MaxString]
    let mut olds: String; //[MaxString];
    let mut cursor: i32;
    let mut len: i32;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut temp: i32;
    let mut curtime: i32;
    let mut lasttime: i32;
    let mut lastdirtime: i32;
    let mut lastbuttontime: i32;
    let mut lastdirmovetime: i32;
    let mut ci: ControlInfo = ControlInfo::new();
    let mut lastdir: Direction = Direction::dir_None;

    if def != String::new() {
        s = def.clone();
    } else {
        s = String::new();
    }
    olds = String::new();
    cursor = s.len() as i32;
    cursormoved = true;
    redraw = true;

    cursorvis = false;
    done = false;
    lasttime = GetTimeCount(w3d);
    lastdirtime = GetTimeCount(w3d);
    lastdirmovetime = GetTimeCount(w3d);
    lastbuttontime = lasttime + TickBase / 4; // 250 ms => first button press accepted after 500 ms
    w3d.id_in.LastASCII = Keycode::F24;
    w3d.id_in.LastScan = Scancode::F24;

    while !done {
        ReadAnyControl(w3d, &mut ci);

        if cursorvis {
            USL_XORICursor(w3d, x, y, s.clone(), cursor);
        }

        sc = w3d.id_in.LastScan;
        w3d.id_in.LastScan = Scancode::F24;
        c = w3d.id_in.LastASCII;
        w3d.id_in.LastASCII = Keycode::F24;

        checkkey = true;
        curtime = GetTimeCount(w3d);

        // After each direction change accept the next change after 250 ms and then everz 125 ms
        if ci.dir != lastdir
            || curtime - lastdirtime > TickBase / 4 && curtime - lastdirmovetime > TickBase / 8
        {
            if ci.dir != lastdir {
                lastdir = ci.dir;
                lastdirtime = curtime;
            }
            lastdirmovetime = curtime;

            match ci.dir {
                Direction::dir_West => {}
                //TODO
                Direction::dir_East => {
                    //TODO
                }

                Direction::dir_North => {
                    //TODO
                }

                Direction::dir_South => {
                    //TODO
                }
                _ => (),
            }
        }

        if (curtime - lastbuttontime) > TickBase / 4
        // 250 ms
        {
            if ci.button0 != 0
            // acts as return
            {
                *buf = s.clone();
                done = true;
                result = true;
                checkkey = false;
            }
            if ci.button1 != 0 && escok
            // acts as escape
            {
                done = true;
                result = false;
                checkkey = false;
            }
            if ci.button2 != 0
            // acts as backspace
            {
                lastbuttontime = curtime;
                if cursor != 0 {
                    //strcpy(s + cursor - 1, s + cursor);
                    s.pop();
                    cursor -= 1;
                    redraw = true;
                }
                cursormoved = true;
                checkkey = false;
            }
        }

        if checkkey {
            match sc
			{
				Scancode::Left => {
					//TODO
                }
				Scancode::Right  => {
					//TODO
                }
				Scancode::Home => {
					//TODO
                }
				Scancode::End => {
					//TODO
                }

				Scancode::Return => {
					*buf = s.clone();
					done = true;
					result = true;
					c = Keycode::F24;
                }
				Scancode::Escape => {
					if escok
					{
						done = true;
						result = false;
					}
					c = Keycode::F24;
                }

				Scancode::Backspace => {
					if cursor !=0
					{
						//strcpy(s + cursor - 1,s + cursor);
                        s.pop();
                        cursor -=1;
						redraw = true;
					}
					c = Keycode::F24;
					cursormoved = true;
                }
				Scancode::Delete => {
					//TODO
                }

				Scancode::Kp5 | //0x4c:	// Keypad 5 // TODO: hmmm...
				Scancode::Up |
				Scancode::Down |
				Scancode::PageUp |
				Scancode::PageDown |
				Scancode::Insert => {
					c = Keycode::F24;
                }
                _ => (),
			}

            if c != Keycode::F24 {
                let mut c = c as u8;
                len = s.len() as i32;
                USL_MeasureString(w3d, s.clone(), &mut w, &mut h);

                if c >= 32
                    && c <= 127
                    && (len < MaxString as i32 - 1)
                    && ((maxchars == 0) || (len < maxchars) && ((maxwidth == 0) || (w < maxwidth)))
                {
                    // convert to uppercase
                    let mode = w3d.id_vl.keyboard.mod_state();
                    if (mode.bits() & Mod::CAPSMOD.bits()) != 0
                        || (mode.bits() & Mod::LSHIFTMOD.bits()) != 0
                        || (mode.bits() & Mod::RSHIFTMOD.bits()) != 0
                    {
                        if (c >= 97) && (c <= 122) {
                            c -= 32;
                        }
                    }
                    s.push(c as char);
                    cursor += 1;
                    redraw = true;
                }
            }
        }

        if redraw {
            w3d.id_vh.px = x;
            w3d.id_vh.py = y;
            temp = w3d.id_vh.fontcolor;
            w3d.id_vh.fontcolor = w3d.id_vh.backcolor;
            USL_DrawString(w3d, olds);
            w3d.id_vh.fontcolor = temp;
            olds = s.clone();

            w3d.id_vh.px = x;
            w3d.id_vh.py = y;
            USL_DrawString(w3d, s.clone());

            redraw = false;
        }

        if cursormoved {
            cursorvis = false;
            lasttime = curtime - TickBase;

            cursormoved = false;
        }
        if curtime - lasttime > TickBase / 2
        // 500 ms
        {
            lasttime = curtime;

            cursorvis ^= true;
        } else {
            SDL_Delay(w3d, 5);
        }
        if cursorvis {
            USL_XORICursor(w3d, x, y, s.clone(), cursor);
        }

        VW_UpdateScreen(w3d);
    }

    if cursorvis {
        USL_XORICursor(w3d, x, y, s.clone(), cursor);
    }
    if !result {
        w3d.id_vh.px = x;
        w3d.id_vh.py = y;
        USL_DrawString(w3d, olds);
    }
    VW_UpdateScreen(w3d);

    IN_ClearKeysDown(w3d);

    return result;
}

///////////////////////////////////////////////////////////////////////////
//
// US_InitRndT - Initializes the pseudo random number generator.
//      If randomize is true, the seed will be initialized depending on the
//      current time
//
///////////////////////////////////////////////////////////////////////////

pub fn US_InitRndT(w3d: &mut modules, randomize: bool) {
    //println!("US_InitRndT");

    let timer = w3d.id_vl.sdl_context.timer().unwrap();

    if randomize {
        w3d.id_us.rndindex = (timer.ticks() as usize >> 4) & 0xff;
    } else {
        w3d.id_us.rndindex = 0;
    }
}

///////////////////////////////////////////////////////////////////////////
//
// US_RndT - Returns the next 8-bit pseudo random number
//
///////////////////////////////////////////////////////////////////////////

pub fn US_RndT(w3d: &mut modules) -> i32 {
    //println!("US_RndT");

    w3d.id_us.rndindex = (w3d.id_us.rndindex + 1) & 0xff;

    return rndtable[w3d.id_us.rndindex as usize];
}
