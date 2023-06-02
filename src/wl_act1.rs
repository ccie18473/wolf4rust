#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_act1
//
//===========================================================================

pub struct wl_act1 {
    pub statobjlist: [statobj_t; MAXSTATS as usize], //MAXSTATS
    pub statobjlist_i: usize,
    pub laststatobj: statobj_t,
    pub laststatobj_i: usize,

    pub doorobjlist: Vec<doorobj_t>, //MAXDOORS
    pub doorobjlist_i: usize,
    pub lastdoorobj: doorobj_t,

    pub doornum: i32,
    pub doorposition: [i32; MAXDOORS as usize], // leading edge of door 0=closed                                                // 0xffff = fully open
    pub areaconnect: [[i32; NUMAREAS as usize]; NUMAREAS as usize],
    pub areabyplayer: [bool; NUMAREAS as usize],
    pub pwallstate: i32,
    pub pwallpos: i32, // amount a pushable wall has been moved (0-63)
    pub pwallx: i32,
    pub pwally: i32,
    pub pwalldir: controldir_t,
    pub pwalltile: tiletype,
    pub dirs: [[i8; 2]; 4],
}

impl wl_act1 {
    pub fn new() -> Self {
        Self {
            statobjlist: [statobj_t::new(); MAXSTATS as usize],
            statobjlist_i: 0,
            laststatobj: statobj_t::new(),
            laststatobj_i: 0,

            doorobjlist: Vec::new(),
            doorobjlist_i: 0,
            lastdoorobj: doorobj_t::new(),
            doornum: 0,

            doorposition: [0; MAXDOORS as usize], // leading edge of door 0=closed                                                // 0xffff = fully open
            areaconnect: [[0; NUMAREAS as usize]; NUMAREAS as usize],
            areabyplayer: [false; NUMAREAS as usize],
            pwallstate: 0,
            pwallpos: 0,
            pwallx: 0,
            pwally: 0,
            pwalldir: controldir_t::di_none,
            pwalltile: 0,
            dirs: [[0, -1], [1, 0], [0, 1], [-1, 0]],
        }
    }
    pub fn clear(&mut self) {
        self.statobjlist = [statobj_t::new(); MAXSTATS as usize];
        self.statobjlist_i = 0;
        self.laststatobj = statobj_t::new();
        self.laststatobj_i = 0;
        self.doorobjlist = Vec::new();
        self.doorobjlist_i = 0;
        self.lastdoorobj = doorobj_t::new();
        self.doornum = 0;
        self.doorposition = [0; MAXDOORS as usize];
        self.areaconnect = [[0; NUMAREAS as usize]; NUMAREAS as usize];
        self.areabyplayer = [false; NUMAREAS as usize];
        self.pwallstate = 0;
        self.pwallpos = 0;
        self.pwallx = 0;
        self.pwally = 0;
        self.pwalldir = controldir_t::di_none;
        self.pwalltile = 0;
        self.dirs = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub struct statinfo_t {
    pub picnum: i32,
    pub Type: wl_stat_t,
    pub specialFlags: i32, // they are ORed to the statobj_t flags
}

pub const statinfo: [statinfo_t; 50] = [
    statinfo_t {
        picnum: SPRITES::SPR_STAT_0 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // puddle          spr1v
    statinfo_t {
        picnum: SPRITES::SPR_STAT_1 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Green Barrel    "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_2 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Table/chairs    "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_3 as i32,
        Type: wl_stat_t::block,
        specialFlags: objflag_t::FL_FULLBRIGHT as i32,
    }, // Floor lamp      "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_4 as i32,
        Type: wl_stat_t::none,
        specialFlags: objflag_t::FL_FULLBRIGHT as i32,
    }, // Chandelier      "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_5 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Hanged man      "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_6 as i32,
        Type: wl_stat_t::bo_alpo,
        specialFlags: 0,
    }, // Bad food        "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_7 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Red pillar      "
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_8 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Tree            spr2v
    statinfo_t {
        picnum: SPRITES::SPR_STAT_9 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // Skeleton flat   "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_10 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Sink            " (SOD:gibs)
    statinfo_t {
        picnum: SPRITES::SPR_STAT_11 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Potted plant    "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_12 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Urn             "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_13 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Bare table      "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_14 as i32,
        Type: wl_stat_t::none,
        specialFlags: objflag_t::FL_FULLBRIGHT as i32,
    }, // Ceiling light   "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_15 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // Kitchen stuff   "
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_16 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // suit of armor   spr3v
    statinfo_t {
        picnum: SPRITES::SPR_STAT_17 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Hanging cage    "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_18 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // SkeletoninCage  "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_19 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // Skeleton relax  "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_20 as i32,
        Type: wl_stat_t::bo_key1,
        specialFlags: 0,
    }, // Key 1           "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_21 as i32,
        Type: wl_stat_t::bo_key2,
        specialFlags: 0,
    }, // Key 2           "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_22 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // stuff             (SOD:gibs)
    statinfo_t {
        picnum: SPRITES::SPR_STAT_23 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // stuff
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_24 as i32,
        Type: wl_stat_t::bo_food,
        specialFlags: 0,
    }, // Good food       spr4v
    statinfo_t {
        picnum: SPRITES::SPR_STAT_25 as i32,
        Type: wl_stat_t::bo_firstaid,
        specialFlags: 0,
    }, // First aid       "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_26 as i32,
        Type: wl_stat_t::bo_clip,
        specialFlags: 0,
    }, // Clip            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_27 as i32,
        Type: wl_stat_t::bo_machinegun,
        specialFlags: 0,
    }, // Machine gun     "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_28 as i32,
        Type: wl_stat_t::bo_chaingun,
        specialFlags: 0,
    }, // Gatling gun     "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_29 as i32,
        Type: wl_stat_t::bo_cross,
        specialFlags: 0,
    }, // Cross           "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_30 as i32,
        Type: wl_stat_t::bo_chalice,
        specialFlags: 0,
    }, // Chalice         "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_31 as i32,
        Type: wl_stat_t::bo_bible,
        specialFlags: 0,
    }, // Bible           "
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_32 as i32,
        Type: wl_stat_t::bo_crown,
        specialFlags: 0,
    }, // crown           spr5v
    statinfo_t {
        picnum: SPRITES::SPR_STAT_33 as i32,
        Type: wl_stat_t::bo_fullheal,
        specialFlags: objflag_t::FL_FULLBRIGHT as i32,
    }, // one up          "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_34 as i32,
        Type: wl_stat_t::bo_gibs,
        specialFlags: 0,
    }, // gibs            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_35 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // barrel          "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_36 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // well            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_37 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Empty well      "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_38 as i32,
        Type: wl_stat_t::bo_gibs,
        specialFlags: 0,
    }, // Gibs 2          "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_39 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // flag            "
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_40 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // Call Apogee          spr7v
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_41 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // junk            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_42 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // junk            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_43 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // junk            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_44 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // pots            "
    statinfo_t {
        picnum: SPRITES::SPR_STAT_45 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // stove           " (SOD:gibs)
    statinfo_t {
        picnum: SPRITES::SPR_STAT_46 as i32,
        Type: wl_stat_t::block,
        specialFlags: 0,
    }, // spears          " (SOD:gibs)
    statinfo_t {
        picnum: SPRITES::SPR_STAT_47 as i32,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // vines           "
    //
    // NEW PAGE
    //
    statinfo_t {
        picnum: SPRITES::SPR_STAT_26 as i32,
        Type: wl_stat_t::bo_clip2,
        specialFlags: 0,
    }, // Clip            "
    statinfo_t {
        picnum: -1,
        Type: wl_stat_t::none,
        specialFlags: 0,
    }, // terminator
];

pub const OPENTICS: i32 = 300;

/*
===============
=
= InitStaticList
=
===============
*/

pub fn InitStaticList(w3d: &mut modules) {
    //println!("InitStaticList");

    //laststatobj = &statobjlist[0];
    w3d.wl_act1.laststatobj_i = w3d.wl_act1.statobjlist_i;
}

/*
===============
=
= SpawnStatic
=
===============
*/

pub fn SpawnStatic(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32, Type: i32) {
    //println!("SpawnStatic");

    w3d.wl_act1.laststatobj.shapenum = statinfo[Type as usize].picnum;
    w3d.wl_act1.laststatobj.tilex = tilex;
    w3d.wl_act1.laststatobj.tiley = tiley;

    //BUG
    //laststatobj->visspot = &spotvis[tilex][tiley];
    //w3d.wl_act1.laststatobj.visspot = w3d.wl_play.spotvis[tilex as usize][tiley as usize];

    let visspot_prt = unsafe {
        w3d.wl_play.spotvis[tilex as usize]
            .as_mut_ptr()
            .add(tiley as usize)
    };
    w3d.wl_act1.laststatobj.visspot = visspot_prt;

    w3d.wl_act1.laststatobj.itemnumber = statinfo[Type as usize].Type as i32;

    match statinfo[Type as usize].Type {
        wl_stat_t::block => {
            ob.actorat[tilex as usize][tiley as usize] = BIT_WALL as *mut objtype;
            // consider it a blocking tile
            w3d.wl_act1.laststatobj.flags = 0;
        }

        wl_stat_t::none => {
            w3d.wl_act1.laststatobj.flags = 0;
        }

        wl_stat_t::bo_cross
        | wl_stat_t::bo_chalice
        | wl_stat_t::bo_bible
        | wl_stat_t::bo_crown
        | wl_stat_t::bo_fullheal => {
            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.treasuretotal += 1;
            }
            w3d.wl_act1.laststatobj.flags = objflag_t::FL_BONUS as i32;
        }

        wl_stat_t::bo_firstaid
        | wl_stat_t::bo_key1
        | wl_stat_t::bo_key2
        | wl_stat_t::bo_key3
        | wl_stat_t::bo_key4
        | wl_stat_t::bo_clip
        | wl_stat_t::bo_25clip
        | wl_stat_t::bo_machinegun
        | wl_stat_t::bo_chaingun
        | wl_stat_t::bo_food
        | wl_stat_t::bo_alpo
        | wl_stat_t::bo_gibs
        | wl_stat_t::bo_spear => {
            w3d.wl_act1.laststatobj.flags = objflag_t::FL_BONUS as i32;
        }
        _ => (),
    }

    w3d.wl_act1.laststatobj.flags |= statinfo[Type as usize].specialFlags;

    w3d.wl_act1.laststatobj_i += 1;

    w3d.wl_act1.statobjlist[w3d.wl_act1.statobjlist_i] = w3d.wl_act1.laststatobj;
    w3d.wl_act1.statobjlist_i += 1;

    if w3d.wl_act1.statobjlist_i == MAXSTATS as usize {
        Quit("Too many static objects!\n");
    }
}

