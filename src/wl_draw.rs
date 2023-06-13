#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_draw
//
//===========================================================================

pub struct wl_draw {
    pub vbuf: Vec<u8>,
    pub vbuf_i: usize,
    pub lasttimecount: i32,
    pub frameon: i32,
    pub fpscounter: bool,
    pub fps_frames: i32,
    pub fps_time: i32,
    pub fps: i32,
    pub wallheight: Vec<i32>,
    //
    // math tables
    //
    pub pixelangle: Vec<i32>,
    pub finetangent: [i32; FINEANGLES as usize / 4],
    pub sintable: [i32; ANGLES as usize + ANGLES as usize / 4],
    pub costable: [i32; ANGLES as usize + ANGLES as usize / 4],
    //
    // refresh variables
    //
    pub viewx: i32, // the focal point
    pub viewy: i32,
    pub viewangle: i32,
    pub viewsin: i32,
    pub viewcos: i32,
    pub postx: i32,
    pub postsource: Vec<u8>,
    pub postsource_i: i32,
    //
    // wall optimization variables
    //
    pub lastside: i32, // true for vertical
    pub lasttilehit: i32,
    pub lasttexture: i32,

    //
    // ray tracing variables
    //
    pub focaltx: i32,
    pub focalty: i32,
    pub xpartialup: i32,
    pub xpartialdown: i32,
    pub ypartialup: i32,
    pub ypartialdown: i32,

    pub midangle: i32,

    pub tilehit: i32,
    pub pixx: i32,

    pub xtile: i32,
    pub ytile: i32,
    pub xtilestep: i32,
    pub ytilestep: i32,
    pub xintercept: i32,
    pub yintercept: i32,
    pub xinttile: i32,
    pub yinttile: i32,
    pub texdelta: i32,

    pub horizwall: [i32; MAXWALLTILES],
    pub vertwall: [i32; MAXWALLTILES],

    pub weaponscale: [i32; weapontype::NUMWEAPONS as usize],

    pub vislist: Vec<visobj_t>,
    pub vislist_i: usize,
    pub visptr: Vec<visobj_t>,
    pub visptr_i: usize,
    pub visstep: Vec<visobj_t>,
    pub visstep_i: usize,
    pub farthest: visobj_t,

    pub angle: i32,
    pub xstep: i32,
    pub ystep: i32,
    pub xinttemp: i32, // holds temporary intercept position
    pub yinttemp: i32,
    pub xpartial: i32,
    pub ypartial: i32,
    pub door: doorobj_t,
    pub pwallposnorm: i32, // holds modified pwallpos
    pub pwallposinv: i32,
    pub pwallposi: i32,
    pub passdoor: bool,
    pub breakcore: bool,
}

impl wl_draw {
    pub fn new() -> Self {
        let weaponscale: [i32; weapontype::NUMWEAPONS as usize] = [
            SPRITES::SPR_KNIFEREADY as i32,
            SPRITES::SPR_PISTOLREADY as i32,
            SPRITES::SPR_MACHINEGUNREADY as i32,
            SPRITES::SPR_CHAINREADY as i32,
        ];

        let vislist = vec![visobj_t::default(); MAXVISABLE as usize];

        Self {
            vbuf: Vec::new(),
            vbuf_i: 0,
            lasttimecount: 0,
            frameon: 0,
            fpscounter: false,
            fps_frames: 0,
            fps_time: 0,
            fps: 0,
            wallheight: Vec::new(),
            //
            // math tables
            //
            pixelangle: Vec::new(),
            finetangent: [0; FINEANGLES as usize / 4],
            sintable: [0; ANGLES as usize + ANGLES as usize / 4],
            costable: [0; ANGLES as usize + ANGLES as usize / 4],
            //
            // refresh variables
            //
            viewx: 0, // the focal point
            viewy: 0,
            viewangle: 0,
            viewsin: 0,
            viewcos: 0,
            postx: 0,
            postsource: Vec::new(),
            postsource_i: 0,
            //
            // wall optimization variables
            //
            lastside: 0, // true for vertical
            lasttilehit: 0,
            lasttexture: 0,

            //
            // ray tracing variables
            //
            focaltx: 0,
            focalty: 0,
            xpartialup: 0,
            xpartialdown: 0,
            ypartialup: 0,
            ypartialdown: 0,

            midangle: 0,

            tilehit: 0,
            pixx: 0,

            xtile: 0,
            ytile: 0,
            xtilestep: 0,
            ytilestep: 0,
            xintercept: 0,
            yintercept: 0,
            xinttile: 0,
            yinttile: 0,
            texdelta: 0,

            horizwall: [0i32; MAXWALLTILES],
            vertwall: [0i32; MAXWALLTILES],

            weaponscale,

            vislist,
            vislist_i: 0,
            visptr: Vec::new(),
            visptr_i: 0,
            visstep: Vec::new(),
            visstep_i: 0,
            farthest: visobj_t::default(),
            //
            // wallrefresh vars
            //
            angle: 0,
            xstep: 0,
            ystep: 0,
            xinttemp: 0, // holds temporary intercept position
            yinttemp: 0,
            xpartial: 0,
            ypartial: 0,
            door: doorobj_t::new(),
            pwallposnorm: 0, // holds modified pwallpos
            pwallposinv: 0,
            pwallposi: 0,
            passdoor: false,
            breakcore: false,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const ACTORSIZE: u16 = 0x4000;

pub const vgaCeiling: [i32; 60] = [
    0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0xbf, 0x4e, 0x4e, 0x4e, 0x1d, 0x8d, 0x4e,
    0x1d, 0x2d, 0x1d, 0x8d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x2d, 0xdd, 0x1d, 0x1d, 0x98, 0x1d, 0x9d,
    0x2d, 0xdd, 0xdd, 0x9d, 0x2d, 0x4d, 0x1d, 0xdd, 0x7d, 0x1d, 0x2d, 0x2d, 0xdd, 0xd7, 0x1d, 0x1d,
    0x1d, 0x2d, 0x1d, 0x1d, 0x1d, 0x1d, 0xdd, 0xdd, 0x7d, 0xdd, 0xdd, 0xdd,
];

pub const MAXVISABLE: u16 = 250;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct visobj_t {
    pub viewx: i16,
    pub viewheight: i16,
    pub shapenum: i16,
    pub flags: i16, // this must be changed to uint32_t, when you
                    // you need more than 16-flags for drawing
}

/*
============================================================================

                           3 - D  DEFINITIONS

============================================================================
*/

/*
========================
=
= TransformActor
=
= Takes paramaters:
=   gx,gy               : globalx/globaly of point
=
= globals:
=   viewx,viewy         : point of view
=   viewcos,viewsin     : sin/cos of viewangle
=   scale               : conversion from global value to screen value
=
= sets:
=   screenx,transx,transy,screenheight: projected edge location and size
=
========================
*/

//
// transform actor
//

pub fn TransformActor(w3d: &mut modules, obj: &mut objtype) {
    //println!("TransformActor");

    let gx: i32;
    let gy: i32;
    let mut gxt: i32;
    let mut gyt: i32;
    let nx: i32;
    let ny: i32;

    //
    // translate point to view centered coordinates
    //
    gx = obj.x - w3d.wl_draw.viewx;
    gy = obj.y - w3d.wl_draw.viewy;

    //
    // calculate newx
    //
    gxt = FixedMul(gx, w3d.wl_draw.viewcos);
    gyt = FixedMul(gy, w3d.wl_draw.viewsin);
    nx = gxt - gyt - ACTORSIZE as i32; // fudge the shape forward a bit, because
                                       // the midpoint could put parts of the shape
                                       // into an adjacent wall

    //
    // calculate newy
    //
    gxt = FixedMul(gx, w3d.wl_draw.viewsin);
    gyt = FixedMul(gy, w3d.wl_draw.viewcos);
    ny = gyt + gxt;

    //
    // calculate perspective ratio
    //
    obj.transx = nx;
    obj.transy = ny;

    if nx < MINDIST
    // too close, don't overflow the divide
    {
        obj.viewheight = 0;
        return;
    }

    obj.viewx = w3d.wl_main.centerx + ny * w3d.wl_main.scale / nx;

    //
    // calculate height (heightnumerator/(nx>>8))
    //
    obj.viewheight = w3d.wl_main.heightnumerator / (nx >> 8);
}

//==========================================================================

/*
========================
=
= TransformTile
=
= Takes paramaters:
=   tx,ty               : tile the object is centered in
=
= globals:
=   viewx,viewy         : point of view
=   viewcos,viewsin     : sin/cos of viewangle
=   scale               : conversion from global value to screen value
=
= sets:
=   screenx,transx,transy,screenheight: projected edge location and size
=
= Returns true if the tile is withing getting distance
=
========================
*/

pub fn TransformTile(
    w3d: &mut modules,
    _ob: &mut object,
    tx: i32,
    ty: i32,
    //dispx: &mut i32,
    //dispheight: &mut i32,
) -> bool {
    //println!("TransformTile");

    let gx: i32;
    let gy: i32;
    let mut gxt: i32;
    let mut gyt: i32;
    let nx: i32;
    let ny: i32;

    //
    // translate point to view centered coordinates
    //
    gx = (tx << TILESHIFT) + 0x8000 - w3d.wl_draw.viewx;
    gy = (ty << TILESHIFT) + 0x8000 - w3d.wl_draw.viewy;

    //
    // calculate newx
    //
    gxt = FixedMul(gx, w3d.wl_draw.viewcos);
    gyt = FixedMul(gy, w3d.wl_draw.viewsin);
    nx = gxt - gyt - 0x2000; // 0x2000 is size of object

    //
    // calculate newy
    //

    gxt = FixedMul(gx, w3d.wl_draw.viewsin);
    gyt = FixedMul(gy, w3d.wl_draw.viewcos);
    ny = gyt + gxt;

    //
    // calculate height / perspective ratio
    //
    if nx < MINDIST {
        // too close, don't overflow the divide
        //*dispheight = 0;
        w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].viewheight = 0;
    } else {
        //*dispx = w3d.wl_main.centerx + ny * w3d.wl_main.scale / nx;
        w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].viewx =
            (w3d.wl_main.centerx + ny * w3d.wl_main.scale / nx) as i16;

        //*dispheight = w3d.wl_main.heightnumerator / (nx >> 8);
        w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].viewheight =
            (w3d.wl_main.heightnumerator / (nx >> 8)) as i16;
    }

