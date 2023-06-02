#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_state
//
//===========================================================================

pub struct wl_state {
    pub doornum: i32,
    pub temp: *mut objtype,
}

impl wl_state {
    pub fn new() -> Self {
        Self {
            doornum: -1,
            temp: ptr::null_mut(),
        }
    }
    pub fn clear(&mut self) {
        self.doornum = -1;
        self.temp = ptr::null_mut();
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const opposite: [dirtype; 9] = [
    dirtype::west,
    dirtype::southwest,
    dirtype::south,
    dirtype::southeast,
    dirtype::east,
    dirtype::northeast,
    dirtype::north,
    dirtype::northwest,
    dirtype::nodir,
];

pub const diagonal: [[dirtype; 9]; 9] = [
    /* east */
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::northeast,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::southeast,
        dirtype::nodir,
        dirtype::nodir,
    ],
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
    /* north */
    [
        dirtype::northeast,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::northwest,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
    /* west */
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::northwest,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::southwest,
        dirtype::nodir,
        dirtype::nodir,
    ],
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
    /* south */
    [
        dirtype::southeast,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::southwest,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
    [
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
        dirtype::nodir,
    ],
];

pub const MINSIGHT: i32 = 0x18000;

/*
===================
=
= SpawnNewObj
=
= Spaws a new actor at the given TILE coordinates, with the given state, and
= the given size in GLOBAL units.
=
= newobj = a pointer to an initialized new actor
=
===================
*/

pub fn SpawnNewObj(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32, state: statetype) {
    //println!("SpawnNewObj");

    GetNewActor(ob);

    let mut newobj = ob.newobj;
    let newstate = state;

    //BUG
    if state.tictime != 0 {
        //DEMOCHOOSE_ORIG_SDL
        // Chris' moonwalk bugfix ;D
        if w3d.wl_play.demorecord || w3d.wl_play.demoplayback {
            newobj.ticcount = US_RndT(w3d) % state.tictime
        } else {
            newobj.ticcount = US_RndT(w3d) % state.tictime + 1
        }
    } else {
        newobj.ticcount = 0;
    }

    newobj.tilex = tilex;
    newobj.tiley = tiley;

    newobj.x = ((tilex) << TILESHIFT) + TILEGLOBAL / 2;
    newobj.y = ((tiley) << TILESHIFT) + TILEGLOBAL / 2;
    newobj.dir = dirtype::nodir;

    ob.actorat[tilex as usize][tiley as usize] = &mut newobj as *mut objtype;

    let spot = MAPSPOT(w3d, tilex, tiley, 0);
    newobj.areanumber = ((spot - AREATILE) as u8) as i32;

    newobj.state = newstate;
    ob.newobj = newobj;

    ob.objlist.push(ob.newobj); // adds a new actor to list
    ob.objlist_i += 1;
}

/*
===================
=
= NewState
=
= Changes ob to a new state, setting ticcount to the max for that state
=
===================
*/

pub fn NewState(_w3d: &mut modules, ob: &mut object, which: usize, state: statetype) {
    //println!("NewState");

    ob.objlist[which].state = state;
    ob.objlist[which].ticcount = state.tictime;
}

/*
=============================================================================

                        ENEMY TILE WORLD MOVEMENT CODE

=============================================================================
*/

/*
==================================
=
= TryWalk
=
= Attempts to move ob in its current (ob->dir) direction.
=
= If blocked by either a wall or an actor returns FALSE
=
= If move is either clear or blocked only by a door, returns TRUE and sets
=
= ob->tilex         = new destination
= ob->tiley
= ob->areanumber    = the floor tile number (0-(NUMAREAS-1)) of destination
= ob->distance      = TILEGLOBAl, or -doornumber if a door is blocking the way
=
= If a door is in the way, an OpenDoor call is made to start it opening.
= The actor code should wait until
=       doorobjlist[-ob->distance].action = dr_open, meaning the door has been
=       fully opened
=
==================================
*/

pub fn CHECKDIAG(w3d: &mut modules, ob: &mut object, x: i32, y: i32) -> bool {
    w3d.wl_state.temp = ob.actorat[x as usize][y as usize];
    if !w3d.wl_state.temp.is_null() {
        if (w3d.wl_state.temp as *mut u8) < (BIT_ALLTILES as *mut u8) {
            //256
            return false;
        }
        if (unsafe { (*w3d.wl_state.temp).flags } & objflag_t::FL_SHOOTABLE as i32) != 0 {
            return false;
        }
    }
    return true;
}

pub fn DOORCHECK(w3d: &mut modules, ob: &mut object) -> bool {
    if DEMOCOND_ORIG(w3d) {
        w3d.wl_state.doornum = w3d.wl_state.temp as i32 & 63;
    } else {
        w3d.wl_state.doornum = w3d.wl_state.temp as i32 & !BIT_DOOR; //128

        if ob.objlist[ob.objlist_i].obclass != classtype::ghostobj
            && ob.objlist[ob.objlist_i].obclass != classtype::spectreobj
        {
            OpenDoor(w3d, ob, w3d.wl_state.doornum);
            ob.objlist[ob.objlist_i].distance = -w3d.wl_state.doornum - 1;
            return true;
        }
    }
    return true;
}

pub fn CHECKSIDE(w3d: &mut modules, ob: &mut object, x: i32, y: i32) -> bool {
    w3d.wl_state.temp = ob.actorat[x as usize][y as usize];
    if !w3d.wl_state.temp.is_null() {
        if (w3d.wl_state.temp as *mut u8) < (BIT_DOOR as *mut u8) {
            //128
            return false;
        }
        if (w3d.wl_state.temp as *mut u8) < (BIT_ALLTILES as *mut u8) {
            //256
            DOORCHECK(w3d, ob);
        } else if (unsafe { (*w3d.wl_state.temp).flags } & objflag_t::FL_SHOOTABLE as i32) != 0 {
            return false;
        }
    }
    return true;
}

pub fn TryWalk(w3d: &mut modules, ob: &mut object) -> bool {
    //println!("TryWalk");

    w3d.wl_state.doornum = -1;

    if ob.objlist[ob.objlist_i].obclass == classtype::inertobj {
        match ob.objlist[ob.objlist_i].dir {
            dirtype::north => {
                ob.objlist[ob.objlist_i].tiley -= 1;
            }
            dirtype::northeast => {
                ob.objlist[ob.objlist_i].tilex += 1;
                ob.objlist[ob.objlist_i].tiley -= 1;
            }
            dirtype::east => {
                ob.objlist[ob.objlist_i].tilex += 1;
            }
            dirtype::southeast => {
                ob.objlist[ob.objlist_i].tilex += 1;
                ob.objlist[ob.objlist_i].tiley += 1;
            }
            dirtype::south => {
                ob.objlist[ob.objlist_i].tiley += 1;
            }
            dirtype::southwest => {
                ob.objlist[ob.objlist_i].tilex -= 1;
                ob.objlist[ob.objlist_i].tiley += 1;
            }
            dirtype::west => {
                ob.objlist[ob.objlist_i].tilex -= 1;
            }
            dirtype::northwest => {
                ob.objlist[ob.objlist_i].tilex -= 1;
                ob.objlist[ob.objlist_i].tiley -= 1;
            }
            _ => (),
        }
    } else {
        match ob.objlist[ob.objlist_i].dir {
            dirtype::north => {
                if ob.objlist[ob.objlist_i].obclass == classtype::dogobj
                    || ob.objlist[ob.objlist_i].obclass == classtype::fakeobj
                {
                    if !CHECKDIAG(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex,
                        ob.objlist[ob.objlist_i].tiley - 1,
                    ) {
                        return false;
                    }
                } else {
                    if !CHECKSIDE(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex,
                        ob.objlist[ob.objlist_i].tiley - 1,
                    ) {
                        return false;
                    }
                }
                ob.objlist[ob.objlist_i].tiley -= 1;
            }
            dirtype::northeast => {
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex + 1,
                    ob.objlist[ob.objlist_i].tiley - 1,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex + 1,
                    ob.objlist[ob.objlist_i].tiley,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex,
                    ob.objlist[ob.objlist_i].tiley - 1,
                ) {
                    return false;
                }
                ob.objlist[ob.objlist_i].tilex += 1;
                ob.objlist[ob.objlist_i].tiley -= 1;
            }
            dirtype::east => {
                if ob.objlist[ob.objlist_i].obclass == classtype::dogobj
                    || ob.objlist[ob.objlist_i].obclass == classtype::fakeobj
                {
                    if !CHECKDIAG(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex + 1,
                        ob.objlist[ob.objlist_i].tiley,
                    ) {
                        return false;
                    }
                } else {
                    if !CHECKSIDE(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex + 1,
                        ob.objlist[ob.objlist_i].tiley,
                    ) {
                        return false;
                    }
                }
                ob.objlist[ob.objlist_i].tilex += 1;
            }
            dirtype::southeast => {
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex + 1,
                    ob.objlist[ob.objlist_i].tiley + 1,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex + 1,
                    ob.objlist[ob.objlist_i].tiley,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex,
                    ob.objlist[ob.objlist_i].tiley + 1,
                ) {
                    return false;
                }
                ob.objlist[ob.objlist_i].tilex += 1;
                ob.objlist[ob.objlist_i].tiley += 1;
            }
            dirtype::south => {
                if ob.objlist[ob.objlist_i].obclass == classtype::dogobj
                    || ob.objlist[ob.objlist_i].obclass == classtype::fakeobj
                {
                    if !CHECKDIAG(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex,
                        ob.objlist[ob.objlist_i].tiley + 1,
                    ) {
                        return false;
                    }
                } else {
                    if !CHECKSIDE(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex,
                        ob.objlist[ob.objlist_i].tiley + 1,
                    ) {
                        return false;
                    }
                }
                ob.objlist[ob.objlist_i].tiley += 1;
            }
            dirtype::southwest => {
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex - 1,
                    ob.objlist[ob.objlist_i].tiley + 1,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex - 1,
                    ob.objlist[ob.objlist_i].tiley,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex,
                    ob.objlist[ob.objlist_i].tiley + 1,
                ) {
                    return false;
                }
                ob.objlist[ob.objlist_i].tilex -= 1;
                ob.objlist[ob.objlist_i].tiley += 1;
            }
            dirtype::west => {
                if ob.objlist[ob.objlist_i].obclass == classtype::dogobj
                    || ob.objlist[ob.objlist_i].obclass == classtype::fakeobj
                {
                    if !CHECKDIAG(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex - 1,
                        ob.objlist[ob.objlist_i].tiley,
                    ) {
                        return false;
                    }
                } else {
                    if !CHECKSIDE(
                        w3d,
                        ob,
                        ob.objlist[ob.objlist_i].tilex - 1,
                        ob.objlist[ob.objlist_i].tiley,
                    ) {
                        return false;
                    }
                }
                ob.objlist[ob.objlist_i].tilex -= 1;
            }
            dirtype::northwest => {
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex - 1,
                    ob.objlist[ob.objlist_i].tiley - 1,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex - 1,
                    ob.objlist[ob.objlist_i].tiley,
                ) {
                    return false;
                }
                if !CHECKDIAG(
                    w3d,
                    ob,
                    ob.objlist[ob.objlist_i].tilex,
                    ob.objlist[ob.objlist_i].tiley - 1,
                ) {
                    return false;
                }
                ob.objlist[ob.objlist_i].tilex -= 1;
                ob.objlist[ob.objlist_i].tiley -= 1;
            }
            dirtype::nodir => {
                return false;
            }
        }
    }

    if w3d.wl_state.doornum != -1 {
        OpenDoor(w3d, ob, w3d.wl_state.doornum);
        ob.objlist[ob.objlist_i].distance = -w3d.wl_state.doornum - 1;
        return true;
    }

    let spot = MAPSPOT(
        w3d,
        ob.objlist[ob.objlist_i].tilex,
        ob.objlist[ob.objlist_i].tiley,
        0,
    );

    ob.objlist[ob.objlist_i].areanumber = ((spot - AREATILE) as u8) as i32;

    ob.objlist[ob.objlist_i].distance = TILEGLOBAL; //65536

    return true;
}

