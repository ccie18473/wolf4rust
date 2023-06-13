#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  id_vh
//
//===========================================================================

pub struct id_vh {
    pub pictable: Vec<pictabletype>,
    pub pictable_bytes: Vec<u8>,

    pub px: i32,
    pub py: i32,
    pub fontcolor: i32,
    pub backcolor: i32,
    pub fontnumber: i32,
    pub rndbits_y: u32,
    pub rndmask: u32,
}

impl id_vh {
    pub fn new() -> Self {
        Self {
            pictable: Vec::new(),
            pictable_bytes: Vec::new(),
            px: 0,
            py: 0,
            fontcolor: 0,
            backcolor: 0,
            fontnumber: 0,
            rndbits_y: 0,
            rndmask: 0,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

#[derive(Copy, Clone)]
pub struct pictabletype {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fontstruct {
    pub height: u16,
    #[serde(with = "BigArray")]
    pub location: [u16; 256],
    #[serde(with = "BigArray")]
    pub width: [u8; 256],
}

// graphics mode independant colors
pub const BLACK: i32 = 0;

pub fn SETFONTCOLOR(w3d: &mut modules, f: i32, b: i32) {
    //println!("VWB_BarScaledCoord");

    w3d.id_vh.fontcolor = f;
    w3d.id_vh.backcolor = b;
}

pub fn VWB_BarScaledCoord(w3d: &mut modules, x: i32, y: i32, width: i32, height: i32, color: i32) {
    //println!("VWB_BarScaledCoord");

    VL_BarScaledCoord(w3d, x, y, width, height, color);
}

/*
=============================================================================

                Double buffer management routines

=============================================================================
*/

pub fn VW_UpdateScreen(w3d: &mut modules) {
    //println!("VW_UpdateScreen");

    VH_UpdateScreen(w3d);
}

pub fn VW_Bar(w3d: &mut modules, x: i32, y: i32, width: i32, height: i32, color: i32) {
    //println!("VW_Bar");

    VL_Bar(w3d, x, y, width, height, color);
}

pub fn VW_Hlin(w3d: &mut modules, x: i32, z: i32, y: i32, c: i32) {
    //println!("VW_Hlin");

    VL_Hlin(w3d, x, y, (z) - (x) + 1, c);
}

pub fn VW_Vlin(w3d: &mut modules, y: i32, z: i32, x: i32, c: i32) {
    //println!("VW_Vlin");

    VL_Vlin(w3d, x, y, (z) - (y) + 1, c);
}

pub fn VW_WaitVBL(w3d: &mut modules, delay: i32) {
    //println!("VW_WaitVBL");

    VL_WaitVBL(w3d, delay);
}

pub fn VW_FadeIn(w3d: &mut modules) {
    //println!("VW_FadeIn");

    VL_FadeIn(w3d, 0, 255, w3d.id_vl.gamepal, 30);
}

pub fn VW_FadeOut(w3d: &mut modules) {
    //println!("VW_FadeOut");

    VL_FadeOut(w3d, 0, 255, 0, 0, 0, 30);
}

pub fn VWB_DrawPropString(w3d: &mut modules, s: String) {
    //println!("VWB_DrawPropString");

    let _font: fontstruct;
    let mut width: i32 = 0;
    let mut step: i32 = 0;
    let height: i32;
    let mut source: Vec<u8> = Vec::new();
    let _ch: i32;
    let mut source_index: usize = 0;
    let mut dest_index: usize = 0;

    let font_bytes = &w3d.id_ca.grsegs[(STARTFONT + w3d.id_vh.fontnumber) as usize];

    let font_height = (font_bytes[0] as u16) + ((font_bytes[1] as u16) << 8);
    let mut font_location: [u16; 256] = [0; 256];
    for i in 0..256 {
        font_location[i] = font_bytes[2 + i * 2] as u16 + ((font_bytes[3 + i * 2] as u16) << 8);
    }
    let mut font_width: [u8; 256] = [0; 256];
    for i in 0..256 {
        font_width[i] = font_bytes[514 + i];
    }
    let font: fontstruct = fontstruct {
        height: font_height,
        location: font_location,
        width: font_width,
    };

    height = font.height as i32;

    dest_index += w3d.id_vl.scaleFactor as usize
        * (w3d.id_vl.ylookup[w3d.id_vh.py as usize] + w3d.id_vh.px as usize);

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        for i in 0..s.len() {
            let ch = s.as_bytes()[i];
            width = font.width[ch as usize] as i32;
            step = width;
            source = font_bytes.to_vec();
            source_index += font.location[ch as usize] as usize;

            while width > 0 {
                for i in 0..height {
                    if source[source_index + (i * step) as usize] != 0 {
                        for sy in 0..w3d.id_vl.scaleFactor {
                            for sx in 0..w3d.id_vl.scaleFactor {
                                dest[dest_index
                                    + w3d.id_vl.ylookup
                                        [(w3d.id_vl.scaleFactor * i + sy) as usize]
                                    + sx as usize] = w3d.id_vh.fontcolor as u8;
                            }
                        }
                    }
                }

                source_index += 1;
                w3d.id_vh.px += 1;
                dest_index += w3d.id_vl.scaleFactor as usize;

                width -= 1;
            }

            source_index = 0;
        }
    });
}

pub fn VWL_MeasureString(
    _w3d: &mut modules,
    string: String,
    width: &mut i32,
    height: &mut i32,
    font: fontstruct,
) {
    //println!("VWL_MeasureString");

    *height = font.height as i32;
    *width = 0;

    for i in 0..string.len() {
        let ch = string.as_bytes()[i];
        *width += font.width[ch as usize] as i32; // proportional width
    }
}

pub fn VW_MeasurePropString(w3d: &mut modules, string: String, width: &mut i32, height: &mut i32) {
    //println!("VW_MeasurePropString");

    let font_bytes = &w3d.id_ca.grsegs[(STARTFONT + w3d.id_vh.fontnumber) as usize];

    let font_height = (font_bytes[0] as u16) + ((font_bytes[1] as u16) << 8);
    let mut font_location: [u16; 256] = [0; 256];
    for i in 0..256 {
        font_location[i] = font_bytes[2 + i * 2] as u16 + ((font_bytes[3 + i * 2] as u16) << 8);
    }
    let mut font_width: [u8; 256] = [0; 256];
    for i in 0..256 {
        font_width[i] = font_bytes[514 + i];
    }
    let font: fontstruct = fontstruct {
        height: font_height,
        location: font_location,
        width: font_width,
    };

    VWL_MeasureString(w3d, string, width, height, font);
}

/*
=============================================================================

                Double buffer management routines

=============================================================================
*/

pub fn VH_UpdateScreen(w3d: &mut modules) {
    //println!("VH_UpdateScreen");

    w3d.id_vl
        .screenBuffer
        .blit(None, &mut w3d.id_vl.screen, None)
        .unwrap();

    Present(w3d);
}

pub fn VWB_DrawPic(w3d: &mut modules, mut x: i32, y: i32, chunknum: i32) {
    //println!("VWB_DrawPic");

    let picnum: i32 = chunknum - STARTPICS as i32;
    let width: i32;
    let height: i32;

    x &= !7;

    width = w3d.id_vh.pictable[picnum as usize].width as i32;
    height = w3d.id_vh.pictable[picnum as usize].height as i32;

    let grsegs = w3d.id_ca.grsegs[chunknum as usize].clone();

    VL_MemToScreen(w3d, &grsegs, width, height, x, y);
}

pub fn VWB_DrawPicScaledCoord(w3d: &mut modules, scx: i32, scy: i32, chunknum: i32) {
    //println!("VWB_DrawPicScaledCoord");

    let picnum = chunknum - STARTPICS as i32;
    let width: i32;
    let height: i32;

    width = w3d.id_vh.pictable[picnum as usize].width as i32;
    height = w3d.id_vh.pictable[picnum as usize].height as i32;

    let grsegs = w3d.id_ca.grsegs[chunknum as usize].clone();

    VL_MemToScreenScaledCoord(w3d, &grsegs, width, height, scx, scy);
}

pub fn VWB_Bar(w3d: &mut modules, x: i32, y: i32, width: i32, height: i32, color: i32) {
    //println!("VWB_Bar");

    VW_Bar(w3d, x, y, width, height, color);
}

pub fn VWB_Hlin(w3d: &mut modules, x1: i32, x2: i32, y: i32, color: i32) {
    //println!("VWB_Hlin");

    if w3d.id_vl.scaleFactor == 1 {
        VW_Hlin(w3d, x1, x2, y, color);
    } else {
        VW_Bar(w3d, x1, y, x2 - x1 + 1, 1, color);
    }
}

pub fn VWB_Vlin(w3d: &mut modules, y1: i32, y2: i32, x: i32, color: i32) {
    //println!("VWB_Vlin");

    if w3d.id_vl.scaleFactor == 1 {
        VW_Vlin(w3d, y1, y2, x, color);
    } else {
        VW_Bar(w3d, x, y1, 1, y2 - y1 + 1, color);
    }
}

/*
=============================================================================

                        WOLFENSTEIN STUFF

=============================================================================
*/

/*
===================
=
= FizzleFade
=
= returns true if aborted
=
= It uses maximum-length Linear Feedback Shift Registers (LFSR) counters.
= You can find a list of them with lengths from 3 to 168 at:
= http://www.xilinx.com/support/documentation/application_notes/xapp052.pdf
= Many thanks to Xilinx for this list!!!
=
===================
*/

// XOR masks for the pseudo-random number sequence starting with n=17 bits
pub const rndmasks: [u32; 9] = [
    // n    XNOR from (starting at 1, not 0 as usual)
    0x00012000, // 17   17,14
    0x00020400, // 18   18,11
    0x00040023, // 19   19,6,2,1
    0x00090000, // 20   20,17
    0x00140000, // 21   21,19
    0x00300000, // 22   22,21
    0x00420000, // 23   23,18
    0x00e10000, // 24   24,23,22,17
    0x01200000, // 25   25,22      (this is enough for 8191x4095)
];

pub fn log2_ceil(x: u32) -> u32 {
    //println!("log2_ceil");

    let mut n: u32 = 0;
    let mut v: u32 = 1;

    while v < x {
        n += 1;
        v <<= 1;
    }
    return n;
}

pub fn VH_Startup(w3d: &mut modules) {
    //println!("VH_Startup");

    let rndbits_x = log2_ceil(w3d.id_vl.screenWidth as u32);
    w3d.id_vh.rndbits_y = log2_ceil(w3d.id_vl.screenHeight as u32);

    let mut rndbits = rndbits_x + w3d.id_vh.rndbits_y as u32;
    if rndbits < 17 {
        rndbits = 17; // no problem, just a bit slower
    } else if rndbits > 25 {
        rndbits = 25; // fizzle fade will not fill whole screen
    }

    w3d.id_vh.rndmask = rndmasks[(rndbits - 17) as usize];
}

pub fn FizzleFade(
    w3d: &mut modules,
    //source: &mut Surface, //screenbuffer
    x1: i32,
    y1: i32,
    width: i32,
    height: i32,
    frames: i32,
    abortable: bool,
) -> bool {
    //println!("FizzleFade");

    let mut x: i32;
    let mut y: i32;
    let mut frame: i32;
    let pixperframe: i32;
    let mut rndval: i32 = 0;
    let mut lastrndval: i32;
    let mut first: i32 = 1;

    lastrndval = 0;
    pixperframe = width * height / frames;

    IN_StartAck(w3d);

    frame = GetTimeCount(w3d);
    let srcptr = VL_LockSurface(&mut w3d.id_vl.screenBuffer);

    if srcptr.is_empty() {
        return false;
    }

    'label: loop {
        IN_ProcessEvents(w3d);

        if abortable && IN_CheckAck(w3d) {
            VL_UnlockSurface(&mut w3d.id_vl.screenBuffer);
            VH_UpdateScreen(w3d);
            return true;
        }

        let mut destptr = VL_LockSurface(&mut w3d.id_vl.screen);

        if !destptr.is_empty() {
            rndval = lastrndval;

            // When using double buffering, we have to copy the pixels of the last AND the current frame.
            // Only for the first frame, there is no "last frame"

            for i in first..2 {
                let mut p = 0;
                while p < pixperframe {
                    //
                    // seperate random value into x/y pair
                    //

                    x = rndval >> w3d.id_vh.rndbits_y;
                    y = rndval & ((1 << w3d.id_vh.rndbits_y) - 1);

                    //
                    // advance to next random element
                    //

                    let value: i32;
                    if (rndval & 1) != 0 {
                        value = 0;
                    } else {
                        value = w3d.id_vh.rndmask as i32;
                    }
                    rndval = (rndval >> 1) ^ value;

                    if x >= width || y >= height {
                        if rndval == 0
                        // entire sequence has been completed
                        {
                            //goto finished;
                            break 'label;
                        }
                        continue;
                    }

                    //
                    // copy one pixel
                    //
                    let screen_pitch = w3d.id_vl.screen.pitch();
                    let screen_buffer_pitch = w3d.id_vl.screenBuffer.pitch();

                    if w3d.id_vl.screenBits == 8 {
                        destptr[((y1 + y) * screen_pitch as i32 + x1 + x) as usize] =
                            srcptr[((y1 + y) * screen_buffer_pitch as i32 + x1 + x) as usize];
                    } else {
                        let col = srcptr[((y1 + y) * screen_buffer_pitch as i32 + x1 + x) as usize];

                        let pixel = Color::RGB(
                            w3d.id_vl.curpal[col as usize].r,
                            w3d.id_vl.curpal[col as usize].g,
                            w3d.id_vl.curpal[col as usize].b,
                        );

                        let pixel_format = w3d.id_vl.screen.pixel_format();

                        let fullcol = pixel.to_u32(&pixel_format);
                        let fullcol1 = fullcol as u8;
                        let fullcol2 = (fullcol >> 8) as u8;
                        let fullcol3 = (fullcol >> 16) as u8;
                        let fullcol4 = (fullcol >> 24) as u8;

                        let bytes = w3d.id_vl.screen.pixel_format_enum().byte_size_per_pixel();

                        //destptr[(y1 + y) as usize * screen_pitch as usize
                        //    + (x1 + x) as usize * bytes] = fullcol as u8;

                        w3d.id_vl.screen.with_lock_mut(|dest: &mut [u8]| {
                            let offset = (y1 + y) as usize * screen_pitch as usize
                                + (x1 + x) as usize * bytes;
                            dest[offset] = fullcol1;
                            dest[offset + 1] = fullcol2;
                            dest[offset + 2] = fullcol3;
                            dest[offset + 3] = fullcol4;
                        });
                    }

                    if rndval == 0
                    // entire sequence has been completed
                    {
                        //goto finished;
                        break 'label;
                    }
                    p += 1;
                }

                if i == 0 || first != 0 {
                    lastrndval = rndval;
                }
            }

            // If there is no double buffering, we always use the "first frame" case
            if w3d.id_vl.usedoublebuffering {
                first = 0;
            }

            VL_UnlockSurface(&mut w3d.id_vl.screen);
            Present(w3d);
        } else {
            // No surface, so only enhance rndval
            for _i in first..2 {
                for _p in 0..pixperframe {
                    let value: i32;
                    if (rndval & 1) != 0 {
                        value = 0;
                    } else {
                        value = w3d.id_vh.rndmask as i32;
                    }
                    rndval = (rndval >> 1) ^ value;
                    if rndval == 0 {
                        //goto finished;
                        break 'label;
                    }
                }
            }
        }

        frame += 1;
        let time = GetTimeCount(w3d);
        Delay(w3d, frame - time); // don't go too fast
    }

    //finished:

    VL_UnlockSurface(&mut w3d.id_vl.screenBuffer);
    VL_UnlockSurface(&mut w3d.id_vl.screen);
    VH_UpdateScreen(w3d);

    return false;
}
