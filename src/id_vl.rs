#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use sdl2::TimerSubsystem;

use crate::*;

//===========================================================================
//
//  id_vl
//
//===========================================================================

pub struct id_vl<'a> {
    pub sdl_context: Sdl,
    pub timer: TimerSubsystem,
    pub renderer: Canvas<Window>,
    pub title: String,
    pub screen: Surface<'a>,
    pub screenBuffer: Surface<'a>,
    pub fullscreen: bool,
    pub usedoublebuffering: bool,
    pub screenWidth: i32,
    pub screenHeight: i32,
    pub screenBits: i32,
    pub screenPitch: i32,
    pub bufferPitch: i32,
    pub scaleFactor: i32,
    pub screenfaded: bool,
    pub bordercolor: i32,
    pub ylookup: Vec<usize>,
    pub palette1: [Color; 256],
    pub palette2: [Color; 256],
    pub curpal: [Color; 256],
    pub gamepal: [Color; 256],
    //
    pub keyboard: KeyboardUtil,
    pub mouse: MouseUtil,
    pub joystick: JoystickSubsystem,
    pub Joystick: Result<Joystick, IntegerOrSdlError>,
}

impl<'a> id_vl<'a> {
    pub fn new() -> Self {
        ///////////////////////////////////////////////////////
        let sdl_context = sdl2::init().unwrap();
        ///////////////////////////////////////////////////////
        let timer = sdl_context.timer().unwrap();
        ///////////////////////////////////////////////////////
        // BUG
        // Need to get these from CheckParameters (--res, --resf)
        // You can use multiple of 320 x 200
        // 640 x 400
        // 1280 x 800
        // 1920 x 1200
        // The window is made resizable but not sure if it will cause problems
        let screenWidth: i32 = 640;
        let screenHeight: i32 = 400;
        ///////////////////////////////////////////////////////
        let title: String;
        {
            title = "Wolfenstein 3D (Rust)".to_string();
        }
        ///////////////////////////////////////////////////////
        let video_subsystem = sdl_context.video().unwrap();
        ///////////////////////////////////////////////////////
        let window = video_subsystem
            .window(&title, screenWidth as u32, screenHeight as u32)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        ///////////////////////////////////////////////////////
        let renderer = window
            .into_canvas()
            .accelerated()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        ///////////////////////////////////////////////////////
        let palette1 = [RGB((0, 0, 0)); 256];
        let palette2 = [RGB((0, 0, 0)); 256];
        let mut gamepal = [RGB((0, 0, 0)); 256];

        for i in 0..256 {
            gamepal[i] = RGB(wolfpal[i]);
        }

        ///////////////////////////////////////////////////////
        let screenBits = PixelFormatEnum::ARGB8888.into_masks().unwrap();
        let screen =
            Surface::from_pixelmasks(screenWidth as u32, screenHeight as u32, screenBits).unwrap();
        ///////////////////////////////////////////////////////
        let curpal = gamepal;
        ///////////////////////////////////////////////////////
        let screenBits = PixelFormatEnum::Index8.into_masks().unwrap();
        let screenBuffer =
            Surface::from_pixelmasks(screenWidth as u32, screenHeight as u32, screenBits).unwrap();
        ///////////////////////////////////////////////////////
        let keyboard = sdl_context.keyboard();
        let mouse = sdl_context.mouse();
        let joystick = sdl_context.joystick().unwrap();
        let Joystick = joystick.open(0);
        ///////////////////////////////////////////////////////
        let s = Self {
            sdl_context,
            timer,
            renderer,
            title,
            screen,
            screenBuffer,
            fullscreen: true,
            usedoublebuffering: true,
            screenWidth,
            screenHeight,
            screenBits: -1, // use "best" color depth according to libSDL
            screenPitch: 0,
            bufferPitch: 0,
            scaleFactor: 0,
            screenfaded: false,
            bordercolor: 0,
            ylookup: Vec::new(),
            palette1,
            palette2,
            curpal,
            gamepal,
            //
            keyboard,
            mouse,
            joystick,
            Joystick,
        };
        s
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub fn RGB(rgb: (u8, u8, u8)) -> Color {
    let new_r: u16 = (rgb.0 as u16) * 255 / 63;
    let new_g: u16 = (rgb.1 as u16) * 255 / 63;
    let new_b: u16 = (rgb.2 as u16) * 255 / 63;
    return Color::RGBA(new_r as u8, new_g as u8, new_b as u8, 0);
}

pub fn VL_WaitVBL(w3d: &mut modules, delay: i32) {
    //println!("VL_WaitVBL");

    SDL_Delay(w3d, 8 * delay);
}

pub fn VL_ClearScreen(w3d: &mut modules, color: Color) {
    //println!("VL_ClearScreen");

    w3d.id_vl.screenBuffer.fill_rect(None, color).unwrap();
}

/*
=======================
=
= VL_SetVGAPlaneMode
=
=======================
*/

pub fn VL_SetVGAPlaneMode(w3d: &mut modules) {
    //println!("VL_SetVGAPlaneMode");

    w3d.id_vl.renderer.set_blend_mode(BlendMode::Blend);

    hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "0");

    w3d.id_vl.mouse.show_cursor(false);

    let palette = Palette::with_colors(&w3d.id_vl.gamepal).unwrap();

    // BUG can't apply this palette !!!
    //w3d.id_vl.screen.set_palette(&palette).unwrap();

    w3d.id_vl.screenBuffer.set_palette(&palette).unwrap();

    w3d.id_vl.screenPitch = w3d.id_vl.screen.pitch() as i32; // 2560
    w3d.id_vl.bufferPitch = w3d.id_vl.screenBuffer.pitch() as i32; //640

    w3d.id_vl.scaleFactor = w3d.id_vl.screenWidth / 320;
    if (w3d.id_vl.screenHeight / 200) < w3d.id_vl.scaleFactor {
        w3d.id_vl.scaleFactor = w3d.id_vl.screenHeight / 200
    };

    w3d.id_vl.ylookup = Vec::with_capacity(w3d.id_vl.screenHeight as usize);

    w3d.wl_draw.pixelangle = vec![0; w3d.id_vl.screenWidth as usize];

    w3d.wl_draw.wallheight = vec![0; w3d.id_vl.screenWidth as usize];

    for i in 0..w3d.id_vl.screenHeight as usize {
        w3d.id_vl.ylookup.push(i * (w3d.id_vl.bufferPitch as usize));
    }
}

/*
=============================================================================

                        PALETTE OPS

        To avoid snow, do a WaitVBL BEFORE calling these

=============================================================================
*/

/*
=================
=
= VL_FillPalette
=
=================
*/

pub fn VL_FillPalette(w3d: &mut modules, red: i32, green: i32, blue: i32) {
    //println!("VL_FillPalette");

    let mut pal: [Color; 256] = [Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }; 256];

