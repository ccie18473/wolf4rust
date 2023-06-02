#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_game
//
//===========================================================================

pub struct wl_game {
    pub ingame: bool,
    pub fizzlein: bool,
    pub gamestate: gametype,
    pub bordercol: i32, // color of the Change View/Ingame border
    //
    // ELEVATOR BACK MAPS - REMEMBER (-1)!!
    //
    pub ElevatorBackTo: [i32; 6],
    pub leftchannel: i32,
    pub rightchannel: i32,
    pub demoname: String,
}

impl wl_game {
    pub fn new() -> Self {
        let gs: gametype = gametype {
            difficulty: 0,
            mapon: 0,
            oldscore: 0,
            score: 0,
            nextextra: 0,
            lives: 0,
            health: 0,
            ammo: 0,
            keys: 0,
            bestweapon: weapontype::wp_knife,
            weapon: weapontype::wp_knife,
            chosenweapon: weapontype::wp_knife,

            faceframe: 0,
            attackframe: 0,
            attackcount: 0,
            weaponframe: 0,

            episode: 0,
            secretcount: 0,
            treasurecount: 0,
            killcount: 0,
            secrettotal: 0,
            treasuretotal: 0,
            killtotal: 0,
            TimeCount: 0,
            killx: 0,
            killy: 0,
            victoryflag: false,
        };

        Self {
            ingame: false,
            fizzlein: false,
            gamestate: gs,
            bordercol: VIEWCOLOR,
            ElevatorBackTo: [1, 1, 7, 3, 5, 3],
            leftchannel: 0,
            rightchannel: 0,

            demoname: String::from("DEMO?."),
        }
    }
}

//
// ELEVATOR BACK MAPS - REMEMBER (-1)!!
//
pub const ElevatorBackTo: [i32; 6] = [1, 1, 7, 3, 5, 3];

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const ATABLEMAX: i8 = 15;

