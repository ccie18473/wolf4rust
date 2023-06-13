#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use crate::*;

//===========================================================================
//
//  wl_def
//
//===========================================================================

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const MAXTICS: i32 = 10;
pub const DEMOTICS: i32 = 4;
pub const WALLSHIFT: i32 = 6;
pub const BIT_WALL: i32 = 1 << WALLSHIFT;
pub const BIT_DOOR: i32 = 1 << (WALLSHIFT + 1);
pub const BIT_ALLTILES: i32 = 1 << (WALLSHIFT + 2);
pub fn DOORWALL(w3d: &mut modules) -> i32 {
    w3d.id_pm.PMSpriteStart - 8
}
pub const MAXACTORS: i32 = 150; // max number of nazis, etc / map
pub const MAXSTATS: i32 = 400; // max number of lamps, bonus, etc
pub const MAXDOORS: i32 = 64; // max number of sliding doors
pub const MAXWALLTILES: usize = 64; // max number of wall tiles

pub type tiletype = i32;

pub const ICONARROWS: i32 = 90;
pub const PUSHABLETILE: i32 = 98;
pub const EXITTILE: i32 = 99; // at end of castle
pub const AREATILE: i32 = 107; // first of NUMAREAS floor tiles
pub const NUMAREAS: i32 = 37;
pub const ELEVATORTILE: i32 = 21;
pub const AMBUSHTILE: i32 = 106;
pub const ALTELEVATORTILE: i32 = 107;
pub const EXTRAPOINTS: i32 = 40000;
pub const RUNSPEED: i32 = 6000;
pub const HEIGHTRATIO: f32 = 0.50;

pub const LRpack: usize = 8; // # of levels to store in endgame
                             //} else {
                             //pub const LRpack:i32 = 20;
                             //}
pub const PLAYERSIZE: i32 = MINDIST; // player radius
pub const MINACTORDIST: i32 = 0x10000; // minimum dist from player center
pub const PI: f64 = 3.141592657;
pub const M_PI: f64 = PI;
pub const GLOBAL1: i32 = 1 << 16;
pub const TILEGLOBAL: i32 = GLOBAL1 as i32;
pub const TILESHIFT: i32 = 16;
pub const UNSIGNEDSHIFT: i32 = 8;
pub const ANGLES: i32 = 360; // must be divisable by 4
pub const ANGLEQUAD: i32 = ANGLES / 4;
pub const FINEANGLES: i32 = 3600;
pub const ANG90: i32 = FINEANGLES / 4;
pub const ANG180: i32 = ANG90 * 2;
pub const ANG270: i32 = ANG90 * 3;
pub const ANG360: i32 = ANG90 * 4;
pub const MINDIST: i32 = 0x5800;
pub const MAPSHIFT: i32 = 6;
pub const MAPSIZE: u16 = 1 << MAPSHIFT;
pub const MAPAREA: u16 = MAPSIZE * MAPSIZE;
pub const TEXTURESHIFT: i32 = 6;
pub const FIXED2TEXSHIFT: i32 = 4;
pub const TEXTURESIZE: i32 = 1 << TEXTURESHIFT;
pub const TEXTUREMASK: i32 = TEXTURESIZE * (TEXTURESIZE - 1);
pub const NORTH: i32 = 0;
pub const STATUSLINES: i32 = 40;
pub const STARTAMMO: i32 = 8;

pub fn PlaySoundLocTile(w3d: &mut modules, s: soundnames, tx: i32, ty: i32) {
    PlaySoundLocGlobal(
        w3d,
        s,
        &mut ((tx << TILESHIFT) + (TILEGLOBAL as i32 / 2)),
        &mut ((ty << TILESHIFT) + (TILEGLOBAL as i32 / 2)),
    );
}

pub fn PlaySoundLocActor(w3d: &mut modules, ob: &mut object, s: soundnames) {
    let mut obj = ob.objlist[ob.objlist_i];

    PlaySoundLocGlobal(w3d, s, &mut obj.x, &mut obj.y);
}

// object flag values

#[derive(PartialEq, Clone, Copy)]
pub enum objflag_t {
    FL_SHOOTABLE = 0x00000001,
    FL_BONUS = 0x00000002,
    FL_NEVERMARK = 0x00000004,
    FL_VISABLE = 0x00000008,
    FL_ATTACKMODE = 0x00000010,
    FL_FIRSTATTACK = 0x00000020,
    FL_AMBUSH = 0x00000040,
    FL_NONMARK = 0x00000080,
    FL_FULLBRIGHT = 0x00000100,
    // next free bit is   0x00001000
}