    for i in 0..256 as usize {
        pal[i].r = red as u8;
        pal[i].g = green as u8;
        pal[i].b = blue as u8;
    }

    VL_SetPalette(w3d, pal, true);
}

/*
=================
=
= VL_SetPalette
=
=================
*/

pub fn VL_SetPalette(w3d: &mut modules, palette: [Color; 256], forceupdate: bool) {
    //println!("VL_SetPalette");

    w3d.id_vl.curpal = palette;

    let palette = Palette::with_colors(&palette).unwrap();

    //BUG
    if w3d.id_vl.screenBits == 8 {
        w3d.id_vl.screen.set_palette(&palette).unwrap();
    } else {
        w3d.id_vl.screenBuffer.set_palette(&palette).unwrap();

        if forceupdate {
            //SDL_BlitSurface(screenBuffer, NULL, screen, NULL);
            w3d.id_vl
                .screenBuffer
                .blit(None, &mut w3d.id_vl.screen, None)
                .unwrap();

            Present(w3d);
        }
    }
}

//===========================================================================

/*
=================
=
= VL_GetPalette
=
=================
*/

pub fn VL_GetPalette(w3d: &mut modules) {
    //println!("VL_GetPalette");

    //memcpy(palette, curpal, sizeof(SDL_Color) * 256);
    w3d.id_vl.palette1 = w3d.id_vl.curpal;
}

//===========================================================================

/*
=================
=
= VL_FadeOut
=
= Fades the current palette to the given color in the given number of steps
=
=================
*/

pub fn VL_FadeOut(
    w3d: &mut modules,
    start: i32,
    end: i32,
    red: i32,
    green: i32,
    blue: i32,
    steps: i32,
) {
    //println!("VL_FadeOut");

    let mut orig: i32;
    let mut delta: i32;
    let mut origptr: Color;
    let mut newptr: Color;

    let red = red * 255 / 63;
    let green = green * 255 / 63;
    let blue = blue * 255 / 63;

    VL_WaitVBL(w3d, 1);
    VL_GetPalette(w3d); //get to palette1

    w3d.id_vl.palette2 = w3d.id_vl.palette1;

    //
    // fade through intermediate frames
    //
    for i in 0..steps {
        for j in start..=end {
            origptr = w3d.id_vl.palette1[j as usize];
            newptr = w3d.id_vl.palette2[j as usize];

            orig = origptr.r as i32;
            delta = red - orig;
            newptr.r = (orig + delta * i / steps) as u8;
            orig = origptr.g as i32;
            delta = green - orig;
            newptr.g = (orig + delta * i / steps) as u8;
            orig = origptr.b as i32;
            delta = blue - orig;
            newptr.b = (orig + delta * i / steps) as u8;

            w3d.id_vl.palette1[j as usize] = origptr;
            w3d.id_vl.palette2[j as usize] = newptr;
        }

        if !w3d.id_vl.usedoublebuffering || w3d.id_vl.screenBits == 8 {
            VL_WaitVBL(w3d, 1);
        }
        VL_SetPalette(w3d, w3d.id_vl.palette2, true);
    }
    //
    // final color
    //
    VL_FillPalette(w3d, red, green, blue);

    w3d.id_vl.screenfaded = true;
}