    //
    // see if it should be grabbed
    //
    if nx < TILEGLOBAL && ny > -TILEGLOBAL / 2 && ny < TILEGLOBAL / 2 {
        return true;
    } else {
        return false;
    }
}

//==========================================================================

/*
====================
=
= CalcHeight
=
= Calculates the height of xintercept,yintercept from viewx,viewy
=
====================
*/

pub fn CalcHeight(w3d: &mut modules) -> i32 {
    //println!("CalcHeight");

    let height: i32;
    let gx: i32;
    let gy: i32;
    let gxt: i32;
    let gyt: i32;
    let mut nx: i32;
    let _ny: i32;

    //
    // translate point to view centered coordinates
    //
    gx = w3d.wl_draw.xintercept - w3d.wl_draw.viewx;
    gy = w3d.wl_draw.yintercept - w3d.wl_draw.viewy;

    //
    // calculate nx
    //
    gxt = FixedMul(gx, w3d.wl_draw.viewcos);
    gyt = FixedMul(gy, w3d.wl_draw.viewsin);
    nx = gxt - gyt;

    //
    // calculate perspective ratio
    //

    if nx < MINDIST {
        nx = MINDIST; // don't let divide overflow
    }

    height = w3d.wl_main.heightnumerator / (nx >> 8);

    return height;
}

//==========================================================================

/*
===================
=
= ScalePost
=
===================
*/

pub fn ScalePost(w3d: &mut modules) {
    //println!("ScalePost");

    let mut ywcount: i32;
    let mut yoffs: i32;
    let mut yw: i32;
    let mut yd: i32;
    let mut yendoffs: i32;
    let mut col: u8;

    ywcount = w3d.wl_draw.wallheight[w3d.wl_draw.postx as usize] >> 3;
    yd = ywcount;

    if yd <= 0 {
        yd = 100;
    }

    yoffs = (w3d.wl_main.centery - ywcount) * w3d.id_vl.bufferPitch;

    if yoffs < 0 {
        yoffs = 0;
    }
    yoffs += w3d.wl_draw.postx;

    yendoffs = w3d.wl_main.centery + ywcount - 1;
    yw = TEXTURESIZE - 1;

    while yendoffs >= w3d.wl_main.viewheight {
        ywcount -= TEXTURESIZE / 2;
        while ywcount <= 0 {
            ywcount += yd;
            yw -= 1;
        }
        yendoffs -= 1;
    }
    if yw < 0 {
        return;
    }

    col = w3d.wl_draw.postsource[w3d.wl_draw.postsource_i as usize + yw as usize] as u8;

    yendoffs = yendoffs * w3d.id_vl.bufferPitch + w3d.wl_draw.postx;

    while yoffs <= yendoffs {
        //vbuf[yendoffs] = col;

        w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
            dest[yendoffs as usize + w3d.wl_main.screenofs as usize] = col;
        });

        ywcount -= TEXTURESIZE / 2;

        if ywcount <= 0 {
            loop {
                ywcount += yd;
                yw -= 1;

                if ywcount > 0 {
                    break;
                }
            }

            if yw < 0 {
                break;
            }

            col = w3d.wl_draw.postsource[w3d.wl_draw.postsource_i as usize + yw as usize] as u8;
        }
        yendoffs -= w3d.id_vl.bufferPitch;
    }
}

/*
====================
=
= HitVertWall
=
= tilehit bit 7 is 0, because it's not a door tile
= if bit 6 is 1 and the adjacent tile is a door tile, use door side pic
=
====================
*/

pub fn HitVertWall(w3d: &mut modules) {
    //println!("HitVertWall");

    let wallpic: i32;
    let mut texture: i32;

    texture = ((w3d.wl_draw.yintercept - w3d.wl_draw.texdelta) >> FIXED2TEXSHIFT) & TEXTUREMASK;

    if w3d.wl_draw.xtilestep == -1 {
        texture = TEXTUREMASK - texture;
        w3d.wl_draw.xintercept += TILEGLOBAL;
    }

    w3d.wl_draw.wallheight[w3d.wl_draw.pixx as usize] = CalcHeight(w3d);
    w3d.wl_draw.postx = w3d.wl_draw.pixx;

    if w3d.wl_draw.lastside == 1
        && w3d.wl_draw.lasttilehit == w3d.wl_draw.tilehit
        && !(w3d.wl_draw.tilehit & BIT_WALL) != 0
    {
        //
        // in the same wall type as last time, so use the last postsource
        //
        if texture != w3d.wl_draw.lasttexture {
            w3d.wl_draw.postsource_i += texture - w3d.wl_draw.lasttexture;
            w3d.wl_draw.lasttexture = texture;
        }
    } else {
        w3d.wl_draw.lastside = 1;
        w3d.wl_draw.lasttilehit = w3d.wl_draw.tilehit;
        w3d.wl_draw.lasttexture = texture;

        if (w3d.wl_draw.tilehit & BIT_WALL) != 0 {
            //
            // check for adjacent doors
            //
            if (w3d.wl_play.tilemap[(w3d.wl_draw.xtile - w3d.wl_draw.xtilestep) as usize]
                [w3d.wl_draw.yinttile as usize]
                & BIT_DOOR)
                != 0
            {
                wallpic = DOORWALL(w3d) + 3;
            } else {
                wallpic = w3d.wl_draw.vertwall[(w3d.wl_draw.tilehit & !BIT_WALL) as usize];
            }
        } else {
            wallpic = w3d.wl_draw.vertwall[w3d.wl_draw.tilehit as usize];
        }

        w3d.wl_draw.postsource = PM_GetPage(w3d, wallpic);
        w3d.wl_draw.postsource_i = texture;
    }

    ScalePost(w3d);
}

/*
====================
=
= HitHorizWall
=
= tilehit bit 7 is 0, because it's not a door tile
= if bit 6 is 1 and the adjacent tile is a door tile, use door side pic
=
====================
*/