//
// sprite constants
//

#[derive(PartialEq, Clone, Copy)]
pub enum SPRITES {
    SPR_DEMO,
    SPR_DEATHCAM,
    //
    // static sprites
    //
    SPR_STAT_0,
    SPR_STAT_1,
    SPR_STAT_2,
    SPR_STAT_3,
    SPR_STAT_4,
    SPR_STAT_5,
    SPR_STAT_6,
    SPR_STAT_7,

    SPR_STAT_8,
    SPR_STAT_9,
    SPR_STAT_10,
    SPR_STAT_11,
    SPR_STAT_12,
    SPR_STAT_13,
    SPR_STAT_14,
    SPR_STAT_15,

    SPR_STAT_16,
    SPR_STAT_17,
    SPR_STAT_18,
    SPR_STAT_19,
    SPR_STAT_20,
    SPR_STAT_21,
    SPR_STAT_22,
    SPR_STAT_23,

    SPR_STAT_24,
    SPR_STAT_25,
    SPR_STAT_26,
    SPR_STAT_27,
    SPR_STAT_28,
    SPR_STAT_29,
    SPR_STAT_30,
    SPR_STAT_31,

    SPR_STAT_32,
    SPR_STAT_33,
    SPR_STAT_34,
    SPR_STAT_35,
    SPR_STAT_36,
    SPR_STAT_37,
    SPR_STAT_38,
    SPR_STAT_39,

    SPR_STAT_40,
    SPR_STAT_41,
    SPR_STAT_42,
    SPR_STAT_43,
    SPR_STAT_44,
    SPR_STAT_45,
    SPR_STAT_46,
    SPR_STAT_47,

    //
    // guard
    //
    SPR_GRD_S_1,
    SPR_GRD_S_2,
    SPR_GRD_S_3,
    SPR_GRD_S_4,
    SPR_GRD_S_5,
    SPR_GRD_S_6,
    SPR_GRD_S_7,
    SPR_GRD_S_8,

    SPR_GRD_W1_1,
    SPR_GRD_W1_2,
    SPR_GRD_W1_3,
    SPR_GRD_W1_4,
    SPR_GRD_W1_5,
    SPR_GRD_W1_6,
    SPR_GRD_W1_7,
    SPR_GRD_W1_8,

    SPR_GRD_W2_1,
    SPR_GRD_W2_2,
    SPR_GRD_W2_3,
    SPR_GRD_W2_4,
    SPR_GRD_W2_5,
    SPR_GRD_W2_6,
    SPR_GRD_W2_7,
    SPR_GRD_W2_8,

    SPR_GRD_W3_1,
    SPR_GRD_W3_2,
    SPR_GRD_W3_3,
    SPR_GRD_W3_4,
    SPR_GRD_W3_5,
    SPR_GRD_W3_6,
    SPR_GRD_W3_7,
    SPR_GRD_W3_8,

    SPR_GRD_W4_1,
    SPR_GRD_W4_2,
    SPR_GRD_W4_3,
    SPR_GRD_W4_4,
    SPR_GRD_W4_5,
    SPR_GRD_W4_6,
    SPR_GRD_W4_7,
    SPR_GRD_W4_8,

    SPR_GRD_PAIN_1,
    SPR_GRD_DIE_1,
    SPR_GRD_DIE_2,
    SPR_GRD_DIE_3,
    SPR_GRD_PAIN_2,
    SPR_GRD_DEAD,

    SPR_GRD_SHOOT1,
    SPR_GRD_SHOOT2,
    SPR_GRD_SHOOT3,

    //
    // dogs
    //
    SPR_DOG_W1_1,
    SPR_DOG_W1_2,
    SPR_DOG_W1_3,
    SPR_DOG_W1_4,
    SPR_DOG_W1_5,
    SPR_DOG_W1_6,
    SPR_DOG_W1_7,
    SPR_DOG_W1_8,

    SPR_DOG_W2_1,
    SPR_DOG_W2_2,
    SPR_DOG_W2_3,
    SPR_DOG_W2_4,
    SPR_DOG_W2_5,
    SPR_DOG_W2_6,
    SPR_DOG_W2_7,
    SPR_DOG_W2_8,

    SPR_DOG_W3_1,
    SPR_DOG_W3_2,
    SPR_DOG_W3_3,
    SPR_DOG_W3_4,
    SPR_DOG_W3_5,
    SPR_DOG_W3_6,
    SPR_DOG_W3_7,
    SPR_DOG_W3_8,