/*
=================
=
= VL_FadeIn
=
=================
*/

pub fn VL_FadeIn(w3d: &mut modules, start: i32, end: i32, palette: [Color; 256], steps: i32) {
    //println!("VL_FadeIn");

    let mut delta: i32;

    VL_WaitVBL(w3d, 1);
    VL_GetPalette(w3d); //get to palette1
                        //memcpy(palette2, palette1, sizeof(SDL_Color) * 256);
    w3d.id_vl.palette2 = w3d.id_vl.palette1;

    //
    // fade through intermediate frames
    //
    for i in 0..steps {
        for j in start as usize..=end as usize {
            delta = (palette[j].r - w3d.id_vl.palette1[j].r) as i32;
            w3d.id_vl.palette2[j].r = w3d.id_vl.palette1[j].r + (delta * i / steps) as u8;
            delta = (palette[j].g - w3d.id_vl.palette1[j].g) as i32;
            w3d.id_vl.palette2[j].g = w3d.id_vl.palette1[j].g + (delta * i / steps) as u8;
            delta = (palette[j].b - w3d.id_vl.palette1[j].b) as i32;
            w3d.id_vl.palette2[j].b = w3d.id_vl.palette1[j].b + (delta * i / steps) as u8;
        }

        if !w3d.id_vl.usedoublebuffering || w3d.id_vl.screenBits == 8 {
            VL_WaitVBL(w3d, 1);
        }
        VL_SetPalette(w3d, w3d.id_vl.palette2, true);
    }

    //
    // final color
    //
    VL_SetPalette(w3d, palette, true);
    w3d.id_vl.screenfaded = false;
}

/*
=============================================================================

                            PIXEL OPS

=============================================================================
*/

pub fn VL_LockSurface(surface: &mut Surface) -> Vec<u8> {
    //println!("VL_LockSurface");

    return surface.without_lock().unwrap().to_vec();
}

pub fn VL_UnlockSurface(_surface: &mut Surface) {
    //println!("VL_UnlockSurface");
}

/*
=================
=
= VL_GetPixel
=
=================
*/

pub fn VL_GetPixel(w3d: &mut modules, x: usize, y: usize) -> u8 {
    //println!("VL_GetPixel");

    let pixels = w3d.id_vl.screenBuffer.without_lock_mut().unwrap();
    let col = pixels[w3d.id_vl.ylookup[y] + x];

    col
}

/*
=================
=
= VL_Hlin
=
=================
*/

pub fn VL_Hlin(w3d: &mut modules, _x: i32, y: i32, width: i32, color: i32) {
    //println!("VL_Hlin");

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        let pos = w3d.id_vl.ylookup[y as usize] as usize;

        for i in 0..width {
            dest[pos + i as usize] = color as u8;
        }
    });
}

/*
=================
=
= VL_Vlin
=
=================
*/

pub fn VL_Vlin(w3d: &mut modules, x: i32, y: i32, height: i32, color: i32) {
    //println!("VL_Vlin");

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        let mut pos = w3d.id_vl.ylookup[y as usize] + x as usize;

        for _i in (0..height).rev() {
            dest[pos as usize] = color as u8;
            pos += w3d.id_vl.bufferPitch as usize;
        }
    });
}

/*
=================
=
= VL_Bar
=
=================
*/

pub fn VL_Bar(w3d: &mut modules, x: i32, y: i32, width: i32, height: i32, color: i32) {
    //println!("VL_Bar");

    VL_BarScaledCoord(
        w3d,
        w3d.id_vl.scaleFactor * x,
        w3d.id_vl.scaleFactor * y,
        w3d.id_vl.scaleFactor * width,
        w3d.id_vl.scaleFactor * height,
        color,
    );
}