/*
==================================
=
= SelectDodgeDir
=
= Attempts to choose and initiate a movement for ob that sends it towards
= the player while dodging
=
= If there is no possible move (ob is totally surrounded)
=
= ob->dir           =       nodir
=
= Otherwise
=
= ob->dir           = new direction to follow
= ob->distance      = TILEGLOBAL or -doornumber
= ob->tilex         = new destination
= ob->tiley
= ob->areanumber    = the floor tile number (0-(NUMAREAS-1)) of destination
=
==================================
*/

pub fn SelectDodgeDir(w3d: &mut modules, ob: &mut object) {
    //println!("SelectDodgeDir");

    let deltax: i32;
    let deltay: i32;

    let absdx: i32;
    let absdy: i32;
    let mut dirtry: [dirtype; 5] = [dirtype::nodir; 5];
    let turnaround: dirtype;
    let mut tdir: dirtype;

    if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_FIRSTATTACK as i32) != 0 {
        //
        // turning around is only ok the very first time after noticing the
        // player
        //
        turnaround = dirtype::nodir;
        ob.objlist[ob.objlist_i].flags &= !(objflag_t::FL_FIRSTATTACK as i32);
    } else {
        turnaround = opposite[(ob.objlist[ob.objlist_i].dir) as usize];
    }

    deltax = ob.objlist[0].tilex - ob.objlist[ob.objlist_i].tilex;
    deltay = ob.objlist[0].tiley - ob.objlist[ob.objlist_i].tiley;

    //
    // arange 5 direction choices in order of preference
    // the four cardinal directions plus the diagonal straight towards
    // the player
    //

    if deltax > 0 {
        dirtry[1] = dirtype::east;
        dirtry[3] = dirtype::west;
    } else {
        dirtry[1] = dirtype::west;
        dirtry[3] = dirtype::east;
    }

    if deltay > 0 {
        dirtry[2] = dirtype::south;
        dirtry[4] = dirtype::north;
    } else {
        dirtry[2] = dirtype::north;
        dirtry[4] = dirtype::south;
    }

    //
    // randomize a bit for dodging
    //
    absdx = abs(deltax);
    absdy = abs(deltay);

    if absdx > absdy {
        tdir = dirtry[1];
        dirtry[1] = dirtry[2];
        dirtry[2] = tdir;
        tdir = dirtry[3];
        dirtry[3] = dirtry[4];
        dirtry[4] = tdir;
    }

    if US_RndT(w3d) < 128 {
        tdir = dirtry[1];
        dirtry[1] = dirtry[2];
        dirtry[2] = tdir;
        tdir = dirtry[3];
        dirtry[3] = dirtry[4];
        dirtry[4] = tdir;
    }

    dirtry[0] = diagonal[dirtry[1] as usize][dirtry[2] as usize];

    //
    // try the directions util one works
    //
    for i in 0..5 {
        if dirtry[i] == dirtype::nodir || dirtry[i] == turnaround {
            continue;
        }

        ob.objlist[ob.objlist_i].dir = dirtry[i];

        if TryWalk(w3d, ob) {
            return;
        }
    }

    //
    // turn around only as a last resort
    //
    if turnaround != dirtype::nodir {
        ob.objlist[ob.objlist_i].dir = turnaround;

        if TryWalk(w3d, ob) {
            return;
        }
    }

    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
}