    SPR_DOG_W4_1,
    SPR_DOG_W4_2,
    SPR_DOG_W4_3,
    SPR_DOG_W4_4,
    SPR_DOG_W4_5,
    SPR_DOG_W4_6,
    SPR_DOG_W4_7,
    SPR_DOG_W4_8,

    SPR_DOG_DIE_1,
    SPR_DOG_DIE_2,
    SPR_DOG_DIE_3,
    SPR_DOG_DEAD,
    SPR_DOG_JUMP1,
    SPR_DOG_JUMP2,
    SPR_DOG_JUMP3,

    //
    // ss
    //
    SPR_SS_S_1,
    SPR_SS_S_2,
    SPR_SS_S_3,
    SPR_SS_S_4,
    SPR_SS_S_5,
    SPR_SS_S_6,
    SPR_SS_S_7,
    SPR_SS_S_8,

    SPR_SS_W1_1,
    SPR_SS_W1_2,
    SPR_SS_W1_3,
    SPR_SS_W1_4,
    SPR_SS_W1_5,
    SPR_SS_W1_6,
    SPR_SS_W1_7,
    SPR_SS_W1_8,

    SPR_SS_W2_1,
    SPR_SS_W2_2,
    SPR_SS_W2_3,
    SPR_SS_W2_4,
    SPR_SS_W2_5,
    SPR_SS_W2_6,
    SPR_SS_W2_7,
    SPR_SS_W2_8,

    SPR_SS_W3_1,
    SPR_SS_W3_2,
    SPR_SS_W3_3,
    SPR_SS_W3_4,
    SPR_SS_W3_5,
    SPR_SS_W3_6,
    SPR_SS_W3_7,
    SPR_SS_W3_8,

    SPR_SS_W4_1,
    SPR_SS_W4_2,
    SPR_SS_W4_3,
    SPR_SS_W4_4,
    SPR_SS_W4_5,
    SPR_SS_W4_6,
    SPR_SS_W4_7,
    SPR_SS_W4_8,

    SPR_SS_PAIN_1,
    SPR_SS_DIE_1,
    SPR_SS_DIE_2,
    SPR_SS_DIE_3,
    SPR_SS_PAIN_2,
    SPR_SS_DEAD,

    SPR_SS_SHOOT1,
    SPR_SS_SHOOT2,
    SPR_SS_SHOOT3,

    //
    // mutant
    //
    SPR_MUT_S_1,
    SPR_MUT_S_2,
    SPR_MUT_S_3,
    SPR_MUT_S_4,
    SPR_MUT_S_5,
    SPR_MUT_S_6,
    SPR_MUT_S_7,
    SPR_MUT_S_8,

    SPR_MUT_W1_1,
    SPR_MUT_W1_2,
    SPR_MUT_W1_3,
    SPR_MUT_W1_4,
    SPR_MUT_W1_5,
    SPR_MUT_W1_6,
    SPR_MUT_W1_7,
    SPR_MUT_W1_8,

    SPR_MUT_W2_1,
    SPR_MUT_W2_2,
    SPR_MUT_W2_3,
    SPR_MUT_W2_4,
    SPR_MUT_W2_5,
    SPR_MUT_W2_6,
    SPR_MUT_W2_7,
    SPR_MUT_W2_8,

    SPR_MUT_W3_1,
    SPR_MUT_W3_2,
    SPR_MUT_W3_3,
    SPR_MUT_W3_4,
    SPR_MUT_W3_5,
    SPR_MUT_W3_6,
    SPR_MUT_W3_7,
    SPR_MUT_W3_8,

    SPR_MUT_W4_1,
    SPR_MUT_W4_2,
    SPR_MUT_W4_3,
    SPR_MUT_W4_4,
    SPR_MUT_W4_5,
    SPR_MUT_W4_6,
    SPR_MUT_W4_7,
    SPR_MUT_W4_8,

    SPR_MUT_PAIN_1,
    SPR_MUT_DIE_1,
    SPR_MUT_DIE_2,
    SPR_MUT_DIE_3,
    SPR_MUT_PAIN_2,
    SPR_MUT_DIE_4,
    SPR_MUT_DEAD,

    SPR_MUT_SHOOT1,
    SPR_MUT_SHOOT2,
    SPR_MUT_SHOOT3,
    SPR_MUT_SHOOT4,