pub const righttable: [[i32; ATABLEMAX as usize * 2]; ATABLEMAX as usize] = [
    [
        8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 6, 0, 0, 0, 0, 0, 1, 3, 5, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 6, 4, 0, 0, 0, 0, 0, 2, 4, 6, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 6, 6, 4, 1, 0, 0, 0, 1, 2, 4, 6, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 6, 5, 4, 2, 1, 0, 1, 2, 3, 5, 7, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 6, 5, 4, 3, 2, 2, 3, 3, 5, 6, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 6, 6, 5, 4, 4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 6, 6, 5, 5, 5, 6, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 6, 6, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
];

pub const lefttable: [[i32; ATABLEMAX as usize * 2]; ATABLEMAX as usize] = [
    [
        8, 8, 8, 8, 8, 8, 8, 8, 5, 3, 1, 0, 0, 0, 0, 0, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 6, 4, 2, 0, 0, 0, 0, 0, 4, 6, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 6, 4, 2, 1, 0, 0, 0, 1, 4, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 7, 5, 3, 2, 1, 0, 1, 2, 4, 5, 6, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 6, 5, 3, 3, 2, 2, 3, 4, 5, 6, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 7, 6, 5, 4, 4, 4, 4, 5, 6, 6, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 7, 6, 6, 5, 5, 5, 6, 6, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 6, 6, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
];

pub const DEATHROTATE: i32 = 2;

pub fn ClearMemory(w3d: &mut modules, _ob: &mut object) {
    //println!("ClearMemory");

    SD_StopDigitized(w3d);
}

pub fn ClearMemory_NewGame(w3d: &mut modules) {
    //println!("ClearMemory_NewGame");

    SD_StopDigitized(w3d);

    //BUG
    // clear all needed to restart game
    w3d.id_ca.clear();
    //w3d.id_in.clear();
    //w3d.id_pm.clear();
    //w3d.id_sd.clear();
    //w3d.id_us.clear();
    //w3d.id_vh.clear();
    //w3d.id_vl.clear();

    w3d.wl_act1.clear();
    //w3d.wl_act2.clear();
    w3d.wl_agent.clear();
    //w3d.wl_debug.clear();
    //w3d.wl_draw.clear();
    w3d.wl_inter.clear();
    //w3d.wl_main.clear();
    //w3d.wl_menu.clear();
    w3d.wl_play.clear();
    //w3d.wl_scale.clear();
    w3d.wl_state.clear();
    //w3d.wl_text.clear();
    //w3d.wl_utils.clear();

    //ob.clear();
}

/*
==========================
=
= SetSoundLoc - Given the location of an object (in terms of global
=       coordinates, held in globalsoundx and globalsoundy), munges the values
=       for an approximate distance from the left and right ear, and puts
=       those values into leftchannel and rightchannel.
=
= JAB
=
==========================
*/

pub fn SetSoundLoc(w3d: &mut modules, gx: &mut i32, gy: &mut i32) {
    //println!("SetSoundLoc");

    let mut xt: i32;
    let mut yt: i32;
    let mut x: i32;
    let mut y: i32;

    //
    // translate point to view centered coordinates
    //
    *gx -= w3d.wl_draw.viewx;
    *gy -= w3d.wl_draw.viewy;

    //
    // calculate newx
    //
    xt = FixedMul(*gx, w3d.wl_draw.viewcos);
    yt = FixedMul(*gy, w3d.wl_draw.viewsin);
    x = (xt - yt) >> TILESHIFT;

    //
    // calculate newy
    //
    xt = FixedMul(*gx, w3d.wl_draw.viewsin);
    yt = FixedMul(*gy, w3d.wl_draw.viewcos);
    y = (yt + xt) >> TILESHIFT;

    if y >= ATABLEMAX as i32 {
        y = ATABLEMAX as i32 - 1;
    } else if y <= -ATABLEMAX as i32 {
        y = -ATABLEMAX as i32;
    }
    if x < 0 {
        x = -x;
    }
    if x >= ATABLEMAX as i32 {
        x = ATABLEMAX as i32 - 1;
    }
    w3d.wl_game.leftchannel = lefttable[x as usize][(y + ATABLEMAX as i32) as usize] as i32;
    w3d.wl_game.rightchannel = righttable[x as usize][(y + ATABLEMAX as i32) as usize] as i32;
}

/*
==========================
=
= SetSoundLocGlobal - Sets up globalsoundx & globalsoundy and then calls
=       UpdateSoundLoc() to transform that into relative channel volumes. Those
=       values are then passed to the Sound Manager so that they'll be used for
=       the next sound played (if possible).
=
= JAB
=
==========================
*/

pub fn PlaySoundLocGlobal(w3d: &mut modules, s: soundnames, gx: &mut i32, gy: &mut i32) {
    //println!("PlaySoundLocGlobal");

    SetSoundLoc(w3d, gx, gy);
    SD_PositionSound(
        w3d,
        w3d.wl_game.leftchannel as i32,
        w3d.wl_game.rightchannel as i32,
    );

    let channel = SD_PlaySound(w3d, s);

    if channel != 0 {
        w3d.id_sd.channelSoundPos[channel as usize - 1].globalsoundx = *gx;
        w3d.id_sd.channelSoundPos[channel as usize - 1].globalsoundy = *gy;
        w3d.id_sd.channelSoundPos[channel as usize - 1].valid = 1;
    }
}

pub fn UpdateSoundLoc(w3d: &mut modules) {
    //println!("UpdateSoundLoc");

    for i in 0..MIX_CHANNELS {
        if w3d.id_sd.channelSoundPos[i as usize].valid != 0 {
            let mut soundx = w3d.id_sd.channelSoundPos[i as usize].globalsoundx;
            let mut soundy = w3d.id_sd.channelSoundPos[i as usize].globalsoundy;

            SetSoundLoc(w3d, &mut soundx, &mut soundy);

            SD_SetPosition(
                w3d,
                i as i32,
                w3d.wl_game.leftchannel as i32,
                w3d.wl_game.rightchannel as i32,
            );
        }
    }
}

/*
**      JAB End
*/

/*
==========================
=
= ScanInfoPlane
=
= Spawn all actors and mark down special places
=
==========================
*/

pub fn ScanInfoPlane(w3d: &mut modules, ob: &mut object) {
    //println!("ScanInfoPlane");

    let mut tile: i32;
    let start: Vec<u16>;
    let mut start_index: usize = 0;

    start = w3d.id_ca.mapsegs[1].clone();

    for y in 0..w3d.wl_play.mapheight as i32 {
        for x in 0..w3d.wl_play.mapwidth as i32 {
            tile = start[start_index] as i32;

            start_index += 1;

            if tile == 0 {
                continue;
            }

            if tile >= 19 && tile <= 22 {
                SpawnPlayer(w3d, ob, x, y, NORTH as i32 + tile - 19);
                continue;
            }

            if tile >= 23 && tile <= 72 {
                SpawnStatic(w3d, ob, x, y, tile - 23);
                continue;
            }
            //
            // P wall
            //

            if tile == 98 {
                if !w3d.wl_main.loadedgame {
                    w3d.wl_game.gamestate.secrettotal += 1;
                }
                continue;
            }
            //
            // guard
            //
            if tile >= 180 && tile <= 183 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 144 && tile <= 147 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 108 && tile <= 111 {
                SpawnStand(w3d, ob, enemy_t::en_guard, x, y, tile - 108);
                continue;
            }

            if tile >= 184 && tile <= 187 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 148 && tile <= 151 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 112 && tile <= 115 {
                SpawnPatrol(w3d, ob, enemy_t::en_guard, x, y, tile - 112);
                continue;
            }
            if tile == 124 {
                SpawnDeadGuard(w3d, ob, x, y);
                continue;
            }
            //
            // officer
            //
            if tile >= 188 && tile <= 191 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 152 && tile <= 155 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 116 && tile <= 119 {
                SpawnStand(w3d, ob, enemy_t::en_officer, x, y, tile - 116);
                continue;
            }
            if tile >= 192 && tile <= 195 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 156 && tile <= 159 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 120 && tile <= 123 {
                SpawnPatrol(w3d, ob, enemy_t::en_officer, x, y, tile - 120);
                continue;
            }
            //
            // ss
            //
            if tile >= 198 && tile <= 201 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 162 && tile <= 165 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 126 && tile <= 129 {
                SpawnStand(w3d, ob, enemy_t::en_ss, x, y, tile - 126);
                continue;
            }
            if tile >= 202 && tile <= 205 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 166 && tile <= 169 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 130 && tile <= 133 {
                SpawnPatrol(w3d, ob, enemy_t::en_ss, x, y, tile - 130);
                continue;
            }
            //
            // dogs
            //
            if tile >= 206 && tile <= 209 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 170 && tile <= 173 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 134 && tile <= 137 {
                SpawnStand(w3d, ob, enemy_t::en_dog, x, y, tile - 134);
                continue;
            }
            if tile >= 210 && tile <= 213 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 174 && tile <= 177 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 36;
            }
            if tile >= 138 && tile <= 141 {
                SpawnPatrol(w3d, ob, enemy_t::en_dog, x, y, tile - 138);
                continue;
            }
            //
            // boss
            //
            if tile == 214 {
                SpawnBoss(w3d, ob, x, y);
                continue;
            }
            if tile == 197 {
                SpawnGretel(w3d, ob, x, y);
                continue;
            }
            if tile == 215 {
                SpawnGift(w3d, ob, x, y);
                continue;
            }
            if tile == 179 {
                SpawnFat(w3d, ob, x, y);
                continue;
            }
            if tile == 196 {
                SpawnSchabbs(w3d, ob, x, y);
                continue;
            }
            if tile == 160 {
                SpawnFakeHitler(w3d, ob, x, y);
                continue;
            }
            if tile == 178 {
                SpawnHitler(w3d, ob, x, y);
                continue;
            }
            //
            // mutants
            //
            if tile >= 252 && tile <= 255 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 18;
            }
            if tile >= 234 && tile <= 237 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 18;
            }
            if tile >= 216 && tile <= 219 {
                SpawnStand(w3d, ob, enemy_t::en_mutant, x, y, tile - 216);
                continue;
            }
            if tile >= 256 && tile <= 259 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_hard as i32 {
                    continue;
                }
                tile -= 18;
            }
            if tile >= 238 && tile <= 241 {
                if w3d.wl_game.gamestate.difficulty < gd::gd_medium as i32 {
                    continue;
                }
                tile -= 18;
            }
            if tile >= 220 && tile <= 223 {
                SpawnPatrol(w3d, ob, enemy_t::en_mutant, x, y, tile - 216);
                continue;
            }
            //
            // ghosts
            //
            if tile == 224 {
                SpawnGhosts(w3d, ob, enemy_t::en_blinky, x, y);
                continue;
            }
            if tile == 225 {
                SpawnGhosts(w3d, ob, enemy_t::en_clyde, x, y);
                continue;
            }
            if tile == 226 {
                SpawnGhosts(w3d, ob, enemy_t::en_pinky, x, y);
                continue;
            }
            if tile == 226 {
                SpawnGhosts(w3d, ob, enemy_t::en_inky, x, y);
                continue;
            }
        }
    }
}