/*
============================
=
= SelectChaseDir
=
= As SelectDodgeDir, but doesn't try to dodge
=
============================
*/

pub fn SelectChaseDir(w3d: &mut modules, ob: &mut object) {
    //println!("SelectChaseDir");

    let deltax: i32;
    let deltay: i32;
    let mut d: [dirtype; 3] = [dirtype::nodir; 3];
    let tdir: dirtype;
    let olddir: dirtype;
    let turnaround: dirtype;

    olddir = ob.objlist[ob.objlist_i].dir;
    turnaround = opposite[olddir as usize];

    deltax = ob.objlist[0].tilex - ob.objlist[ob.objlist_i].tilex;
    deltay = ob.objlist[0].tiley - ob.objlist[ob.objlist_i].tiley;

    d[1] = dirtype::nodir;
    d[2] = dirtype::nodir;

    if deltax > 0 {
        d[1] = dirtype::east;
    } else if deltax < 0 {
        d[1] = dirtype::west;
    }
    if deltay > 0 {
        d[2] = dirtype::south;
    } else if deltay < 0 {
        d[2] = dirtype::north;
    }

    if abs(deltay) > abs(deltax) {
        tdir = d[1];
        d[1] = d[2];
        d[2] = tdir;
    }

    if d[1] == turnaround {
        d[1] = dirtype::nodir;
    }
    if d[2] == turnaround {
        d[2] = dirtype::nodir;
    }

    if d[1] != dirtype::nodir {
        ob.objlist[ob.objlist_i].dir = d[1];
        if TryWalk(w3d, ob) {
            return; /*either moved forward or attacked*/
        }
    }

    if d[2] != dirtype::nodir {
        ob.objlist[ob.objlist_i].dir = d[2];
        if TryWalk(w3d, ob) {
            return;
        }
    }

    /* there is no direct path to the player, so pick another direction */

    if olddir != dirtype::nodir {
        ob.objlist[ob.objlist_i].dir = olddir;
        if TryWalk(w3d, ob) {
            return;
        }
    }

    if US_RndT(w3d) > 128
    /*randomly determine direction of search*/
    {
        // tdir=(dirtype)(tdir+1))
        for tdir in dirtype::north as usize..=dirtype::west as usize {
            if tdir != turnaround as usize {
                ob.objlist[ob.objlist_i].dir = dirtype::from_u8(tdir as u8);
                if TryWalk(w3d, ob) {
                    return;
                }
            }
        }
    } else {
        // tdir=(dirtype)(tdir-1))
        for tdir in (dirtype::west as usize..=dirtype::north as usize).rev() {
            if tdir != turnaround as usize {
                ob.objlist[ob.objlist_i].dir = dirtype::from_u8(tdir as u8);
                if TryWalk(w3d, ob) {
                    return;
                }
            }
        }
    }

    if turnaround != dirtype::nodir {
        ob.objlist[ob.objlist_i].dir = turnaround;
        if ob.objlist[ob.objlist_i].dir != dirtype::nodir {
            if TryWalk(w3d, ob) {
                return;
            }
        }
    }

    ob.objlist[ob.objlist_i].dir = dirtype::nodir; // can't move
}