pub fn HitHorizWall(w3d: &mut modules) {
    //println!("HitHorizWall");

    let wallpic: i32;
    let mut texture: i32;

    texture =
        ((w3d.wl_draw.xintercept - w3d.wl_draw.texdelta) >> FIXED2TEXSHIFT) & TEXTUREMASK as i32;

    if w3d.wl_draw.ytilestep == -1 {
        w3d.wl_draw.yintercept += TILEGLOBAL;
    } else {
        texture = TEXTUREMASK - texture;
    }

    w3d.wl_draw.wallheight[w3d.wl_draw.pixx as usize] = CalcHeight(w3d);
    w3d.wl_draw.postx = w3d.wl_draw.pixx;

    if w3d.wl_draw.lastside == 0
        && w3d.wl_draw.lasttilehit == w3d.wl_draw.tilehit
        && (w3d.wl_draw.tilehit & BIT_WALL) == 0
    {
        //
        // in the same wall type as last time, so use the last postsource
        //
        if texture != w3d.wl_draw.lasttexture {
            w3d.wl_draw.postsource_i += texture - w3d.wl_draw.lasttexture;
            w3d.wl_draw.lasttexture = texture;
        }
    } else {
        w3d.wl_draw.lastside = 0;
        w3d.wl_draw.lasttilehit = w3d.wl_draw.tilehit;
        w3d.wl_draw.lasttexture = texture;

        if w3d.wl_draw.tilehit & BIT_WALL != 0 {
            //
            // check for adjacent doors
            //
            if w3d.wl_play.tilemap[w3d.wl_draw.xinttile as usize]
                [(w3d.wl_draw.ytile - w3d.wl_draw.ytilestep) as usize]
                & BIT_DOOR
                != 0
            {
                wallpic = DOORWALL(w3d) + 2;
            } else {
                wallpic = w3d.wl_draw.horizwall[(w3d.wl_draw.tilehit & !BIT_WALL) as usize];
            }
        } else {
            wallpic = w3d.wl_draw.horizwall[w3d.wl_draw.tilehit as usize];
        }

        w3d.wl_draw.postsource = PM_GetPage(w3d, wallpic);
        w3d.wl_draw.postsource_i = texture;
    }

    ScalePost(w3d);
}

//==========================================================================

/*
====================
=
= HitHorizDoor
=
====================
*/

pub fn HitHorizDoor(w3d: &mut modules) {
    //println!("HitHorizDoor");

    let doorpage: i32;
    let doornum: i32;
    let texture: i32;

    doornum = w3d.wl_draw.tilehit & !BIT_DOOR;
    texture = ((w3d.wl_draw.xintercept - w3d.wl_act1.doorposition[doornum as usize])
        >> FIXED2TEXSHIFT)
        & TEXTUREMASK;

    w3d.wl_draw.wallheight[w3d.wl_draw.pixx as usize] = CalcHeight(w3d);
    w3d.wl_draw.postx = w3d.wl_draw.pixx;

    if w3d.wl_draw.lasttilehit == w3d.wl_draw.tilehit {
        //
        // in the same door as last time, so use the last postsource
        //
        if texture != w3d.wl_draw.lasttexture {
            w3d.wl_draw.postsource_i += texture - w3d.wl_draw.lasttexture;
            w3d.wl_draw.lasttexture = texture;
        }
    } else {
        w3d.wl_draw.lasttilehit = w3d.wl_draw.tilehit;
        w3d.wl_draw.lasttexture = texture;

        let door_type = door_t::from_i32(w3d.wl_act1.doorobjlist[doornum as usize].lock);

        match door_type {
            door_t::dr_normal => {
                doorpage = DOORWALL(w3d);
            }

            door_t::dr_lock1 | door_t::dr_lock2 | door_t::dr_lock3 | door_t::dr_lock4 => {
                doorpage = DOORWALL(w3d) + 6;
            }

            door_t::dr_elevator => {
                doorpage = DOORWALL(w3d) + 4;
            }
        }

        w3d.wl_draw.postsource = PM_GetPage(w3d, doorpage);
        w3d.wl_draw.postsource_i = texture;
    }

    ScalePost(w3d);
}

//==========================================================================

/*
====================
=
= HitVertDoor
=
====================
*/

pub fn HitVertDoor(w3d: &mut modules) {
    //println!("HitVertDoor");

    let mut doorpage: i32 = 0;
    let doornum: i32;
    let texture: i32;

    doornum = w3d.wl_draw.tilehit & !BIT_DOOR;
    texture = ((w3d.wl_draw.yintercept - w3d.wl_act1.doorposition[doornum as usize])
        >> FIXED2TEXSHIFT)
        & TEXTUREMASK;

    w3d.wl_draw.wallheight[w3d.wl_draw.pixx as usize] = CalcHeight(w3d);
    w3d.wl_draw.postx = w3d.wl_draw.pixx;

    if w3d.wl_draw.lasttilehit == w3d.wl_draw.tilehit {
        //
        // in the same door as last time, so use the last postsource
        //
        if texture != w3d.wl_draw.lasttexture {
            w3d.wl_draw.postsource_i += texture - w3d.wl_draw.lasttexture;
            w3d.wl_draw.lasttexture = texture;
        }
    } else {
        w3d.wl_draw.lasttilehit = w3d.wl_draw.tilehit;
        w3d.wl_draw.lasttexture = texture;

        match w3d.wl_act1.doorobjlist[doornum as usize].lock {
            //door_t::dr_normal
            0 => {
                doorpage = DOORWALL(w3d) + 1;
            }

            //door_t::dr_lock1 | door_t::dr_lock2 | door_t::dr_lock3 | door_t::dr_lock4
            1 | 2 | 3 | 4 => {
                doorpage = DOORWALL(w3d) + 7;
            }

            //door_t::dr_elevator
            5 => {
                doorpage = DOORWALL(w3d) + 5;
            }
            _ => (),
        }

        w3d.wl_draw.postsource = PM_GetPage(w3d, doorpage);
        w3d.wl_draw.postsource_i = texture;
    }

    ScalePost(w3d);
}

/*
=====================
=
= VGAClearScreen
=
=====================
*/

pub fn VGAClearScreen(w3d: &mut modules) {
    //println!("VGAClearScreen");

    let ceiling =
        vgaCeiling[(w3d.wl_game.gamestate.episode * 10 + w3d.wl_game.gamestate.mapon) as usize];
    let mut dest_i = w3d.wl_main.screenofs as usize;
    //byte *dest = vbuf;

    w3d.id_vl.screenBuffer.with_lock_mut(|dest: &mut [u8]| {
        for _y in 0..w3d.wl_main.viewheight as usize / 2 {
            for x in 0..w3d.wl_main.viewwidth as usize {
                dest[dest_i + x] = ceiling as u8;
            }
            dest_i += w3d.id_vl.bufferPitch as usize;
        }
        for _y in w3d.wl_main.viewheight as usize / 2..w3d.wl_main.viewheight as usize {
            for x in 0..w3d.wl_main.viewwidth as usize {
                dest[dest_i + x] = COL_FLOOR;
            }
            dest_i += w3d.id_vl.bufferPitch as usize;
        }
    });
}

//==========================================================================

/*
=====================
=
= CalcRotate
=
=====================
*/

pub fn CalcRotate(w3d: &mut modules, ob: &mut object, obj: &mut objtype) -> i32 {
    //println!("CalcRotate");

    let mut angle: i32;
    let viewangle: i32;

    // this isn't exactly correct, as it should vary by a trig value,
    // but it is close enough with only eight rotations

    viewangle = (ob.objlist[0].angle as f32
        + (w3d.wl_main.centerx as f32 - obj.viewx as f32)
            / (8.0 * w3d.wl_main.viewwidth as f32 / 320.0)) as i32;

    if obj.obclass == classtype::rocketobj || obj.obclass == classtype::hrocketobj {
        angle = (viewangle - 180) - obj.angle;
    } else {
        angle = (viewangle - 180) - w3d.wl_main.dirangle[obj.dir as usize];
    }

    angle += ANGLES / 16;
    while angle >= ANGLES {
        angle -= ANGLES;
    }
    while angle < 0 {
        angle += ANGLES;
    }

    if obj.state.rotate == 2 {
        // 2 rotation pain frame
        return 0; // pain with shooting frame bugfix
    }

    return angle / (ANGLES / 8);
}

/*
=====================
=
= DrawScaleds
=
= Draws all objects that are visable
=
=====================
*/