//==========================================================================

/*
==================
=
= SetupGameLevel
=
==================
*/

pub fn SetupGameLevel(w3d: &mut modules, ob: &mut object) {
    //println!("SetupGameLevel");

    let mapnum: i32;
    let mut map: Vec<u16>;
    let mut tile: i32;

    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.TimeCount = 0;
        w3d.wl_game.gamestate.secrettotal = 0;
        w3d.wl_game.gamestate.killtotal = 0;
        w3d.wl_game.gamestate.treasuretotal = 0;
        w3d.wl_game.gamestate.secretcount = 0;
        w3d.wl_game.gamestate.killcount = 0;
        w3d.wl_game.gamestate.treasurecount = 0;

        w3d.wl_act1.pwallstate = 0;
        w3d.wl_act1.pwallpos = 0;
        w3d.wl_agent.facetimes = 0;

        w3d.wl_agent.LastAttacker = objtype::new();
        ob.killerobj = objtype::new();
    }

    if w3d.wl_play.demoplayback || w3d.wl_play.demorecord {
        US_InitRndT(w3d, false);
    } else {
        US_InitRndT(w3d, true);
    }

    //
    // load the level
    //
    mapnum = (w3d.wl_game.gamestate.mapon + 10 * w3d.wl_game.gamestate.episode) as i32;

    CA_CacheMap(w3d, mapnum);

    w3d.wl_play.mapwidth = w3d.id_ca.mapheaderseg[mapnum as usize].width as i32;
    w3d.wl_play.mapheight = w3d.id_ca.mapheaderseg[mapnum as usize].height as i32;

    //
    // copy the wall data to a data segment array
    //
    //memset (tilemap,0,sizeof(tilemap));
    //memset (actorat,0,sizeof(actorat));

    map = w3d.id_ca.mapsegs[0].clone();

    let mut map_i: usize = 0;
    for y in 0..w3d.wl_play.mapheight {
        for x in 0..w3d.wl_play.mapwidth {
            tile = map[map_i] as i32;
            map_i += 1;

            if tile < AREATILE {
                // solid wall
                w3d.wl_play.tilemap[x as usize][y as usize] = tile;
                ob.actorat[x as usize][y as usize] = tile as *mut objtype;
            } else {
                // area floor
                w3d.wl_play.tilemap[x as usize][y as usize] = 0;
                ob.actorat[x as usize][y as usize] = 0 as *mut objtype;
            }
        }
    }

    //
    // spawn doors
    //
    InitActorList(ob); // start spawning things with a clean slate
    InitDoorList(w3d);
    InitStaticList(w3d);

    map = w3d.id_ca.mapsegs[0].clone();

    let mut map_i: usize = 0;
    for y in 0..w3d.wl_play.mapheight as i32 {
        for x in 0..w3d.wl_play.mapwidth as i32 {
            tile = map[map_i] as i32;
            map_i += 1;
            if tile >= 90 && tile <= 101 {
                // door
                match tile {
                    90 | 92 | 94 | 96 | 98 | 100 => {
                        SpawnDoor(w3d, ob, x, y, true, (tile - 90) / 2);
                    }
                    91 | 93 | 95 | 97 | 99 | 101 => {
                        SpawnDoor(w3d, ob, x, y, false, (tile - 91) / 2);
                    }
                    _ => (),
                }
            }
        }
    }

    //
    // spawn actors
    //
    ScanInfoPlane(w3d, ob);

    //
    // take out the ambush markers
    //
    map = w3d.id_ca.mapsegs[0].clone();

    let mut map_i: usize = 0;
    for y in 0..w3d.wl_play.mapheight {
        for x in 0..w3d.wl_play.mapwidth {
            tile = map[map_i] as i32;
            map_i += 1;
            if tile == AMBUSHTILE {
                w3d.wl_play.tilemap[x as usize][y as usize] = 0;

                if ob.actorat[x as usize][y as usize] == AMBUSHTILE as *mut objtype {
                    ob.actorat[x as usize][y as usize] = ptr::null_mut();
                }

                if map[map_i] >= AREATILE as u16 {
                    tile = map[map_i] as i32;
                }
                if map[map_i - 1 - w3d.wl_play.mapwidth as usize] >= AREATILE as u16 {
                    tile = map[map_i - 1 - w3d.wl_play.mapwidth as usize] as i32;
                }
                if map[map_i - 1 + w3d.wl_play.mapwidth as usize] >= AREATILE as u16 {
                    tile = map[map_i - 1 + w3d.wl_play.mapwidth as usize] as i32;
                }
                if map[map_i - 2] >= AREATILE as u16 {
                    tile = map[map_i - 2] as i32;
                }

                map[map_i - 1] = tile as u16;
            }
        }
    }

    //
    // have the caching manager load and purge stuff to make sure all marks
    // are in memory
    //

    CA_LoadAllSounds(w3d);
}