/*
============================
=
= SelectRunDir
=
= Run Away from player
=
============================
*/

pub fn SelectRunDir(w3d: &mut modules, ob: &mut object) {
    //println!("SelectRunDir");

    let deltax: i32;
    let deltay: i32;
    let mut d: [dirtype; 3] = [dirtype::nodir; 3];
    let tdir: dirtype;

    deltax = ob.objlist[0].tilex - ob.objlist[ob.objlist_i].tilex;
    deltay = ob.objlist[0].tiley - ob.objlist[ob.objlist_i].tiley;

    d[1] = dirtype::nodir;
    d[2] = dirtype::nodir;

    if deltax < 0 {
        d[1] = dirtype::east;
    } else {
        d[1] = dirtype::west;
    }
    if deltay < 0 {
        d[2] = dirtype::south;
    } else {
        d[2] = dirtype::north;
    }

    if abs(deltay) > abs(deltax) {
        tdir = d[1];
        d[1] = d[2];
        d[2] = tdir;
    }

    ob.objlist[ob.objlist_i].dir = d[1];
    if TryWalk(w3d, ob) {
        return; /*either moved forward or attacked*/
    }

    ob.objlist[ob.objlist_i].dir = d[2];
    if TryWalk(w3d, ob) {
        return;
    }

    if US_RndT(w3d) > 128
    /*randomly determine direction of search*/
    {
        // tdir=(dirtype)(tdir+1))
        for tdir in dirtype::north as usize..=dirtype::west as usize {
            if tdir <= dirtype::west as usize {
                ob.objlist[ob.objlist_i].dir = dirtype::from_u8(tdir as u8);
                if TryWalk(w3d, ob) {
                    return;
                }
            }
        }
    } else {
        // tdir=(dirtype)(tdir-1))
        for tdir in (dirtype::west as usize..=dirtype::north as usize).rev() {
            if tdir >= dirtype::north as usize {
                ob.objlist[ob.objlist_i].dir = dirtype::from_u8(tdir as u8);
                if TryWalk(w3d, ob) {
                    return;
                }
            }
        }
    }

    ob.objlist[ob.objlist_i].dir = dirtype::nodir; // can't move
}

/*
=================
=
= MoveObj
=
= Moves ob be move global units in ob->dir direction
= Actors are not allowed to move inside the player
= Does NOT check to see if the move is tile map valid
=
= ob->x                 = adjusted for new position
= ob->y
=
=================
*/

