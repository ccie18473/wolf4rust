#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_agent
//
//===========================================================================

pub struct wl_agent {
    //
    // player state info
    //
    pub thrustspeed: i32,
    pub plux: i16, // player coordinates scaled to unsigned
    pub pluy: i16,
    pub anglefrac: i32,
    pub LastAttacker: objtype,
    pub facecount: i32,
    pub facetimes: i32,
}

impl wl_agent {
    pub fn new() -> Self {
        Self {
            thrustspeed: 0,
            plux: 0, // player coordinates scaled to unsigned
            pluy: 0,
            anglefrac: 0,
            LastAttacker: objtype::new(),
            facecount: 0,
            facetimes: 0,
        }
    }
    pub fn clear(&mut self) {
        self.thrustspeed = 0;
        self.plux = 0;
        self.pluy = 0;
        self.anglefrac = 0;
        self.LastAttacker = objtype::new();
        self.facecount = 0;
        self.facetimes = 0;
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const MOVESCALE: i32 = 150;
pub const BACKMOVESCALE: i32 = 100;
pub const ANGLESCALE: i32 = 20;

pub const PUSHWALLMINDIST: i32 = PLAYERSIZE;

//statetype   s_player = {false,0,0,(statefunc) T_Player,NULL,NULL};
//statetype   s_attack = {false,0,0,(statefunc) T_Attack,NULL,NULL};

pub const s_player: statetype = statetype {
    rotate: 0,
    shapenum: 0,
    tictime: 0,
    think: true,
    action: false,
    id: 1,
};

pub const s_attack: statetype = statetype {
    rotate: 0,
    shapenum: 0,
    tictime: 0,
    think: true,
    action: false,
    id: 2,
};

#[derive(Copy, Clone)]
pub struct atkinf {
    pub tics: i32, // attack is 1 for gun, 2 for knife
    pub attack: i8,
    pub frame: i32,
}

pub const attackinfo: [[atkinf; 4]; 4] = [
    [
        atkinf {
            tics: 6,
            attack: 0,
            frame: 1,
        },
        atkinf {
            tics: 6,
            attack: 2,
            frame: 2,
        },
        atkinf {
            tics: 6,
            attack: 0,
            frame: 3,
        },
        atkinf {
            tics: 6,
            attack: -1,
            frame: 4,
        },
    ],
    [
        atkinf {
            tics: 6,
            attack: 0,
            frame: 1,
        },
        atkinf {
            tics: 6,
            attack: 1,
            frame: 2,
        },
        atkinf {
            tics: 6,
            attack: 0,
            frame: 3,
        },
        atkinf {
            tics: 6,
            attack: -1,
            frame: 4,
        },
    ],
    [
        atkinf {
            tics: 6,
            attack: 0,
            frame: 1,
        },
        atkinf {
            tics: 6,
            attack: 1,
            frame: 2,
        },
        atkinf {
            tics: 6,
            attack: 3,
            frame: 3,
        },
        atkinf {
            tics: 6,
            attack: -1,
            frame: 4,
        },
    ],
    [
        atkinf {
            tics: 6,
            attack: 0,
            frame: 1,
        },
        atkinf {
            tics: 6,
            attack: 1,
            frame: 2,
        },
        atkinf {
            tics: 6,
            attack: 4,
            frame: 3,
        },
        atkinf {
            tics: 6,
            attack: -1,
            frame: 4,
        },
    ],
];

/*
=============================================================================

                                CONTROL STUFF

=============================================================================
*/

/*
======================
=
= CheckWeaponChange
=
= Keys 1-4 change weapons
=
======================
*/

pub fn CheckWeaponChange(w3d: &mut modules) {
    //println!("CheckWeaponChange");

    let mut newWeapon: i32 = -1;

    if w3d.wl_game.gamestate.ammo == 0 {
        // must use knife with no ammo
        return;
    }

    if w3d.wl_play.buttonstate[buttontype::bt_nextweapon as usize]
        && !w3d.wl_play.buttonheld[buttontype::bt_nextweapon as usize]
    {
        newWeapon = w3d.wl_game.gamestate.weapon as i32 + 1;

        if newWeapon > w3d.wl_game.gamestate.bestweapon as i32 {
            newWeapon = 0;
        }
    } else if w3d.wl_play.buttonstate[buttontype::bt_prevweapon as usize]
        && !w3d.wl_play.buttonheld[buttontype::bt_prevweapon as usize]
    {
        newWeapon = w3d.wl_game.gamestate.weapon as i32 - 1;

        if newWeapon < 0 {
            newWeapon = w3d.wl_game.gamestate.bestweapon as i32;
        }
    } else {
        for i in weapontype::wp_knife as i32..=w3d.wl_game.gamestate.bestweapon as i32 {
            if w3d.wl_play.buttonstate
                [(buttontype::bt_readyknife as i32 + i - weapontype::wp_knife as i32) as usize]
            {
                newWeapon = i;
                break;
            }
        }
    }

    if newWeapon != -1 {
        let newWeapon_enum = weapontype::from_u8(newWeapon as i8);

        w3d.wl_game.gamestate.weapon = newWeapon_enum;
        w3d.wl_game.gamestate.chosenweapon = newWeapon_enum;

        DrawWeapon(w3d);
    }
}

/*
=======================
=
= ControlMovement
=
= Takes controlx,controly, and buttonstate[bt_strafe]
=
= Changes the player's angle and position
=
= There is an angle hack because when going 70 fps, the roundoff becomes
= significant
=
=======================
*/

pub fn ControlMovement(w3d: &mut modules, ob: &mut object) {
    //println!("ControlMovement");

    let mut angle: i32;
    let angleunits: i32;

    w3d.wl_agent.thrustspeed = 0;

    if w3d.wl_play.buttonstate[buttontype::bt_strafeleft as usize] {
        angle = ob.objlist[0].angle + ANGLES / 4;

        if angle >= ANGLES {
            angle -= ANGLES;
        }
        if w3d.wl_play.buttonstate[buttontype::bt_run as usize] {
            Thrust(w3d, ob, angle, RUNMOVE * MOVESCALE * w3d.wl_play.tics);
        } else {
            Thrust(w3d, ob, angle, BASEMOVE * MOVESCALE * w3d.wl_play.tics);
        }
    }

    if w3d.wl_play.buttonstate[buttontype::bt_straferight as usize] {
        angle = ob.objlist[0].angle - ANGLES / 4;

        if angle < 0 {
            angle += ANGLES;
        }
        if w3d.wl_play.buttonstate[buttontype::bt_run as usize] {
            Thrust(w3d, ob, angle, RUNMOVE * MOVESCALE * w3d.wl_play.tics);
        } else {
            Thrust(w3d, ob, angle, BASEMOVE * MOVESCALE * w3d.wl_play.tics);
        }
    }

    //
    // side to side move
    //
    if w3d.wl_play.buttonstate[buttontype::bt_strafe as usize] {
        //
        // strafing
        //
        //
        if w3d.wl_play.controlx > 0 {
            angle = ob.objlist[0].angle - ANGLES / 4;
            if angle < 0 {
                angle += ANGLES;
            }
            Thrust(w3d, ob, angle, w3d.wl_play.controlx * MOVESCALE); // move to left
        } else if w3d.wl_play.controlx < 0 {
            angle = ob.objlist[0].angle + ANGLES / 4;
            if angle >= ANGLES {
                angle -= ANGLES;
            }
            Thrust(w3d, ob, angle, -w3d.wl_play.controlx * MOVESCALE); // move to right
        }
    } else {
        //
        // not strafing
        //
        w3d.wl_agent.anglefrac += w3d.wl_play.controlx;
        angleunits = w3d.wl_agent.anglefrac / ANGLESCALE;
        w3d.wl_agent.anglefrac -= angleunits * ANGLESCALE;
        ob.objlist[0].angle -= angleunits;

        if ob.objlist[0].angle >= ANGLES {
            ob.objlist[0].angle -= ANGLES;
        }
        if ob.objlist[0].angle < 0 {
            ob.objlist[0].angle += ANGLES;
        }
    }

    //
    // forward/backwards move
    //
    if w3d.wl_play.controly < 0 {
        Thrust(
            w3d,
            ob,
            ob.objlist[0].angle,
            -w3d.wl_play.controly * MOVESCALE,
        ); // move forwards
    } else if w3d.wl_play.controly > 0 {
        angle = ob.objlist[0].angle + ANGLES / 2;
        if angle >= ANGLES {
            angle -= ANGLES;
        }
        Thrust(w3d, ob, angle, w3d.wl_play.controly * BACKMOVESCALE); // move backwards
    }

    if w3d.wl_game.gamestate.victoryflag {
        // watching the BJ actor
        return;
    }
}

/*
=============================================================================

                            STATUS WINDOW STUFF

=============================================================================
*/

/*
==================
=
= StatusDrawPic
=
==================
*/

pub fn StatusDrawPic(w3d: &mut modules, x: i32, y: i32, picnum: i32) {
    //println!("StatusDrawPic");

    VWB_DrawPicScaledCoord(
        w3d,
        ((w3d.id_vl.screenWidth - w3d.id_vl.scaleFactor * 320) / 16 + w3d.id_vl.scaleFactor * x)
            * 8,
        w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * (STATUSLINES as i32 - y),
        picnum,
    );
}

pub fn StatusDrawFace(w3d: &mut modules, picnum: i32) {
    //println!("StatusDrawFace");

    StatusDrawPic(w3d, 17, 4, picnum);
}

/*
==================
=
= DrawFace
=
==================
*/

pub fn DrawFace(w3d: &mut modules) {
    //println!("DrawFace");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }
    if SD_SoundPlaying(w3d) == soundnames::GETGATLINGSND as i32 {
        StatusDrawFace(w3d, graphicnums::GOTGATLINGPIC as i32);
    } else if w3d.wl_game.gamestate.health != 0 {
        StatusDrawFace(
            w3d,
            graphicnums::FACE1APIC as i32
                + 3 * ((100 - w3d.wl_game.gamestate.health as i32) / 16)
                + w3d.wl_game.gamestate.faceframe as i32,
        );
    } else {
        if w3d.wl_agent.LastAttacker != objtype::new()
            && w3d.wl_agent.LastAttacker.obclass == classtype::needleobj
        {
            StatusDrawFace(w3d, graphicnums::MUTANTBJPIC as i32);
        } else {
            StatusDrawFace(w3d, graphicnums::FACE8APIC as i32);
        }
    }
}

/*
===============
=
= UpdateFace
=
= Calls draw face if time to change
=
===============
*/

pub fn UpdateFace(w3d: &mut modules) {
    //println!("UpdateFace");

    // don't make demo depend on sound playback
    if w3d.wl_play.demoplayback || w3d.wl_play.demorecord {
        if w3d.wl_agent.facetimes > 0 {
            w3d.wl_agent.facetimes -= 1;
            return;
        }
    } else if SD_SoundPlaying(w3d) == soundnames::GETGATLINGSND as i32 {
        return;
    }

    w3d.wl_agent.facecount += w3d.wl_play.tics as i32;
    if w3d.wl_agent.facecount > US_RndT(w3d) as i32 {
        w3d.wl_game.gamestate.faceframe = (US_RndT(w3d) >> 6) as i32;
        if w3d.wl_game.gamestate.faceframe == 3 {
            w3d.wl_game.gamestate.faceframe = 1;
        }

        w3d.wl_agent.facecount = 0;
        DrawFace(w3d);
    }
}

/*
===============
=
= LatchNumber
=
= right justifies and pads with blanks
=
===============
*/

pub fn LatchNumber(w3d: &mut modules, x: &mut i32, y: i32, width: &mut i32, number: i32) {
    //println!("LatchNumber");

    let mut c: usize;
    let str = number.to_string();
    let mut chars = str.chars();
    let length = str.len();

    while length < *width as usize {
        StatusDrawPic(w3d, *x, y, graphicnums::N_BLANKPIC as i32);
        *x += 1;
        *width -= 1;
    }

    if length <= *width as usize {
        c = 0;
    } else {
        c = length - *width as usize;
    }

    while c < length {
        StatusDrawPic(
            w3d,
            *x,
            y,
            chars.next().unwrap() as i32 - '0' as i32 + graphicnums::N_0PIC as i32,
        );
        *x += 1;
        c += 1;
    }
}

/*
===============
=
= DrawHealth
=
===============
*/

pub fn DrawHealth(w3d: &mut modules) {
    //println!("DrawHealth");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }
    LatchNumber(w3d, &mut 21, 16, &mut 3, w3d.wl_game.gamestate.health);
}

/*
===============
=
= TakeDamage
=
===============
*/

pub fn TakeDamage(w3d: &mut modules, ob: &mut object, points: i32) {
    //println!("TakeDamage");

    let attacker = ob.objlist[ob.objlist_i];

    w3d.wl_agent.LastAttacker = attacker;

    if w3d.wl_game.gamestate.victoryflag {
        return;
    }
    let mut points = points;
    if w3d.wl_game.gamestate.difficulty == gd::gd_baby as i32 {
        points >>= 2;
    }

    if w3d.wl_play.godmode == 0 {
        w3d.wl_game.gamestate.health -= points;
    }

    if w3d.wl_game.gamestate.health <= 0 {
        w3d.wl_game.gamestate.health = 0;
        w3d.wl_play.playstate = exit_t::ex_died;
        ob.killerobj = attacker;
    }

    if w3d.wl_play.godmode != 2 {
        StartDamageFlash(w3d, points);
    }

    DrawHealth(w3d);
    DrawFace(w3d);

    ob.objlist[ob.objlist_i] = attacker;
}

/*
===============
=
= HealSelf
=
===============
*/

pub fn HealSelf(w3d: &mut modules, points: i32) {
    //println!("HealSelf");

    w3d.wl_game.gamestate.health += points;

    if w3d.wl_game.gamestate.health > 100 {
        w3d.wl_game.gamestate.health = 100;
    }

    DrawHealth(w3d);
    DrawFace(w3d);
}

//===========================================================================

/*
===============
=
= DrawLevel
=
===============
*/

pub fn DrawLevel(w3d: &mut modules) {
    //println!("DrawLevel");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }
    LatchNumber(w3d, &mut 2, 16, &mut 2, w3d.wl_game.gamestate.mapon + 1);
}

//===========================================================================

/*
===============
=
= DrawLives
=
===============
*/

pub fn DrawLives(w3d: &mut modules) {
    //println!("DrawLives");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }
    LatchNumber(w3d, &mut 14, 16, &mut 1, w3d.wl_game.gamestate.lives);
}

/*
===============
=
= GiveExtraMan
=
===============
*/

pub fn GiveExtraMan(w3d: &mut modules) {
    //println!("GiveExtraMan");

    if w3d.wl_game.gamestate.lives < 9 {
        w3d.wl_game.gamestate.lives += 1;
    }
    DrawLives(w3d);
    SD_PlaySound(w3d, soundnames::BONUS1UPSND);
}

//===========================================================================

/*
===============
=
= DrawScore
=
===============
*/

pub fn DrawScore(w3d: &mut modules) {
    //println!("DrawScore");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }

    LatchNumber(w3d, &mut 6, 16, &mut 6, w3d.wl_game.gamestate.score);
}

/*
===============
=
= GivePoints
=
===============
*/

pub fn GivePoints(w3d: &mut modules, points: i32) {
    //println!("GivePoints");

    w3d.wl_game.gamestate.score += points;

    while w3d.wl_game.gamestate.score >= w3d.wl_game.gamestate.nextextra {
        w3d.wl_game.gamestate.nextextra += EXTRAPOINTS;
        GiveExtraMan(w3d);
    }
    DrawScore(w3d);
}

//===========================================================================

/*
==================
=
= DrawWeapon
=
==================
*/

pub fn DrawWeapon(w3d: &mut modules) {
    //println!("DrawWeapon");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }

    StatusDrawPic(
        w3d,
        32,
        8,
        graphicnums::KNIFEPIC as i32 + w3d.wl_game.gamestate.weapon as i32,
    );
}

/*
==================
=
= DrawKeys
=
==================
*/

pub fn DrawKeys(w3d: &mut modules) {
    //println!("DrawKeys");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }

    if w3d.wl_game.gamestate.keys & 1 != 0 {
        StatusDrawPic(w3d, 30, 4, graphicnums::GOLDKEYPIC as i32);
    } else {
        StatusDrawPic(w3d, 30, 4, graphicnums::NOKEYPIC as i32);
    }

    if w3d.wl_game.gamestate.keys & 2 != 0 {
        StatusDrawPic(w3d, 30, 20, graphicnums::SILVERKEYPIC as i32);
    } else {
        StatusDrawPic(w3d, 30, 20, graphicnums::NOKEYPIC as i32);
    }
}

/*
==================
=
= GiveWeapon
=
==================
*/

pub fn GiveWeapon(w3d: &mut modules, weapon: weapontype) {
    //println!("GiveWeapon");

    GiveAmmo(w3d, 6);

    //    let weapon = weapontype::default();

    if (w3d.wl_game.gamestate.bestweapon as i32) < (weapon as i32) {
        w3d.wl_game.gamestate.bestweapon = weapon;
        w3d.wl_game.gamestate.weapon = weapon;
        w3d.wl_game.gamestate.chosenweapon = weapon;
    }

    DrawWeapon(w3d);
}

//===========================================================================

/*
===============
=
= DrawAmmo
=
===============
*/

pub fn DrawAmmo(w3d: &mut modules) {
    //println!("DrawAmmo");

    if w3d.wl_play.viewsize == 21 && w3d.wl_game.ingame {
        return;
    }
    LatchNumber(w3d, &mut 27, 16, &mut 2, w3d.wl_game.gamestate.ammo as i32);
}

/*
===============
=
= GiveAmmo
=
===============
*/

pub fn GiveAmmo(w3d: &mut modules, ammo: i32) {
    //println!("GiveAmmo");

    if w3d.wl_game.gamestate.ammo == 0
    // knife was out
    {
        if w3d.wl_game.gamestate.attackframe == 0 {
            w3d.wl_game.gamestate.weapon = w3d.wl_game.gamestate.chosenweapon;
            DrawWeapon(w3d);
        }
    }

    w3d.wl_game.gamestate.ammo += ammo;

    if w3d.wl_game.gamestate.ammo > 99 {
        w3d.wl_game.gamestate.ammo = 99;
    }
    DrawAmmo(w3d);
}

//===========================================================================

/*
==================
=
= GiveKey
=
==================
*/

pub fn GiveKey(w3d: &mut modules, key: i32) {
    //println!("GiveKey");

    w3d.wl_game.gamestate.keys |= 1 << key;
    DrawKeys(w3d);
}

/*
=============================================================================

                                MOVEMENT

=============================================================================
*/

/*
===================
=
= GetBonus
=
===================
*/

pub fn GetBonus(w3d: &mut modules, check: &mut statobj_t) {
    //println!("GetBonus");

    if w3d.wl_play.playstate == exit_t::ex_died {
        // ADDEDFIX 31 - Chris
        return;
    }

    match check.itemnumber
    {
        //wl_stat_t::bo_firstaid
        4 => {
            if w3d.wl_game.gamestate.health == 100 {
                return;
            }
            SD_PlaySound (w3d, soundnames::HEALTH2SND);
            HealSelf (w3d,25);
        }

        //wl_stat_t::bo_key1
        5 |
        //wl_stat_t::bo_key2 
        6 |
        //wl_stat_t::bo_key3 
        7 |
        //wl_stat_t::bo_key4 
        8 => {
            GiveKey (w3d,check.itemnumber - wl_stat_t::bo_key1 as i32);
            SD_PlaySound (w3d,soundnames::GETKEYSND);
        }

        //wl_stat_t::bo_cross 
        9 => {
            SD_PlaySound (w3d,soundnames::BONUS1SND);
            GivePoints (w3d,100);
            w3d.wl_game.gamestate.treasurecount +=1;
        }
        //wl_stat_t::bo_chalice 
        10 => {
            SD_PlaySound (w3d,soundnames::BONUS2SND);
            GivePoints (w3d,500);
            w3d.wl_game.gamestate.treasurecount +=1;
        }
        //wl_stat_t::bo_bible
        11 => {
            SD_PlaySound (w3d,soundnames::BONUS3SND);
            GivePoints (w3d,1000);
            w3d.wl_game.gamestate.treasurecount +=1;
        }
        //wl_stat_t::bo_crown
        12 => {
            SD_PlaySound (w3d,soundnames::BONUS4SND);
            GivePoints (w3d,5000);
            w3d.wl_game.gamestate.treasurecount +=1;
        }

        //wl_stat_t::bo_clip
        13 => {
            if w3d.wl_game.gamestate.ammo == 99 {
                return;
            }
            SD_PlaySound (w3d, soundnames::GETAMMOSND);
            GiveAmmo (w3d,8);
        }
        //wl_stat_t::bo_clip2
        14 => {
            if w3d.wl_game.gamestate.ammo == 99 {
                return;
            }
            SD_PlaySound (w3d, soundnames::GETAMMOSND);
            GiveAmmo (w3d,4);
        }

        //wl_stat_t::bo_machinegun
        15 => {
            SD_PlaySound (w3d,soundnames::GETMACHINESND);
            GiveWeapon (w3d,weapontype::wp_machinegun);
        }
        //wl_stat_t::bo_chaingun
        16 => {
            SD_PlaySound (w3d,soundnames::GETGATLINGSND);
            w3d.wl_agent.facetimes = 38;
            GiveWeapon (w3d,weapontype::wp_chaingun);

            if w3d.wl_play.viewsize != 21 {
                StatusDrawFace (w3d, graphicnums::GOTGATLINGPIC as i32);
            }
            w3d.wl_agent.facecount = 0;
        }

        //wl_stat_t::bo_fullheal
        18 => {
            SD_PlaySound (w3d,soundnames::BONUS1UPSND);
            HealSelf (w3d,99);
            GiveAmmo (w3d,25);
            GiveExtraMan (w3d);
            w3d.wl_game.gamestate.treasurecount +=1;
        }

        //wl_stat_t::bo_food
        17 => {
            if w3d.wl_game.gamestate.health == 100 {
                return;
            }

            SD_PlaySound (w3d,soundnames::HEALTH1SND);
            HealSelf (w3d,10);
        }

        //wl_stat_t::bo_alpo
        3 => {
            if w3d.wl_game.gamestate.health == 100 {
                return;
            }

            SD_PlaySound (w3d,soundnames::HEALTH1SND);
            HealSelf (w3d,4);
        }

        //wl_stat_t::bo_gibs
        2 => {
            if w3d.wl_game.gamestate.health >10 {
                return;
            }

            SD_PlaySound (w3d,soundnames::SLURPIESND);
            HealSelf (w3d,1);
        }
        _ => (),
    }

    StartBonusFlash(w3d);
    check.shapenum = -1; // remove from list
}

/*
===================
=
= TryMove
=
= returns true if move ok
= debug: use pointers to optimize
===================
*/

pub fn TryMove(w3d: &mut modules, ob: &mut object) -> bool {
    //println!("TryMove");

    let mut xl: i32;
    let mut yl: i32;
    let mut xh: i32;
    let mut yh: i32;

    let mut check: *mut objtype;

    let mut deltax: i32;
    let mut deltay: i32;

    //let player = ob.objlist[0];

    xl = (ob.objlist[0].x - PLAYERSIZE) >> TILESHIFT;
    yl = (ob.objlist[0].y - PLAYERSIZE) >> TILESHIFT;

    xh = (ob.objlist[0].x + PLAYERSIZE) >> TILESHIFT;
    yh = (ob.objlist[0].y + PLAYERSIZE) >> TILESHIFT;

    //
    // check for solid walls
    //
    for y in yl..=yh {
        for x in xl..=xh {
            check = ob.actorat[x as usize][y as usize];

            if check.is_null() {
                continue;
            }
            if check > 65535 as *mut objtype {
                // it's a pointer not a value
                continue;
            }
            if check == 0 as *mut objtype {
                continue;
            }

            //if (check && !ISPOINTER(check))
            if !check.is_null() {
                if w3d.wl_play.tilemap[x as usize][y as usize] == BIT_WALL
                    && x == w3d.wl_act1.pwallx
                    && y == w3d.wl_act1.pwally
                // back of moving pushwall?
                {
                    match w3d.wl_act1.pwalldir {
                        controldir_t::di_north => {
                            if ob.objlist[0].y - PUSHWALLMINDIST
                                <= (w3d.wl_act1.pwally << TILESHIFT)
                                    + ((63 - w3d.wl_act1.pwallpos) << 10)
                            {
                                return false;
                            }
                        }

                        controldir_t::di_west => {
                            if ob.objlist[0].x - PUSHWALLMINDIST
                                <= (w3d.wl_act1.pwallx << TILESHIFT)
                                    + ((63 - w3d.wl_act1.pwallpos) << 10)
                            {
                                return false;
                            }
                        }
                        controldir_t::di_east => {
                            if ob.objlist[0].x + PUSHWALLMINDIST
                                >= (w3d.wl_act1.pwallx << TILESHIFT) + (w3d.wl_act1.pwallpos << 10)
                            {
                                return false;
                            }
                        }
                        controldir_t::di_south => {
                            if ob.objlist[0].y + PUSHWALLMINDIST
                                >= (w3d.wl_act1.pwally << TILESHIFT) + (w3d.wl_act1.pwallpos << 10)
                            {
                                return false;
                            }
                        }
                        _ => (),
                    }
                } else {
                    return false;
                }
            }
        }
    }

    //
    // check for actors
    //
    if yl > 0 {
        yl -= 1;
    }
    if yh < MAPSIZE as i32 - 1 {
        yh += 1;
    }
    if xl > 0 {
        xl -= 1;
    }
    if xh < MAPSIZE as i32 - 1 {
        xh += 1;
    }

    for y in yl..=yh {
        for x in xl..=xh {
            check = ob.actorat[x as usize][y as usize];

            if check.is_null() {
                continue;
            }
            if check < 65535 as *mut objtype {
                // it's a value not a pointer
                continue;
            }
            //if (ISPOINTER(check) && check != player && (check->flags & FL_SHOOTABLE) )
            if !check.is_null()
                && check != &mut ob.objlist[0] as *mut objtype
                && unsafe { (*check).flags } & objflag_t::FL_SHOOTABLE as i32 != 0
            {
                deltax = ob.objlist[0].x - unsafe { (*check).x };
                if deltax < -MINACTORDIST || deltax > MINACTORDIST {
                    continue;
                }
                deltay = ob.objlist[0].y - unsafe { (*check).y };
                if deltay < -MINACTORDIST || deltay > MINACTORDIST {
                    continue;
                }
                return false;
            }
        }
    }

    return true;
}

/*
===================
=
= ClipMove
=
===================
*/

pub fn ClipMove(w3d: &mut modules, ob: &mut object, xmove: i32, ymove: i32) {
    //println!("ClipMove");

    let basex: i32;
    let basey: i32;

    basex = ob.objlist[0].x;
    basey = ob.objlist[0].y;

    ob.objlist[0].x = basex + xmove;
    ob.objlist[0].y = basey + ymove;

    if TryMove(w3d, ob) {
        return;
    }

    if w3d.wl_play.noclip
        && ob.objlist[0].x > 2 * TILEGLOBAL
        && ob.objlist[0].y > 2 * TILEGLOBAL
        && ob.objlist[0].x < ((w3d.wl_play.mapwidth - 1) << TILESHIFT)
        && ob.objlist[0].y < ((w3d.wl_play.mapheight - 1) << TILESHIFT)
    {
        return; // walk through walls
    }

    if SD_SoundPlaying(w3d) == 0 {
        SD_PlaySound(w3d, soundnames::HITWALLSND);
    }

    ob.objlist[0].x = basex + xmove;
    ob.objlist[0].y = basey;

    if TryMove(w3d, ob) {
        return;
    }

    ob.objlist[0].x = basex;
    ob.objlist[0].y = basey + ymove;

    if TryMove(w3d, ob) {
        return;
    }

    ob.objlist[0].x = basex;
    ob.objlist[0].y = basey;
}

//==========================================================================

/*
===================
=
= VictoryTile
=
===================
*/

pub fn VictoryTile(w3d: &mut modules, ob: &mut object) {
    //println!("VictoryTile");

    SpawnBJVictory(w3d, ob);

    w3d.wl_game.gamestate.victoryflag = true;
}

/*
===================
=
= Thrust
=
===================
*/

pub fn FixedByFracOrig(a: i32, b: i32) -> i32 {
    //println!("FixedByFracOrig");

    let mut sign: i32 = 0;

    let mut a = a;
    let mut b = b;

    if b == 65536 {
        b = 65535;
    } else if b == -65536 {
        b = 65535;
        sign = 1;
    } else if b < 0 {
        b = -b;
        sign = 1;
    }

    if a < 0 {
        a = -a;
        sign = !sign;
    }

    let mut res = (a as i64 * b as i64) >> 16 as i32;

    if sign != 0 {
        res = -res;
    }
    return res as i32;
}

pub fn Thrust(w3d: &mut modules, ob: &mut object, angle: i32, speed: i32) {
    //println!("Thrust");

    let xmove: i32;
    let ymove: i32;

    //
    // ZERO FUNNY COUNTER IF MOVED!
    //
    let mut speed = speed;

    w3d.wl_agent.thrustspeed += speed;
    //
    // moving bounds speed
    //
    if speed >= MINDIST * 2 {
        speed = MINDIST * 2 - 1;
    }

    //DEMOCHOOSE_ORIG_SDL
    if w3d.wl_play.demorecord || w3d.wl_play.demoplayback {
        xmove = FixedByFracOrig(speed, w3d.wl_draw.costable[angle as usize]);
    } else {
        xmove = FixedMul(speed, w3d.wl_draw.costable[angle as usize]);
    }
    //DEMOCHOOSE_ORIG_SDL
    if w3d.wl_play.demorecord || w3d.wl_play.demoplayback {
        ymove = -FixedByFracOrig(speed, w3d.wl_draw.sintable[angle as usize]);
    } else {
        ymove = -FixedMul(speed, w3d.wl_draw.sintable[angle as usize]);
    }

    //ClipMove(player,xmove,ymove);
    ClipMove(w3d, ob, xmove, ymove);

    //let mut player = ob.objlist[0];

    ob.objlist[0].tilex = ob.objlist[0].x >> TILESHIFT; // scale to tile values
    ob.objlist[0].tiley = ob.objlist[0].y >> TILESHIFT;

    ob.objlist[0].areanumber =
        ((MAPSPOT(w3d, ob.objlist[0].tilex, ob.objlist[0].tiley, 0) - AREATILE) as u8) as i32;

    if MAPSPOT(w3d, ob.objlist[0].tilex, ob.objlist[0].tiley, 1) == EXITTILE {
        VictoryTile(w3d, ob);
    }
}

/*
=============================================================================

                                ACTIONS

=============================================================================
*/

/*
===============
=
= Cmd_Fire
=
===============
*/

pub fn Cmd_Fire(w3d: &mut modules, ob: &mut object) {
    //println!("Cmd_Fire");

    w3d.wl_play.buttonheld[buttontype::bt_attack as usize] = true;

    w3d.wl_game.gamestate.weaponframe = 0;

    ob.objlist[0].state = s_attack;

    w3d.wl_game.gamestate.attackframe = 0;
    w3d.wl_game.gamestate.attackcount = attackinfo[w3d.wl_game.gamestate.weapon as usize]
        [w3d.wl_game.gamestate.attackframe as usize]
        .tics;
    w3d.wl_game.gamestate.weaponframe = attackinfo[w3d.wl_game.gamestate.weapon as usize]
        [w3d.wl_game.gamestate.attackframe as usize]
        .frame;
}

//===========================================================================

/*
===============
=
= Cmd_Use
=
===============
*/

pub fn Cmd_Use(w3d: &mut modules, ob: &mut object) {
    //println!("Cmd_Use");

    let checkx: i32;
    let checky: i32;
    let doornum: i32;
    let dir: i32;
    let elevatorok: bool;

    //player = ob.objlist[0];
    //
    // find which cardinal direction the player is facing
    //
    if ob.objlist[0].angle < ANGLES / 8 || ob.objlist[0].angle > 7 * ANGLES / 8 {
        checkx = ob.objlist[0].tilex + 1;
        checky = ob.objlist[0].tiley;
        dir = controldir_t::di_east as i32;
        elevatorok = true;
    } else if ob.objlist[0].angle < 3 * ANGLES / 8 {
        checkx = ob.objlist[0].tilex;
        checky = ob.objlist[0].tiley - 1;
        dir = controldir_t::di_north as i32;
        elevatorok = false;
    } else if ob.objlist[0].angle < 5 * ANGLES / 8 {
        checkx = ob.objlist[0].tilex - 1;
        checky = ob.objlist[0].tiley;
        dir = controldir_t::di_west as i32;
        elevatorok = true;
    } else {
        checkx = ob.objlist[0].tilex;
        checky = ob.objlist[0].tiley + 1;
        dir = controldir_t::di_south as i32;
        elevatorok = false;
    }

    doornum = w3d.wl_play.tilemap[checkx as usize][checky as usize];
    if MAPSPOT(w3d, checkx, checky, 1) == PUSHABLETILE {
        //
        // pushable wall
        //

        PushWall(w3d, ob, checkx, checky, dir);
        return;
    }
    if !w3d.wl_play.buttonheld[buttontype::bt_use as usize] && doornum == ELEVATORTILE && elevatorok
    {
        //
        // use elevator
        //
        w3d.wl_play.buttonheld[buttontype::bt_use as usize] = true;

        w3d.wl_play.tilemap[checkx as usize][checky as usize] += 1; // flip switch
        if MAPSPOT(w3d, ob.objlist[0].tilex, ob.objlist[0].tiley, 0) == ALTELEVATORTILE {
            w3d.wl_play.playstate = exit_t::ex_secretlevel;
        } else {
            w3d.wl_play.playstate = exit_t::ex_completed;
        }
        SD_PlaySound(w3d, soundnames::LEVELDONESND);
        SD_WaitSoundDone(w3d);
    } else if !w3d.wl_play.buttonheld[buttontype::bt_use as usize]
        && (doornum & BIT_DOOR as i32) != 0
    {
        w3d.wl_play.buttonheld[buttontype::bt_use as usize] = true;
        OperateDoor(w3d, ob, doornum & !BIT_DOOR);
    } else {
        SD_PlaySound(w3d, soundnames::DONOTHINGSND);
    }
}

/*
=============================================================================

                                PLAYER CONTROL

=============================================================================
*/

/*
===============
=
= SpawnPlayer
=
===============
*/

pub fn SpawnPlayer(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32, dir: i32) {
    //println!("SpawnPlayer");

    //let mut player = ob.objlist[0];

    ob.objlist[0].obclass = classtype::playerobj;
    ob.objlist[0].active = activetype::ac_yes;
    ob.objlist[0].tilex = tilex;
    ob.objlist[0].tiley = tiley;
    //BUG
    ob.objlist[0].dir = dirtype::east; //east=0

    ob.objlist[0].areanumber = ((MAPSPOT(w3d, tilex, tiley, 0) - AREATILE) as u8) as i32;

    ob.objlist[0].x = ((tilex as i32) << TILESHIFT) + TILEGLOBAL as i32 / 2;
    ob.objlist[0].y = ((tiley as i32) << TILESHIFT) + TILEGLOBAL as i32 / 2;

    ob.objlist[0].state = s_player;
    ob.objlist[0].angle = (1 - dir) as i32 * 90;

    if ob.objlist[0].angle < 0 {
        ob.objlist[0].angle += ANGLES as i32;
    }
    ob.objlist[0].flags = objflag_t::FL_NEVERMARK as i32;

    Thrust(w3d, ob, 0, 0); // set some variables

    InitAreas(w3d, ob);
}

/*
===============
=
= T_KnifeAttack
=
= Update player hands, and try to do damage when the proper frame is reached
=
===============
*/

pub fn KnifeAttack(w3d: &mut modules, ob: &mut object) {
    //println!("KnifeAttack");

    let mut check: objtype;
    let mut closest: objtype;
    let mut closest_i: usize = 0;
    let damage: i32;
    let mut dist: i32;

    SD_PlaySound(w3d, soundnames::ATKKNIFESND);
    // actually fire
    dist = 0x7fffffff;
    closest = objtype::new();

    //for (check=ob->next; check; check=check->next)
    for i in 1..ob.objlist.len() {
        check = ob.objlist[i];

        if (check.flags & objflag_t::FL_SHOOTABLE as i32) != 0
            && (check.flags & objflag_t::FL_VISABLE as i32) != 0
            && abs(check.viewx - w3d.wl_main.centerx) < w3d.wl_main.shootdelta
        {
            if check.transx < dist {
                dist = check.transx;
                closest = check;
                closest_i = i;
            }
        }
    }

    if closest == objtype::new() || dist > 0x18000 {
        // missed
        return;
    }

    // hit something
    damage = US_RndT(w3d) >> 4;

    DamageActor(w3d, ob, closest_i, damage);
}

pub fn GunAttack(w3d: &mut modules, ob: &mut object) {
    //println!("GunAttack");

    //objtype *check,*closest,*oldclosest;

    let mut check: objtype;
    let mut closest: objtype;
    let mut oldclosest: objtype;
    let mut closest_i: usize = 0;
    let damage: i32;
    let dx: i32;
    let dy: i32;
    let dist: i32;
    let mut viewdist: i32;

    match w3d.wl_game.gamestate.weapon {
        weapontype::wp_pistol => {
            SD_PlaySound(w3d, soundnames::ATKPISTOLSND);
        }
        weapontype::wp_machinegun => {
            SD_PlaySound(w3d, soundnames::ATKMACHINEGUNSND);
        }
        weapontype::wp_chaingun => {
            SD_PlaySound(w3d, soundnames::ATKGATLINGSND);
        }
        _ => (),
    }

    w3d.wl_play.madenoise = true;

    //
    // find potential targets
    //
    viewdist = 0x7fffffff;
    closest = objtype::new();

    loop {
        oldclosest = closest;

        //for (check=ob->next ; check ; check=check->next)
        for i in 1..ob.objlist.len() {
            check = ob.objlist[i];

            if (check.flags & objflag_t::FL_SHOOTABLE as i32) != 0
                && (check.flags & objflag_t::FL_VISABLE as i32) != 0
                && (abs(check.viewx - w3d.wl_main.centerx) < w3d.wl_main.shootdelta)
            {
                if check.transx < viewdist {
                    viewdist = check.transx;
                    closest = check;
                    closest_i = i;
                }
            }
        }

        if closest == oldclosest {
            return; // no more targets, all missed
        }

        //
        // trace a line from player to enemey
        //
        if CheckLine(w3d, ob, closest) {
            break;
        }
    }

    //
    // hit something
    //
    dx = ABS(closest.tilex - ob.objlist[0].tilex);
    dy = ABS(closest.tiley - ob.objlist[0].tiley);

    if dx > dy {
        dist = dx;
    } else {
        dist = dy;
    }

    if dist < 2 {
        damage = US_RndT(w3d) / 4;
    } else if dist < 4 {
        damage = US_RndT(w3d) / 6;
    } else {
        if (US_RndT(w3d) / 12) < dist {
            // missed
            return;
        }
        damage = US_RndT(w3d) / 6;
    }

    DamageActor(w3d, ob, closest_i, damage);
}

/*
===============
=
= VictorySpin
=
===============
*/

pub fn VictorySpin(w3d: &mut modules, ob: &mut object) {
    //println!("VictorySpin");

    let desty: i32;

    if ob.objlist[0].angle > 270 {
        ob.objlist[0].angle -= w3d.wl_play.tics * 3;

        if ob.objlist[0].angle < 270 {
            ob.objlist[0].angle = 270;
        }
    } else if ob.objlist[0].angle < 270 {
        ob.objlist[0].angle += w3d.wl_play.tics * 3;

        if ob.objlist[0].angle > 270 {
            ob.objlist[0].angle = 270;
        }
    }

    desty = ((ob.objlist[0].tiley - 5) << TILESHIFT) - 0x3000;

    if ob.objlist[0].y > desty {
        ob.objlist[0].y -= w3d.wl_play.tics * 4096;

        if ob.objlist[0].y < desty {
            ob.objlist[0].y = desty;
        }
    }
}

/*
===============
=
= T_Attack
=
===============
*/

pub fn T_Attack(w3d: &mut modules, ob: &mut object) {
    //println!("T_Attack");

    let mut cur: atkinf;

    UpdateFace(w3d);

    if w3d.wl_game.gamestate.victoryflag
    // watching the BJ actor
    {
        VictorySpin(w3d, ob);
        return;
    }

    if w3d.wl_play.buttonstate[buttontype::bt_use as usize]
        && !w3d.wl_play.buttonheld[buttontype::bt_use as usize]
    {
        w3d.wl_play.buttonstate[buttontype::bt_use as usize] = false;
    }

    if w3d.wl_play.buttonstate[buttontype::bt_attack as usize]
        && !w3d.wl_play.buttonheld[buttontype::bt_attack as usize]
    {
        w3d.wl_play.buttonstate[buttontype::bt_attack as usize] = false;
    }

    ControlMovement(w3d, ob);
    if w3d.wl_game.gamestate.victoryflag {
        // watching the BJ actor
        return;
    }

    w3d.wl_agent.plux = (ob.objlist[0].x >> UNSIGNEDSHIFT) as i16; // scale to fit in unsigned
    w3d.wl_agent.pluy = (ob.objlist[0].y >> UNSIGNEDSHIFT) as i16;
    ob.objlist[0].tilex = ob.objlist[0].x >> TILESHIFT; // scale to tile values
    ob.objlist[0].tiley = ob.objlist[0].y >> TILESHIFT;

    //
    // change frame and fire
    //
    w3d.wl_game.gamestate.attackcount -= w3d.wl_play.tics;
    while w3d.wl_game.gamestate.attackcount <= 0 {
        cur = attackinfo[w3d.wl_game.gamestate.weapon as usize]
            [w3d.wl_game.gamestate.attackframe as usize];

        match cur.attack {
            -1 => {
                ob.objlist[ob.objlist_i].state = s_player;

                if w3d.wl_game.gamestate.ammo == 0 {
                    w3d.wl_game.gamestate.weapon = weapontype::wp_knife;
                    DrawWeapon(w3d);
                } else {
                    if w3d.wl_game.gamestate.weapon != w3d.wl_game.gamestate.chosenweapon {
                        w3d.wl_game.gamestate.weapon = w3d.wl_game.gamestate.chosenweapon;
                        DrawWeapon(w3d);
                    }
                }
                w3d.wl_game.gamestate.attackframe = 0;
                w3d.wl_game.gamestate.weaponframe = 0;
                return;
            }
            4 => {
                if w3d.wl_game.gamestate.ammo == 0 {
                    //
                }
                if w3d.wl_play.buttonstate[buttontype::bt_attack as usize] {
                    w3d.wl_game.gamestate.attackframe -= 2;
                }
            }
            1 => {
                if w3d.wl_game.gamestate.ammo == 0 {
                    // can only happen with chain gun
                    w3d.wl_game.gamestate.attackframe += 1;
                }
                GunAttack(w3d, ob);
                if !w3d.wl_play.ammocheat {
                    w3d.wl_game.gamestate.ammo -= 1;
                }
                DrawAmmo(w3d);
            }
            2 => {
                KnifeAttack(w3d, ob);
            }
            3 => {
                if w3d.wl_game.gamestate.ammo != 0
                    && w3d.wl_play.buttonstate[buttontype::bt_attack as usize]
                {
                    w3d.wl_game.gamestate.attackframe -= 2;
                }
            }
            _ => (),
        }

        w3d.wl_game.gamestate.attackcount += cur.tics;
        w3d.wl_game.gamestate.attackframe += 1;
        w3d.wl_game.gamestate.weaponframe = attackinfo[w3d.wl_game.gamestate.weapon as usize]
            [w3d.wl_game.gamestate.attackframe as usize]
            .frame;
    }
}

//===========================================================================

/*
===============
=
= T_Player
=
===============
*/

pub fn T_Player(w3d: &mut modules, ob: &mut object) {
    //println!("T_Player");

    //let player = ob.objlist[0];

    if w3d.wl_game.gamestate.victoryflag
    // watching the BJ actor
    {
        VictorySpin(w3d, ob);
        return;
    }

    UpdateFace(w3d);
    CheckWeaponChange(w3d);

    if w3d.wl_play.buttonstate[buttontype::bt_use as usize] {
        Cmd_Use(w3d, ob);
    }

    if w3d.wl_play.buttonstate[buttontype::bt_attack as usize]
        && !w3d.wl_play.buttonheld[buttontype::bt_attack as usize]
    {
        Cmd_Fire(w3d, ob);
    }

    ControlMovement(w3d, ob);
    if w3d.wl_game.gamestate.victoryflag {
        // watching the BJ actor
        return;
    }

    // scale to fit in unsigned
    w3d.wl_agent.plux = (ob.objlist[0].x >> UNSIGNEDSHIFT) as i16;
    w3d.wl_agent.pluy = (ob.objlist[0].y >> UNSIGNEDSHIFT) as i16;
    // scale to tile values
    ob.objlist[0].tilex = ob.objlist[0].x >> TILESHIFT;
    ob.objlist[0].tiley = ob.objlist[0].y >> TILESHIFT;
}