//==========================================================================

/*
===================
=
= DrawPlayBorderSides
=
= To fix window overwrites
=
===================
*/

pub fn DrawPlayBorderSides(w3d: &mut modules) {
    //println!("DrawPlayBorderSides");

    if w3d.wl_play.viewsize == 21 {
        return;
    }

    let sw = w3d.id_vl.screenWidth;
    let sh = w3d.id_vl.screenHeight;
    let vw = w3d.wl_main.viewwidth;
    let vh = w3d.wl_main.viewheight;
    let px = w3d.id_vl.scaleFactor; // size of one "pixel"

    let h = sh - px * STATUSLINES as i32;
    let xl = sw / 2 - vw / 2;
    let yl = (h - vh) / 2;

    if xl != 0 {
        VWB_BarScaledCoord(w3d, 0, 0, xl - px, h, w3d.wl_game.bordercol); // left side
        VWB_BarScaledCoord(w3d, xl + vw + px, 0, xl - px * 2, h, w3d.wl_game.bordercol);
        // right side
    }

    if yl != 0 {
        VWB_BarScaledCoord(w3d, 0, 0, sw, yl - px, w3d.wl_game.bordercol); // upper side
        VWB_BarScaledCoord(w3d, 0, yl + vh + px, sw, yl - px, w3d.wl_game.bordercol);
        // lower side
    }

    if xl != 0 {
        // Paint game view border lines
        VWB_BarScaledCoord(w3d, xl - px, yl - px, vw + px, px, 0); // upper border
        VWB_BarScaledCoord(w3d, xl, yl + vh, vw + px, px, w3d.wl_game.bordercol - 2); // lower border
        VWB_BarScaledCoord(w3d, xl - px, yl - px, px, vh + px, 0); // left border
        VWB_BarScaledCoord(
            w3d,
            xl + vw,
            yl - px,
            px,
            vh + px * 2,
            w3d.wl_game.bordercol - 2,
        ); // right border
        VWB_BarScaledCoord(w3d, xl - px, yl + vh, px, px, w3d.wl_game.bordercol - 3);
    // lower left highlight
    } else {
        // Just paint a lower border line
        VWB_BarScaledCoord(w3d, 0, yl + vh, vw, px, w3d.wl_game.bordercol - 2);
        // lower border
    }
}