pub fn MoveObj(w3d: &mut modules, ob: &mut object, Move: i32) {
    //println!("MoveObj");

    //let player = ob.objlist[0];

    let mut moveok: bool = false;
    let deltax: i32;
    let deltay: i32;

    match ob.objlist[ob.objlist_i].dir {
        dirtype::north => {
            ob.objlist[ob.objlist_i].y -= Move;
        }
        dirtype::northeast => {
            ob.objlist[ob.objlist_i].x += Move;
            ob.objlist[ob.objlist_i].y -= Move;
        }
        dirtype::east => {
            ob.objlist[ob.objlist_i].x += Move;
        }
        dirtype::southeast => {
            ob.objlist[ob.objlist_i].x += Move;
            ob.objlist[ob.objlist_i].y += Move;
        }
        dirtype::south => {
            ob.objlist[ob.objlist_i].y += Move;
        }
        dirtype::southwest => {
            ob.objlist[ob.objlist_i].x -= Move;
            ob.objlist[ob.objlist_i].y += Move;
        }
        dirtype::west => {
            ob.objlist[ob.objlist_i].x -= Move;
        }
        dirtype::northwest => {
            ob.objlist[ob.objlist_i].x -= Move;
            ob.objlist[ob.objlist_i].y -= Move;
        }
        dirtype::nodir => {
            return;
        }
    }

    //
    // check to make sure it's not on top of player
    //

    if ob.objlist[ob.objlist_i].areanumber >= NUMAREAS
        || w3d.wl_act1.areabyplayer[ob.objlist[ob.objlist_i].areanumber as usize]
    {
        deltax = ob.objlist[ob.objlist_i].x - ob.objlist[0].x;
        if deltax < -MINACTORDIST || deltax > MINACTORDIST {
            //BUG
            //goto moveok;
            moveok = true;
        }

        if !moveok {
            deltay = ob.objlist[ob.objlist_i].y - ob.objlist[0].y;
            if deltay < -MINACTORDIST || deltay > MINACTORDIST {
                //BUG
                //goto moveok;
                moveok = true;
            }
        }

        if !moveok {
            if ob.objlist[ob.objlist_i].hidden != 0
                && w3d.wl_play.spotvis[ob.objlist[0].tilex as usize][ob.objlist[0].tiley as usize]
            {
                //BUG
                //goto moveok;
                moveok = true; // move closer until he meets CheckLine
            }
        }

        if !moveok {
            if ob.objlist[ob.objlist_i].obclass == classtype::ghostobj
                || ob.objlist[ob.objlist_i].obclass == classtype::spectreobj
            {
                TakeDamage(w3d, ob, w3d.wl_play.tics * 2);
            }

            //
            // back up
            //
            match ob.objlist[ob.objlist_i].dir {
                dirtype::north => {
                    ob.objlist[ob.objlist_i].y += Move;
                }
                dirtype::northeast => {
                    ob.objlist[ob.objlist_i].x -= Move;
                    ob.objlist[ob.objlist_i].y += Move;
                }
                dirtype::east => {
                    ob.objlist[ob.objlist_i].x -= Move;
                }
                dirtype::southeast => {
                    ob.objlist[ob.objlist_i].x -= Move;
                    ob.objlist[ob.objlist_i].y -= Move;
                }
                dirtype::south => {
                    ob.objlist[ob.objlist_i].y -= Move;
                }
                dirtype::southwest => {
                    ob.objlist[ob.objlist_i].x += Move;
                    ob.objlist[ob.objlist_i].y -= Move;
                }
                dirtype::west => {
                    ob.objlist[ob.objlist_i].x += Move;
                }
                dirtype::northwest => {
                    ob.objlist[ob.objlist_i].x += Move;
                    ob.objlist[ob.objlist_i].y += Move;
                }

                dirtype::nodir => {
                    return;
                }
            }
            return;
        }
    }
    //moveok:

    ob.objlist[ob.objlist_i].distance -= Move;
}

/*
=============================================================================

                                STUFF

=============================================================================
*/

/*
===============
=
= KillActor
=
===============
*/

pub fn KillActor(w3d: &mut modules, ob: &mut object, closest_i: usize) {
    //println!("KillActor");

    let tilex: i32;
    let tiley: i32;

    tilex = ob.objlist[closest_i].x >> TILESHIFT; // drop item on center
    tiley = ob.objlist[closest_i].y >> TILESHIFT;

    match ob.objlist[closest_i].obclass {
        classtype::guardobj => {
            GivePoints(w3d, 100);
            NewState(w3d, ob, closest_i, s_grddie1);
            PlaceItemType(w3d, wl_stat_t::bo_clip2 as i32, tilex, tiley);
        }

        classtype::officerobj => {
            GivePoints(w3d, 400);
            NewState(w3d, ob, closest_i, s_ofcdie1);
            PlaceItemType(w3d, wl_stat_t::bo_clip2 as i32, tilex, tiley);
        }

        classtype::mutantobj => {
            GivePoints(w3d, 700);
            NewState(w3d, ob, closest_i, s_mutdie1);
            PlaceItemType(w3d, wl_stat_t::bo_clip2 as i32, tilex, tiley);
        }

        classtype::ssobj => {
            GivePoints(w3d, 500);
            NewState(w3d, ob, closest_i, s_ssdie1);
            if w3d.wl_game.gamestate.bestweapon < weapontype::wp_machinegun {
                PlaceItemType(w3d, wl_stat_t::bo_machinegun as i32, tilex, tiley);
            } else {
                PlaceItemType(w3d, wl_stat_t::bo_clip2 as i32, tilex, tiley);
            }
        }

        classtype::dogobj => {
            GivePoints(w3d, 200);
            NewState(w3d, ob, closest_i, s_dogdie1);
        }

        classtype::bossobj => {
            GivePoints(w3d, 5000);
            NewState(w3d, ob, closest_i, s_bossdie1);
            PlaceItemType(w3d, wl_stat_t::bo_key1 as i32, tilex, tiley);
        }

        classtype::gretelobj => {
            GivePoints(w3d, 5000);
            NewState(w3d, ob, closest_i, s_greteldie1);
            PlaceItemType(w3d, wl_stat_t::bo_key1 as i32, tilex, tiley);
        }

        classtype::giftobj => {
            GivePoints(w3d, 5000);
            w3d.wl_game.gamestate.killx = ob.objlist[0].x;
            w3d.wl_game.gamestate.killy = ob.objlist[0].y;
            NewState(w3d, ob, closest_i, s_giftdie1);
        }

        classtype::fatobj => {
            GivePoints(w3d, 5000);
            w3d.wl_game.gamestate.killx = ob.objlist[0].x;
            w3d.wl_game.gamestate.killy = ob.objlist[0].y;
            NewState(w3d, ob, closest_i, s_fatdie1);
        }

        classtype::schabbobj => {
            GivePoints(w3d, 5000);
            w3d.wl_game.gamestate.killx = ob.objlist[0].x;
            w3d.wl_game.gamestate.killy = ob.objlist[0].y;
            NewState(w3d, ob, closest_i, s_schabbdie1);
        }
        classtype::fakeobj => {
            GivePoints(w3d, 2000);
            NewState(w3d, ob, closest_i, s_fakedie1);
        }

        classtype::mechahitlerobj => {
            GivePoints(w3d, 5000);
            NewState(w3d, ob, closest_i, s_mechadie1);
        }
        classtype::realhitlerobj => {
            GivePoints(w3d, 5000);
            w3d.wl_game.gamestate.killx = ob.objlist[0].x;
            w3d.wl_game.gamestate.killy = ob.objlist[0].y;
            NewState(w3d, ob, closest_i, s_hitlerdie1);
        }
        _ => (),
    }

    w3d.wl_game.gamestate.killcount += 1;
    ob.objlist[closest_i].flags &= !(objflag_t::FL_SHOOTABLE as i32);
    ob.actorat[ob.objlist[closest_i].tilex as usize][ob.objlist[closest_i].tiley as usize] =
        ptr::null_mut();
    ob.objlist[closest_i].flags |= objflag_t::FL_NONMARK as i32;
}