    //
    // officer
    //
    SPR_OFC_S_1,
    SPR_OFC_S_2,
    SPR_OFC_S_3,
    SPR_OFC_S_4,
    SPR_OFC_S_5,
    SPR_OFC_S_6,
    SPR_OFC_S_7,
    SPR_OFC_S_8,

    SPR_OFC_W1_1,
    SPR_OFC_W1_2,
    SPR_OFC_W1_3,
    SPR_OFC_W1_4,
    SPR_OFC_W1_5,
    SPR_OFC_W1_6,
    SPR_OFC_W1_7,
    SPR_OFC_W1_8,

    SPR_OFC_W2_1,
    SPR_OFC_W2_2,
    SPR_OFC_W2_3,
    SPR_OFC_W2_4,
    SPR_OFC_W2_5,
    SPR_OFC_W2_6,
    SPR_OFC_W2_7,
    SPR_OFC_W2_8,

    SPR_OFC_W3_1,
    SPR_OFC_W3_2,
    SPR_OFC_W3_3,
    SPR_OFC_W3_4,
    SPR_OFC_W3_5,
    SPR_OFC_W3_6,
    SPR_OFC_W3_7,
    SPR_OFC_W3_8,

    SPR_OFC_W4_1,
    SPR_OFC_W4_2,
    SPR_OFC_W4_3,
    SPR_OFC_W4_4,
    SPR_OFC_W4_5,
    SPR_OFC_W4_6,
    SPR_OFC_W4_7,
    SPR_OFC_W4_8,

    SPR_OFC_PAIN_1,
    SPR_OFC_DIE_1,
    SPR_OFC_DIE_2,
    SPR_OFC_DIE_3,
    SPR_OFC_PAIN_2,
    SPR_OFC_DIE_4,
    SPR_OFC_DEAD,

    SPR_OFC_SHOOT1,
    SPR_OFC_SHOOT2,
    SPR_OFC_SHOOT3,

    //
    // ghosts
    //
    SPR_BLINKY_W1,
    SPR_BLINKY_W2,
    SPR_PINKY_W1,
    SPR_PINKY_W2,
    SPR_CLYDE_W1,
    SPR_CLYDE_W2,
    SPR_INKY_W1,
    SPR_INKY_W2,

    //
    // hans
    //
    SPR_BOSS_W1,
    SPR_BOSS_W2,
    SPR_BOSS_W3,
    SPR_BOSS_W4,
    SPR_BOSS_SHOOT1,
    SPR_BOSS_SHOOT2,
    SPR_BOSS_SHOOT3,
    SPR_BOSS_DEAD,
    SPR_BOSS_DIE1,
    SPR_BOSS_DIE2,
    SPR_BOSS_DIE3,

    //
    // schabbs
    //
    SPR_SCHABB_W1,
    SPR_SCHABB_W2,
    SPR_SCHABB_W3,
    SPR_SCHABB_W4,
    SPR_SCHABB_SHOOT1,
    SPR_SCHABB_SHOOT2,
    SPR_SCHABB_DIE1,
    SPR_SCHABB_DIE2,
    SPR_SCHABB_DIE3,
    SPR_SCHABB_DEAD,
    SPR_HYPO1,
    SPR_HYPO2,
    SPR_HYPO3,
    SPR_HYPO4,

    //
    // fake
    //
    SPR_FAKE_W1,
    SPR_FAKE_W2,
    SPR_FAKE_W3,
    SPR_FAKE_W4,
    SPR_FAKE_SHOOT,
    SPR_FIRE1,
    SPR_FIRE2,
    SPR_FAKE_DIE1,
    SPR_FAKE_DIE2,
    SPR_FAKE_DIE3,
    SPR_FAKE_DIE4,
    SPR_FAKE_DIE5,
    SPR_FAKE_DEAD,

    //
    // hitler
    //
    SPR_MECHA_W1,
    SPR_MECHA_W2,
    SPR_MECHA_W3,
    SPR_MECHA_W4,
    SPR_MECHA_SHOOT1,
    SPR_MECHA_SHOOT2,
    SPR_MECHA_SHOOT3,
    SPR_MECHA_DEAD,
    SPR_MECHA_DIE1,
    SPR_MECHA_DIE2,
    SPR_MECHA_DIE3,
    SPR_HITLER_W1,
    SPR_HITLER_W2,
    SPR_HITLER_W3,
    SPR_HITLER_W4,
    SPR_HITLER_SHOOT1,
    SPR_HITLER_SHOOT2,
    SPR_HITLER_SHOOT3,
    SPR_HITLER_DEAD,
    SPR_HITLER_DIE1,
    SPR_HITLER_DIE2,
    SPR_HITLER_DIE3,
    SPR_HITLER_DIE4,
    SPR_HITLER_DIE5,
    SPR_HITLER_DIE6,
    SPR_HITLER_DIE7,