pub fn DrawScaleds(w3d: &mut modules, ob: &mut object) {
    //println!("DrawScaleds");

    let mut least: i32;
    let numvisable: i32;
    let mut height: i32;
    let mut visspot: [[bool; 64]; 64];
    let mut tilespot: [[i32; 64]; 64];
    let _spotloc: i32;

    let mut statptr: statobj_t;

    //visptr = &vislist[0];
    w3d.wl_draw.vislist_i = 0;
    w3d.wl_draw.visptr = w3d.wl_draw.vislist.clone();
    w3d.wl_draw.visptr_i = w3d.wl_draw.vislist_i;

    //
    // place static objects
    //

    for i in 0..w3d.wl_act1.laststatobj_i {
        statptr = w3d.wl_act1.statobjlist[i];

        w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum = statptr.shapenum as i16;

        if w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum == -1 {
            continue; // object has been deleted
        }

        if !unsafe { *statptr.visspot } {
            continue; // not visable
        }

        if TransformTile(w3d, ob, statptr.tilex, statptr.tiley)
            && (statptr.flags & objflag_t::FL_BONUS as i32) != 0
        {
            GetBonus(w3d, &mut statptr);
            w3d.wl_act1.statobjlist[i] = statptr; //update
            if statptr.shapenum == -1 {
                continue; // object has been taken
            }
        }

        if w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].viewheight == 0 {
            continue; // to close to the object
        }

        if w3d.wl_draw.visptr_i < (w3d.wl_draw.vislist_i + MAXVISABLE as usize - 1)
        // don't let it overflow
        {
            w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].flags = statptr.flags as i16;
            w3d.wl_draw.visptr_i += 1;
        }
    }

    //
    // place active objects
    //

    for i in 1..ob.objlist.len() {
        w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum = ob.objlist[i].state.shapenum as i16;
        if w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum == 0 {
            continue; // no shape
        }

        visspot = w3d.wl_play.spotvis.clone();
        tilespot = w3d.wl_play.tilemap.clone();
        let x = ob.objlist[i].tilex as usize;
        let y = ob.objlist[i].tiley as usize;
        //
        // could be in any of the nine surrounding tiles
        //
        if visspot[x][y]
            || visspot[x - 1][y - 1] && tilespot[x - 1][y - 1] == 0
            || visspot[x][y - 1] && tilespot[x][y - 1] == 0
            || visspot[x + 1][y - 1] && tilespot[x + 1][y - 1] == 0
            || visspot[x - 1][y] && tilespot[x - 1][y] == 0
            || visspot[x + 1][y] && tilespot[x + 1][y] == 0
            || visspot[x - 1][y + 1] && tilespot[x - 1][y + 1] == 0
            || visspot[x][y + 1] && tilespot[x][y + 1] == 0
            || visspot[x + 1][y + 1] && tilespot[x + 1][y + 1] == 0
        {
            ob.objlist[i].active = activetype::ac_yes;
            TransformActor(w3d, &mut ob.objlist[i]);
            if ob.objlist[i].viewheight == 0 {
                continue; // too close or far away
            }

            w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].viewx = ob.objlist[i].viewx as i16;
            w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].viewheight = ob.objlist[i].viewheight as i16;

            if w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum == -1 {
                w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum = ob.objlist[i].temp1 as i16;
                // special shape
            }

            if ob.objlist[i].state.rotate != 0 {
                let mut obj = ob.objlist[i];
                w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].shapenum +=
                    CalcRotate(w3d, ob, &mut obj) as i16;
            }
            if w3d.wl_draw.visptr_i < w3d.wl_draw.vislist_i + MAXVISABLE as usize - 1
            // don't let it overflow
            {
                w3d.wl_draw.visptr[w3d.wl_draw.visptr_i].flags = ob.objlist[i].flags as i16;
                w3d.wl_draw.visptr_i += 1;
            }
            ob.objlist[i].flags |= objflag_t::FL_VISABLE as i32;
        } else {
            ob.objlist[i].flags &= !(objflag_t::FL_VISABLE as i32);
        }
    }

    //
    // draw from back to front
    //
    numvisable = (w3d.wl_draw.visptr_i - w3d.wl_draw.vislist_i) as i32;

    if numvisable == 0 {
        return;
    } // no visable objects

    let mut mark: usize = 0;
    w3d.wl_draw.visstep = w3d.wl_draw.visptr.clone();
    for _i in 0..numvisable {
        least = 32000;
        w3d.wl_draw.vislist_i = 0;
        w3d.wl_draw.visstep_i = w3d.wl_draw.vislist_i;

        for _i in w3d.wl_draw.visstep_i..w3d.wl_draw.visptr_i {
            height = w3d.wl_draw.visstep[w3d.wl_draw.visstep_i].viewheight as i32;

            if height < least {
                least = height;
                w3d.wl_draw.farthest = w3d.wl_draw.visstep[w3d.wl_draw.visstep_i];
                mark = w3d.wl_draw.visstep_i;
            }
            w3d.wl_draw.visstep_i += 1;
        }
        //
        // draw farthest
        //
        ScaleShape(
            w3d,
            w3d.wl_draw.farthest.viewx as i32,
            w3d.wl_draw.farthest.shapenum as i32,
            w3d.wl_draw.farthest.viewheight as i32,
            w3d.wl_draw.farthest.flags as i32,
        );
        w3d.wl_draw.farthest.viewheight = 32000;
        w3d.wl_draw.visstep[mark].viewheight = 32000;
    }
}

//==========================================================================

/*
==============
=
= DrawPlayerWeapon
=
= Draw the player's hands
=
==============
*/

pub fn DrawPlayerWeapon(w3d: &mut modules, ob: &mut object) {
    //println!("DrawPlayerWeapon");

    //let player = ob.objlist[0];

    let shapenum: i32;

    if w3d.wl_game.gamestate.victoryflag {
        let time = GetTimeCount(w3d) & 32;
        if ob.objlist[0].state == s_deathcam && time != 0 {
            SimpleScaleShape(
                w3d,
                w3d.wl_main.viewwidth / 2,
                SPRITES::SPR_DEATHCAM as i32,
                w3d.wl_main.viewheight + 1,
            );
        }
        return;
    }

    if w3d.wl_game.gamestate.weapon as i8 != -1 {
        shapenum = w3d.wl_draw.weaponscale[w3d.wl_game.gamestate.weapon as usize]
            + w3d.wl_game.gamestate.weaponframe as i32;

        SimpleScaleShape(
            w3d,
            w3d.wl_main.viewwidth / 2,
            shapenum,
            w3d.wl_main.viewheight + 1,
        );
    }
    if w3d.wl_play.demorecord || w3d.wl_play.demoplayback {
        SimpleScaleShape(
            w3d,
            w3d.wl_main.viewwidth / 2,
            SPRITES::SPR_DEMO as i32,
            w3d.wl_main.viewheight + 1,
        );
    }
}

//==========================================================================

/*
=====================
=
= CalcTics
=
=====================
*/

pub fn CalcTics(w3d: &mut modules) {
    //println!("CalcTics");

    //
    // calculate tics since last refresh for adaptive timing
    //
    if w3d.wl_draw.lasttimecount > GetTimeCount(w3d) {
        w3d.wl_draw.lasttimecount = GetTimeCount(w3d); // if the game was paused a LONG time
    }

    let curtime = SDL_GetTicks(w3d);
    w3d.wl_play.tics = (curtime * 7) / 100 - w3d.wl_draw.lasttimecount;
    if w3d.wl_play.tics == 0 {
        // wait until end of current tic
        SDL_Delay(w3d, ((w3d.wl_draw.lasttimecount + 1) * 100) / 7 - curtime);
        w3d.wl_play.tics = 1;
    }

    w3d.wl_draw.lasttimecount += w3d.wl_play.tics;

    if w3d.wl_play.tics > MAXTICS {
        w3d.wl_play.tics = MAXTICS;
    }
}

//==========================================================================

/*
=====================
=
= WallRefresh
=
= For each column of pixels on screen, cast a ray from the player
= to the nearest wall
=
= There is no check to stop the ray from passing outside of the map
= boundaries, so it is the map designer's responsibility to make sure there
= are solid walls covering the entire outer edge of each map
=
=====================
*/