/*
===================
=
= DamageActor
=
= Called when the player succesfully hits an enemy.
=
= Does damage points to enemy ob, either putting it into a stun frame or
= killing it.
=
===================
*/

pub fn DamageActor(w3d: &mut modules, ob: &mut object, closest_i: usize, damage: i32) {
    //println!("DamageActor");

    let mut damage_mut = damage;

    w3d.wl_play.madenoise = true;

    //
    // do double damage if shooting a non attack mode actor
    //
    if (ob.objlist[closest_i].flags & objflag_t::FL_ATTACKMODE as i32) == 0 {
        damage_mut <<= 1;
    }

    ob.objlist[closest_i].hitpoints -= damage_mut;

    if ob.objlist[closest_i].hitpoints <= 0 {
        KillActor(w3d, ob, closest_i);
    } else {
        if (ob.objlist[closest_i].flags & objflag_t::FL_ATTACKMODE as i32) == 0 {
            FirstSighting(w3d, ob); // put into combat mode
        }

        match ob.objlist[closest_i].obclass                // dogs only have one hit point
        {
            classtype::guardobj => {
                if ob.objlist[closest_i].hitpoints&1 !=0 {
                    NewState (w3d,ob,closest_i,s_grdpain);
                }
                else {
                    NewState (w3d,ob,closest_i,s_grdpain1);
                }
            }
                classtype::officerobj => {
                if ob.objlist[closest_i].hitpoints&1 !=0{
                    NewState (w3d,ob,closest_i,s_ofcpain);
                }
                else {
                    NewState (w3d,ob,closest_i,s_ofcpain1);
                }
                }

                classtype::mutantobj => {
                if ob.objlist[closest_i].hitpoints&1 !=0{
                    NewState (w3d,ob,closest_i,s_mutpain);
                }
                else {
                    NewState (w3d,ob,closest_i,s_mutpain1);
                }
            }
                classtype::ssobj => {
                if ob.objlist[closest_i].hitpoints&1 !=0{
                    NewState (w3d,ob,closest_i,s_sspain);
                }
                else {
                    NewState (w3d,ob,closest_i,s_sspain1);
                }

                }
                _ => (),
        }
    }
}

/*
=============================================================================

                                CHECKSIGHT

=============================================================================
*/

/*
=====================
=
= CheckLine
=
= Returns true if a straight line between the player and ob is unobstructed
=
=====================
*/