pub fn VL_BarScaledCoord(
    w3d: &mut modules,
    scx: i32,
    scy: i32,
    scwidth: i32,
    mut scheight: i32,
    color: i32,
) {
    //println!("VL_BarScaledCoord");

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        let mut pos = w3d.id_vl.ylookup[scy as usize] + scx as usize;

        for _j in (0..scheight).rev() {
            for i in 0..scwidth {
                dest[pos + i as usize] = color as u8;
            }
            pos += w3d.id_vl.bufferPitch as usize;
        }
    });

    while scheight > 0 {
        //dest[scwidth as usize] = color;
        //dest += w3d.id_vl.bufferPitch;
        scheight -= 1;
    }
}

/*
============================================================================

                            MEMORY OPS

============================================================================
*/

/*
===================
=
= VL_DePlaneVGA
=
= Unweave a VGA graphic to simplify drawing
=
===================
*/

pub fn VL_DePlaneVGA(source: &mut [u8], width: i32, height: i32) {
    //println!("VL_DePlaneVGA");

    let size: i32;
    let pwidth: i32;
    let mut dest: Vec<u8>;
    let srcline: Vec<u8>;

    size = width * height; // 64 = 8x8

    if (width & 3) != 0 {
        Quit("DePlaneVGA: width not divisible by 4!");
    }

    //temp = SafeMalloc(size);
    //temp = vec![0; size as usize];
    dest = vec![0; size as usize];

    //
    // munge pic into the temp buffer
    //
    srcline = source.to_vec();

    pwidth = width >> 2; // 8/2=4, 4/2=2

    let mut dest_index;
    let mut srcline_index = 0;

    for plane in 0..4 {
        dest_index = 0;

        for _y in 0..height {
            for x in 0..pwidth {
                //*(dest + (x << 2) + plane) = *srcline++;
                let index = ((x << 2) + plane) as usize;
                dest[index + dest_index] = srcline[srcline_index];
                srcline_index += 1;
            }

            dest_index += width as usize;
        }
    }

    //
    // copy the temp buffer back into the original source
    //
    //memcpy (source,temp,size);

    for i in 0..(size as usize) {
        source[i as usize] = dest[i as usize];
    }

    //free (temp);
}

/*
=================
=
= VL_MemToScreenScaledCoord
=
= Draws a block of data to the screen with scaling according to scaleFactor.
=
=================
*/

pub fn VL_MemToScreen(w3d: &mut modules, source: &[u8], width: i32, height: i32, x: i32, y: i32) {
    //println!("VL_MemToScreen");

    VL_MemToScreenScaledCoord(
        w3d,
        source,
        width,
        height,
        w3d.id_vl.scaleFactor * x,
        w3d.id_vl.scaleFactor * y,
    );
}

pub fn VL_MemToScreenScaledCoord(
    w3d: &mut modules,
    source: &[u8],
    width: i32,
    height: i32,
    destx: i32,
    desty: i32,
) {
    //println!("VL_MemToScreenScaledCoord");

    let mut col: u8 = 0;
    let mut sci: i32 = 0;
    let mut scj: i32 = 0;

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        for j in 0..height {
            scj = j * w3d.id_vl.scaleFactor;
            for i in 0..width {
                sci = i * w3d.id_vl.scaleFactor;
                col = source[(j * width + i) as usize];
                for m in 0..w3d.id_vl.scaleFactor {
                    for n in 0..w3d.id_vl.scaleFactor {
                        dest[w3d.id_vl.ylookup[(scj + m + desty) as usize]
                            + (sci + n + destx) as usize] = col;
                    }
                }
            }
        }
    });
}

/*
=================
=
= VL_MemToScreenScaledCoord
=
= Draws a part of a block of data to the screen.
= The block has the size origwidth*origheight.
= The part at (srcx, srcy) has the size width*height
= and will be painted to (destx, desty) with scaling according to scaleFactor.
=
=================
*/

pub fn VL_MemToScreenScaledCoord2(
    w3d: &mut modules,
    source: &[u8],
    origwidth: i32,
    _origheight: i32,
    srcx: i32,
    srcy: i32,
    destx: i32,
    desty: i32,
    width: i32,
    height: i32,
) {
    //println!("VL_MemToScreenScaledCoord2");

    let mut col: u8 = 0;
    let mut sci: i32 = 0;
    let mut scj: i32 = 0;

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        for j in 0..height {
            scj = j * w3d.id_vl.scaleFactor;
            for i in 0..width {
                sci = i * w3d.id_vl.scaleFactor;
                col = source[((j + srcy) * origwidth + i + srcx) as usize];
                for m in 0..w3d.id_vl.scaleFactor {
                    for n in 0..w3d.id_vl.scaleFactor {
                        dest[w3d.id_vl.ylookup[(scj + m + desty) as usize]
                            + (sci + n + destx) as usize] = col;
                    }
                }
            }
        }
    });
}