/*
===================
=
= DrawStatusBorder
=
===================
*/

pub fn DrawStatusBorder(w3d: &mut modules, color: i32) {
    //println!("DrawStatusBorder");

    let sw = w3d.id_vl.screenWidth;
    let sh = w3d.id_vl.screenHeight;
    let px = w3d.id_vl.scaleFactor; // size of one "pixel"

    let statusborderw = (sw - px * 320) / 2;

    VWB_BarScaledCoord(w3d, 0, 0, sw, sh - px * (STATUSLINES - 3), color);
    VWB_BarScaledCoord(
        w3d,
        0,
        sh - px * (STATUSLINES - 3),
        statusborderw + px * 8,
        px * (STATUSLINES - 4),
        color,
    );
    VWB_BarScaledCoord(w3d, 0, sh - px * 2, sw, px * 2, color);
    VWB_BarScaledCoord(
        w3d,
        sw - statusborderw - px * 8,
        sh - px * (STATUSLINES - 3),
        statusborderw + px * 8,
        px * (STATUSLINES - 4),
        color,
    );

    VWB_BarScaledCoord(
        w3d,
        statusborderw + px * 9,
        sh - px * 3,
        px * 97,
        px * 1,
        color - 1,
    );
    VWB_BarScaledCoord(
        w3d,
        statusborderw + px * 106,
        sh - px * 3,
        px * 161,
        px * 1,
        color - 2,
    );
    VWB_BarScaledCoord(
        w3d,
        statusborderw + px * 267,
        sh - px * 3,
        px * 44,
        px * 1,
        color - 3,
    );
    VWB_BarScaledCoord(
        w3d,
        sw - statusborderw - px * 9,
        sh - px * (STATUSLINES - 4),
        px * 1,
        px * 20,
        color - 2,
    );
    VWB_BarScaledCoord(
        w3d,
        sw - statusborderw - px * 9,
        sh - px * (STATUSLINES / 2 - 4),
        px * 1,
        px * 14,
        color - 3,
    );
}

/*
===================
=
= DrawPlayBorder
=
===================
*/

pub fn DrawPlayBorder(w3d: &mut modules) {
    //println!("DrawPlayBorder");

    let sw = w3d.id_vl.screenWidth;
    let sh = w3d.id_vl.screenHeight;
    let vw = w3d.wl_main.viewwidth;
    let vh = w3d.wl_main.viewheight;
    let px = w3d.id_vl.scaleFactor; // size of one "pixel"

    if w3d.wl_game.bordercol != VIEWCOLOR {
        DrawStatusBorder(w3d, w3d.wl_game.bordercol);
    } else {
        let statusborderw = (sw - px * 320) / 2;

        VWB_BarScaledCoord(
            w3d,
            0,
            sh - px * STATUSLINES,
            statusborderw + px * 8,
            px * STATUSLINES,
            w3d.wl_game.bordercol,
        );
        VWB_BarScaledCoord(
            w3d,
            sw - statusborderw - px * 8,
            sh - px * STATUSLINES,
            statusborderw + px * 8,
            px * STATUSLINES,
            w3d.wl_game.bordercol,
        );
    }

    if vh == sh {
        return;
    }

    VWB_BarScaledCoord(w3d, 0, 0, sw, sh - px * STATUSLINES, w3d.wl_game.bordercol);

    let xl = sw / 2 - vw / 2;
    let yl = (sh - px * STATUSLINES - vh) / 2;

    VWB_BarScaledCoord(w3d, xl, yl, vw, vh, 0);

    if xl != 0 {
        // Paint game view border lines
        VWB_BarScaledCoord(w3d, xl - px, yl - px, vw + px, px, 0); // upper border
        VWB_BarScaledCoord(w3d, xl, yl + vh, vw + px, px, w3d.wl_game.bordercol - 2); // lower border
        VWB_BarScaledCoord(w3d, xl - px, yl - px, px, vh + px, 0); // left border
        VWB_BarScaledCoord(
            w3d,
            xl + vw,
            yl - px,
            px,
            vh + 2 * px,
            w3d.wl_game.bordercol - 2,
        ); // right border
        VWB_BarScaledCoord(w3d, xl - px, yl + vh, px, px, w3d.wl_game.bordercol - 3);
        // lower left highlight
    } else {
        // Just paint a lower border line
        VWB_BarScaledCoord(w3d, 0, yl + vh, vw, px, w3d.wl_game.bordercol - 2);
        // lower border
    }
}