pub fn CheckLine(w3d: &mut modules, ob: &mut object, obj: objtype) -> bool {
    //println!("CheckLine");

    //let obj = ob.objlist[ob.objlist_i];
    //let player = ob.objlist[0];

    let x1: i32;
    let y1: i32;
    let xt1: i32;
    let yt1: i32;
    let x2: i32;
    let y2: i32;
    let mut xt2: i32;
    let mut yt2: i32;
    let mut x: i32;
    let mut y: i32;
    let xdist: i32;
    let ydist: i32;
    let mut xstep: i32;
    let mut ystep: i32;
    let mut partial: i32;
    let mut delta: i32;
    let mut ltemp: i32;
    let mut xfrac: i32;
    let mut yfrac: i32;
    let mut deltafrac: i32;
    let mut value: i32;
    let mut intercept: i32;

    x1 = obj.x >> UNSIGNEDSHIFT; // 1/256 tile precision
    y1 = obj.y >> UNSIGNEDSHIFT;
    xt1 = x1 >> 8;
    yt1 = y1 >> 8;

    x2 = w3d.wl_agent.plux as i32;
    y2 = w3d.wl_agent.pluy as i32;
    xt2 = ob.objlist[0].tilex;
    yt2 = ob.objlist[0].tiley;

    xdist = abs(xt2 - xt1);

    if xdist > 0 {
        if xt2 > xt1 {
            partial = 256 - (x1 & 0xff);
            xstep = 1;
        } else {
            partial = x1 & 0xff;
            xstep = -1;
        }

        deltafrac = abs(x2 - x1);
        delta = y2 - y1;
        ltemp = (delta << 8) / deltafrac;
        if ltemp > 0x7fff {
            ystep = 0x7fff;
        } else if ltemp < -0x7fff {
            ystep = -0x7fff;
        } else {
            ystep = ltemp;
        }
        yfrac = y1 + ((ystep * partial) >> 8);

        x = xt1 + xstep;
        xt2 += xstep;

        loop {
            y = yfrac >> 8;
            yfrac += ystep;

            value = w3d.wl_play.tilemap[x as usize][y as usize];
            x += xstep;

            if value == 0 {
                if x == xt2 {
                    break;
                }
                continue;
            }

            if value < BIT_DOOR || value > BIT_ALLTILES {
                return false;
            }

            //
            // see if the door is open enough
            //
            value &= !BIT_DOOR;
            intercept = yfrac - ystep / 2;

            if intercept > w3d.wl_act1.doorposition[value as usize] {
                return false;
            }

            if x == xt2 {
                break;
            }
        }
    }

    ydist = abs(yt2 - yt1);

    if ydist > 0 {
        if yt2 > yt1 {
            partial = 256 - (y1 & 0xff);
            ystep = 1;
        } else {
            partial = y1 & 0xff;
            ystep = -1;
        }

        deltafrac = abs(y2 - y1);
        delta = x2 - x1;
        ltemp = (delta << 8) / deltafrac;
        if ltemp > 0x7fff {
            xstep = 0x7fff;
        } else if ltemp < -0x7fff {
            xstep = -0x7fff;
        } else {
            xstep = ltemp;
        }
        xfrac = x1 + ((xstep * partial) >> 8);

        y = yt1 + ystep;
        yt2 += ystep;

        loop {
            x = xfrac >> 8;
            xfrac += xstep;

            value = w3d.wl_play.tilemap[x as usize][y as usize];
            y += ystep;

            if value == 0 {
                if y == yt2 {
                    break;
                }
                continue;
            }

            if value < BIT_DOOR || value > BIT_ALLTILES {
                return false;
            }

            //
            // see if the door is open enough
            //
            value &= !BIT_DOOR;
            intercept = xfrac - xstep / 2;

            if intercept > w3d.wl_act1.doorposition[value as usize] {
                return false;
            }
            if y == yt2 {
                break;
            }
        }
    }

    return true;
}

/*
================
=
= CheckSight
=
= Checks a straight line between player and current object
=
= If the sight is ok, check alertness and angle to see if they notice
=
= returns true if the player has been spoted
=
================
*/

pub fn CheckSight(w3d: &mut modules, ob: &mut object) -> bool {
    //println!("CheckSight");

    let deltax: i32;
    let deltay: i32;

    //
    // don't bother tracing a line if the area isn't connected to the player's
    //
    if ob.objlist[ob.objlist_i].areanumber < NUMAREAS
        && !w3d.wl_act1.areabyplayer[ob.objlist[ob.objlist_i].areanumber as usize]
    {
        return false;
    }

    //
    // if the player is real close, sight is automatic
    //
    deltax = ob.objlist[0].x - ob.objlist[ob.objlist_i].x;
    deltay = ob.objlist[0].y - ob.objlist[ob.objlist_i].y;

    if deltax > -MINSIGHT && deltax < MINSIGHT && deltay > -MINSIGHT && deltay < MINSIGHT {
        return true;
    }

    //
    // see if they are looking in the right direction
    //
    match ob.objlist[ob.objlist_i].dir {
        dirtype::north => {
            if deltay > 0 {
                return false;
            }
        }

        dirtype::east => {
            if deltax < 0 {
                return false;
            }
        }

        dirtype::south => {
            if deltay < 0 {
                return false;
            }
        }

        dirtype::west => {
            if deltax > 0 {
                return false;
            }
        }

        // check diagonal moving guards fix
        dirtype::northwest => {
            if DEMOCOND_SDL(w3d) && deltay > -deltax {
                return false;
            }
        }

        dirtype::northeast => {
            if DEMOCOND_SDL(w3d) && deltay > deltax {
                return false;
            }
        }

        dirtype::southwest => {
            if DEMOCOND_SDL(w3d) && deltax > deltay {
                return false;
            }
        }

        dirtype::southeast => {
            if DEMOCOND_SDL(w3d) && -deltax > deltay {
                return false;
            }
        }
        _ => (),
    }

    //
    // trace a line to check for blocking tiles (corners)
    //
    let obj = ob.objlist[ob.objlist_i];
    return CheckLine(w3d, ob, obj);
}

/*
===============
=
= FirstSighting
=
= Puts an actor into attack mode and possibly reverses the direction
= if the player is behind it
=
===============
*/