pub fn WallRefresh(w3d: &mut modules, _ob: &mut object) {
    //println!("WallRefresh");

    for pixx in 0..w3d.wl_main.viewwidth {
        w3d.wl_draw.pixx = pixx;
        //
        // setup to trace a ray through pixx view pixel
        //
        w3d.wl_draw.angle = w3d.wl_draw.midangle + w3d.wl_draw.pixelangle[pixx as usize]; // delta for this pixel

        if w3d.wl_draw.angle < 0 {
            // -90 - -1 degree arc
            w3d.wl_draw.angle += ANG360; // -90 is the same as 270
        }
        if w3d.wl_draw.angle >= ANG360 {
            // 360-449 degree arc
            w3d.wl_draw.angle -= ANG360;
        } // -449 is the same as 89
          //
          // setup xstep/ystep based on angle
          //
        if w3d.wl_draw.angle < ANG90
        // 0-89 degree arc
        {
            w3d.wl_draw.xtilestep = 1;
            w3d.wl_draw.ytilestep = -1;
            w3d.wl_draw.xstep = w3d.wl_draw.finetangent[(ANG90 - 1 - w3d.wl_draw.angle) as usize];
            w3d.wl_draw.ystep = -w3d.wl_draw.finetangent[w3d.wl_draw.angle as usize];
            w3d.wl_draw.xpartial = w3d.wl_draw.xpartialup;
            w3d.wl_draw.ypartial = w3d.wl_draw.ypartialdown;
        } else if w3d.wl_draw.angle < ANG180
        // 90-179 degree arc
        {
            w3d.wl_draw.xtilestep = -1;
            w3d.wl_draw.ytilestep = -1;
            w3d.wl_draw.xstep = -w3d.wl_draw.finetangent[(w3d.wl_draw.angle - ANG90) as usize];
            w3d.wl_draw.ystep = -w3d.wl_draw.finetangent[(ANG180 - 1 - w3d.wl_draw.angle) as usize];
            w3d.wl_draw.xpartial = w3d.wl_draw.xpartialdown;
            w3d.wl_draw.ypartial = w3d.wl_draw.ypartialdown;
        } else if w3d.wl_draw.angle < ANG270
        // 180-269 degree arc
        {
            w3d.wl_draw.xtilestep = -1;
            w3d.wl_draw.ytilestep = 1;
            w3d.wl_draw.xstep = -w3d.wl_draw.finetangent[(ANG270 - 1 - w3d.wl_draw.angle) as usize];
            w3d.wl_draw.ystep = w3d.wl_draw.finetangent[(w3d.wl_draw.angle - ANG180) as usize];
            w3d.wl_draw.xpartial = w3d.wl_draw.xpartialdown;
            w3d.wl_draw.ypartial = w3d.wl_draw.ypartialup;
        } else if w3d.wl_draw.angle < ANG360
        // 270-359 degree arc
        {
            w3d.wl_draw.xtilestep = 1;
            w3d.wl_draw.ytilestep = 1;
            w3d.wl_draw.xstep = w3d.wl_draw.finetangent[(w3d.wl_draw.angle - ANG270) as usize];
            w3d.wl_draw.ystep = w3d.wl_draw.finetangent[(ANG360 - 1 - w3d.wl_draw.angle) as usize];
            w3d.wl_draw.xpartial = w3d.wl_draw.xpartialup;
            w3d.wl_draw.ypartial = w3d.wl_draw.ypartialup;
        }

        //
        // initialise variables for intersection testing
        //
        w3d.wl_draw.yintercept =
            FixedMul(w3d.wl_draw.ystep, w3d.wl_draw.xpartial) + w3d.wl_draw.viewy;
        w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;

        w3d.wl_draw.xtile = w3d.wl_draw.focaltx + w3d.wl_draw.xtilestep;
        w3d.wl_draw.xintercept =
            FixedMul(w3d.wl_draw.xstep, w3d.wl_draw.ypartial) + w3d.wl_draw.viewx;

        w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
        w3d.wl_draw.ytile = w3d.wl_draw.focalty + w3d.wl_draw.ytilestep;

        w3d.wl_draw.texdelta = 0;

        //
        // special treatment when player is in back tile of pushwall
        //
        if w3d.wl_play.tilemap[w3d.wl_draw.focaltx as usize][w3d.wl_draw.focalty as usize]
            == BIT_WALL
        {
            if (w3d.wl_act1.pwalldir == controldir_t::di_east && w3d.wl_draw.xtilestep == 1)
                || (w3d.wl_act1.pwalldir == controldir_t::di_west && w3d.wl_draw.xtilestep == -1)
            {
                // allow multiply with overflow
                w3d.wl_draw.yinttemp = w3d.wl_draw.yintercept
                    - ((w3d.wl_draw.ystep.wrapping_mul(64 - w3d.wl_act1.pwallpos)) >> 6);
                //
                //  trace hit vertical pushwall back?
                //
                if (w3d.wl_draw.yinttemp >> TILESHIFT) == w3d.wl_draw.focalty {
                    if w3d.wl_act1.pwalldir == controldir_t::di_east {
                        w3d.wl_draw.xintercept =
                            ((w3d.wl_draw.focaltx) << TILESHIFT) + ((w3d.wl_act1.pwallpos) << 10);
                    } else {
                        w3d.wl_draw.xintercept = ((w3d.wl_draw.focaltx) << TILESHIFT)
                            - TILEGLOBAL as i32
                            + ((64 - w3d.wl_act1.pwallpos) << 10);
                    }

                    w3d.wl_draw.yintercept = w3d.wl_draw.yinttemp;
                    w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;
                    w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                    HitVertWall(w3d);
                    continue;
                }
            } else if w3d.wl_act1.pwalldir == controldir_t::di_south && w3d.wl_draw.ytilestep == 1
                || w3d.wl_act1.pwalldir == controldir_t::di_north && w3d.wl_draw.ytilestep == -1
            {
                w3d.wl_draw.xinttemp = w3d.wl_draw.xintercept
                    - ((w3d.wl_draw.xstep * (64 - w3d.wl_act1.pwallpos)) >> 6);
                //
                // trace hit horizontal pushwall back?
                //
                if (w3d.wl_draw.xinttemp >> TILESHIFT) == w3d.wl_draw.focaltx {
                    if w3d.wl_act1.pwalldir == controldir_t::di_south {
                        w3d.wl_draw.yintercept =
                            ((w3d.wl_draw.focalty) << TILESHIFT) + ((w3d.wl_act1.pwallpos) << 10);
                    } else {
                        w3d.wl_draw.yintercept = ((w3d.wl_draw.focalty) << TILESHIFT)
                            - TILEGLOBAL as i32
                            + ((64 - w3d.wl_act1.pwallpos) << 10);
                    }

                    w3d.wl_draw.xintercept = w3d.wl_draw.xinttemp;
                    w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
                    w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                    HitHorizWall(w3d);
                    continue;
                }
            }
        }

        //
        // trace along this angle until we hit a wall
        //
        // CORE LOOP!
        //
        'core: loop {
            'vertentry: loop {
                //
                // check intersections with vertical walls
                //
                if (w3d.wl_draw.ytilestep == -1 && w3d.wl_draw.yinttile <= w3d.wl_draw.ytile)
                    || (w3d.wl_draw.ytilestep == 1 && w3d.wl_draw.yinttile >= w3d.wl_draw.ytile)
                {
                    horizentry(w3d);
                    if w3d.wl_draw.breakcore {
                        w3d.wl_draw.breakcore = false;
                        break 'core;
                    } else {
                        break 'vertentry;
                    }
                }
                vertentry(w3d);
                if w3d.wl_draw.breakcore {
                    w3d.wl_draw.breakcore = false;
                    break 'core;
                }
            }

            'horizentry: loop {
                //
                // check intersections with horizontal walls
                //
                if (w3d.wl_draw.xtilestep == -1 && w3d.wl_draw.xinttile <= w3d.wl_draw.xtile)
                    || (w3d.wl_draw.xtilestep == 1 && w3d.wl_draw.xinttile >= w3d.wl_draw.xtile)
                {
                    vertentry(w3d);
                    if w3d.wl_draw.breakcore {
                        w3d.wl_draw.breakcore = false;
                        break 'core;
                    } else {
                        break 'horizentry;
                    }
                }
                horizentry(w3d);
                if w3d.wl_draw.breakcore {
                    w3d.wl_draw.breakcore = false;
                    break 'core;
                }
            }
        }
    }
}