/*
===============
=
= PlaceItemType
=
= Called during game play to drop actors' items.  It finds the proper
= item number based on the item type (bo_???).  If there are no free item
= spots, nothing is done.
=
===============
*/

pub fn PlaceItemType(w3d: &mut modules, itemtype: i32, tilex: i32, tiley: i32) {
    //println!("PlaceItemType");

    let mut Type: i32;
    let mut spot: statobj_t;

    //
    // find the item number
    //
    Type = 0;
    loop {
        if statinfo[Type as usize].picnum == -1 {
            // end of list
            Quit("PlaceItemType: couldn't find type!");
        }
        if statinfo[Type as usize].Type == wl_stat_t::from_u8(itemtype as u8) {
            break;
        }
        Type += 1;
    }
    //
    // find a spot in statobjlist to put it in
    //

    //for (spot=&w3d.wl_act1.statobjlist[0]; ; spot++)
    w3d.wl_act1.statobjlist_i = 0;
    loop {
        spot = w3d.wl_act1.statobjlist[w3d.wl_act1.statobjlist_i];
        if w3d.wl_act1.statobjlist_i == w3d.wl_act1.laststatobj_i {
            if w3d.wl_act1.statobjlist_i == MAXSTATS as usize - 1 {
                return; // no free spots
            }
            // space at end
            w3d.wl_act1.laststatobj_i += 1;
            break;
        }

        if spot.shapenum == -1 {
            // -1 is a free spot
            break;
        }
        w3d.wl_act1.statobjlist_i += 1;
    }
    //
    // place it
    //
    spot.shapenum = statinfo[Type as usize].picnum;
    spot.tilex = tilex;
    spot.tiley = tiley;
    //BUG
    //laststatobj->visspot = &spotvis[tilex][tiley];
    //spot.visspot = w3d.wl_play.spotvis[tilex as usize][tiley as usize];

    let visspot_prt = unsafe {
        w3d.wl_play.spotvis[tilex as usize]
            .as_mut_ptr()
            .add(tiley as usize)
    };
    spot.visspot = visspot_prt;

    spot.flags = objflag_t::FL_BONUS as i32 | statinfo[Type as usize].specialFlags;
    spot.itemnumber = statinfo[Type as usize].Type as i32;

    w3d.wl_act1.statobjlist[w3d.wl_act1.statobjlist_i] = spot;
}