pub fn FirstSighting(w3d: &mut modules, ob: &mut object) {
    //println!("FirstSighting");

    //
    // react to the player
    //
    match ob.objlist[ob.objlist_i].obclass {
        classtype::guardobj => {
            PlaySoundLocActor(w3d, ob, soundnames::HALTSND);
            NewState(w3d, ob, ob.objlist_i, s_grdchase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::officerobj => {
            PlaySoundLocActor(w3d, ob, soundnames::SPIONSND);
            NewState(w3d, ob, ob.objlist_i, s_ofcchase1);
            ob.objlist[ob.objlist_i].speed *= 5; // go faster when chasing player
        }

        classtype::mutantobj => {
            NewState(w3d, ob, ob.objlist_i, s_mutchase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::ssobj => {
            PlaySoundLocActor(w3d, ob, soundnames::SCHUTZADSND);
            NewState(w3d, ob, ob.objlist_i, s_sschase1);
            ob.objlist[ob.objlist_i].speed *= 4; // go faster when chasing player
        }

        classtype::dogobj => {
            PlaySoundLocActor(w3d, ob, soundnames::DOGBARKSND);
            NewState(w3d, ob, ob.objlist_i, s_dogchase1);
            ob.objlist[ob.objlist_i].speed *= 2; // go faster when chasing player
        }

        classtype::bossobj => {
            SD_PlaySound(w3d, soundnames::GUTENTAGSND);
            NewState(w3d, ob, ob.objlist_i, s_bosschase1);
            ob.objlist[ob.objlist_i].speed = SPDPATROL * 3; // go faster when chasing player
        }

        classtype::gretelobj => {
            SD_PlaySound(w3d, soundnames::KEINSND);
            NewState(w3d, ob, ob.objlist_i, s_gretelchase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::giftobj => {
            SD_PlaySound(w3d, soundnames::EINESND);
            NewState(w3d, ob, ob.objlist_i, s_giftchase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::fatobj => {
            SD_PlaySound(w3d, soundnames::ERLAUBENSND);
            NewState(w3d, ob, ob.objlist_i, s_fatchase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::schabbobj => {
            SD_PlaySound(w3d, soundnames::SCHABBSHASND);
            NewState(w3d, ob, ob.objlist_i, s_schabbchase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::fakeobj => {
            SD_PlaySound(w3d, soundnames::TOT_HUNDSND);
            NewState(w3d, ob, ob.objlist_i, s_fakechase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::mechahitlerobj => {
            SD_PlaySound(w3d, soundnames::DIESND);
            NewState(w3d, ob, ob.objlist_i, s_mechachase1);
            ob.objlist[ob.objlist_i].speed *= 3; // go faster when chasing player
        }

        classtype::realhitlerobj => {
            SD_PlaySound(w3d, soundnames::DIESND);
            NewState(w3d, ob, ob.objlist_i, s_hitlerchase1);
            ob.objlist[ob.objlist_i].speed *= 5; // go faster when chasing player
        }

        classtype::ghostobj => {
            NewState(w3d, ob, ob.objlist_i, s_blinkychase1);
            ob.objlist[ob.objlist_i].speed *= 2; // go faster when chasing player
        }
        _ => (),
    }

    if ob.objlist[ob.objlist_i].distance < 0 {
        ob.objlist[ob.objlist_i].distance = 0; // ignore the door opening command
    }

    ob.objlist[ob.objlist_i].flags |=
        objflag_t::FL_ATTACKMODE as i32 | objflag_t::FL_FIRSTATTACK as i32;
}

/*
===============
=
= SightPlayer
=
= Called by actors that ARE NOT chasing the player.  If the player
= is detected (by sight, noise, or proximity), the actor is put into
= it's combat frame and true is returned.
=
= Incorporates a random reaction delay
=
===============
*/

pub fn SightPlayer(w3d: &mut modules, ob: &mut object) -> bool {
    //println!("SightPlayer");

    if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_ATTACKMODE as i32) != 0 {
        Quit("An actor in ATTACKMODE called SightPlayer!");
    }

    if ob.objlist[ob.objlist_i].temp2 != 0 {
        //
        // count down reaction time
        //
        ob.objlist[ob.objlist_i].temp2 -= w3d.wl_play.tics;
        if ob.objlist[ob.objlist_i].temp2 > 0 {
            return false;
        }
        ob.objlist[ob.objlist_i].temp2 = 0; // time to react
    } else {
        if ob.objlist[ob.objlist_i].areanumber < NUMAREAS
            && !w3d.wl_act1.areabyplayer[ob.objlist[ob.objlist_i].areanumber as usize]
        {
            return false;
        }

        if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_AMBUSH as i32) != 0 {
            if !CheckSight(w3d, ob) {
                return false;
            }
            ob.objlist[ob.objlist_i].flags &= !(objflag_t::FL_AMBUSH as i32);
        } else {
            if !w3d.wl_play.madenoise && !CheckSight(w3d, ob) {
                return false;
            }
        }

        match ob.objlist[ob.objlist_i].obclass {
            classtype::guardobj => {
                ob.objlist[ob.objlist_i].temp2 = 1 + US_RndT(w3d) / 4;
            }
            classtype::officerobj => {
                ob.objlist[ob.objlist_i].temp2 = 2;
            }
            classtype::mutantobj => {
                ob.objlist[ob.objlist_i].temp2 = 1 + US_RndT(w3d) / 6;
            }
            classtype::ssobj => {
                ob.objlist[ob.objlist_i].temp2 = 1 + US_RndT(w3d) / 6;
            }
            classtype::dogobj => {
                ob.objlist[ob.objlist_i].temp2 = 1 + US_RndT(w3d) / 8;
            }

            classtype::bossobj
            | classtype::schabbobj
            | classtype::fakeobj
            | classtype::mechahitlerobj
            | classtype::realhitlerobj
            | classtype::gretelobj
            | classtype::giftobj
            | classtype::fatobj
            | classtype::spectreobj
            | classtype::angelobj
            | classtype::transobj
            | classtype::uberobj
            | classtype::willobj
            | classtype::deathobj => {
                ob.objlist[ob.objlist_i].temp2 = 1;
            }
            _ => (),
        }
        return false;
    }

    FirstSighting(w3d, ob);

    return true;
}