    //
    // giftmacher
    //
    SPR_GIFT_W1,
    SPR_GIFT_W2,
    SPR_GIFT_W3,
    SPR_GIFT_W4,
    SPR_GIFT_SHOOT1,
    SPR_GIFT_SHOOT2,
    SPR_GIFT_DIE1,
    SPR_GIFT_DIE2,
    SPR_GIFT_DIE3,
    SPR_GIFT_DEAD,

    //
    // Rocket, smoke and small explosion
    //
    SPR_ROCKET_1,
    SPR_ROCKET_2,
    SPR_ROCKET_3,
    SPR_ROCKET_4,
    SPR_ROCKET_5,
    SPR_ROCKET_6,
    SPR_ROCKET_7,
    SPR_ROCKET_8,
    SPR_SMOKE_1,
    SPR_SMOKE_2,
    SPR_SMOKE_3,
    SPR_SMOKE_4,
    SPR_BOOM_1,
    SPR_BOOM_2,
    SPR_BOOM_3,

    //
    // gretel
    //
    SPR_GRETEL_W1,
    SPR_GRETEL_W2,
    SPR_GRETEL_W3,
    SPR_GRETEL_W4,
    SPR_GRETEL_SHOOT1,
    SPR_GRETEL_SHOOT2,
    SPR_GRETEL_SHOOT3,
    SPR_GRETEL_DEAD,
    SPR_GRETEL_DIE1,
    SPR_GRETEL_DIE2,
    SPR_GRETEL_DIE3,

    //
    // fat face
    //
    SPR_FAT_W1,
    SPR_FAT_W2,
    SPR_FAT_W3,
    SPR_FAT_W4,
    SPR_FAT_SHOOT1,
    SPR_FAT_SHOOT2,
    SPR_FAT_SHOOT3,
    SPR_FAT_SHOOT4,
    SPR_FAT_DIE1,
    SPR_FAT_DIE2,
    SPR_FAT_DIE3,
    SPR_FAT_DEAD,

    //
    // bj
    //
    SPR_BJ_W1,
    SPR_BJ_W2,
    SPR_BJ_W3,
    SPR_BJ_W4,
    SPR_BJ_JUMP1,
    SPR_BJ_JUMP2,
    SPR_BJ_JUMP3,
    SPR_BJ_JUMP4,

    //
    // player attack frames
    //
    SPR_KNIFEREADY,
    SPR_KNIFEATK1,
    SPR_KNIFEATK2,
    SPR_KNIFEATK3,
    SPR_KNIFEATK4,

    SPR_PISTOLREADY,
    SPR_PISTOLATK1,
    SPR_PISTOLATK2,
    SPR_PISTOLATK3,
    SPR_PISTOLATK4,

    SPR_MACHINEGUNREADY,
    SPR_MACHINEGUNATK1,
    SPR_MACHINEGUNATK2,
    MACHINEGUNATK3,
    SPR_MACHINEGUNATK4,

    SPR_CHAINREADY,
    SPR_CHAINATK1,
    SPR_CHAINATK2,
    SPR_CHAINATK3,
    SPR_CHAINATK4,
}

/*
=============================================================================

                               GLOBAL TYPES

=============================================================================
*/