/*
==============
=
= ConnectAreas
=
= Scans outward from playerarea, marking all connected areas
=
==============
*/

pub fn RecursiveConnect(w3d: &mut modules, ob: &mut object, areanumber: i32) {
    //println!("RecursiveConnect");

    for i in 0..NUMAREAS {
        if w3d.wl_act1.areaconnect[areanumber as usize][i as usize] != 0
            && !w3d.wl_act1.areabyplayer[i as usize]
        {
            w3d.wl_act1.areabyplayer[i as usize] = true;
            RecursiveConnect(w3d, ob, i);
        }
    }
}

pub fn ConnectAreas(w3d: &mut modules, ob: &mut object) {
    //println!("ConnectAreas");

    //memset (areabyplayer,0,sizeof(areabyplayer));
    w3d.wl_act1.areabyplayer = [false; NUMAREAS as usize];

    w3d.wl_act1.areabyplayer[ob.objlist[0].areanumber as usize] = true;
    RecursiveConnect(w3d, ob, ob.objlist[0].areanumber);
}

pub fn InitAreas(w3d: &mut modules, ob: &mut object) {
    //println!("InitAreas");

    //memset (areabyplayer,0,sizeof(areabyplayer));
    w3d.wl_act1.areabyplayer = [false; NUMAREAS as usize];

    if ob.objlist[0].areanumber < NUMAREAS {
        w3d.wl_act1.areabyplayer[ob.objlist[0].areanumber as usize] = true;
    }
}