/*
===================
=
= DrawPlayScreen
=
===================
*/

pub fn DrawPlayScreen(w3d: &mut modules) {
    //println!("DrawPlayScreen");

    VWB_DrawPicScaledCoord(
        w3d,
        (w3d.id_vl.screenWidth - w3d.id_vl.scaleFactor * 320) / 2,
        w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * (STATUSLINES as i32),
        graphicnums::STATUSBARPIC as i32,
    );

    DrawPlayBorder(w3d);

    DrawFace(w3d);
    DrawHealth(w3d);
    DrawLives(w3d);
    DrawLevel(w3d);
    DrawAmmo(w3d);
    DrawKeys(w3d);
    DrawWeapon(w3d);
    DrawScore(w3d);
}

pub fn ShowActStatus(w3d: &mut modules) {
    //println!("ShowActStatus");

    // Draw status bar without borders
    let source = w3d.id_ca.grsegs[graphicnums::STATUSBARPIC as usize].clone();
    let picnum = graphicnums::STATUSBARPIC as usize - STARTPICS as usize;
    let width = w3d.id_vh.pictable[picnum].width as i32;
    let height = w3d.id_vh.pictable[picnum].height as i32;
    let destx =
        (w3d.id_vl.screenWidth - w3d.id_vl.scaleFactor * 320) / 2 + 9 * w3d.id_vl.scaleFactor;
    let desty = w3d.id_vl.screenHeight - (height - 4) * w3d.id_vl.scaleFactor;
    VL_MemToScreenScaledCoord2(
        w3d,
        &source,
        width,
        height,
        9,
        4,
        destx,
        desty,
        width - 18,
        height - 7,
    );

    w3d.wl_game.ingame = false;
    DrawFace(w3d);
    DrawHealth(w3d);
    DrawLives(w3d);
    DrawLevel(w3d);
    DrawAmmo(w3d);
    DrawKeys(w3d);
    DrawWeapon(w3d);
    DrawScore(w3d);
    w3d.wl_game.ingame = true;
}

//==========================================================================

/*
==================
=
= FinishDemoRecord
=
==================
*/

pub fn FinishDemoRecord() {
    //println!("FinishDemoRecord");
}

//==========================================================================

/*
==================
=
= RecordDemo
=
= Fades the screen out, then starts a demo.  Exits with the screen faded
=
==================
*/

pub fn RecordDemo() {
    //println!("RecordDemo");
}

//==========================================================================

/*
==================
=
= PlayDemo
=
= Fades the screen out, then starts a demo.  Exits with the screen unfaded
=
==================
*/

pub fn PlayDemo(w3d: &mut modules, ob: &mut object, demonumber: usize) {
    //println!("PlayDemo");

    let length: usize;
    let dems: [i32; 4] = [
        graphicnums::T_DEMO0 as i32,
        graphicnums::T_DEMO1 as i32,
        graphicnums::T_DEMO2 as i32,
        graphicnums::T_DEMO3 as i32,
    ];

    //demoptr = (int8_t *) grsegs[dems[demonumber]];

    let demoptr_u8 = w3d.id_ca.grsegs[dems[demonumber] as usize].clone();
    for i in 0..demoptr_u8.len() {
        w3d.wl_play.demoptr.push(demoptr_u8[i] as i8);
    }

    NewGame(w3d, 1, 0);

    w3d.wl_game.gamestate.mapon = w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] as i32;
    w3d.wl_play.demoptr_i += 1;
    w3d.wl_game.gamestate.difficulty = gd::gd_hard as i32;

    length = w3d.wl_play.demoptr.len();
    // TODO: Seems like the original demo format supports 16 MB demos
    //       But T_DEM00 and T_DEM01 of Wolf have a 0xd8 as third length size...
    w3d.wl_play.demoptr_i += 3;
    w3d.wl_play.lastdemoptr = w3d.wl_play.demoptr.clone();
    w3d.wl_play.lastdemoptr_i = w3d.wl_play.demoptr_i - 4 + length;

    VW_FadeOut(w3d);

    SETFONTCOLOR(w3d, 0, 15);
    DrawPlayScreen(w3d);

    w3d.wl_main.startgame = false;
    w3d.wl_play.demoplayback = true;

    SetupGameLevel(w3d, ob);
    StartMusic(w3d);

    PlayLoop(w3d, ob);

    w3d.wl_play.demoplayback = false;

    StopMusic(w3d);
    ClearMemory_NewGame(w3d);
}