pub fn vertentry(w3d: &mut modules) {
    //println!("vertentry");

    //
    // get the wall value from tilemap
    //
    if w3d.wl_play.tilemap[w3d.wl_draw.xtile as usize][w3d.wl_draw.ytile as usize] != 0
        && (w3d.wl_draw.xtile - w3d.wl_draw.xtilestep) == w3d.wl_draw.xinttile
        && (w3d.wl_draw.ytile - w3d.wl_draw.ytilestep) == w3d.wl_draw.yinttile
    {
        //
        // exactly in the wall corner, so use the last tile
        //
        w3d.wl_draw.tilehit = w3d.wl_draw.lasttilehit;

        if (w3d.wl_draw.tilehit & BIT_DOOR) != 0 {
            w3d.wl_draw.passdoor = false; // don't let the trace continue if it's a door
        }
    } else {
        w3d.wl_draw.tilehit =
            w3d.wl_play.tilemap[w3d.wl_draw.xtile as usize][w3d.wl_draw.yinttile as usize];
        if (w3d.wl_draw.tilehit & BIT_DOOR) != 0 {
            w3d.wl_draw.passdoor = true;
        }
    }

    if w3d.wl_draw.tilehit != 0 {
        if (w3d.wl_draw.tilehit & BIT_DOOR) != 0 {
            //
            // hit a vertical door, so find which coordinate the door would be
            // intersected at, and check to see if the door is open past that point
            //
            w3d.wl_draw.door = w3d.wl_act1.doorobjlist[(w3d.wl_draw.tilehit & !BIT_DOOR) as usize];

            if w3d.wl_draw.door.action == doortype::dr_open {
                //goto passvert;                       // door is open, continue tracing
                passvert(w3d);
                return;
            }

            w3d.wl_draw.yinttemp = w3d.wl_draw.yintercept + (w3d.wl_draw.ystep >> 1); // add halfstep to current intercept position

            //
            // midpoint is outside tile, so it hit the side of the wall before a door
            //
            if (w3d.wl_draw.yinttemp >> TILESHIFT) != w3d.wl_draw.yinttile && w3d.wl_draw.passdoor {
                //goto passvert;
                passvert(w3d);
                return;
            }

            if w3d.wl_draw.door.action != doortype::dr_closed {
                //
                // the trace hit the door plane at pixel position yintercept, see if the door is
                // closed that much
                //
                if (w3d.wl_draw.yinttemp as u16)
                    < (w3d.wl_act1.doorposition
                        [(w3d.wl_draw.tilehit as u16 & !BIT_DOOR as u16) as usize]
                        as u16)
                {
                    //goto passvert;
                    passvert(w3d);
                    return;
                }
            }

            w3d.wl_draw.yintercept = w3d.wl_draw.yinttemp;
            w3d.wl_draw.xintercept = ((w3d.wl_draw.xtile) << TILESHIFT) + (TILEGLOBAL / 2);
            HitVertDoor(w3d);
            w3d.wl_draw.breakcore = true;
            return;
        } else if w3d.wl_draw.tilehit == BIT_WALL {
            //
            // hit a sliding vertical wall
            //
            if w3d.wl_act1.pwalldir == controldir_t::di_west
                || w3d.wl_act1.pwalldir == controldir_t::di_east
            {
                if w3d.wl_act1.pwalldir == controldir_t::di_west {
                    w3d.wl_draw.pwallposnorm = 64 - w3d.wl_act1.pwallpos;
                    w3d.wl_draw.pwallposinv = w3d.wl_act1.pwallpos;
                } else {
                    w3d.wl_draw.pwallposnorm = w3d.wl_act1.pwallpos;
                    w3d.wl_draw.pwallposinv = 64 - w3d.wl_act1.pwallpos;
                }

                if w3d.wl_act1.pwalldir == controldir_t::di_east
                    && w3d.wl_draw.xtile == w3d.wl_act1.pwallx
                    && w3d.wl_draw.yinttile == w3d.wl_act1.pwally
                    || w3d.wl_act1.pwalldir == controldir_t::di_west
                        && !(w3d.wl_draw.xtile == w3d.wl_act1.pwallx
                            && w3d.wl_draw.yinttile == w3d.wl_act1.pwally)
                {
                    w3d.wl_draw.yinttemp = w3d.wl_draw.yintercept
                        + ((w3d.wl_draw.ystep * w3d.wl_draw.pwallposnorm) >> 6);

                    if (w3d.wl_draw.yinttemp >> TILESHIFT) != w3d.wl_draw.yinttile {
                        //goto passvert;
                        passvert(w3d);
                        return;
                    }

                    w3d.wl_draw.yintercept = w3d.wl_draw.yinttemp;
                    w3d.wl_draw.xintercept = ((w3d.wl_draw.xtile) << TILESHIFT) + TILEGLOBAL
                        - ((w3d.wl_draw.pwallposinv) << 10);
                    w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;

                    w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                    HitVertWall(w3d);
                    w3d.wl_draw.breakcore = true;
                    return;
                } else {
                    // allow multiply with overflow
                    w3d.wl_draw.yinttemp = w3d.wl_draw.yintercept
                        + ((w3d.wl_draw.ystep.wrapping_mul(w3d.wl_draw.pwallposinv)) >> 6);

                    if (w3d.wl_draw.yinttemp >> TILESHIFT) != w3d.wl_draw.yinttile {
                        //goto passvert;
                        passvert(w3d);
                        return;
                    }

                    w3d.wl_draw.yintercept = w3d.wl_draw.yinttemp;
                    w3d.wl_draw.xintercept =
                        ((w3d.wl_draw.xtile) << TILESHIFT) - ((w3d.wl_draw.pwallposinv) << 10);
                    w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;

                    w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                    HitVertWall(w3d);
                    w3d.wl_draw.breakcore = true;
                    return;
                }
            } else {
                if w3d.wl_act1.pwalldir == controldir_t::di_north {
                    w3d.wl_draw.pwallposi = 64 - w3d.wl_act1.pwallpos;
                } else {
                    w3d.wl_draw.pwallposi = w3d.wl_act1.pwallpos;
                }

                if w3d.wl_act1.pwalldir == controldir_t::di_south
                    && w3d.wl_draw.yintercept < (w3d.wl_draw.pwallposi << 10)
                    || w3d.wl_act1.pwalldir == controldir_t::di_north
                        && w3d.wl_draw.yintercept > (w3d.wl_draw.pwallposi << 10)
                {
                    if w3d.wl_draw.xtile == w3d.wl_act1.pwallx
                        && w3d.wl_draw.yinttile == w3d.wl_act1.pwally
                    {
                        if w3d.wl_act1.pwalldir == controldir_t::di_south
                            && w3d.wl_draw.yintercept + w3d.wl_draw.ystep
                                < (w3d.wl_draw.pwallposi << 10)
                            || w3d.wl_act1.pwalldir == controldir_t::di_north
                                && w3d.wl_draw.yintercept + w3d.wl_draw.ystep
                                    > (w3d.wl_draw.pwallposi << 10)
                        {
                            //goto passvert;
                            passvert(w3d);
                            return;
                        }

                        //
                        // set up a horizontal intercept position
                        //
                        if w3d.wl_act1.pwalldir == controldir_t::di_south {
                            w3d.wl_draw.yintercept =
                                (w3d.wl_draw.yinttile << TILESHIFT) + (w3d.wl_draw.pwallposi << 10);
                        } else {
                            w3d.wl_draw.yintercept = ((w3d.wl_draw.yinttile << TILESHIFT)
                                - TILEGLOBAL)
                                + (w3d.wl_draw.pwallposi << 10);
                        }

                        w3d.wl_draw.xintercept -=
                            (w3d.wl_draw.xstep * (64 - w3d.wl_act1.pwallpos)) >> 6;
                        w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitHorizWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    } else {
                        w3d.wl_draw.texdelta = (w3d.wl_draw.pwallposi) << 10;
                        w3d.wl_draw.xintercept = (w3d.wl_draw.xtile) << TILESHIFT;
                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitVertWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    }
                } else {
                    if w3d.wl_draw.xtile == w3d.wl_act1.pwallx
                        && w3d.wl_draw.yinttile == w3d.wl_act1.pwally
                    {
                        w3d.wl_draw.texdelta = (w3d.wl_draw.pwallposi) << 10;
                        w3d.wl_draw.xintercept = (w3d.wl_draw.xtile) << TILESHIFT;
                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitVertWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    } else {
                        if w3d.wl_act1.pwalldir == controldir_t::di_south
                            && (w3d.wl_draw.yintercept) + w3d.wl_draw.ystep
                                > (w3d.wl_draw.pwallposi << 10)
                            || w3d.wl_act1.pwalldir == controldir_t::di_north
                                && (w3d.wl_draw.yintercept) + w3d.wl_draw.ystep
                                    < (w3d.wl_draw.pwallposi << 10)
                        {
                            //goto passvert;
                            passvert(w3d);
                            return;
                        }

                        //
                        // set up a horizontal intercept position
                        //
                        if w3d.wl_act1.pwalldir == controldir_t::di_south {
                            w3d.wl_draw.yintercept = (w3d.wl_draw.yinttile << TILESHIFT)
                                - (64 - w3d.wl_act1.pwallpos)
                                << 10;
                        } else {
                            w3d.wl_draw.yintercept = (w3d.wl_draw.yinttile << TILESHIFT)
                                + (64 - w3d.wl_act1.pwallpos)
                                << 10;
                        }

                        w3d.wl_draw.xintercept -= (w3d.wl_draw.xstep * w3d.wl_act1.pwallpos) >> 6;
                        w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitHorizWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    }
                }
            }
        } else {
            w3d.wl_draw.xintercept = (w3d.wl_draw.xtile) << TILESHIFT;
            HitVertWall(w3d);
            w3d.wl_draw.breakcore = true;
            return;
        }
    }
    passvert(w3d);
}