/*
===============
=
= InitDoorList
=
===============
*/

pub fn InitDoorList(w3d: &mut modules) {
    //println!("InitDoorList");

    w3d.wl_act1.areaconnect = [[0; NUMAREAS as usize]; NUMAREAS as usize];
    w3d.wl_act1.areabyplayer = [false; NUMAREAS as usize];

    //lastdoorobj = &doorobjlist[0];

    w3d.wl_act1.doornum = 0;
}

/*
===============
=
= SpawnDoor
=
===============
*/

pub fn SpawnDoor(
    w3d: &mut modules,
    ob: &mut object,
    tilex: i32,
    tiley: i32,
    vertical: bool,
    lock: i32,
) {
    //println!("SpawnDoor");

    let map_i: usize;

    if w3d.wl_act1.doornum == MAXDOORS as i32 {
        Quit("64+ doors on level!");
    }

    w3d.wl_act1.doorposition[w3d.wl_act1.doornum as usize] = 0; // doors start out fully closed
    w3d.wl_act1.lastdoorobj.tilex = tilex;
    w3d.wl_act1.lastdoorobj.tiley = tiley;
    w3d.wl_act1.lastdoorobj.vertical = vertical;
    w3d.wl_act1.lastdoorobj.lock = lock;
    w3d.wl_act1.lastdoorobj.action = doortype::dr_closed;

    ob.actorat[tilex as usize][tiley as usize] =
        (w3d.wl_act1.doornum | BIT_DOOR as i32) as *mut objtype; // consider it a solid wall

    //
    // make the door tile a special tile, and mark the adjacent tiles
    // for door sides
    //
    w3d.wl_play.tilemap[tilex as usize][tiley as usize] = w3d.wl_act1.doornum | BIT_DOOR;

    map_i = ((tiley << MAPSHIFT) + tilex) as usize;

    if vertical {
        // set area number
        w3d.id_ca.mapsegs[0][map_i] = w3d.id_ca.mapsegs[0][map_i - 1];

        w3d.wl_play.tilemap[tilex as usize][tiley as usize - 1] |= BIT_WALL;
        w3d.wl_play.tilemap[tilex as usize][tiley as usize + 1] |= BIT_WALL;
    } else {
        // set area number
        w3d.id_ca.mapsegs[0][map_i] = w3d.id_ca.mapsegs[0][map_i - w3d.wl_play.mapwidth as usize];

        w3d.wl_play.tilemap[tilex as usize - 1][tiley as usize] |= BIT_WALL;
        w3d.wl_play.tilemap[tilex as usize + 1][tiley as usize] |= BIT_WALL;
    }

    w3d.wl_act1.doornum += 1;

    w3d.wl_act1.doorobjlist.push(w3d.wl_act1.lastdoorobj);
    w3d.wl_act1.doorobjlist_i += 1;
}

/*
=====================
=
= OpenDoor
=
=====================
*/