//==========================================================================

/*
==================
=
= Died
=
==================
*/

pub fn Died(w3d: &mut modules, ob: &mut object) {
    //println!("Died");

    let mut fangle: f64;
    let dx: i32;
    let dy: i32;
    let mut iangle: i32;
    let mut curangle: i32;
    let clockwise: i32;
    let counter: i32;
    let mut change: i32;

    if w3d.id_vl.screenfaded {
        ThreeDRefresh(w3d, ob);
        VW_FadeIn(w3d);
    }

    w3d.wl_game.gamestate.weapon = weapontype::wp_noweapon; // take away weapon
    SD_PlaySound(w3d, soundnames::PLAYERDEATHSND);

    //
    // swing around to face attacker
    //
    if ob.killerobj != objtype::new() {
        dx = ob.killerobj.x - ob.objlist[0].x;
        dy = ob.objlist[0].y - ob.killerobj.y;

        fangle = libm::atan2(dy as f64, dx as f64); // returns -pi to pi
        if fangle < 0.0 {
            fangle = M_PI * 2.0 + fangle;
        }

        iangle = (fangle / (M_PI * 2.0) * ANGLES as f64) as i32;
    } else {
        iangle = ob.objlist[0].angle + ANGLES / 2;
        if iangle >= ANGLES {
            iangle -= ANGLES;
        }
    }

    if ob.objlist[0].angle > iangle {
        counter = ob.objlist[0].angle - iangle;
        clockwise = ANGLES - ob.objlist[0].angle + iangle;
    } else {
        clockwise = iangle - ob.objlist[0].angle;
        counter = ob.objlist[0].angle + ANGLES - iangle;
    }

    curangle = ob.objlist[0].angle;

    if clockwise < counter {
        //
        // rotate clockwise
        //
        if curangle > iangle {
            curangle -= ANGLES;
        }

        loop {
            change = w3d.wl_play.tics * DEATHROTATE;
            if curangle + change > iangle {
                change = iangle - curangle;
            }

            curangle += change;
            ob.objlist[0].angle += change;
            if ob.objlist[0].angle >= ANGLES {
                ob.objlist[0].angle -= ANGLES;
            }

            ThreeDRefresh(w3d, ob);
            CalcTics(w3d);

            if curangle == iangle {
                break;
            }
        }
    } else {
        //
        // rotate counterclockwise
        //
        if curangle < iangle {
            curangle += ANGLES;
        }

        loop {
            change = -w3d.wl_play.tics * DEATHROTATE;
            if curangle + change < iangle {
                change = iangle - curangle;
            }

            curangle += change;
            ob.objlist[0].angle += change;
            if ob.objlist[0].angle < 0 {
                ob.objlist[0].angle += ANGLES;
            }

            ThreeDRefresh(w3d, ob);
            CalcTics(w3d);

            if curangle == iangle {
                break;
            }
        }
    }

    //
    // fade to red
    //
    FinishPaletteShifts(w3d);

    if w3d.id_vl.usedoublebuffering {
        VW_UpdateScreen(w3d);
    }

    VL_BarScaledCoord(
        w3d,
        w3d.wl_main.viewscreenx,
        w3d.wl_main.viewscreeny,
        w3d.wl_main.viewwidth,
        w3d.wl_main.viewheight,
        4,
    );

    IN_ClearKeysDown(w3d);

    FizzleFade(
        w3d,
        w3d.wl_main.viewscreenx,
        w3d.wl_main.viewscreeny,
        w3d.wl_main.viewwidth,
        w3d.wl_main.viewheight,
        70,
        false,
    );

    IN_UserInput(w3d, 100);
    SD_WaitSoundDone(w3d);
    ClearMemory(w3d, ob);

    w3d.wl_game.gamestate.lives -= 1;

    if w3d.wl_game.gamestate.lives > -1 {
        w3d.wl_game.gamestate.health = 100;
        w3d.wl_game.gamestate.weapon = weapontype::wp_pistol;
        w3d.wl_game.gamestate.bestweapon = weapontype::wp_pistol;
        w3d.wl_game.gamestate.chosenweapon = weapontype::wp_pistol;
        w3d.wl_game.gamestate.ammo = STARTAMMO;
        w3d.wl_game.gamestate.keys = 0;
        w3d.wl_act1.pwallstate = 0;
        w3d.wl_act1.pwallpos = 0;
        w3d.wl_game.gamestate.attackframe = 0;
        w3d.wl_game.gamestate.attackcount = 0;
        w3d.wl_game.gamestate.weaponframe = 0;

        if w3d.wl_play.viewsize != 21 {
            DrawKeys(w3d);
            DrawWeapon(w3d);
            DrawAmmo(w3d);
            DrawHealth(w3d);
            DrawFace(w3d);
            DrawLives(w3d);
        }
    }
}

//==========================================================================