#[derive(PartialEq, Clone, Copy)]
pub enum controldir_t {
    di_none = -1,
    di_north = 0,
    di_east,
    di_south,
    di_west,
}
impl controldir_t {
    pub fn from_i32(value: i32) -> controldir_t {
        match value {
            -1 => controldir_t::di_none,
            0 => controldir_t::di_north,
            1 => controldir_t::di_east,
            2 => controldir_t::di_south,
            3 => controldir_t::di_west,
            _ => controldir_t::di_none,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum door_t {
    dr_normal,
    dr_lock1,
    dr_lock2,
    dr_lock3,
    dr_lock4,
    dr_elevator,
}
impl door_t {
    pub fn from_i32(value: i32) -> door_t {
        match value {
            0 => door_t::dr_normal,
            1 => door_t::dr_lock1,
            2 => door_t::dr_lock2,
            3 => door_t::dr_lock3,
            4 => door_t::dr_lock4,
            5 => door_t::dr_elevator,
            _ => door_t::dr_normal,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum activetype {
    ac_badobject = -1,
    ac_no = 0,
    ac_yes,
    ac_allways,
}
impl activetype {
    pub fn from_i32(value: i32) -> activetype {
        match value {
            -1 => activetype::ac_badobject,
            0 => activetype::ac_no,
            1 => activetype::ac_yes,
            2 => activetype::ac_allways,
            _ => activetype::ac_badobject,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum classtype {
    nothing,
    playerobj,
    inertobj,
    guardobj,
    officerobj,
    ssobj,
    dogobj,
    bossobj,
    schabbobj,
    fakeobj,
    mechahitlerobj,
    mutantobj,
    needleobj,
    fireobj,
    bjobj,
    ghostobj,
    realhitlerobj,
    gretelobj,
    giftobj,
    fatobj,
    rocketobj,

    spectreobj,
    angelobj,
    transobj,
    uberobj,
    willobj,
    deathobj,
    hrocketobj,
    sparkobj,
}

impl classtype {
    pub fn from_u8(value: u8) -> classtype {
        match value {
            0 => classtype::nothing,
            1 => classtype::playerobj,
            2 => classtype::inertobj,
            3 => classtype::guardobj,
            4 => classtype::officerobj,
            5 => classtype::ssobj,
            6 => classtype::dogobj,
            7 => classtype::bossobj,
            8 => classtype::schabbobj,
            9 => classtype::fakeobj,
            10 => classtype::mechahitlerobj,
            11 => classtype::mutantobj,
            12 => classtype::needleobj,
            13 => classtype::fireobj,
            14 => classtype::bjobj,
            15 => classtype::ghostobj,
            16 => classtype::realhitlerobj,
            17 => classtype::gretelobj,
            18 => classtype::giftobj,
            19 => classtype::fatobj,
            20 => classtype::rocketobj,

            21 => classtype::spectreobj,
            22 => classtype::angelobj,
            23 => classtype::transobj,
            24 => classtype::uberobj,
            25 => classtype::willobj,
            26 => classtype::deathobj,
            27 => classtype::hrocketobj,
            28 => classtype::sparkobj,

            _ => classtype::nothing,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum wl_stat_t {
    none,
    block,
    bo_gibs,
    bo_alpo,
    bo_firstaid,
    bo_key1,
    bo_key2,
    bo_key3,
    bo_key4,
    bo_cross,
    bo_chalice,
    bo_bible,
    bo_crown,
    bo_clip,
    bo_clip2,
    bo_machinegun,
    bo_chaingun,
    bo_food,
    bo_fullheal,
    bo_25clip,
    bo_spear,
}

impl wl_stat_t {
    pub fn from_u8(value: u8) -> wl_stat_t {
        match value {
            0 => wl_stat_t::none,
            1 => wl_stat_t::block,
            2 => wl_stat_t::bo_gibs,
            3 => wl_stat_t::bo_alpo,
            4 => wl_stat_t::bo_firstaid,
            5 => wl_stat_t::bo_key1,
            6 => wl_stat_t::bo_key2,
            7 => wl_stat_t::bo_key3,
            8 => wl_stat_t::bo_key4,
            9 => wl_stat_t::bo_cross,
            10 => wl_stat_t::bo_chalice,
            11 => wl_stat_t::bo_bible,
            12 => wl_stat_t::bo_crown,
            13 => wl_stat_t::bo_clip,
            14 => wl_stat_t::bo_clip2,
            15 => wl_stat_t::bo_machinegun,
            16 => wl_stat_t::bo_chaingun,
            17 => wl_stat_t::bo_food,
            18 => wl_stat_t::bo_fullheal,
            19 => wl_stat_t::bo_25clip,
            20 => wl_stat_t::bo_spear,

            _ => wl_stat_t::none,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum dirtype {
    east,
    northeast,
    north,
    northwest,
    west,
    southwest,
    south,
    southeast,
    nodir,
}

impl dirtype {
    pub fn from_u8(value: u8) -> dirtype {
        match value {
            0 => dirtype::east,
            1 => dirtype::northeast,
            2 => dirtype::north,
            3 => dirtype::northwest,
            4 => dirtype::west,
            5 => dirtype::southwest,
            6 => dirtype::south,
            7 => dirtype::southeast,
            8 => dirtype::nodir,
            _ => dirtype::east,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum enemy_t {
    en_guard,
    en_officer,
    en_ss,
    en_dog,
    en_boss,
    en_schabbs,
    en_fake,
    en_hitler,
    en_mutant,
    en_blinky,
    en_clyde,
    en_pinky,
    en_inky,
    en_gretel,
    en_gift,
    en_fat,

    NUMENEMIES,
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Serialize, Deserialize)]
pub struct statetype {
    pub rotate: u8,
    pub shapenum: i32, // a shapenum of -1 means get from ob->temp1
    pub tictime: i32,
    pub think: bool,
    pub action: bool,
    pub id: u16,
}

impl statetype {
    pub fn new() -> Self {
        Self {
            rotate: 0,
            shapenum: 0,
            tictime: 0,
            think: false,
            action: false,
            id: 0,
        }
    }
}

//---------------------
//
// trivial actor structure
//
//---------------------

#[derive(PartialEq, Clone, Copy)]
pub struct statobj_t {
    pub tilex: i32,
    pub tiley: i32,
    pub shapenum: i32, // if shapenum == -1 the obj has been removed
    pub visspot: *mut bool,
    pub flags: i32,
    pub itemnumber: i32,
}
impl statobj_t {
    pub fn new() -> Self {
        Self {
            tilex: 0,
            tiley: 0,
            shapenum: 0,
            visspot: ptr::null_mut(),
            flags: 0,
            itemnumber: 0,
        }
    }
}

//---------------------
//
// door actor structure
//
//---------------------

#[derive(Default, Clone, Copy, PartialEq)]
pub enum doortype {
    dr_open,
    #[default]
    dr_closed,
    dr_opening,
    dr_closing,
}

#[derive(Default, Clone, Copy)]
pub struct doorobj_t {
    pub tilex: i32,
    pub tiley: i32,
    pub vertical: bool,
    pub lock: i32,
    pub action: doortype,
    pub ticcount: i32,
}

impl doorobj_t {
    pub fn new() -> Self {
        Self {
            tilex: 0,
            tiley: 0,
            vertical: false,
            lock: 0,
            action: doortype::dr_closed,
            ticcount: 0,
        }
    }
}

//--------------------
//
// thinking actor structure
//
//--------------------

//
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct objtype {
    pub active: activetype,
    pub ticcount: i32,
    pub obclass: classtype,
    pub state: statetype,

    pub flags: i32, // FL_SHOOTABLE, etc

    pub distance: i32, // if negative, wait for that door to open
    pub dir: dirtype,

    pub x: i32,
    pub y: i32,
    pub tilex: i32,
    pub tiley: i32,
    pub areanumber: i32,

    pub viewx: i32,
    pub viewheight: i32,
    pub transx: i32, // in global coord
    pub transy: i32,

    pub angle: i32,
    pub hitpoints: i32,
    pub speed: i32,

    pub temp1: i32,
    pub temp2: i32,
    pub hidden: i32,
    pub id: i32, //keep track of the objects
}

impl objtype {
    pub fn new() -> Self {
        Self {
            active: activetype::ac_no,
            ticcount: 0,
            obclass: classtype::nothing,
            state: statetype::new(),

            flags: 0, // FL_SHOOTABLE, etc

            distance: 0, // if negative, wait for that door to open
            dir: dirtype::nodir,

            x: 0,
            y: 0,
            tilex: 0,
            tiley: 0,
            areanumber: 0,

            viewx: 0,
            viewheight: 0,
            transx: 0, // in global coord
            transy: 0,

            angle: 0,
            hitpoints: 0,
            speed: 0,

            temp1: 0,
            temp2: 0,
            hidden: 0,
            id: 0,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum buttontype {
    bt_nobutton = -1,
    bt_attack = 0,
    bt_strafe,
    bt_run,
    bt_use,
    bt_readyknife,
    bt_readypistol,
    bt_readymachinegun,
    bt_readychaingun,
    bt_nextweapon,
    bt_prevweapon,
    bt_esc,
    bt_pause,
    bt_strafeleft,
    bt_straferight,
    bt_moveforward,
    bt_movebackward,
    bt_turnleft,
    bt_turnright,

    NUMBUTTONS,
}

#[derive(PartialEq, PartialOrd, Default, Clone, Copy)]
pub enum weapontype {
    wp_noweapon = -1,
    #[default]
    wp_knife = 0,
    wp_pistol,
    wp_machinegun,
    wp_chaingun,

    NUMWEAPONS,
}

impl weapontype {
    pub fn from_u8(value: i8) -> weapontype {
        match value {
            0 => weapontype::wp_knife,
            1 => weapontype::wp_pistol,
            2 => weapontype::wp_machinegun,
            3 => weapontype::wp_chaingun,
            _ => weapontype::wp_noweapon,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum gd {
    gd_baby,
    gd_easy,
    gd_medium,
    gd_hard,
}

//---------------
//
// gamestate structure
//
//---------------

#[derive(Default)]
pub struct gametype {
    pub difficulty: i32,
    pub mapon: i32,
    pub oldscore: i32,
    pub score: i32,
    pub nextextra: i32,
    pub lives: i32,
    pub health: i32,
    pub ammo: i32,
    pub keys: i32,
    pub bestweapon: weapontype,
    pub weapon: weapontype,
    pub chosenweapon: weapontype,

    pub faceframe: i32,
    pub attackframe: i32,
    pub attackcount: i32,
    pub weaponframe: i32,

    pub episode: i32,
    pub secretcount: i32,
    pub treasurecount: i32,
    pub killcount: i32,
    pub secrettotal: i32,
    pub treasuretotal: i32,
    pub killtotal: i32,
    pub TimeCount: i32,
    pub killx: i32,
    pub killy: i32,
    pub victoryflag: bool, // set during victory animations
}

impl gametype {
    pub fn new() -> Self {
        Self {
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
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum exit_t {
    ex_stillplaying,
    ex_completed,
    ex_died,
    ex_warped,
    ex_resetgame,
    ex_loadedgame,
    ex_victorious,
    ex_abort,
    ex_demodone,
    ex_secretlevel,
}

/*
=============================================================================

                            WL_PLAY DEFINITIONS

=============================================================================
*/

pub const BASEMOVE: i32 = 35;
pub const RUNMOVE: i32 = 70;
pub const BASETURN: i32 = 35;
pub const RUNTURN: i32 = 70;
pub const JOYSCALE: i32 = 2;

/*
=============================================================================

                             WL_SCALE DEFINITIONS

=============================================================================
*/

#[derive(Serialize, Deserialize)]
pub struct compshape_t {
    pub leftpix: u16,
    pub rightpix: u16,
    #[serde(with = "BigArray")]
    pub dataofs: [u16; 64],
    // table data after dataofs[rightpix-leftpix+1]
}

/*
=============================================================================

                             WL_STATE DEFINITIONS

=============================================================================
*/

pub const TURNTICS: i32 = 10;
pub const SPDPATROL: i32 = 512;
pub const SPDDOG: i32 = 1500;

/*
=============================================================================

                             WL_ACT2 DEFINITIONS

=============================================================================
*/

//pub const s_nakedbody :i32 = s_static10;

/*
=============================================================================

                             MISC DEFINITIONS

=============================================================================
*/

pub fn DEMOCOND_ORIG(w3d: &mut modules) -> bool {
    return w3d.wl_play.demorecord || w3d.wl_play.demoplayback;
}

pub fn DEMOIF_SDL(w3d: &mut modules) -> bool {
    if DEMOCOND_SDL(w3d) {
        return true;
    } else {
        return false;
    }
}

pub fn DEMOCOND_SDL(w3d: &mut modules) -> bool {
    return !DEMOCOND_ORIG(w3d);
}

pub fn SDL_GetTicks(w3d: &mut modules) -> i32 {
    w3d.id_vl.timer.ticks() as i32
}

pub fn SDL_Delay(w3d: &mut modules, ticks: i32) {
    w3d.id_vl.timer.delay(ticks as u32)
}

pub fn SIGN(x: i32) -> i8 {
    if x > 0 {
        0
    } else {
        -1
    }
}

pub fn ABS(x: i32) -> i32 {
    if x > 0 {
        x
    } else {
        -x
    }
}

pub fn LABS(x: i32) -> i32 {
    if x > 0 {
        x
    } else {
        -x
    }
}

pub fn abs(x: i32) -> i32 {
    ABS(x)
}

pub fn lengthof<T>(_x: T) -> usize {
    //println!("lengthof");

    //size_of(x) / size_of(*x)
    0
}

pub fn endof(x: i32) -> usize {
    //println!("lengthof");

    x as usize / lengthof(x)
}

pub fn MAPSPOT(w3d: &mut modules, x: i32, y: i32, plane: i32) -> i32 {
    let spot = w3d.id_ca.mapsegs[plane as usize][((y << MAPSHIFT) + x) as usize];

    spot as i32
}