pub fn OpenDoor(w3d: &mut modules, _ob: &mut object, door: i32) {
    //println!("OpenDoor");

    if w3d.wl_act1.doorobjlist[door as usize].action == doortype::dr_open {
        w3d.wl_act1.doorobjlist[door as usize].ticcount = 0; // reset open time
    } else {
        w3d.wl_act1.doorobjlist[door as usize].action = doortype::dr_opening; // start it opening
    }
}

/*
=====================
=
= CloseDoor
=
=====================
*/

pub fn CloseDoor(w3d: &mut modules, ob: &mut object, door: i32) {
    //println!("CloseDoor");

    let tilex: i32;
    let tiley: i32;
    let area: i32;
    let mut check: *mut objtype;

    //
    // don't close on anything solid
    //
    tilex = w3d.wl_act1.doorobjlist[door as usize].tilex as i32;
    tiley = w3d.wl_act1.doorobjlist[door as usize].tiley as i32;

    if !ob.actorat[tilex as usize][tiley as usize].is_null() {
        return;
    }

    if ob.objlist[0].tilex == tilex as i32 && ob.objlist[0].tiley == tiley as i32 {
        return;
    }

    if w3d.wl_act1.doorobjlist[door as usize].vertical {
        if ob.objlist[0].tiley == tiley as i32 {
            if ((ob.objlist[0].x + MINDIST as i32) >> TILESHIFT) == tilex as i32 {
                return;
            }
            if ((ob.objlist[0].x - MINDIST as i32) >> TILESHIFT) == tilex as i32 {
                return;
            }
        }
        check = ob.actorat[tilex as usize - 1][tiley as usize];

        if !check.is_null()
            && ((unsafe { (*check).x } + MINDIST as i32) >> TILESHIFT) == tilex as i32
        {
            return;
        }
        check = ob.actorat[tilex as usize + 1][tiley as usize];
        if !check.is_null()
            && ((unsafe { (*check).x } - MINDIST as i32) >> TILESHIFT) == tilex as i32
        {
            return;
        }
    } else {
        if ob.objlist[0].tilex == tilex as i32 {
            if ((ob.objlist[0].y + MINDIST as i32) >> TILESHIFT) == tiley as i32 {
                return;
            }
            if ((ob.objlist[0].y - MINDIST as i32) >> TILESHIFT) == tiley as i32 {
                return;
            }
        }
        check = ob.actorat[tilex as usize][tiley as usize - 1];

        if !check.is_null()
            && ((unsafe { (*check).y } + MINDIST as i32) >> TILESHIFT) == tiley as i32
        {
            return;
        }
        check = ob.actorat[tilex as usize][tiley as usize + 1];
        if !check.is_null()
            && ((unsafe { (*check).y } - MINDIST as i32) >> TILESHIFT) == tiley as i32
        {
            return;
        }
    }

    //
    // play door sound if in a connected area
    //
    area = MAPSPOT(w3d, tilex as i32, tiley as i32, 0) - AREATILE as i32;

    if w3d.wl_act1.areabyplayer[area as usize] {
        PlaySoundLocTile(
            w3d,
            soundnames::CLOSEDOORSND,
            w3d.wl_act1.doorobjlist[door as usize].tilex as i32,
            w3d.wl_act1.doorobjlist[door as usize].tiley as i32,
        ); // JAB
    }

    w3d.wl_act1.doorobjlist[door as usize].action = doortype::dr_closing;
    //
    // make the door space solid
    //

    ob.actorat[tilex as usize][tiley as usize] = (door | BIT_DOOR as i32) as *mut objtype;
    // consider it a solid wall;
}

/*
=====================
=
= OperateDoor
=
= The player wants to change the door's direction
=
=====================
*/

pub fn OperateDoor(w3d: &mut modules, ob: &mut object, door: i32) {
    //println!("OperateDoor");

    let lock: i32;

    lock = w3d.wl_act1.doorobjlist[door as usize].lock;
    if lock >= door_t::dr_lock1 as i32 && lock <= door_t::dr_lock4 as i32 {
        if (w3d.wl_game.gamestate.keys & (1 << (lock - door_t::dr_lock1 as i32))) == 0 {
            if w3d.wl_act1.doorposition[door as usize] == 0 {
                SD_PlaySound(w3d, soundnames::NOWAYSND); // ADDEDFIX 9       // locked
            }
            return;
        }
    }

    match w3d.wl_act1.doorobjlist[door as usize].action {
        doortype::dr_closed | doortype::dr_closing => {
            OpenDoor(w3d, ob, door);
        }
        doortype::dr_open | doortype::dr_opening => {
            CloseDoor(w3d, ob, door);
        }
    }
}

