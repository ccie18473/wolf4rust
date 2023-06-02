#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_scale
//
//===========================================================================

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

/*
===================
=
= ScaleLine
=
= Reconstruct a sprite and draw it
=
= each vertical line of the shape has a pointer to segment data:
= 	end of segment pixel*2 (0 terminates line)
= 	top of virtual line with segment in proper place
=	start of segment pixel*2
=	<repeat>
=
===================
*/

pub fn ScaleLine(
    w3d: &mut modules,
    x: i32,
    toppix: i32,
    fracstep: i32,
    linesrc: &mut Vec<u8>,
    linecmds: Vec<u8>,
    _curshades: i32,
) {
    //println!("ScaleLine");

    let mut linecmds_i: usize = 0;
    let mut src: &mut Vec<u8>;
    let mut src_i: usize;
    let mut dest_i: usize;
    let mut col: u8;
    let mut start: i16;
    let mut end: i16;
    let mut top: i16;
    let mut startpix: i16;
    let mut endpix: i16;
    let mut frac: i32;

    //for (end = READWORD(linecmds) >> 1; end; end = READWORD(linecmds) >> 1)
    let value1 = linecmds[linecmds_i] as i16;
    let value2 = (linecmds[linecmds_i + 1] as i16) << 8;
    end = (value1 + value2) >> 1;

    while end != 0 {
        let value1 = linecmds[linecmds_i + 2] as i16;
        let value2 = (linecmds[linecmds_i + 3] as i16) << 8;
        top = value1 + value2;

        let value1 = linecmds[linecmds_i + 4] as i16;
        let value2 = (linecmds[linecmds_i + 5] as i16) << 8;
        start = (value1 + value2) >> 1;

        frac = start as i32 * fracstep;

        endpix = ((frac >> FRACBITS) + toppix) as i16;

        src = linesrc;
        src_i = (top + start) as usize;

        //for (src = &linesrc[top + start]; start != end; start++, src++)
        while start != end {
            startpix = endpix;

            if startpix >= w3d.wl_main.viewheight as i16 {
                break; // off the bottom of the view area
            }
            frac += fracstep;
            endpix = ((frac >> FRACBITS) + toppix) as i16;

            if endpix < 0 {
                start += 1;
                src_i += 1;
                continue; // not into the view area
            }

            if startpix < 0 {
                startpix = 0; // clip upper boundary
            }

            if endpix > w3d.wl_main.viewheight as i16 {
                endpix = w3d.wl_main.viewheight as i16; // clip lower boundary
            }

            col = src[src_i];

            //dest = vbuf + ylookup[startpix] + x;
            dest_i = w3d.wl_draw.vbuf_i + w3d.id_vl.ylookup[startpix as usize] + x as usize;

            w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
                while startpix < endpix {
                    dest[dest_i] = col;
                    dest_i += w3d.id_vl.bufferPitch as usize;
                    startpix += 1;
                }
            });
            start += 1;
            src_i += 1;
        }

        linecmds_i += 6; // next segment list

        let value1 = linecmds[linecmds_i] as i16;
        let value2 = (linecmds[linecmds_i + 1] as i16) << 8;
        end = (value1 + value2) >> 1;
    }
}

/*
===================
=
= ScaleShape
=
= Draws a compiled shape at [scale] pixels high
=
===================
*/

pub fn ScaleShape(w3d: &mut modules, xcenter: i32, shapenum: i32, height: i32, _flags: i32) {
    //println!("ScaleShape");

    let shape: compshape_t;
    let mut linesrc: Vec<u8>;
    let mut linecmds: Vec<u8>;
    let scale: i32;
    let toppix: i32;
    let mut x1: i32;
    let mut x2: i32;
    let actx: i32;
    let mut frac: i32;
    let fracstep: i32;

    scale = height >> 3; // low three bits are fractional

    if scale == 0 {
        return; // too close or far away
    }

    linesrc = PM_GetSpritePage(w3d, shapenum);
    //shape = (compshape_t *)linesrc;
    shape = bincode::deserialize(&linesrc).unwrap();

    fracstep = FixedDiv(scale, TEXTURESIZE / 2);
    frac = shape.leftpix as i32 * fracstep;

    actx = xcenter - scale;
    toppix = w3d.wl_main.centery - scale;

    x2 = (frac >> FRACBITS) + actx;

    for i in shape.leftpix..=shape.rightpix {
        //
        // calculate edges of the shape
        //
        x1 = x2;
        if x1 >= w3d.wl_main.viewwidth {
            break; // off the right side of the view area
        }

        frac += fracstep;
        x2 = (frac >> FRACBITS) + actx;

        if x2 < 0 {
            continue; // not into the view area
        }

        if x1 < 0 {
            x1 = 0; // clip left boundary
        }

        if x2 > w3d.wl_main.viewwidth {
            x2 = w3d.wl_main.viewwidth; // clip right boundary
        }

        while x1 < x2 {
            if w3d.wl_draw.wallheight[x1 as usize] < height {
                linecmds = linesrc[shape.dataofs[(i - shape.leftpix) as usize] as usize..].to_vec();

                ScaleLine(w3d, x1, toppix, fracstep, &mut linesrc, linecmds, 0);
            }

            x1 += 1;
        }
    }
}

/*
===================
=
= SimpleScaleShape
=
= NO CLIPPING, height in pixels
=
= Draws a compiled shape at [scale] pixels high
=
===================
*/

pub fn SimpleScaleShape(w3d: &mut modules, xcenter: i32, shapenum: i32, height: i32) {
    //println!("SimpleScaleShape");

    let shape: compshape_t;
    let mut linesrc: Vec<u8>;
    let mut linecmds: Vec<u8>;
    let scale: i32;
    let toppix: i32;
    let mut x1: i32;
    let mut x2: i32;
    let actx: i32;
    let mut frac: i32;
    let fracstep: i32;

    scale = height >> 1;

    linesrc = PM_GetSpritePage(w3d, shapenum);

    //shape = (compshape_t *)linesrc;
    shape = bincode::deserialize(&linesrc).unwrap();

    fracstep = FixedDiv(scale, TEXTURESIZE / 2);
    frac = shape.leftpix as i32 * fracstep;

    actx = xcenter - scale;
    toppix = w3d.wl_main.centery - scale;

    x2 = (frac >> FRACBITS) + actx;

    for i in shape.leftpix..=shape.rightpix {
        //
        // calculate edges of the shape
        //
        x1 = x2;

        frac += fracstep;
        x2 = (frac >> FRACBITS) + actx;

        while x1 < x2 {
            linecmds = linesrc[shape.dataofs[(i - shape.leftpix) as usize] as usize..].to_vec();

            ScaleLine(w3d, x1, toppix, fracstep, &mut linesrc, linecmds, 0);

            x1 += 1;
        }
    }
}