pub fn passvert(w3d: &mut modules) {
    //println!("passvert");

    //passvert:
    //
    // mark the tile as visible and setup for next step
    //
    w3d.wl_play.spotvis[w3d.wl_draw.xtile as usize][w3d.wl_draw.yinttile as usize] = true;
    w3d.wl_draw.xtile += w3d.wl_draw.xtilestep;
    w3d.wl_draw.yintercept += w3d.wl_draw.ystep;
    w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;
}

pub fn horizentry(w3d: &mut modules) {
    //println!("horizentry");

    //
    // get the wall value from tilemap
    //
    if w3d.wl_play.tilemap[w3d.wl_draw.xtile as usize][w3d.wl_draw.ytile as usize] != 0
        && (w3d.wl_draw.xtile - w3d.wl_draw.xtilestep) == w3d.wl_draw.xinttile
        && (w3d.wl_draw.ytile - w3d.wl_draw.ytilestep) == w3d.wl_draw.yinttile
    {
        //
        // exactly in the wall corner, so use the last tile
        //
        w3d.wl_draw.tilehit = w3d.wl_draw.lasttilehit;

        if (w3d.wl_draw.tilehit & BIT_DOOR) != 0 {
            w3d.wl_draw.passdoor = false; // don't let the trace continue if it's a door
        }
    } else {
        w3d.wl_draw.tilehit =
            w3d.wl_play.tilemap[w3d.wl_draw.xinttile as usize][w3d.wl_draw.ytile as usize];
        if (w3d.wl_draw.tilehit & BIT_DOOR) != 0 {
            w3d.wl_draw.passdoor = true;
        }
    }

    if w3d.wl_draw.tilehit != 0 {
        if (w3d.wl_draw.tilehit & BIT_DOOR) != 0 {
            //
            // hit a horizontal door, so find which coordinate the door would be
            // intersected at, and check to see if the door is open past that point
            //
            w3d.wl_draw.door = w3d.wl_act1.doorobjlist[(w3d.wl_draw.tilehit & !BIT_DOOR) as usize];

            if w3d.wl_draw.door.action == doortype::dr_open {
                //goto passhoriz;                      // door is open, continue tracing
                passhoriz(w3d);
                return;
            }

            w3d.wl_draw.xinttemp = w3d.wl_draw.xintercept + (w3d.wl_draw.xstep >> 1); // add half step to current intercept position

            //
            // midpoint is outside tile, so it hit the side of the wall before a door
            //
            if (w3d.wl_draw.xinttemp >> TILESHIFT) != w3d.wl_draw.xinttile && w3d.wl_draw.passdoor {
                //goto passhoriz;
                passhoriz(w3d);
                return;
            }

            if w3d.wl_draw.door.action != doortype::dr_closed {
                //
                // the trace hit the door plane at pixel position w3d.wl_draw.xintercept, see if the door is
                // closed that much
                //
                if (w3d.wl_draw.xinttemp as u16)
                    < (w3d.wl_act1.doorposition[(w3d.wl_draw.tilehit & !BIT_DOOR) as usize] as u16)
                {
                    //goto passhoriz;
                    passhoriz(w3d);
                    return;
                }
            }

            w3d.wl_draw.xintercept = w3d.wl_draw.xinttemp;
            w3d.wl_draw.yintercept = (w3d.wl_draw.ytile << TILESHIFT) + (TILEGLOBAL / 2);
            HitHorizDoor(w3d);
            w3d.wl_draw.breakcore = true;
            return;
        } else if w3d.wl_draw.tilehit == BIT_WALL {
            //
            // hit a sliding horizontal wall
            //
            if w3d.wl_act1.pwalldir == controldir_t::di_north
                || w3d.wl_act1.pwalldir == controldir_t::di_south
            {
                if w3d.wl_act1.pwalldir == controldir_t::di_north {
                    w3d.wl_draw.pwallposnorm = 64 - w3d.wl_act1.pwallpos;
                    w3d.wl_draw.pwallposinv = w3d.wl_act1.pwallpos;
                } else {
                    w3d.wl_draw.pwallposnorm = w3d.wl_act1.pwallpos;
                    w3d.wl_draw.pwallposinv = 64 - w3d.wl_act1.pwallpos;
                }

                if w3d.wl_act1.pwalldir == controldir_t::di_south
                    && w3d.wl_draw.xinttile == w3d.wl_act1.pwallx
                    && w3d.wl_draw.ytile == w3d.wl_act1.pwally
                    || w3d.wl_act1.pwalldir == controldir_t::di_north
                        && !(w3d.wl_draw.xinttile == w3d.wl_act1.pwallx
                            && w3d.wl_draw.ytile == w3d.wl_act1.pwally)
                {
                    w3d.wl_draw.xinttemp = w3d.wl_draw.xintercept
                        + ((w3d.wl_draw.xstep * w3d.wl_draw.pwallposnorm) >> 6);

                    if (w3d.wl_draw.xinttemp >> TILESHIFT) != w3d.wl_draw.xinttile {
                        //goto passhoriz;
                        passhoriz(w3d);
                        return;
                    }

                    w3d.wl_draw.xintercept = w3d.wl_draw.xinttemp;
                    w3d.wl_draw.yintercept = ((w3d.wl_draw.ytile << TILESHIFT) + TILEGLOBAL)
                        - (w3d.wl_draw.pwallposinv << 10);
                    w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
                    w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                    HitHorizWall(w3d);
                    w3d.wl_draw.breakcore = true;
                    return;
                } else {
                    w3d.wl_draw.xinttemp = w3d.wl_draw.xintercept
                        + ((w3d.wl_draw.xstep * w3d.wl_draw.pwallposinv) >> 6);

                    if (w3d.wl_draw.xinttemp >> TILESHIFT) != w3d.wl_draw.xinttile {
                        //goto passhoriz;
                        passhoriz(w3d);
                        return;
                    }

                    w3d.wl_draw.xintercept = w3d.wl_draw.xinttemp;
                    w3d.wl_draw.yintercept =
                        (w3d.wl_draw.ytile << TILESHIFT) - (w3d.wl_draw.pwallposinv << 10);
                    w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
                    w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                    HitHorizWall(w3d);
                    w3d.wl_draw.breakcore = true;
                    return;
                }
            } else {
                if w3d.wl_act1.pwalldir == controldir_t::di_west {
                    w3d.wl_draw.pwallposi = 64 - w3d.wl_act1.pwallpos;
                } else {
                    w3d.wl_draw.pwallposi = w3d.wl_act1.pwallpos;
                }

                if w3d.wl_act1.pwalldir == controldir_t::di_east
                    && w3d.wl_draw.xintercept < (w3d.wl_draw.pwallposi << 10)
                    || w3d.wl_act1.pwalldir == controldir_t::di_west
                        && w3d.wl_draw.xintercept > (w3d.wl_draw.pwallposi << 10)
                {
                    if w3d.wl_draw.xinttile == w3d.wl_act1.pwallx
                        && w3d.wl_draw.ytile == w3d.wl_act1.pwally
                    {
                        if w3d.wl_act1.pwalldir == controldir_t::di_east
                            && (w3d.wl_draw.xintercept) + w3d.wl_draw.xstep
                                < (w3d.wl_draw.pwallposi << 10)
                            || w3d.wl_act1.pwalldir == controldir_t::di_west
                                && (w3d.wl_draw.xintercept) + w3d.wl_draw.xstep
                                    > (w3d.wl_draw.pwallposi << 10)
                        {
                            //goto passhoriz;
                            passhoriz(w3d);
                            return;
                        }

                        //
                        // set up a vertical intercept position
                        //
                        w3d.wl_draw.yintercept -=
                            (w3d.wl_draw.ystep * (64 - w3d.wl_act1.pwallpos)) >> 6;
                        w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;

                        if w3d.wl_act1.pwalldir == controldir_t::di_east {
                            w3d.wl_draw.xintercept =
                                (w3d.wl_draw.xinttile << TILESHIFT) + (w3d.wl_draw.pwallposi << 10);
                        } else {
                            w3d.wl_draw.xintercept = ((w3d.wl_draw.xinttile << TILESHIFT)
                                - TILEGLOBAL)
                                + (w3d.wl_draw.pwallposi << 10);
                        }

                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitVertWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    } else {
                        w3d.wl_draw.texdelta = w3d.wl_draw.pwallposi << 10;
                        w3d.wl_draw.yintercept = w3d.wl_draw.ytile << TILESHIFT;
                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitHorizWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    }
                } else {
                    if w3d.wl_draw.xinttile == w3d.wl_act1.pwallx
                        && w3d.wl_draw.ytile == w3d.wl_act1.pwally
                    {
                        w3d.wl_draw.texdelta = w3d.wl_draw.pwallposi << 10;
                        w3d.wl_draw.yintercept = w3d.wl_draw.ytile << TILESHIFT;
                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitHorizWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    } else {
                        if w3d.wl_act1.pwalldir == controldir_t::di_east
                            && (w3d.wl_draw.xintercept) + w3d.wl_draw.xstep
                                > (w3d.wl_draw.pwallposi << 10)
                            || w3d.wl_act1.pwalldir == controldir_t::di_west
                                && (w3d.wl_draw.xintercept) + w3d.wl_draw.xstep
                                    < (w3d.wl_draw.pwallposi << 10)
                        {
                            //goto passhoriz;
                            passhoriz(w3d);
                            return;
                        }

                        //
                        // set up a vertical intercept position
                        //
                        w3d.wl_draw.yintercept -= (w3d.wl_draw.ystep * w3d.wl_act1.pwallpos) >> 6;
                        w3d.wl_draw.yinttile = w3d.wl_draw.yintercept >> TILESHIFT;

                        if w3d.wl_act1.pwalldir == controldir_t::di_east {
                            w3d.wl_draw.xintercept = (w3d.wl_draw.xinttile << TILESHIFT)
                                - ((64 - w3d.wl_act1.pwallpos) << 10);
                        } else {
                            w3d.wl_draw.xintercept = (w3d.wl_draw.xinttile << TILESHIFT)
                                + ((64 - w3d.wl_act1.pwallpos) << 10);
                        }

                        w3d.wl_draw.tilehit = w3d.wl_act1.pwalltile;
                        HitVertWall(w3d);
                        w3d.wl_draw.breakcore = true;
                        return;
                    }
                }
            }
        } else {
            w3d.wl_draw.yintercept = (w3d.wl_draw.ytile) << TILESHIFT;
            HitHorizWall(w3d);
            w3d.wl_draw.breakcore = true;
            return;
        }
    }
    passhoriz(w3d);
}