/*
===============
=
= DoorOpen
=
= Close the door after three seconds
=
===============
*/

pub fn DoorOpen(w3d: &mut modules, ob: &mut object, door: i32) {
    //println!("DoorOpen");

    w3d.wl_act1.doorobjlist[door as usize].ticcount += w3d.wl_play.tics as i32;
    let ticcount = w3d.wl_act1.doorobjlist[door as usize].ticcount;

    if ticcount as i32 >= OPENTICS {
        CloseDoor(w3d, ob, door);
    }
}

/*
===============
=
= DoorOpening
=
===============
*/

pub fn DoorOpening(w3d: &mut modules, ob: &mut object, door: i32) {
    //println!("DoorOpening");

    let mut area1: i32;
    let mut area2: i32;
    let map: Vec<u16>;
    let map_i: usize;
    let mut position: i32;

    position = w3d.wl_act1.doorposition[door as usize];
    if position == 0 {
        //
        // door is just starting to open, so connect the areas
        //

        //map = MAPSPOT(id,wl,w3d.wl_act1.doorobjlist[door as usize].tilex,w3d.wl_act1.doorobjlist[door as usize].tiley,0) as i32;
        map = w3d.id_ca.mapsegs[0].clone(); //plane 0
        map_i = ((w3d.wl_act1.doorobjlist[door as usize].tiley << MAPSHIFT)
            + w3d.wl_act1.doorobjlist[door as usize].tilex) as usize;

        if w3d.wl_act1.doorobjlist[door as usize].vertical {
            area1 = map[map_i + 1] as i32;
            area2 = map[map_i - 1] as i32;
        } else {
            area1 = map[map_i - w3d.wl_play.mapwidth as usize] as i32;
            area2 = map[map_i + w3d.wl_play.mapwidth as usize] as i32;
        }
        area1 -= AREATILE;
        area2 -= AREATILE;

        if area1 < NUMAREAS && area2 < NUMAREAS {
            w3d.wl_act1.areaconnect[area1 as usize][area2 as usize] += 1;
            w3d.wl_act1.areaconnect[area2 as usize][area1 as usize] += 1;
            //player=ob.objlist[0]:
            if ob.objlist[0].areanumber < NUMAREAS {
                ConnectAreas(w3d, ob);
            }

            if w3d.wl_act1.areabyplayer[area1 as usize] {
                PlaySoundLocTile(
                    w3d,
                    soundnames::OPENDOORSND,
                    w3d.wl_act1.doorobjlist[door as usize].tilex,
                    w3d.wl_act1.doorobjlist[door as usize].tiley,
                ); // JAB
            }
        }
    }

    //
    // slide the door by an adaptive amount
    //
    position += w3d.wl_play.tics << 10;
    if position >= 0xffff {
        //
        // door is all the way open
        //
        position = 0xffff;
        w3d.wl_act1.doorobjlist[door as usize].ticcount = 0;
        w3d.wl_act1.doorobjlist[door as usize].action = doortype::dr_open;
        ob.actorat[w3d.wl_act1.doorobjlist[door as usize].tilex as usize]
            [w3d.wl_act1.doorobjlist[door as usize].tiley as usize] = 0 as *mut objtype;
    }

    w3d.wl_act1.doorposition[door as usize] = position;
}

/*
===============
=
= DoorClosing
=
===============
*/