/*
===================
=
= GameLoop
=
===================
*/

pub fn GameLoop(w3d: &mut modules, ob: &mut object) {
    //println!("GameLoop");

    let mut died: bool;

    loop {
        //restartgame:

        ClearMemory_NewGame(w3d);
        SETFONTCOLOR(w3d, 0, 15);
        VW_FadeOut(w3d);
        DrawPlayScreen(w3d);
        died = false;

        'inner: loop {
            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.score = w3d.wl_game.gamestate.oldscore;
            }
            if !died || w3d.wl_play.viewsize != 21 {
                DrawScore(w3d);
            }

            w3d.wl_main.startgame = false;

            if !w3d.wl_main.loadedgame {
                SetupGameLevel(w3d, ob);
            }

            DrawLevel(w3d); // ADDEDFIX 5 -  Chris Chokan

            w3d.wl_game.ingame = true;
            if w3d.wl_main.loadedgame {
                ContinueMusic(w3d, w3d.wl_play.lastgamemusicoffset);
                w3d.wl_main.loadedgame = false;
            } else {
                StartMusic(w3d);
            }

            if !died {
                PreloadGraphics(w3d); // TODO: Let this do something useful!
            } else {
                died = false;
                w3d.wl_game.fizzlein = true;
            }

            //        DrawLevel ();                     // ADDEDFIX 5 - moved up  Chris Chokan

            PlayLoop(w3d, ob);

            StopMusic(w3d);
            w3d.wl_game.ingame = false;

            if w3d.wl_play.demorecord && w3d.wl_play.playstate != exit_t::ex_warped {
                FinishDemoRecord();
            }

            if w3d.wl_main.startgame || w3d.wl_main.loadedgame {
                //goto restartgame;
                //BUG
                break 'inner;
            }
            match w3d.wl_play.playstate {
                exit_t::ex_completed | exit_t::ex_secretlevel => {
                    if w3d.wl_play.viewsize == 21 {
                        DrawPlayScreen(w3d);
                    }
                    w3d.wl_game.gamestate.keys = 0;
                    DrawKeys(w3d);
                    VW_FadeOut(w3d);

                    ClearMemory_NewGame(w3d);

                    LevelCompleted(w3d); // do the intermission
                    if w3d.wl_play.viewsize == 21 {
                        DrawPlayScreen(w3d);
                    }

                    w3d.wl_game.gamestate.oldscore = w3d.wl_game.gamestate.score;

                    //
                    // COMING BACK FROM SECRET LEVEL
                    //
                    if w3d.wl_game.gamestate.mapon == 9 {
                        w3d.wl_game.gamestate.mapon =
                            ElevatorBackTo[w3d.wl_game.gamestate.episode as usize];
                    // back from secret
                    } else
                    //
                    // GOING TO SECRET LEVEL
                    //
                    if w3d.wl_play.playstate == exit_t::ex_secretlevel {
                        w3d.wl_game.gamestate.mapon = 9;
                    } else {
                        //
                        // GOING TO NEXT LEVEL
                        //
                        w3d.wl_game.gamestate.mapon += 1;
                    }
                }
                exit_t::ex_died => {
                    Died(w3d, ob);
                    //BUG
                    //died = true; // don't "get psyched!"

                    if w3d.wl_game.gamestate.lives > -1 {
                        break 'inner; // more lives left
                    }
                    VW_FadeOut(w3d);
                    if w3d.id_vl.screenHeight % 200 != 0 {
                        VL_ClearScreen(w3d, Color::BLACK);
                    }

                    ClearMemory_NewGame(w3d);

                    CheckHighScore(
                        w3d,
                        w3d.wl_game.gamestate.score,
                        w3d.wl_game.gamestate.mapon + 1,
                    );

                    unsafe { MainMenu[menuitems::viewscores as usize].string = STR_VS };

                    unsafe {
                        MainMenu[menuitems::viewscores as usize].routine =
                            CP_ViewScores as *mut routine_int
                    };
                    return;
                }
                exit_t::ex_victorious => {
                    if w3d.wl_play.viewsize == 21 {
                        DrawPlayScreen(w3d);
                    }

                    VW_FadeOut(w3d);

                    ClearMemory(w3d, ob);

                    Victory(w3d);

                    ClearMemory(w3d, ob);

                    CheckHighScore(
                        w3d,
                        w3d.wl_game.gamestate.score,
                        w3d.wl_game.gamestate.mapon + 1,
                    );

                    unsafe { MainMenu[menuitems::viewscores as usize].string = STR_VS };

                    unsafe {
                        MainMenu[menuitems::viewscores as usize].routine =
                            CP_ViewScores as *mut routine_int
                    };
                    return;
                }
                _ => {
                    if w3d.wl_play.viewsize == 21 {
                        DrawPlayScreen(w3d);
                    }
                    ClearMemory(w3d, ob);
                }
            }
        }
    }
}