pub fn passhoriz(w3d: &mut modules) {
    //println!("passhoriz");

    //passhoriz:
    //
    // mark the tile as visible and setup for next step
    //
    w3d.wl_play.spotvis[w3d.wl_draw.xinttile as usize][w3d.wl_draw.ytile as usize] = true;
    w3d.wl_draw.ytile += w3d.wl_draw.ytilestep;
    w3d.wl_draw.xintercept += w3d.wl_draw.xstep;
    w3d.wl_draw.xinttile = w3d.wl_draw.xintercept >> TILESHIFT;
}
/*
====================
=
= Setup3DView
=
====================
*/

pub fn Setup3DView(w3d: &mut modules, ob: &mut object) {
    //println!("Setup3DView");

    w3d.wl_draw.viewangle = ob.objlist[0].angle;
    w3d.wl_draw.midangle = w3d.wl_draw.viewangle * (FINEANGLES / ANGLES);

    w3d.wl_draw.viewsin = w3d.wl_draw.sintable[w3d.wl_draw.viewangle as usize];
    w3d.wl_draw.viewcos = w3d.wl_draw.costable[w3d.wl_draw.viewangle as usize];

    w3d.wl_draw.viewx = ob.objlist[0].x - FixedMul(w3d.wl_main.focallength, w3d.wl_draw.viewcos);
    w3d.wl_draw.viewy = ob.objlist[0].y + FixedMul(w3d.wl_main.focallength, w3d.wl_draw.viewsin);

    w3d.wl_draw.focaltx = w3d.wl_draw.viewx >> TILESHIFT;
    w3d.wl_draw.focalty = w3d.wl_draw.viewy >> TILESHIFT;

    w3d.wl_draw.xpartialdown = w3d.wl_draw.viewx & (TILEGLOBAL - 1);
    w3d.wl_draw.xpartialup = TILEGLOBAL - w3d.wl_draw.xpartialdown;
    w3d.wl_draw.ypartialdown = w3d.wl_draw.viewy & (TILEGLOBAL - 1);
    w3d.wl_draw.ypartialup = TILEGLOBAL - w3d.wl_draw.ypartialdown;

    w3d.wl_draw.lastside = -1; // no optimization on the first post
}

//==========================================================================

/*
========================
=
= ThreeDRefresh
=
========================
*/

pub fn ThreeDRefresh(w3d: &mut modules, ob: &mut object) {
    //println!("ThreeDRefresh");

    //
    // clear out the traced array
    //
    w3d.wl_play.spotvis = [[false; MAPSIZE as usize]; MAPSIZE as usize];

    if !(w3d.wl_play.demorecord || w3d.wl_play.demoplayback) {
        if w3d.wl_play.tilemap[ob.objlist[0].tilex as usize][ob.objlist[0].tiley as usize] == 0
            || (w3d.wl_play.tilemap[ob.objlist[0].tilex as usize][ob.objlist[0].tiley as usize]
                & BIT_DOOR)
                != 0
        {
            w3d.wl_play.spotvis[ob.objlist[0].tilex as usize][ob.objlist[0].tiley as usize] = true;
            // Detect all sprites over player fix
        }
    }

    w3d.wl_draw.vbuf = VL_LockSurface(&mut w3d.id_vl.screenBuffer);
    if w3d.wl_draw.vbuf.is_empty() {
        return;
    }

    //vbuf += screenofs;
    w3d.wl_draw.vbuf_i += w3d.wl_main.screenofs as usize;

    Setup3DView(w3d, ob);

    //
    // follow the walls from there to the right, drawing as we go
    //
    VGAClearScreen(w3d);

    WallRefresh(w3d, ob);

    //
    // draw all the scaled images
    //
    DrawScaleds(w3d, ob); // draw scaled stuff

    DrawPlayerWeapon(w3d, ob); // draw player's hands

    if Keyboard(w3d, Scancode::Tab)
        && w3d.wl_play.viewsize == 21
        && w3d.wl_game.gamestate.weapon as i8 != -1
    {
        ShowActStatus(w3d);
    }

    VL_UnlockSurface(&mut w3d.id_vl.screenBuffer);

    w3d.wl_draw.vbuf.clear();
    w3d.wl_draw.vbuf_i = 0;

    //
    // show screen and time last cycle
    //

    if w3d.wl_game.fizzlein {
        FizzleFade(
            w3d,
            0,
            0,
            w3d.id_vl.screenWidth,
            w3d.id_vl.screenHeight,
            20,
            false,
        );
        w3d.wl_game.fizzlein = false;

        w3d.wl_draw.lasttimecount = GetTimeCount(w3d); // don't make a big tic count
    } else {
        if w3d.wl_draw.fpscounter {
            w3d.id_vh.fontnumber = 0;
            SETFONTCOLOR(w3d, 7, 127);
            w3d.id_us.PrintX = 4;
            w3d.id_us.PrintY = 1;
            VWB_Bar(w3d, 0, 0, 50, 10, w3d.wl_game.bordercol);
            US_PrintSigned(w3d, w3d.wl_draw.fps);
            US_Print(w3d, "   fps".to_string());
        }
        VW_UpdateScreen(w3d);
    }

    if w3d.wl_draw.fpscounter {
        w3d.wl_draw.fps_frames += 1;
        w3d.wl_draw.fps_time += w3d.wl_play.tics;

        if w3d.wl_draw.fps_time > 35 {
            w3d.wl_draw.fps_time -= 35;
            w3d.wl_draw.fps = w3d.wl_draw.fps_frames << 1;
            w3d.wl_draw.fps_frames = 0;
        }
    }
}