pub fn DoorClosing(w3d: &mut modules, ob: &mut object, door: i32) {
    //println!("DoorClosing");

    let mut area1: i32;
    let mut area2: i32;
    let map: Vec<u16>;
    let map_i: usize;
    let mut position: i32;
    let tilex: i32;
    let tiley: i32;

    tilex = w3d.wl_act1.doorobjlist[door as usize].tilex;
    tiley = w3d.wl_act1.doorobjlist[door as usize].tiley;

    if (ob.actorat[tilex as usize][tiley as usize]) != ((door | BIT_DOOR as i32) as *mut objtype)
        || (ob.objlist[0].tilex == tilex && ob.objlist[0].tiley == tiley)
    {
        // something got inside the door
        OpenDoor(w3d, ob, door);
        return;
    };

    position = w3d.wl_act1.doorposition[door as usize];

    //
    // slide the door by an adaptive amount
    //
    position -= w3d.wl_play.tics << 10;

    if position <= 0 {
        //
        // door is closed all the way, so disconnect the areas
        //
        position = 0;

        w3d.wl_act1.doorobjlist[door as usize].action = doortype::dr_closed;

        //map = &MAPSPOT(doorobjlist[door].tilex,doorobjlist[door].tiley,0);
        map = w3d.id_ca.mapsegs[0].clone(); //plane 0
        map_i = ((w3d.wl_act1.doorobjlist[door as usize].tiley << MAPSHIFT)
            + w3d.wl_act1.doorobjlist[door as usize].tilex) as usize;

        if w3d.wl_act1.doorobjlist[door as usize].vertical {
            area1 = map[map_i + 1] as i32;
            area2 = map[map_i - 1] as i32;
        } else {
            area1 = map[map_i - w3d.wl_play.mapwidth as usize] as i32;
            area2 = map[map_i + w3d.wl_play.mapwidth as usize] as i32;
        }
        area1 -= AREATILE;
        area2 -= AREATILE;

        if area1 < NUMAREAS && area2 < NUMAREAS {
            w3d.wl_act1.areaconnect[area1 as usize][area2 as usize] -= 1;
            w3d.wl_act1.areaconnect[area2 as usize][area1 as usize] -= 1;

            //player=ob.objlist[0]:
            if ob.objlist[0].areanumber < NUMAREAS {
                ConnectAreas(w3d, ob);
            }
        }
    }

    w3d.wl_act1.doorposition[door as usize] = position;
}

/*
=====================
=
= MoveDoors
=
= Called from PlayLoop
=
=====================
*/

pub fn MoveDoors(w3d: &mut modules, ob: &mut object) {
    //println!("MoveDoors");

    if w3d.wl_game.gamestate.victoryflag
    // don't move door during victory sequence
    {
        return;
    }

    for door in 0..w3d.wl_act1.doornum as i32 {
        match w3d.wl_act1.doorobjlist[door as usize].action {
            doortype::dr_open => {
                DoorOpen(w3d, ob, door);
            }

            doortype::dr_opening => {
                DoorOpening(w3d, ob, door);
            }

            doortype::dr_closing => {
                DoorClosing(w3d, ob, door);
            }
            _ => (),
        }
    }
}

/*
=============================================================================

                                PUSHABLE WALLS

=============================================================================
*/

/*
===============
=
= PushWall
=
===============
*/

pub fn PushWall(w3d: &mut modules, ob: &mut object, checkx: i32, checky: i32, dir: i32) {
    //println!("PushWall");

    let oldtile: i32;
    let dx: i32;
    let dy: i32;

    if w3d.wl_act1.pwallstate != 0 {
        return;
    }

    oldtile = w3d.wl_play.tilemap[checkx as usize][checky as usize];
    if oldtile == 0 {
        return;
    }

    dx = w3d.wl_act1.dirs[dir as usize][0] as i32;
    dy = w3d.wl_act1.dirs[dir as usize][1] as i32;

    if !ob.actorat[(checkx + dx) as usize][(checky + dy) as usize].is_null() {
        SD_PlaySound(w3d, soundnames::NOWAYSND);
        return;
    }

    w3d.wl_play.tilemap[(checkx + dx) as usize][(checky + dy) as usize] = oldtile;
    ob.actorat[(checkx + dx) as usize][(checky + dy) as usize] = oldtile as *mut objtype;

    w3d.wl_game.gamestate.secretcount += 1;
    w3d.wl_act1.pwallx = checkx;
    w3d.wl_act1.pwally = checky;
    w3d.wl_act1.pwalldir = controldir_t::from_i32(dir);
    w3d.wl_act1.pwallstate = 1;
    w3d.wl_act1.pwallpos = 0;
    w3d.wl_act1.pwalltile =
        w3d.wl_play.tilemap[w3d.wl_act1.pwallx as usize][w3d.wl_act1.pwally as usize];
    w3d.wl_play.tilemap[w3d.wl_act1.pwallx as usize][w3d.wl_act1.pwally as usize] = BIT_WALL;
    w3d.wl_play.tilemap[(w3d.wl_act1.pwallx + dx) as usize][(w3d.wl_act1.pwally + dy) as usize] =
        BIT_WALL;

    //MAPSPOT(pwallx,pwally,1) = 0;   // remove P tile info
    w3d.id_ca.mapsegs[1][((w3d.wl_act1.pwally << MAPSHIFT) + w3d.wl_act1.pwallx) as usize] = 0;
    //MAPSPOT(pwallx,pwally,0) = MAPSPOT(player->tilex,player->tiley,0); // set correct floorcode (BrotherTank's fix) TODO: use a better method...
    w3d.id_ca.mapsegs[0][((w3d.wl_act1.pwally << MAPSHIFT) + w3d.wl_act1.pwallx) as usize] =
        w3d.id_ca.mapsegs[0][((ob.objlist[0].tiley << MAPSHIFT) + ob.objlist[0].tilex) as usize];

    SD_PlaySound(w3d, soundnames::PUSHWALLSND);
}

/*
=================
=
= MovePWalls
=
=================
*/

pub fn MovePWalls(w3d: &mut modules, ob: &mut object) {
    //println!("MovePWalls");

    let oldblock: i32;
    let oldtile: i32;

    if w3d.wl_act1.pwallstate == 0 {
        return;
    }

    oldblock = w3d.wl_act1.pwallstate / 128;

    w3d.wl_act1.pwallstate += w3d.wl_play.tics;

    if w3d.wl_act1.pwallstate / 128 != oldblock {
        // block crossed into a new block
        oldtile = w3d.wl_act1.pwalltile;

        //
        // the tile can now be walked into
        //
        w3d.wl_play.tilemap[w3d.wl_act1.pwallx as usize][w3d.wl_act1.pwally as usize] = 0;
        ob.actorat[w3d.wl_act1.pwallx as usize][w3d.wl_act1.pwally as usize] = 0 as *mut objtype;
        //MAPSPOT(w3d.wl_act1.pwallx, w3d.wl_act1.pwally, 0) = ob.objlist[0].areanumber + AREATILE; // TODO: this is unnecessary, and makes a mess of mapsegs

        let dx = w3d.wl_act1.dirs[w3d.wl_act1.pwalldir as usize][0] as i32;
        let dy = w3d.wl_act1.dirs[w3d.wl_act1.pwalldir as usize][1] as i32;
        //
        // see if it should be pushed farther
        //
        if w3d.wl_act1.pwallstate >= 256
        // only move two tiles fix
        {
            //
            // the block has been pushed two tiles
            //
            w3d.wl_act1.pwallstate = 0;
            w3d.wl_play.tilemap[(w3d.wl_act1.pwallx + dx) as usize]
                [(w3d.wl_act1.pwally + dy) as usize] = oldtile;
            return;
        } else {
            let xl = (ob.objlist[0].x - PLAYERSIZE) >> TILESHIFT;
            let yl = (ob.objlist[0].y - PLAYERSIZE) >> TILESHIFT;
            let xh = (ob.objlist[0].x + PLAYERSIZE) >> TILESHIFT;
            let yh = (ob.objlist[0].y + PLAYERSIZE) >> TILESHIFT;

            w3d.wl_act1.pwallx += dx;
            w3d.wl_act1.pwally += dy;

            if !ob.actorat[(w3d.wl_act1.pwallx + dx) as usize][(w3d.wl_act1.pwally + dy) as usize]
                .is_null()
                || xl <= (w3d.wl_act1.pwallx + dx)
                    && (w3d.wl_act1.pwallx + dx) <= xh
                    && (yl <= (w3d.wl_act1.pwally + dy))
                    && ((w3d.wl_act1.pwally + dy) <= yh)
            {
                w3d.wl_act1.pwallstate = 0;
                w3d.wl_play.tilemap[w3d.wl_act1.pwallx as usize][w3d.wl_act1.pwally as usize] =
                    oldtile;
                return;
            }

            w3d.wl_play.tilemap[(w3d.wl_act1.pwallx + dx) as usize]
                [(w3d.wl_act1.pwally + dy) as usize] = oldtile;

            ob.actorat[(w3d.wl_act1.pwallx + dx) as usize][(w3d.wl_act1.pwally + dy) as usize] =
                oldtile as *mut objtype;

            w3d.wl_play.tilemap[(w3d.wl_act1.pwallx + dx) as usize]
                [(w3d.wl_act1.pwally + dy) as usize] = BIT_WALL;
        }
    }

    w3d.wl_act1.pwallpos = (w3d.wl_act1.pwallstate / 2) & 63;
}
