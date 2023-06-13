#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_act2
//
//===========================================================================

pub struct wl_act2 {
    pub mechahitlerobj_i: usize,
    pub rocketobj_i: usize,
}

impl wl_act2 {
    pub fn new() -> Self {
        Self {
            mechahitlerobj_i: 0,
            rocketobj_i: 0,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const PROJECTILESIZE: i32 = 0xc000;
pub const BJRUNSPEED: i32 = 2048;
pub const BJJUMPSPEED: i32 = 680;

pub const starthitpoints: [[i32; enemy_t::NUMENEMIES as usize]; 4] =
    //
    // BABY MODE
    //
    [
        [
            25,  // guards
            50,  // officer
            100, // SS
            1,   // dogs
            850, // Hans
            850, // Schabbs
            200, // fake hitler
            800, // mecha hitler
            45,  // mutants
            25,  // ghosts
            25,  // ghosts
            25,  // ghosts
            25,  // ghosts
            850, // Gretel
            850, // Gift
            850, // Fat
        ],
        //
        // DON'T HURT ME MODE
        //
        [
            25,  // guards
            50,  // officer
            100, // SS
            1,   // dogs
            950, // Hans
            950, // Schabbs
            300, // fake hitler
            950, // mecha hitler
            55,  // mutants
            25,  // ghosts
            25,  // ghosts
            25,  // ghosts
            25,  // ghosts
            950, // Gretel
            950, // Gift
            950, // Fat
        ],
        //
        // BRING 'EM ON MODE
        //
        [
            25,   // guards
            50,   // officer
            100,  // SS
            1,    // dogs
            1050, // Hans
            1550, // Schabbs
            400,  // fake hitler
            1050, // mecha hitler
            55,   // mutants
            25,   // ghosts
            25,   // ghosts
            25,   // ghosts
            25,   // ghosts
            1050, // Gretel
            1050, // Gift
            1050, // Fat
        ],
        //
        // DEATH INCARNATE MODE
        //
        [
            25,   // guards
            50,   // officer
            100,  // SS
            1,    // dogs
            1200, // Hans
            2400, // Schabbs
            500,  // fake hitler
            1200, // mecha hitler
            65,   // mutants
            25,   // ghosts
            25,   // ghosts
            25,   // ghosts
            25,   // ghosts
            1200, // Gretel
            1200, // Gift
            1200, // Fat
        ],
    ];

pub const PROJSIZE: i32 = 0x2000;

//statetype s_rocket              = {true,SPR_ROCKET_1,3,(statefunc)T_Projectile,(statefunc)A_Smoke,&s_rocket};

pub const s_rocket: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_ROCKET_1 as i32,
    tictime: 3,
    think: true,
    action: true,
    id: 3,
};
//statetype s_smoke1              = {false,SPR_SMOKE_1,3,NULL,NULL,&s_smoke2};
//statetype s_smoke2              = {false,SPR_SMOKE_2,3,NULL,NULL,&s_smoke3};
//statetype s_smoke3              = {false,SPR_SMOKE_3,3,NULL,NULL,&s_smoke4};
//statetype s_smoke4              = {false,SPR_SMOKE_4,3,NULL,NULL,NULL};

pub const s_smoke1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SMOKE_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 4,
};
pub const s_smoke2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SMOKE_2 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 5,
};
pub const s_smoke3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SMOKE_3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 6,
};
pub const s_smoke4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SMOKE_4 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 7,
};

//statetype s_boom1               = {false,SPR_BOOM_1,6,NULL,NULL,&s_boom2};
//statetype s_boom2               = {false,SPR_BOOM_2,6,NULL,NULL,&s_boom3};
//statetype s_boom3               = {false,SPR_BOOM_3,6,NULL,NULL,NULL};

pub const s_boom1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOOM_1 as i32,
    tictime: 6,
    think: false,
    action: false,
    id: 8,
};
pub const s_boom2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOOM_2 as i32,
    tictime: 6,
    think: false,
    action: false,
    id: 9,
};
pub const s_boom3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOOM_3 as i32,
    tictime: 6,
    think: false,
    action: false,
    id: 10,
};

//
// guards
//

//statetype s_grdstand            = {true,SPR_GRD_S_1,0,(statefunc)T_Stand,NULL,&s_grdstand};
pub const s_grdstand: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_S_1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 11,
};

//statetype s_grdpath1            = {true,SPR_GRD_W1_1,20,(statefunc)T_Path,NULL,&s_grdpath1s};
//statetype s_grdpath1s           = {true,SPR_GRD_W1_1,5,NULL,NULL,&s_grdpath2};
//statetype s_grdpath2            = {true,SPR_GRD_W2_1,15,(statefunc)T_Path,NULL,&s_grdpath3};
//statetype s_grdpath3            = {true,SPR_GRD_W3_1,20,(statefunc)T_Path,NULL,&s_grdpath3s};
//statetype s_grdpath3s           = {true,SPR_GRD_W3_1,5,NULL,NULL,&s_grdpath4};
//statetype s_grdpath4            = {true,SPR_GRD_W4_1,15,(statefunc)T_Path,NULL,&s_grdpath1};

pub const s_grdpath1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W1_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 12,
};
pub const s_grdpath1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W1_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 13,
};
pub const s_grdpath2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W2_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 14,
};
pub const s_grdpath3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W3_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 15,
};

pub const s_grdpath3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W3_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 16,
};
pub const s_grdpath4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W4_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 17,
};

//statetype s_grdpain             = {2,SPR_GRD_PAIN_1,10,NULL,NULL,&s_grdchase1};
//statetype s_grdpain1            = {2,SPR_GRD_PAIN_2,10,NULL,NULL,&s_grdchase1};

pub const s_grdpain: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_GRD_PAIN_1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 18,
};
pub const s_grdpain1: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_GRD_PAIN_2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 19,
};

//statetype s_grdshoot1           = {false,SPR_GRD_SHOOT1,20,NULL,NULL,&s_grdshoot2};
//statetype s_grdshoot2           = {false,SPR_GRD_SHOOT2,20,NULL,(statefunc)T_Shoot,&s_grdshoot3};
//statetype s_grdshoot3           = {false,SPR_GRD_SHOOT3,20,NULL,NULL,&s_grdchase1};

pub const s_grdshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_SHOOT1 as i32,
    tictime: 20,
    think: false,
    action: false,
    id: 20,
};
pub const s_grdshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_SHOOT2 as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 21,
};
pub const s_grdshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_SHOOT3 as i32,
    tictime: 20,
    think: false,
    action: false,
    id: 22,
};

//statetype s_grdchase1           = {true,SPR_GRD_W1_1,10,(statefunc)T_Chase,NULL,&s_grdchase1s};
//statetype s_grdchase1s          = {true,SPR_GRD_W1_1,3,NULL,NULL,&s_grdchase2};
//statetype s_grdchase2           = {true,SPR_GRD_W2_1,8,(statefunc)T_Chase,NULL,&s_grdchase3};
//statetype s_grdchase3           = {true,SPR_GRD_W3_1,10,(statefunc)T_Chase,NULL,&s_grdchase3s};
//statetype s_grdchase3s          = {true,SPR_GRD_W3_1,3,NULL,NULL,&s_grdchase4};
//statetype s_grdchase4           = {true,SPR_GRD_W4_1,8,(statefunc)T_Chase,NULL,&s_grdchase1};

pub const s_grdchase1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W1_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 23,
};
pub const s_grdchase1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W1_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 24,
};
pub const s_grdchase2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W2_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 25,
};
pub const s_grdchase3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W3_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 26,
};
pub const s_grdchase3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W3_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 27,
};
pub const s_grdchase4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W4_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 28,
};

//statetype s_grddie1             = {false,SPR_GRD_DIE_1,15,NULL,(statefunc)A_DeathScream,&s_grddie2};
//statetype s_grddie2             = {false,SPR_GRD_DIE_2,15,NULL,NULL,&s_grddie3};
//statetype s_grddie3             = {false,SPR_GRD_DIE_3,15,NULL,NULL,&s_grddie4};
//statetype s_grddie4             = {false,SPR_GRD_DEAD,0,NULL,NULL,&s_grddie4};

pub const s_grddie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_DIE_1 as i32,
    tictime: 15,
    think: false,
    action: true,
    id: 29,
};
pub const s_grddie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_DIE_2 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 30,
};
pub const s_grddie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_DIE_3 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 31,
};
pub const s_grddie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRD_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 32,
};

//
// ghosts
//

//statetype s_blinkychase1        = {false,SPR_BLINKY_W1,10,(statefunc)T_Ghosts,NULL,&s_blinkychase2};
//statetype s_blinkychase2        = {false,SPR_BLINKY_W2,10,(statefunc)T_Ghosts,NULL,&s_blinkychase1};

pub const s_blinkychase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BLINKY_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 33,
};
pub const s_blinkychase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BLINKY_W2 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 34,
};

//statetype s_inkychase1          = {false,SPR_INKY_W1,10,(statefunc)T_Ghosts,NULL,&s_inkychase2};
//statetype s_inkychase2          = {false,SPR_INKY_W2,10,(statefunc)T_Ghosts,NULL,&s_inkychase1};

pub const s_inkychase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_INKY_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 35,
};
pub const s_inkychase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_INKY_W2 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 36,
};

//statetype s_pinkychase1         = {false,SPR_PINKY_W1,10,(statefunc)T_Ghosts,NULL,&s_pinkychase2};
//statetype s_pinkychase2         = {false,SPR_PINKY_W2,10,(statefunc)T_Ghosts,NULL,&s_pinkychase1};

pub const s_pinkychase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_PINKY_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 37,
};
pub const s_pinkychase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_PINKY_W2 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 38,
};

//statetype s_clydechase1         = {false,SPR_CLYDE_W1,10,(statefunc)T_Ghosts,NULL,&s_clydechase2};
//statetype s_clydechase2         = {false,SPR_CLYDE_W2,10,(statefunc)T_Ghosts,NULL,&s_clydechase1};

pub const s_clydechase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_CLYDE_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 39,
};
pub const s_clydechase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_CLYDE_W2 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 40,
};

//
// dogs
//

//statetype s_dogpath1            = {true,SPR_DOG_W1_1,20,(statefunc)T_Path,NULL,&s_dogpath1s};
//statetype s_dogpath1s           = {true,SPR_DOG_W1_1,5,NULL,NULL,&s_dogpath2};
//statetype s_dogpath2            = {true,SPR_DOG_W2_1,15,(statefunc)T_Path,NULL,&s_dogpath3};
//statetype s_dogpath3            = {true,SPR_DOG_W3_1,20,(statefunc)T_Path,NULL,&s_dogpath3s};
//statetype s_dogpath3s           = {true,SPR_DOG_W3_1,5,NULL,NULL,&s_dogpath4};
//statetype s_dogpath4            = {true,SPR_DOG_W4_1,15,(statefunc)T_Path,NULL,&s_dogpath1};

pub const s_dogpath1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W1_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 41,
};
pub const s_dogpath1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W1_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 42,
};
pub const s_dogpath2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W2_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 43,
};
pub const s_dogpath3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W3_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 44,
};
pub const s_dogpath3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W3_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 45,
};
pub const s_dogpath4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W4_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 46,
};

//statetype s_dogjump1            = {false,SPR_DOG_JUMP1,10,NULL,NULL,&s_dogjump2};
//statetype s_dogjump2            = {false,SPR_DOG_JUMP2,10,NULL,(statefunc)T_Bite,&s_dogjump3};
//statetype s_dogjump3            = {false,SPR_DOG_JUMP3,10,NULL,NULL,&s_dogjump4};
//statetype s_dogjump4            = {false,SPR_DOG_JUMP1,10,NULL,NULL,&s_dogjump5};
//statetype s_dogjump5            = {false,SPR_DOG_W1_1,10,NULL,NULL,&s_dogchase1};

pub const s_dogjump1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_JUMP1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 47,
};
pub const s_dogjump2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_JUMP2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 48,
};
pub const s_dogjump3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_JUMP3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 49,
};
pub const s_dogjump4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_JUMP1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 50,
};
pub const s_dogjump5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_W1_1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 51,
};

//statetype s_dogchase1           = {true,SPR_DOG_W1_1,10,(statefunc)T_DogChase,NULL,&s_dogchase1s};
//statetype s_dogchase1s          = {true,SPR_DOG_W1_1,3,NULL,NULL,&s_dogchase2};
//statetype s_dogchase2           = {true,SPR_DOG_W2_1,8,(statefunc)T_DogChase,NULL,&s_dogchase3};
//statetype s_dogchase3           = {true,SPR_DOG_W3_1,10,(statefunc)T_DogChase,NULL,&s_dogchase3s};
//statetype s_dogchase3s          = {true,SPR_DOG_W3_1,3,NULL,NULL,&s_dogchase4};
//statetype s_dogchase4           = {true,SPR_DOG_W4_1,8,(statefunc)T_DogChase,NULL,&s_dogchase1};

pub const s_dogchase1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W1_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 52,
};
pub const s_dogchase1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W1_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 53,
};
pub const s_dogchase2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W2_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 54,
};
pub const s_dogchase3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W3_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 55,
};
pub const s_dogchase3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W3_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 56,
};
pub const s_dogchase4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_DOG_W4_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 57,
};

//statetype s_dogdie1             = {false,SPR_DOG_DIE_1,15,NULL,(statefunc)A_DeathScream,&s_dogdie2};
//statetype s_dogdie2             = {false,SPR_DOG_DIE_2,15,NULL,NULL,&s_dogdie3};
//statetype s_dogdie3             = {false,SPR_DOG_DIE_3,15,NULL,NULL,&s_dogdead};
//statetype s_dogdead             = {false,SPR_DOG_DEAD,15,NULL,NULL,&s_dogdead};

pub const s_dogdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_DIE_1 as i32,
    tictime: 15,
    think: false,
    action: true,
    id: 58,
};
pub const s_dogdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_DIE_2 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 59,
};
pub const s_dogdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_DIE_3 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 60,
};
pub const s_dogdead: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_DOG_DEAD as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 61,
};

//
// officers
//

//statetype s_ofcstand            = {true,SPR_OFC_S_1,0,(statefunc)T_Stand,NULL,&s_ofcstand};

pub const s_ofcstand: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_S_1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 62,
};

//statetype s_ofcpath1            = {true,SPR_OFC_W1_1,20,(statefunc)T_Path,NULL,&s_ofcpath1s};
//statetype s_ofcpath1s           = {true,SPR_OFC_W1_1,5,NULL,NULL,&s_ofcpath2};
//statetype s_ofcpath2            = {true,SPR_OFC_W2_1,15,(statefunc)T_Path,NULL,&s_ofcpath3};
//statetype s_ofcpath3            = {true,SPR_OFC_W3_1,20,(statefunc)T_Path,NULL,&s_ofcpath3s};
//statetype s_ofcpath3s           = {true,SPR_OFC_W3_1,5,NULL,NULL,&s_ofcpath4};
//statetype s_ofcpath4            = {true,SPR_OFC_W4_1,15,(statefunc)T_Path,NULL,&s_ofcpath1};

pub const s_ofcpath1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W1_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 63,
};
pub const s_ofcpath1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W1_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 64,
};
pub const s_ofcpath2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W2_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 65,
};
pub const s_ofcpath3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W3_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 66,
};
pub const s_ofcpath3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W3_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 67,
};
pub const s_ofcpath4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W4_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 68,
};

//statetype s_ofcpain             = {2,SPR_OFC_PAIN_1,10,NULL,NULL,&s_ofcchase1};
//statetype s_ofcpain1            = {2,SPR_OFC_PAIN_2,10,NULL,NULL,&s_ofcchase1};

pub const s_ofcpain: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_OFC_PAIN_1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 69,
};
pub const s_ofcpain1: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_OFC_PAIN_2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 70,
};

//statetype s_ofcshoot1           = {false,SPR_OFC_SHOOT1,6,NULL,NULL,&s_ofcshoot2};
//statetype s_ofcshoot2           = {false,SPR_OFC_SHOOT2,20,NULL,(statefunc)T_Shoot,&s_ofcshoot3};
//statetype s_ofcshoot3           = {false,SPR_OFC_SHOOT3,10,NULL,NULL,&s_ofcchase1};

pub const s_ofcshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_SHOOT1 as i32,
    tictime: 6,
    think: false,
    action: false,
    id: 71,
};
pub const s_ofcshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_SHOOT2 as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 72,
};
pub const s_ofcshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 73,
};

//statetype s_ofcchase1           = {true,SPR_OFC_W1_1,10,(statefunc)T_Chase,NULL,&s_ofcchase1s};
//statetype s_ofcchase1s          = {true,SPR_OFC_W1_1,3,NULL,NULL,&s_ofcchase2};
//statetype s_ofcchase2           = {true,SPR_OFC_W2_1,8,(statefunc)T_Chase,NULL,&s_ofcchase3};
//statetype s_ofcchase3           = {true,SPR_OFC_W3_1,10,(statefunc)T_Chase,NULL,&s_ofcchase3s};
//statetype s_ofcchase3s          = {true,SPR_OFC_W3_1,3,NULL,NULL,&s_ofcchase4};
//statetype s_ofcchase4           = {true,SPR_OFC_W4_1,8,(statefunc)T_Chase,NULL,&s_ofcchase1};

pub const s_ofcchase1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W1_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 74,
};
pub const s_ofcchase1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W1_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 75,
};
pub const s_ofcchase2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W2_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 76,
};
pub const s_ofcchase3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W3_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 77,
};
pub const s_ofcchase3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W3_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 78,
};
pub const s_ofcchase4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_OFC_W4_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 79,
};

//statetype s_ofcdie1             = {false,SPR_OFC_DIE_1,11,NULL,(statefunc)A_DeathScream,&s_ofcdie2};
//statetype s_ofcdie2             = {false,SPR_OFC_DIE_2,11,NULL,NULL,&s_ofcdie3};
//statetype s_ofcdie3             = {false,SPR_OFC_DIE_3,11,NULL,NULL,&s_ofcdie4};
//statetype s_ofcdie4             = {false,SPR_OFC_DIE_4,11,NULL,NULL,&s_ofcdie5};
//statetype s_ofcdie5             = {false,SPR_OFC_DEAD,0,NULL,NULL,&s_ofcdie5};

pub const s_ofcdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_DIE_1 as i32,
    tictime: 11,
    think: false,
    action: true,
    id: 80,
};
pub const s_ofcdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_DIE_2 as i32,
    tictime: 11,
    think: false,
    action: false,
    id: 81,
};
pub const s_ofcdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_DIE_3 as i32,
    tictime: 11,
    think: false,
    action: false,
    id: 82,
};
pub const s_ofcdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_DIE_4 as i32,
    tictime: 11,
    think: false,
    action: false,
    id: 83,
};
pub const s_ofcdie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_OFC_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 84,
};

//
// mutant
//

//statetype s_mutstand            = {true,SPR_MUT_S_1,0,(statefunc)T_Stand,NULL,&s_mutstand};

pub const s_mutstand: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_S_1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 85,
};

//statetype s_mutpath1            = {true,SPR_MUT_W1_1,20,(statefunc)T_Path,NULL,&s_mutpath1s};
//statetype s_mutpath1s           = {true,SPR_MUT_W1_1,5,NULL,NULL,&s_mutpath2};
//statetype s_mutpath2            = {true,SPR_MUT_W2_1,15,(statefunc)T_Path,NULL,&s_mutpath3};
//statetype s_mutpath3            = {true,SPR_MUT_W3_1,20,(statefunc)T_Path,NULL,&s_mutpath3s};
//statetype s_mutpath3s           = {true,SPR_MUT_W3_1,5,NULL,NULL,&s_mutpath4};
//statetype s_mutpath4            = {true,SPR_MUT_W4_1,15,(statefunc)T_Path,NULL,&s_mutpath1};

pub const s_mutpath1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W1_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 86,
};
pub const s_mutpath1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W1_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 87,
};
pub const s_mutpath2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W2_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 88,
};
pub const s_mutpath3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W3_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 89,
};
pub const s_mutpath3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W3_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 90,
};
pub const s_mutpath4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W4_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 91,
};

//statetype s_mutpain             = {2,SPR_MUT_PAIN_1,10,NULL,NULL,&s_mutchase1};
//statetype s_mutpain1            = {2,SPR_MUT_PAIN_2,10,NULL,NULL,&s_mutchase1};

pub const s_mutpain: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_MUT_PAIN_1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 92,
};
pub const s_mutpain1: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_MUT_PAIN_2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 93,
};

//statetype s_mutshoot1           = {false,SPR_MUT_SHOOT1,6,NULL,(statefunc)T_Shoot,&s_mutshoot2};
//statetype s_mutshoot2           = {false,SPR_MUT_SHOOT2,20,NULL,NULL,&s_mutshoot3};
//statetype s_mutshoot3           = {false,SPR_MUT_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_mutshoot4};
//statetype s_mutshoot4           = {false,SPR_MUT_SHOOT4,20,NULL,NULL,&s_mutchase1};

pub const s_mutshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_SHOOT1 as i32,
    tictime: 6,
    think: false,
    action: true,
    id: 94,
};
pub const s_mutshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_SHOOT2 as i32,
    tictime: 20,
    think: false,
    action: false,
    id: 95,
};
pub const s_mutshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 96,
};
pub const s_mutshoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_SHOOT4 as i32,
    tictime: 20,
    think: false,
    action: false,
    id: 97,
};

//statetype s_mutchase1           = {true,SPR_MUT_W1_1,10,(statefunc)T_Chase,NULL,&s_mutchase1s};
//statetype s_mutchase1s          = {true,SPR_MUT_W1_1,3,NULL,NULL,&s_mutchase2};
//statetype s_mutchase2           = {true,SPR_MUT_W2_1,8,(statefunc)T_Chase,NULL,&s_mutchase3};
//statetype s_mutchase3           = {true,SPR_MUT_W3_1,10,(statefunc)T_Chase,NULL,&s_mutchase3s};
//statetype s_mutchase3s          = {true,SPR_MUT_W3_1,3,NULL,NULL,&s_mutchase4};
//statetype s_mutchase4           = {true,SPR_MUT_W4_1,8,(statefunc)T_Chase,NULL,&s_mutchase1};

pub const s_mutchase1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W1_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 98,
};
pub const s_mutchase1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W1_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 99,
};
pub const s_mutchase2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W2_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 100,
};
pub const s_mutchase3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W3_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 101,
};
pub const s_mutchase3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W3_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 102,
};
pub const s_mutchase4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_MUT_W4_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 103,
};

//statetype s_mutdie1             = {false,SPR_MUT_DIE_1,7,NULL,(statefunc)A_DeathScream,&s_mutdie2};
//statetype s_mutdie2             = {false,SPR_MUT_DIE_2,7,NULL,NULL,&s_mutdie3};
//statetype s_mutdie3             = {false,SPR_MUT_DIE_3,7,NULL,NULL,&s_mutdie4};
//statetype s_mutdie4             = {false,SPR_MUT_DIE_4,7,NULL,NULL,&s_mutdie5};
//statetype s_mutdie5             = {false,SPR_MUT_DEAD,0,NULL,NULL,&s_mutdie5};

pub const s_mutdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_DIE_1 as i32,
    tictime: 7,
    think: false,
    action: true,
    id: 104,
};
pub const s_mutdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_DIE_2 as i32,
    tictime: 7,
    think: false,
    action: false,
    id: 105,
};
pub const s_mutdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_DIE_3 as i32,
    tictime: 7,
    think: false,
    action: false,
    id: 106,
};
pub const s_mutdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_DIE_4 as i32,
    tictime: 7,
    think: false,
    action: false,
    id: 107,
};
pub const s_mutdie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MUT_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 108,
};
//
// SS
//

//statetype s_ssstand             = {true,SPR_SS_S_1,0,(statefunc)T_Stand,NULL,&s_ssstand};

pub const s_ssstand: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_S_1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 109,
};

//statetype s_sspath1             = {true,SPR_SS_W1_1,20,(statefunc)T_Path,NULL,&s_sspath1s};
//statetype s_sspath1s            = {true,SPR_SS_W1_1,5,NULL,NULL,&s_sspath2};
//statetype s_sspath2             = {true,SPR_SS_W2_1,15,(statefunc)T_Path,NULL,&s_sspath3};
//statetype s_sspath3             = {true,SPR_SS_W3_1,20,(statefunc)T_Path,NULL,&s_sspath3s};
//statetype s_sspath3s            = {true,SPR_SS_W3_1,5,NULL,NULL,&s_sspath4};
//statetype s_sspath4             = {true,SPR_SS_W4_1,15,(statefunc)T_Path,NULL,&s_sspath1};

pub const s_sspath1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W1_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 110,
};
pub const s_sspath1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W1_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 111,
};
pub const s_sspath2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W2_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 112,
};
pub const s_sspath3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W3_1 as i32,
    tictime: 20,
    think: true,
    action: false,
    id: 113,
};
pub const s_sspath3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W3_1 as i32,
    tictime: 5,
    think: false,
    action: false,
    id: 114,
};
pub const s_sspath4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_GRD_W4_1 as i32,
    tictime: 15,
    think: true,
    action: false,
    id: 115,
};

//statetype s_sspain              = {2,SPR_SS_PAIN_1,10,NULL,NULL,&s_sschase1};
//statetype s_sspain1             = {2,SPR_SS_PAIN_2,10,NULL,NULL,&s_sschase1};

pub const s_sspain: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_SS_PAIN_1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 116,
};
pub const s_sspain1: statetype = statetype {
    rotate: 2,
    shapenum: SPRITES::SPR_SS_PAIN_2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 117,
};

//statetype s_ssshoot1            = {false,SPR_SS_SHOOT1,20,NULL,NULL,&s_ssshoot2};
//statetype s_ssshoot2            = {false,SPR_SS_SHOOT2,20,NULL,(statefunc)T_Shoot,&s_ssshoot3};
//statetype s_ssshoot3            = {false,SPR_SS_SHOOT3,10,NULL,NULL,&s_ssshoot4};
//statetype s_ssshoot4            = {false,SPR_SS_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_ssshoot5};
//statetype s_ssshoot5            = {false,SPR_SS_SHOOT3,10,NULL,NULL,&s_ssshoot6};
//statetype s_ssshoot6            = {false,SPR_SS_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_ssshoot7};
//statetype s_ssshoot7            = {false,SPR_SS_SHOOT3,10,NULL,NULL,&s_ssshoot8};
//statetype s_ssshoot8            = {false,SPR_SS_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_ssshoot9};
//statetype s_ssshoot9            = {false,SPR_SS_SHOOT3,10,NULL,NULL,&s_sschase1};

pub const s_ssshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT1 as i32,
    tictime: 20,
    think: false,
    action: false,
    id: 118,
};
pub const s_ssshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT2 as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 119,
};
pub const s_ssshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 120,
};
pub const s_ssshoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 121,
};
pub const s_ssshoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 122,
};
pub const s_ssshoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 123,
};
pub const s_ssshoot7: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 124,
};
pub const s_ssshoot8: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 125,
};
pub const s_ssshoot9: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 126,
};

//statetype s_sschase1            = {true,SPR_SS_W1_1,10,(statefunc)T_Chase,NULL,&s_sschase1s};
//statetype s_sschase1s           = {true,SPR_SS_W1_1,3,NULL,NULL,&s_sschase2};
//statetype s_sschase2            = {true,SPR_SS_W2_1,8,(statefunc)T_Chase,NULL,&s_sschase3};
//statetype s_sschase3            = {true,SPR_SS_W3_1,10,(statefunc)T_Chase,NULL,&s_sschase3s};
//statetype s_sschase3s           = {true,SPR_SS_W3_1,3,NULL,NULL,&s_sschase4};
//statetype s_sschase4            = {true,SPR_SS_W4_1,8,(statefunc)T_Chase,NULL,&s_sschase1};

pub const s_sschase1: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_W1_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 127,
};
pub const s_sschase1s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_W1_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 128,
};
pub const s_sschase2: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_W2_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 129,
};
pub const s_sschase3: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_W3_1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 130,
};
pub const s_sschase3s: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_W3_1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 131,
};
pub const s_sschase4: statetype = statetype {
    rotate: 1,
    shapenum: SPRITES::SPR_SS_W4_1 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 132,
};

//statetype s_ssdie1              = {false,SPR_SS_DIE_1,15,NULL,(statefunc)A_DeathScream,&s_ssdie2};
//statetype s_ssdie2              = {false,SPR_SS_DIE_2,15,NULL,NULL,&s_ssdie3};
//statetype s_ssdie3              = {false,SPR_SS_DIE_3,15,NULL,NULL,&s_ssdie4};
//statetype s_ssdie4              = {false,SPR_SS_DEAD,0,NULL,NULL,&s_ssdie4};

pub const s_ssdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_DIE_1 as i32,
    tictime: 15,
    think: false,
    action: true,
    id: 133,
};
pub const s_ssdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_DIE_2 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 134,
};
pub const s_ssdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_DIE_3 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 135,
};
pub const s_ssdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SS_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 136,
};

//
// hans - EPISODE 1 BOSS
//

//statetype s_bossstand           = {false,SPR_BOSS_W1,0,(statefunc)T_Stand,NULL,&s_bossstand};

pub const s_bossstand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 137,
};

//statetype s_bosschase1          = {false,SPR_BOSS_W1,10,(statefunc)T_Chase,NULL,&s_bosschase1s};
//statetype s_bosschase1s         = {false,SPR_BOSS_W1,3,NULL,NULL,&s_bosschase2};
//statetype s_bosschase2          = {false,SPR_BOSS_W2,8,(statefunc)T_Chase,NULL,&s_bosschase3};
//statetype s_bosschase3          = {false,SPR_BOSS_W3,10,(statefunc)T_Chase,NULL,&s_bosschase3s};
//statetype s_bosschase3s         = {false,SPR_BOSS_W3,3,NULL,NULL,&s_bosschase4};
//statetype s_bosschase4          = {false,SPR_BOSS_W4,8,(statefunc)T_Chase,NULL,&s_bosschase1};

pub const s_bosschase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 138,
};
pub const s_bosschase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 139,
};
pub const s_bosschase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 140,
};
pub const s_bosschase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W3 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 141,
};
pub const s_bosschase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 142,
};
pub const s_bosschase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 143,
};

//statetype s_bossdie1            = {false,SPR_BOSS_DIE1,15,NULL,(statefunc)A_DeathScream,&s_bossdie2};
//statetype s_bossdie2            = {false,SPR_BOSS_DIE2,15,NULL,NULL,&s_bossdie3};
//statetype s_bossdie3            = {false,SPR_BOSS_DIE3,15,NULL,NULL,&s_bossdie4};
//statetype s_bossdie4            = {false,SPR_BOSS_DEAD,0,NULL,NULL,&s_bossdie4};

pub const s_bossdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_DIE1 as i32,
    tictime: 15,
    think: false,
    action: true,
    id: 144,
};
pub const s_bossdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_DIE2 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 145,
};
pub const s_bossdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_DIE3 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 146,
};
pub const s_bossdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 147,
};

//statetype s_bossshoot1          = {false,SPR_BOSS_SHOOT1,30,NULL,NULL,&s_bossshoot2};
//statetype s_bossshoot2          = {false,SPR_BOSS_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_bossshoot3};
//statetype s_bossshoot3          = {false,SPR_BOSS_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_bossshoot4};
//statetype s_bossshoot4          = {false,SPR_BOSS_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_bossshoot5};
//statetype s_bossshoot5          = {false,SPR_BOSS_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_bossshoot6};
//statetype s_bossshoot6          = {false,SPR_BOSS_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_bossshoot7};
//statetype s_bossshoot7          = {false,SPR_BOSS_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_bossshoot8};
//statetype s_bossshoot8          = {false,SPR_BOSS_SHOOT1,10,NULL,NULL,&s_bosschase1};

pub const s_bossshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 148,
};
pub const s_bossshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 149,
};
pub const s_bossshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 150,
};
pub const s_bossshoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 151,
};
pub const s_bossshoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 152,
};
pub const s_bossshoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 153,
};
pub const s_bossshoot7: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 154,
};
pub const s_bossshoot8: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BOSS_SHOOT1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 155,
};

//
// gretel - EPISODE 5 BOSS
//

//statetype s_gretelstand         = {false,SPR_GRETEL_W1,0,(statefunc)T_Stand,NULL,&s_gretelstand};

pub const s_gretelstand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 156,
};

//statetype s_gretelchase1        = {false,SPR_GRETEL_W1,10,(statefunc)T_Chase,NULL,&s_gretelchase1s};
//statetype s_gretelchase1s       = {false,SPR_GRETEL_W1,3,NULL,NULL,&s_gretelchase2};
//statetype s_gretelchase2        = {false,SPR_GRETEL_W2,8,(statefunc)T_Chase,NULL,&s_gretelchase3};
//statetype s_gretelchase3        = {false,SPR_GRETEL_W3,10,(statefunc)T_Chase,NULL,&s_gretelchase3s};
//statetype s_gretelchase3s       = {false,SPR_GRETEL_W3,3,NULL,NULL,&s_gretelchase4};
//statetype s_gretelchase4        = {false,SPR_GRETEL_W4,8,(statefunc)T_Chase,NULL,&s_gretelchase1};

pub const s_gretelchase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 157,
};
pub const s_gretelchase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 158,
};
pub const s_gretelchase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 159,
};
pub const s_gretelchase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W3 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 160,
};
pub const s_gretelchase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 161,
};
pub const s_gretelchase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 162,
};

//statetype s_greteldie1          = {false,SPR_GRETEL_DIE1,15,NULL,(statefunc)A_DeathScream,&s_greteldie2};
//statetype s_greteldie2          = {false,SPR_GRETEL_DIE2,15,NULL,NULL,&s_greteldie3};
//statetype s_greteldie3          = {false,SPR_GRETEL_DIE3,15,NULL,NULL,&s_greteldie4};
//statetype s_greteldie4          = {false,SPR_GRETEL_DEAD,0,NULL,NULL,&s_greteldie4};

pub const s_greteldie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_DIE1 as i32,
    tictime: 15,
    think: false,
    action: true,
    id: 163,
};
pub const s_greteldie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_DIE2 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 164,
};
pub const s_greteldie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_DIE3 as i32,
    tictime: 15,
    think: false,
    action: false,
    id: 165,
};
pub const s_greteldie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 166,
};

//statetype s_gretelshoot1        = {false,SPR_GRETEL_SHOOT1,30,NULL,NULL,&s_gretelshoot2};
//statetype s_gretelshoot2        = {false,SPR_GRETEL_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_gretelshoot3};
//statetype s_gretelshoot3        = {false,SPR_GRETEL_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_gretelshoot4};
//statetype s_gretelshoot4        = {false,SPR_GRETEL_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_gretelshoot5};
//statetype s_gretelshoot5        = {false,SPR_GRETEL_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_gretelshoot6};
//statetype s_gretelshoot6        = {false,SPR_GRETEL_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_gretelshoot7};
//statetype s_gretelshoot7        = {false,SPR_GRETEL_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_gretelshoot8};
//statetype s_gretelshoot8        = {false,SPR_GRETEL_SHOOT1,10,NULL,NULL,&s_gretelchase1};

pub const s_gretelshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 167,
};
pub const s_gretelshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 168,
};
pub const s_gretelshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 169,
};
pub const s_gretelshoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 170,
};
pub const s_gretelshoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 171,
};
pub const s_gretelshoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 172,
};
pub const s_gretelshoot7: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 173,
};
pub const s_gretelshoot8: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GRETEL_SHOOT1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 174,
};

//
// schabb - EPISODE 2 BOSS
//

//statetype s_schabbstand         = {false,SPR_SCHABB_W1,0,(statefunc)T_Stand,NULL,&s_schabbstand};

pub const s_schabbstand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 175,
};

//statetype s_schabbchase1        = {false,SPR_SCHABB_W1,10,(statefunc)T_Schabb,NULL,&s_schabbchase1s};
//statetype s_schabbchase1s       = {false,SPR_SCHABB_W1,3,NULL,NULL,&s_schabbchase2};
//statetype s_schabbchase2        = {false,SPR_SCHABB_W2,8,(statefunc)T_Schabb,NULL,&s_schabbchase3};
//statetype s_schabbchase3        = {false,SPR_SCHABB_W3,10,(statefunc)T_Schabb,NULL,&s_schabbchase3s};
//statetype s_schabbchase3s       = {false,SPR_SCHABB_W3,3,NULL,NULL,&s_schabbchase4};
//statetype s_schabbchase4        = {false,SPR_SCHABB_W4,8,(statefunc)T_Schabb,NULL,&s_schabbchase1};

pub const s_schabbchase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 176,
};
pub const s_schabbchase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 177,
};
pub const s_schabbchase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 178,
};
pub const s_schabbchase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W3 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 179,
};
pub const s_schabbchase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 180,
};
pub const s_schabbchase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 181,
};

//statetype s_schabbdeathcam      = {false,SPR_SCHABB_W1,1,NULL,NULL,&s_schabbdie1};

pub const s_schabbdeathcam: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W1 as i32,
    tictime: 1,
    think: false,
    action: false,
    id: 182,
};

//statetype s_schabbdie1          = {false,SPR_SCHABB_W1,10,NULL,(statefunc)A_DeathScream,&s_schabbdie2};
//statetype s_schabbdie2          = {false,SPR_SCHABB_W1,10,NULL,NULL,&s_schabbdie3};
//statetype s_schabbdie3          = {false,SPR_SCHABB_DIE1,10,NULL,NULL,&s_schabbdie4};
//statetype s_schabbdie4          = {false,SPR_SCHABB_DIE2,10,NULL,NULL,&s_schabbdie5};
//statetype s_schabbdie5          = {false,SPR_SCHABB_DIE3,10,NULL,NULL,&s_schabbdie6};
//statetype s_schabbdie6          = {false,SPR_SCHABB_DEAD,20,NULL,(statefunc)A_StartDeathCam,&s_schabbdie6};

pub const s_schabbdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W1 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 183,
};
pub const s_schabbdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_W1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 184,
};
pub const s_schabbdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_DIE1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 185,
};
pub const s_schabbdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_DIE2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 186,
};
pub const s_schabbdie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_DIE3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 187,
};
pub const s_schabbdie6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_DEAD as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 188,
};

//statetype s_schabbshoot1        = {false,SPR_SCHABB_SHOOT1,30,NULL,NULL,&s_schabbshoot2};
//statetype s_schabbshoot2        = {false,SPR_SCHABB_SHOOT2,10,NULL,(statefunc)T_SchabbThrow,&s_schabbchase1};

pub const s_schabbshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 189,
};
pub const s_schabbshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_SCHABB_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 190,
};

//statetype s_needle1             = {false,SPR_HYPO1,6,(statefunc)T_Projectile,NULL,&s_needle2};
//statetype s_needle2             = {false,SPR_HYPO2,6,(statefunc)T_Projectile,NULL,&s_needle3};
//statetype s_needle3             = {false,SPR_HYPO3,6,(statefunc)T_Projectile,NULL,&s_needle4};
//statetype s_needle4             = {false,SPR_HYPO4,6,(statefunc)T_Projectile,NULL,&s_needle1};

pub const s_needle1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HYPO1 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 191,
};
pub const s_needle2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HYPO2 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 192,
};
pub const s_needle3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HYPO3 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 193,
};
pub const s_needle4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HYPO4 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 194,
};

//
// gift - EPISODE 4 BOSS
//

//statetype s_giftstand           = {false,SPR_GIFT_W1,0,(statefunc)T_Stand,NULL,&s_giftstand};

pub const s_giftstand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 195,
};

//statetype s_giftchase1          = {false,SPR_GIFT_W1,10,(statefunc)T_Gift,NULL,&s_giftchase1s};
//statetype s_giftchase1s         = {false,SPR_GIFT_W1,3,NULL,NULL,&s_giftchase2};
//statetype s_giftchase2          = {false,SPR_GIFT_W2,8,(statefunc)T_Gift,NULL,&s_giftchase3};
//statetype s_giftchase3          = {false,SPR_GIFT_W3,10,(statefunc)T_Gift,NULL,&s_giftchase3s};
//statetype s_giftchase3s         = {false,SPR_GIFT_W3,3,NULL,NULL,&s_giftchase4};
//statetype s_giftchase4          = {false,SPR_GIFT_W4,8,(statefunc)T_Gift,NULL,&s_giftchase1};

pub const s_giftchase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 196,
};
pub const s_giftchase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 197,
};
pub const s_giftchase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 198,
};
pub const s_giftchase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W3 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 199,
};
pub const s_giftchase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 200,
};
pub const s_giftchase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W4 as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 201,
};

//statetype s_giftdeathcam        = {false,SPR_GIFT_W1,1,NULL,NULL,&s_giftdie1};

pub const s_giftdeathcam: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W1 as i32,
    tictime: 1,
    think: false,
    action: false,
    id: 202,
};

//statetype s_giftdie1            = {false,SPR_GIFT_W1,1,NULL,(statefunc)A_DeathScream,&s_giftdie2};
//statetype s_giftdie2            = {false,SPR_GIFT_W1,10,NULL,NULL,&s_giftdie3};
//statetype s_giftdie3            = {false,SPR_GIFT_DIE1,10,NULL,NULL,&s_giftdie4};
//statetype s_giftdie4            = {false,SPR_GIFT_DIE2,10,NULL,NULL,&s_giftdie5};
//statetype s_giftdie5            = {false,SPR_GIFT_DIE3,10,NULL,NULL,&s_giftdie6};
//statetype s_giftdie6            = {false,SPR_GIFT_DEAD,20,NULL,(statefunc)A_StartDeathCam,&s_giftdie6};

pub const s_giftdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W1 as i32,
    tictime: 1,
    think: false,
    action: true,
    id: 203,
};
pub const s_giftdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_W1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 204,
};
pub const s_giftdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_DIE1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 205,
};
pub const s_giftdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_DIE2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 206,
};
pub const s_giftdie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_DIE3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 207,
};
pub const s_giftdie6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_DEAD as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 208,
};

//statetype s_giftshoot1          = {false,SPR_GIFT_SHOOT1,30,NULL,NULL,&s_giftshoot2};
//statetype s_giftshoot2          = {false,SPR_GIFT_SHOOT2,10,NULL,(statefunc)T_GiftThrow,&s_giftchase1};

pub const s_giftshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 209,
};
pub const s_giftshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_GIFT_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 210,
};

//
// fat - EPISODE 6 BOSS
//

//statetype s_fatstand            = {false,SPR_FAT_W1,0,(statefunc)T_Stand,NULL,&s_fatstand};

pub const s_fatstand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 211,
};

//statetype s_fatchase1           = {false,SPR_FAT_W1,10,(statefunc)T_Fat,NULL,&s_fatchase1s};
//statetype s_fatchase1s          = {false,SPR_FAT_W1,3,NULL,NULL,&s_fatchase2};
//statetype s_fatchase2           = {false,SPR_FAT_W2,8,(statefunc)T_Fat,NULL,&s_fatchase3};
//statetype s_fatchase3           = {false,SPR_FAT_W3,10,(statefunc)T_Fat,NULL,&s_fatchase3s};
//statetype s_fatchase3s          = {false,SPR_FAT_W3,3,NULL,NULL,&s_fatchase4};
//statetype s_fatchase4           = {false,SPR_FAT_W4,8,(statefunc)T_Fat,NULL,&s_fatchase1};

pub const s_fatchase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 212,
};
pub const s_fatchase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 213,
};
pub const s_fatchase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 214,
};
pub const s_fatchase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W3 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 215,
};
pub const s_fatchase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 216,
};
pub const s_fatchase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 217,
};

//statetype s_fatdeathcam         = {false,SPR_FAT_W1,1,NULL,NULL,&s_fatdie1};

pub const s_fatdeathcam: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W1 as i32,
    tictime: 1,
    think: false,
    action: false,
    id: 218,
};

//statetype s_fatdie1             = {false,SPR_FAT_W1,1,NULL,(statefunc)A_DeathScream,&s_fatdie2};
//statetype s_fatdie2             = {false,SPR_FAT_W1,10,NULL,NULL,&s_fatdie3};
//statetype s_fatdie3             = {false,SPR_FAT_DIE1,10,NULL,NULL,&s_fatdie4};
//statetype s_fatdie4             = {false,SPR_FAT_DIE2,10,NULL,NULL,&s_fatdie5};
//statetype s_fatdie5             = {false,SPR_FAT_DIE3,10,NULL,NULL,&s_fatdie6};
//statetype s_fatdie6             = {false,SPR_FAT_DEAD,20,NULL,(statefunc)A_StartDeathCam,&s_fatdie6};

pub const s_fatdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W1 as i32,
    tictime: 1,
    think: false,
    action: true,
    id: 219,
};
pub const s_fatdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_W1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 220,
};
pub const s_fatdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_DIE1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 221,
};
pub const s_fatdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_DIE2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 222,
};
pub const s_fatdie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_DIE3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 223,
};
pub const s_fatdie6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_DEAD as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 224,
};

//statetype s_fatshoot1           = {false,SPR_FAT_SHOOT1,30,NULL,NULL,&s_fatshoot2};
//statetype s_fatshoot2           = {false,SPR_FAT_SHOOT2,10,NULL,(statefunc)T_GiftThrow,&s_fatshoot3};
//statetype s_fatshoot3           = {false,SPR_FAT_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_fatshoot4};
//statetype s_fatshoot4           = {false,SPR_FAT_SHOOT4,10,NULL,(statefunc)T_Shoot,&s_fatshoot5};
//statetype s_fatshoot5           = {false,SPR_FAT_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_fatshoot6};
//statetype s_fatshoot6           = {false,SPR_FAT_SHOOT4,10,NULL,(statefunc)T_Shoot,&s_fatchase1};

pub const s_fatshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 225,
};
pub const s_fatshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 226,
};
pub const s_fatshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 227,
};
pub const s_fatshoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_SHOOT4 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 228,
};
pub const s_fatshoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 229,
};
pub const s_fatshoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAT_SHOOT4 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 230,
};

//
// fake - EPISODE 3 BOSS1
//

//statetype s_fakestand           = {false,SPR_FAKE_W1,0,(statefunc)T_Stand,NULL,&s_fakestand};

pub const s_fakestand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 231,
};

//statetype s_fakechase1          = {false,SPR_FAKE_W1,10,(statefunc)T_Fake,NULL,&s_fakechase1s};
//statetype s_fakechase1s         = {false,SPR_FAKE_W1,3,NULL,NULL,&s_fakechase2};
//statetype s_fakechase2          = {false,SPR_FAKE_W2,8,(statefunc)T_Fake,NULL,&s_fakechase3};
//statetype s_fakechase3          = {false,SPR_FAKE_W3,10,(statefunc)T_Fake,NULL,&s_fakechase3s};
//statetype s_fakechase3s         = {false,SPR_FAKE_W3,3,NULL,NULL,&s_fakechase4};
//statetype s_fakechase4          = {false,SPR_FAKE_W4,8,(statefunc)T_Fake,NULL,&s_fakechase1};

pub const s_fakechase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W1 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 232,
};
pub const s_fakechase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 233,
};
pub const s_fakechase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 234,
};
pub const s_fakechase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W3 as i32,
    tictime: 10,
    think: true,
    action: false,
    id: 235,
};
pub const s_fakechase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 236,
};
pub const s_fakechase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 237,
};

//statetype s_fakedie1            = {false,SPR_FAKE_DIE1,10,NULL,(statefunc)A_DeathScream,&s_fakedie2};
//statetype s_fakedie2            = {false,SPR_FAKE_DIE2,10,NULL,NULL,&s_fakedie3};
//statetype s_fakedie3            = {false,SPR_FAKE_DIE3,10,NULL,NULL,&s_fakedie4};
//statetype s_fakedie4            = {false,SPR_FAKE_DIE4,10,NULL,NULL,&s_fakedie5};
//statetype s_fakedie5            = {false,SPR_FAKE_DIE5,10,NULL,NULL,&s_fakedie6};
//statetype s_fakedie6            = {false,SPR_FAKE_DEAD,0,NULL,NULL,&s_fakedie6};

pub const s_fakedie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_DIE1 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 238,
};
pub const s_fakedie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_DIE2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 239,
};

pub const s_fakedie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_DIE3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 240,
};

pub const s_fakedie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_DIE4 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 241,
};

pub const s_fakedie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_DIE5 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 242,
};

pub const s_fakedie6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 243,
};

//statetype s_fakeshoot1          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot2};
//statetype s_fakeshoot2          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot3};
//statetype s_fakeshoot3          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot4};
//statetype s_fakeshoot4          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot5};
//statetype s_fakeshoot5          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot6};
//statetype s_fakeshoot6          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot7};
//statetype s_fakeshoot7          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot8};
//statetype s_fakeshoot8          = {false,SPR_FAKE_SHOOT,8,NULL,(statefunc)T_FakeFire,&s_fakeshoot9};
//statetype s_fakeshoot9          = {false,SPR_FAKE_SHOOT,8,NULL,NULL,&s_fakechase1};

pub const s_fakeshoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 244,
};
pub const s_fakeshoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 245,
};
pub const s_fakeshoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 246,
};
pub const s_fakeshoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 247,
};
pub const s_fakeshoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 248,
};
pub const s_fakeshoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 249,
};
pub const s_fakeshoot7: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 250,
};
pub const s_fakeshoot8: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: true,
    id: 251,
};
pub const s_fakeshoot9: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FAKE_SHOOT as i32,
    tictime: 8,
    think: false,
    action: false,
    id: 252,
};

//statetype s_fire1               = {false,SPR_FIRE1,6,(statefunc)T_Projectile,NULL,&s_fire2};
//statetype s_fire2               = {false,SPR_FIRE2,6,(statefunc)T_Projectile,NULL,&s_fire1};

pub const s_fire1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FIRE1 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 253,
};
pub const s_fire2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_FIRE2 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 254,
};

//
// hitler - EPISODE 3 BOSS2
//

//statetype s_mechastand          = {false,SPR_MECHA_W1,0,(statefunc)T_Stand,NULL,&s_mechastand};

pub const s_mechastand: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W1 as i32,
    tictime: 0,
    think: true,
    action: false,
    id: 255,
};

//statetype s_mechachase1         = {false,SPR_MECHA_W1,10,(statefunc)T_Chase,(statefunc)A_MechaSound,&s_mechachase1s};
//statetype s_mechachase1s        = {false,SPR_MECHA_W1,6,NULL,NULL,&s_mechachase2};
//statetype s_mechachase2         = {false,SPR_MECHA_W2,8,(statefunc)T_Chase,NULL,&s_mechachase3};
//statetype s_mechachase3         = {false,SPR_MECHA_W3,10,(statefunc)T_Chase,(statefunc)A_MechaSound,&s_mechachase3s};
//statetype s_mechachase3s        = {false,SPR_MECHA_W3,6,NULL,NULL,&s_mechachase4};
//statetype s_mechachase4         = {false,SPR_MECHA_W4,8,(statefunc)T_Chase,NULL,&s_mechachase1};

pub const s_mechachase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W1 as i32,
    tictime: 10,
    think: true,
    action: true,
    id: 256,
};
pub const s_mechachase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W1 as i32,
    tictime: 6,
    think: false,
    action: false,
    id: 257,
};
pub const s_mechachase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 258,
};
pub const s_mechachase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W3 as i32,
    tictime: 10,
    think: true,
    action: true,
    id: 259,
};
pub const s_mechachase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W3 as i32,
    tictime: 6,
    think: false,
    action: false,
    id: 260,
};
pub const s_mechachase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 261,
};

//statetype s_mechadie1           = {false,SPR_MECHA_DIE1,10,NULL,(statefunc)A_DeathScream,&s_mechadie2};
//statetype s_mechadie2           = {false,SPR_MECHA_DIE2,10,NULL,NULL,&s_mechadie3};
//statetype s_mechadie3           = {false,SPR_MECHA_DIE3,10,NULL,(statefunc)A_HitlerMorph,&s_mechadie4};
//statetype s_mechadie4           = {false,SPR_MECHA_DEAD,0,NULL,NULL,&s_mechadie4};

pub const s_mechadie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_DIE1 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 262,
};
pub const s_mechadie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_DIE2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 263,
};
pub const s_mechadie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_DIE3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 264,
};
pub const s_mechadie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_DEAD as i32,
    tictime: 0,
    think: false,
    action: false,
    id: 265,
};

//statetype s_mechashoot1         = {false,SPR_MECHA_SHOOT1,30,NULL,NULL,&s_mechashoot2};
//statetype s_mechashoot2         = {false,SPR_MECHA_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_mechashoot3};
//statetype s_mechashoot3         = {false,SPR_MECHA_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_mechashoot4};
//statetype s_mechashoot4         = {false,SPR_MECHA_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_mechashoot5};
//statetype s_mechashoot5         = {false,SPR_MECHA_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_mechashoot6};
//statetype s_mechashoot6         = {false,SPR_MECHA_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_mechachase1};

pub const s_mechashoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 266,
};
pub const s_mechashoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 267,
};
pub const s_mechashoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 268,
};
pub const s_mechashoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 269,
};
pub const s_mechashoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 270,
};
pub const s_mechashoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_MECHA_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 271,
};

//
// hitler - EPISODE 3 BOSS3
//

//statetype s_hitlerchase1        = {false,SPR_HITLER_W1,6,(statefunc)T_Chase,NULL,&s_hitlerchase1s};
//statetype s_hitlerchase1s       = {false,SPR_HITLER_W1,4,NULL,NULL,&s_hitlerchase2};
//statetype s_hitlerchase2        = {false,SPR_HITLER_W2,2,(statefunc)T_Chase,NULL,&s_hitlerchase3};
//statetype s_hitlerchase3        = {false,SPR_HITLER_W3,6,(statefunc)T_Chase,NULL,&s_hitlerchase3s};
//statetype s_hitlerchase3s       = {false,SPR_HITLER_W3,4,NULL,NULL,&s_hitlerchase4};
//statetype s_hitlerchase4        = {false,SPR_HITLER_W4,2,(statefunc)T_Chase,NULL,&s_hitlerchase1};

pub const s_hitlerchase1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W1 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 272,
};
pub const s_hitlerchase1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W1 as i32,
    tictime: 4,
    think: false,
    action: false,
    id: 273,
};
pub const s_hitlerchase2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W2 as i32,
    tictime: 2,
    think: true,
    action: false,
    id: 274,
};
pub const s_hitlerchase3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W3 as i32,
    tictime: 6,
    think: true,
    action: false,
    id: 275,
};
pub const s_hitlerchase3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W3 as i32,
    tictime: 4,
    think: false,
    action: false,
    id: 276,
};
pub const s_hitlerchase4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W4 as i32,
    tictime: 2,
    think: true,
    action: false,
    id: 277,
};

//statetype s_hitlerdeathcam      = {false,SPR_HITLER_W1,10,NULL,NULL,&s_hitlerdie1};

pub const s_hitlerdeathcam: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 278,
};

//statetype s_hitlerdie1          = {false,SPR_HITLER_W1,1,NULL,(statefunc)A_DeathScream,&s_hitlerdie2};
//statetype s_hitlerdie2          = {false,SPR_HITLER_W1,10,NULL,NULL,&s_hitlerdie3};
//statetype s_hitlerdie3          = {false,SPR_HITLER_DIE1,10,NULL,(statefunc)A_Slurpie,&s_hitlerdie4};
//statetype s_hitlerdie4          = {false,SPR_HITLER_DIE2,10,NULL,NULL,&s_hitlerdie5};
//statetype s_hitlerdie5          = {false,SPR_HITLER_DIE3,10,NULL,NULL,&s_hitlerdie6};
//statetype s_hitlerdie6          = {false,SPR_HITLER_DIE4,10,NULL,NULL,&s_hitlerdie7};
//statetype s_hitlerdie7          = {false,SPR_HITLER_DIE5,10,NULL,NULL,&s_hitlerdie8};
//statetype s_hitlerdie8          = {false,SPR_HITLER_DIE6,10,NULL,NULL,&s_hitlerdie9};
//statetype s_hitlerdie9          = {false,SPR_HITLER_DIE7,10,NULL,NULL,&s_hitlerdie10};
//statetype s_hitlerdie10         = {false,SPR_HITLER_DEAD,20,NULL,(statefunc)A_StartDeathCam,&s_hitlerdie10};

pub const s_hitlerdie1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W1 as i32,
    tictime: 1,
    think: false,
    action: true,
    id: 279,
};
pub const s_hitlerdie2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_W1 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 280,
};
pub const s_hitlerdie3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE1 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 281,
};
pub const s_hitlerdie4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE2 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 282,
};
pub const s_hitlerdie5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE3 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 283,
};
pub const s_hitlerdie6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE4 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 284,
};
pub const s_hitlerdie7: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE5 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 285,
};
pub const s_hitlerdie8: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE6 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 286,
};
pub const s_hitlerdie9: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DIE7 as i32,
    tictime: 10,
    think: false,
    action: false,
    id: 287,
};
pub const s_hitlerdie10: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_DEAD as i32,
    tictime: 20,
    think: false,
    action: true,
    id: 288,
};

//statetype s_hitlershoot1        = {false,SPR_HITLER_SHOOT1,30,NULL,NULL,&s_hitlershoot2};
//statetype s_hitlershoot2        = {false,SPR_HITLER_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_hitlershoot3};
//statetype s_hitlershoot3        = {false,SPR_HITLER_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_hitlershoot4};
//statetype s_hitlershoot4        = {false,SPR_HITLER_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_hitlershoot5};
//statetype s_hitlershoot5        = {false,SPR_HITLER_SHOOT3,10,NULL,(statefunc)T_Shoot,&s_hitlershoot6};
//statetype s_hitlershoot6        = {false,SPR_HITLER_SHOOT2,10,NULL,(statefunc)T_Shoot,&s_hitlerchase1};

pub const s_hitlershoot1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_SHOOT1 as i32,
    tictime: 30,
    think: false,
    action: false,
    id: 289,
};
pub const s_hitlershoot2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 290,
};
pub const s_hitlershoot3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 291,
};
pub const s_hitlershoot4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 292,
};
pub const s_hitlershoot5: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_SHOOT3 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 293,
};
pub const s_hitlershoot6: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_HITLER_SHOOT2 as i32,
    tictime: 10,
    think: false,
    action: true,
    id: 294,
};

//
// BJ victory
//

//statetype s_bjrun1              = {false,SPR_BJ_W1,12,(statefunc)T_BJRun,NULL,&s_bjrun1s};
//statetype s_bjrun1s             = {false,SPR_BJ_W1,3, NULL,NULL,&s_bjrun2};
//statetype s_bjrun2              = {false,SPR_BJ_W2,8,(statefunc)T_BJRun,NULL,&s_bjrun3};
//statetype s_bjrun3              = {false,SPR_BJ_W3,12,(statefunc)T_BJRun,NULL,&s_bjrun3s};
//statetype s_bjrun3s             = {false,SPR_BJ_W3,3, NULL,NULL,&s_bjrun4};
//statetype s_bjrun4              = {false,SPR_BJ_W4,8,(statefunc)T_BJRun,NULL,&s_bjrun1};

pub const s_bjrun1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_W1 as i32,
    tictime: 12,
    think: true,
    action: false,
    id: 295,
};
pub const s_bjrun1s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_W1 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 296,
};
pub const s_bjrun2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_W2 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 297,
};
pub const s_bjrun3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_W3 as i32,
    tictime: 12,
    think: true,
    action: false,
    id: 298,
};
pub const s_bjrun3s: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_W3 as i32,
    tictime: 3,
    think: false,
    action: false,
    id: 299,
};
pub const s_bjrun4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_W4 as i32,
    tictime: 8,
    think: true,
    action: false,
    id: 300,
};

//statetype s_bjjump1             = {false,SPR_BJ_JUMP1,14,(statefunc)T_BJJump,NULL,&s_bjjump2};
//statetype s_bjjump2             = {false,SPR_BJ_JUMP2,14,(statefunc)T_BJJump,(statefunc)T_BJYell,&s_bjjump3};
//statetype s_bjjump3             = {false,SPR_BJ_JUMP3,14,(statefunc)T_BJJump,NULL,&s_bjjump4};
//statetype s_bjjump4             = {false,SPR_BJ_JUMP4,300,NULL,(statefunc)T_BJDone,&s_bjjump4};

pub const s_bjjump1: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_JUMP1 as i32,
    tictime: 14,
    think: true,
    action: false,
    id: 301,
};
pub const s_bjjump2: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_JUMP2 as i32,
    tictime: 14,
    think: true,
    action: true,
    id: 302,
};
pub const s_bjjump3: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_JUMP3 as i32,
    tictime: 14,
    think: true,
    action: false,
    id: 303,
};
pub const s_bjjump4: statetype = statetype {
    rotate: 0,
    shapenum: SPRITES::SPR_BJ_JUMP4 as i32,
    tictime: 300,
    think: false,
    action: true,
    id: 304,
};

//statetype s_deathcam            = {false,0,0,NULL,NULL,NULL};

pub const s_deathcam: statetype = statetype {
    rotate: 0,
    shapenum: 0,
    tictime: 0,
    think: false,
    action: false,
    id: 305,
};

/*
=================
=
= A_Smoke
=
=================
*/

pub fn A_Smoke(w3d: &mut modules, ob: &mut object) {
    //println!("A_Smoke");

    GetNewActor(ob);

    ob.newobj.state = s_smoke1;
    ob.newobj.ticcount = 6;

    ob.newobj.tilex = ob.objlist[w3d.wl_act2.rocketobj_i].tilex;
    ob.newobj.tiley = ob.objlist[w3d.wl_act2.rocketobj_i].tiley;
    ob.newobj.x = ob.objlist[w3d.wl_act2.rocketobj_i].x;
    ob.newobj.y = ob.objlist[w3d.wl_act2.rocketobj_i].y;
    ob.newobj.obclass = classtype::inertobj;
    ob.newobj.active = activetype::ac_yes;

    ob.newobj.flags = objflag_t::FL_NEVERMARK as i32;

    //
    ob.objlist.push(ob.newobj);
}

/*
===================
=
= ProjectileTryMove
=
= returns true if move ok
===================
*/

pub fn ProjectileTryMove(_w3d: &mut modules, ob: &mut object) -> bool {
    //println!("ProjectileTryMove");

    let xl: i32;
    let yl: i32;
    let xh: i32;
    let yh: i32;
    let mut check: *mut objtype;

    xl = (ob.objlist[ob.objlist_i].x - PROJSIZE) >> TILESHIFT;
    yl = (ob.objlist[ob.objlist_i].y - PROJSIZE) >> TILESHIFT;

    xh = (ob.objlist[ob.objlist_i].x + PROJSIZE) >> TILESHIFT;
    yh = (ob.objlist[ob.objlist_i].y + PROJSIZE) >> TILESHIFT;

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

            //if (check && !ISPOINTER(check)) {
            if !check.is_null() {
                return false;
            }
        }
    }

    return true;
}

/*
=================
=
= T_Projectile
=
=================
*/

pub fn T_Projectile(w3d: &mut modules, ob: &mut object) {
    //println!("T_Projectile");

    let mut deltax: i32;
    let mut deltay: i32;
    let mut damage: i32 = 0;
    let speed: i32;

    speed = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    deltax = FixedMul(
        speed,
        w3d.wl_draw.costable[ob.objlist[ob.objlist_i].angle as usize],
    );
    deltay = -FixedMul(
        speed,
        w3d.wl_draw.sintable[ob.objlist[ob.objlist_i].angle as usize],
    );

    if deltax > 0x10000 {
        deltax = 0x10000;
    }
    if deltay > 0x10000 {
        deltay = 0x10000;
    }

    ob.objlist[ob.objlist_i].x += deltax;
    ob.objlist[ob.objlist_i].y += deltay;

    deltax = LABS(ob.objlist[ob.objlist_i].x - ob.objlist[0].x);
    deltay = LABS(ob.objlist[ob.objlist_i].y - ob.objlist[0].y);

    if !ProjectileTryMove(w3d, ob) {
        if ob.objlist[ob.objlist_i].obclass == classtype::rocketobj {
            PlaySoundLocActor(w3d, ob, soundnames::MISSILEHITSND);
            ob.objlist[ob.objlist_i].state = s_boom1;
        } else {
            // mark for removal
            ob.objlist[ob.objlist_i].state = statetype::new();
        }

        return;
    }

    if deltax < PROJECTILESIZE && deltay < PROJECTILESIZE {
        // hit the player
        match ob.objlist[ob.objlist_i].obclass {
            classtype::needleobj => {
                damage = (US_RndT(w3d) >> 3) + 20;
            }
            classtype::rocketobj | classtype::hrocketobj | classtype::sparkobj => {
                damage = (US_RndT(w3d) >> 3) + 30;
            }
            classtype::fireobj => {
                damage = US_RndT(w3d) >> 3;
            }
            _ => (),
        }

        TakeDamage(w3d, ob, damage);
        // mark for removal
        ob.objlist[ob.objlist_i].state = statetype::new();

        return;
    }

    ob.objlist[ob.objlist_i].tilex = ob.objlist[ob.objlist_i].x >> TILESHIFT;
    ob.objlist[ob.objlist_i].tiley = ob.objlist[ob.objlist_i].y >> TILESHIFT;
}

/*
=============================================================================

GUARD

=============================================================================
*/

/*
===============
=
= SpawnStand
=
===============
*/

pub fn SpawnStand(
    w3d: &mut modules,
    ob: &mut object,
    which: enemy_t,
    tilex: i32,
    tiley: i32,
    dir: i32,
) {
    //println!("SpawnStand");

    let mut map: Vec<u16>;
    let map_i: usize;
    let mut tile: i32;

    match which {
        enemy_t::en_guard => {
            let state = s_grdstand;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_officer => {
            let state = s_ofcstand;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_mutant => {
            let state = s_ofcstand;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_ss => {
            let state = s_ssstand;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }
        _ => (),
    }

    //map = &MAPSPOT(tilex,tiley,0);
    map = w3d.id_ca.mapsegs[0].clone(); //plane 0
    map_i = ((tiley << MAPSHIFT) + tilex) as usize;

    tile = map[map_i] as i32;

    if tile == AMBUSHTILE as i32 {
        w3d.wl_play.tilemap[tilex as usize][tiley as usize] = 0;

        if map[map_i + 1] >= AREATILE as u16 {
            tile = map[map_i + 1] as i32;
        }

        if map[map_i - w3d.wl_play.mapwidth as usize] >= AREATILE as u16 {
            tile = map[map_i - w3d.wl_play.mapwidth as usize] as i32;
        }

        if map[map_i + w3d.wl_play.mapwidth as usize] >= AREATILE as u16 {
            tile = map[map_i + w3d.wl_play.mapwidth as usize] as i32;
        }

        if map[map_i - 1] >= AREATILE as u16 {
            tile = map[map_i - 1] as i32;
        }

        map[map_i] = tile as u16;

        ob.objlist[ob.objlist_i].areanumber = tile - AREATILE;
        ob.objlist[ob.objlist_i].flags |= objflag_t::FL_AMBUSH as i32;
    }

    let bytes: u8 = classtype::guardobj as u8 + which as u8;
    let objclass = classtype::from_u8(bytes);

    ob.objlist[ob.objlist_i].obclass = objclass;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][which as usize];

    let bytes: u8 = dir as u8 * 2;
    let dir = dirtype::from_u8(bytes);

    ob.objlist[ob.objlist_i].dir = dir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32;
}

/*
===============
=
= SpawnDeadGuard
=
===============
*/

pub fn SpawnDeadGuard(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnDeadGuard");

    let state = s_grddie4;
    SpawnNewObj(w3d, ob, tilex, tiley, state);

    if DEMOIF_SDL(w3d) {
        ob.objlist[ob.objlist_i].flags |= objflag_t::FL_NONMARK as i32; // walk through moving enemy fix
    }
    ob.objlist[ob.objlist_i].obclass = classtype::inertobj;
}

/*
===============
=
= SpawnBoss
=
===============
*/

pub fn SpawnBoss(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnBoss");

    let state = s_bossstand;
    SpawnNewObj(w3d, ob, tilex, tiley, state);

    ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

    ob.objlist[ob.objlist_i].obclass = classtype::bossobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_boss as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;

    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
}

/*
===============
=
= SpawnGretel
=
===============
*/

pub fn SpawnGretel(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnGretel");

    SpawnNewObj(w3d, ob, tilex, tiley, s_gretelstand);
    ob.objlist[ob.objlist_i].speed = SPDPATROL;

    ob.objlist[ob.objlist_i].obclass = classtype::gretelobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_gretel as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;
    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
}

/*
===============
=
= SpawnPatrol
=
===============
*/

pub fn SpawnPatrol(
    w3d: &mut modules,
    ob: &mut object,
    which: enemy_t,
    tilex: i32,
    tiley: i32,
    dir: i32,
) {
    //println!("SpawnPatrol");

    match which {
        enemy_t::en_guard => {
            let state = s_grdpath1;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_officer => {
            let state = s_ofcpath1;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_ss => {
            let state = s_sspath1;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_mutant => {
            let state = s_mutpath1;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDPATROL as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }

        enemy_t::en_dog => {
            let state = s_dogpath1;
            SpawnNewObj(w3d, ob, tilex, tiley, state);
            ob.objlist[ob.objlist_i].speed = SPDDOG as i32;

            if !w3d.wl_main.loadedgame {
                w3d.wl_game.gamestate.killtotal += 1;
            }
        }
        _ => (),
    }

    let bytes: u8 = classtype::guardobj as u8 + which as u8;
    let objclass = classtype::from_u8(bytes);
    ob.objlist[ob.objlist_i].obclass = objclass;

    let bytes: u8 = dir as u8;
    let dir = dirtype::from_u8(bytes);

    let bytes: u8 = dir as u8 * 2;
    let newdir = dirtype::from_u8(bytes);
    ob.objlist[ob.objlist_i].dir = newdir;

    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][which as usize];
    ob.objlist[ob.objlist_i].distance = TILEGLOBAL as i32;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32;
    ob.objlist[ob.objlist_i].active = activetype::ac_yes;

    ob.actorat[ob.objlist[ob.objlist_i].tilex as usize][ob.objlist[ob.objlist_i].tiley as usize] =
        ptr::null_mut(); // don't use original spot

    match dir {
        dirtype::east => {
            ob.objlist[ob.objlist_i].tilex += 1;
        }
        dirtype::northeast => {
            ob.objlist[ob.objlist_i].tiley -= 1;
        }
        dirtype::north => {
            ob.objlist[ob.objlist_i].tilex -= 1;
        }
        dirtype::northwest => {
            ob.objlist[ob.objlist_i].tiley += 1;
        }
        _ => (),
    }

    ob.actorat[ob.objlist[ob.objlist_i].tilex as usize][ob.objlist[ob.objlist_i].tiley as usize] =
        &mut ob.objlist[ob.objlist_i] as *mut objtype;
}

/*
==================
=
= A_DeathScream
=
==================
*/

pub fn A_DeathScream(w3d: &mut modules, ob: &mut object) {
    //println!("A_DeathScream");

    let obj = ob.objlist[ob.objlist_i];
    match obj.obclass {
        classtype::mutantobj => {
            PlaySoundLocActor(w3d, ob, soundnames::AHHHGSND);
        }

        classtype::guardobj => {
            let sounds: [soundnames; 8] = [
                soundnames::DEATHSCREAM1SND,
                soundnames::DEATHSCREAM2SND,
                soundnames::DEATHSCREAM3SND,
                soundnames::DEATHSCREAM4SND,
                soundnames::DEATHSCREAM5SND,
                soundnames::DEATHSCREAM7SND,
                soundnames::DEATHSCREAM8SND,
                soundnames::DEATHSCREAM9SND,
            ];
            #[cfg(feature = "UPLOAD")]
            let sound = sounds[(US_RndT(w3d) % 2) as usize];

            #[cfg(feature = "GOODTIMES")]
            let sound = sounds[(US_RndT(w3d) % 8) as usize];

            PlaySoundLocActor(w3d, ob, sound);
        }
        classtype::officerobj => {
            PlaySoundLocActor(w3d, ob, soundnames::NEINSOVASSND);
        }
        classtype::ssobj => {
            PlaySoundLocActor(w3d, ob, soundnames::LEBENSND); // JAB
        }
        classtype::dogobj => {
            PlaySoundLocActor(w3d, ob, soundnames::DOGDEATHSND); // JAB
        }
        classtype::bossobj => {
            SD_PlaySound(w3d, soundnames::MUTTISND); // JAB
        }
        classtype::schabbobj => {
            SD_PlaySound(w3d, soundnames::MEINGOTTSND);
        }
        classtype::fakeobj => {
            SD_PlaySound(w3d, soundnames::HITLERHASND);
        }
        classtype::mechahitlerobj => {
            SD_PlaySound(w3d, soundnames::SCHEISTSND);
        }
        classtype::realhitlerobj => {
            SD_PlaySound(w3d, soundnames::EVASND);
        }
        classtype::gretelobj => {
            SD_PlaySound(w3d, soundnames::MEINSND);
        }
        classtype::giftobj => {
            SD_PlaySound(w3d, soundnames::DONNERSND);
        }
        classtype::fatobj => {
            SD_PlaySound(w3d, soundnames::ROSESND);
        }
        _ => (),
    }
}

/*
=============================================================================

                            SCHABBS / GIFT / FAT

=============================================================================
*/

/*
===============
=
= SpawnGhosts
=
===============
*/

pub fn SpawnGhosts(w3d: &mut modules, ob: &mut object, which: enemy_t, tilex: i32, tiley: i32) {
    //println!("SpawnGhosts");

    match which {
        enemy_t::en_blinky => {
            SpawnNewObj(w3d, ob, tilex, tiley, s_blinkychase1);
        }
        enemy_t::en_clyde => {
            SpawnNewObj(w3d, ob, tilex, tiley, s_clydechase1);
        }
        enemy_t::en_pinky => {
            SpawnNewObj(w3d, ob, tilex, tiley, s_pinkychase1);
        }
        enemy_t::en_inky => {
            SpawnNewObj(w3d, ob, tilex, tiley, s_inkychase1);
        }
        _ => (),
    }

    ob.objlist[ob.objlist_i].obclass = classtype::ghostobj;
    ob.objlist[ob.objlist_i].speed = SPDDOG;

    ob.objlist[ob.objlist_i].dir = dirtype::east;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_AMBUSH as i32;

    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
        w3d.wl_game.gamestate.killcount += 1;
    }
}

/*
===============
=
= SpawnSchabbs
=
===============
*/

pub fn SpawnSchabbs(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnSchabbs");
    //BUG
    if w3d.id_sd.DigiMode != SDSMode::sds_Off {
        //s_schabbdie2.tictime = 140;
    } else {
        //s_schabbdie2.tictime = 5;
    }

    SpawnNewObj(w3d, ob, tilex, tiley, s_schabbstand);
    ob.objlist[ob.objlist_i].speed = SPDPATROL;

    ob.objlist[ob.objlist_i].obclass = classtype::schabbobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_schabbs as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;
    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
}

/*
===============
=
= SpawnGift
=
===============
*/

pub fn SpawnGift(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnGift");
    //BUG
    if w3d.id_sd.DigiMode != SDSMode::sds_Off {
        //s_giftdie2.tictime = 140;
    } else {
        //s_giftdie2.tictime = 5;
    }

    SpawnNewObj(w3d, ob, tilex, tiley, s_giftstand);
    ob.objlist[ob.objlist_i].speed = SPDPATROL;

    ob.objlist[ob.objlist_i].obclass = classtype::giftobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_gift as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;
    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
}

/*
===============
=
= SpawnFat
=
===============
*/

pub fn SpawnFat(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnFat");
    //BUG
    if w3d.id_sd.DigiMode != SDSMode::sds_Off {
        //s_fatdie2.tictime = 140;
    } else {
        //s_fatdie2.tictime = 5;
    }

    SpawnNewObj(w3d, ob, tilex, tiley, s_fatstand);
    ob.objlist[ob.objlist_i].speed = SPDPATROL;

    ob.objlist[ob.objlist_i].obclass = classtype::fatobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_fat as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;
    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
}

/*
=================
=
= T_SchabbThrow
=
=================
*/

pub fn T_SchabbThrow(w3d: &mut modules, ob: &mut object) {
    //println!("T_SchabbThrow");

    let deltax: i32;
    let deltay: i32;
    let mut angle: f64;
    let iangle: i32;

    deltax = ob.objlist[0].x - ob.objlist[ob.objlist_i].x;
    deltay = ob.objlist[ob.objlist_i].y - ob.objlist[0].y;

    angle = libm::atan2(deltay as f64, deltax as f64);

    if angle < 0.0 {
        angle = M_PI * 2.0 + angle;
    }
    iangle = (angle / (M_PI * 2.0) * ANGLES as f64) as i32;

    GetNewActor(ob);

    ob.newobj.state = s_needle1;
    ob.newobj.ticcount = 1;

    ob.newobj.tilex = ob.objlist[ob.objlist_i].tilex;
    ob.newobj.tiley = ob.objlist[ob.objlist_i].tiley;
    ob.newobj.x = ob.objlist[ob.objlist_i].x;
    ob.newobj.y = ob.objlist[ob.objlist_i].y;
    ob.newobj.obclass = classtype::needleobj;
    ob.newobj.dir = dirtype::nodir;
    ob.newobj.angle = iangle;
    ob.newobj.speed = 0x2000;
    ob.newobj.flags = objflag_t::FL_NEVERMARK as i32;
    ob.newobj.active = activetype::ac_yes;

    //
    ob.objlist.push(ob.newobj);

    PlaySoundLocActor(w3d, ob, soundnames::SCHABBSTHROWSND);
}

/*
=================
=
= T_GiftThrow
=
=================
*/

pub fn T_GiftThrow(w3d: &mut modules, ob: &mut object) {
    //println!("T_GiftThrow");

    let deltax: i32;
    let deltay: i32;
    let mut angle: f64;
    let iangle: i32;

    deltax = ob.objlist[0].x - ob.objlist[ob.objlist_i].x;
    deltay = ob.objlist[ob.objlist_i].y - ob.objlist[0].y;

    angle = libm::atan2(deltay as f64, deltax as f64);

    if angle < 0.0 {
        angle = M_PI * 2.0 + angle;
    }
    iangle = (angle / (M_PI * 2.0) * ANGLES as f64) as i32;

    GetNewActor(ob);

    ob.newobj.state = s_rocket;
    ob.newobj.ticcount = 1;

    ob.newobj.tilex = ob.objlist[ob.objlist_i].tilex;
    ob.newobj.tiley = ob.objlist[ob.objlist_i].tiley;
    ob.newobj.x = ob.objlist[ob.objlist_i].x;
    ob.newobj.y = ob.objlist[ob.objlist_i].y;
    ob.newobj.obclass = classtype::rocketobj;
    ob.newobj.dir = dirtype::nodir;
    ob.newobj.angle = iangle;
    ob.newobj.speed = 0x2000;
    ob.newobj.flags = objflag_t::FL_NEVERMARK as i32;
    ob.newobj.active = activetype::ac_yes;

    //
    ob.objlist.push(ob.newobj);

    //save rocketobj id
    w3d.wl_act2.rocketobj_i = ob.objlist_i;

    PlaySoundLocActor(w3d, ob, soundnames::MISSILEFIRESND);
}

/*
=================
=
= T_Schabb
=
=================
*/

pub fn T_Schabb(w3d: &mut modules, ob: &mut object) {
    //println!("T_Schabb");

    let mut Move: i32;
    let dx: i32;
    let dy: i32;
    let dist: i32;
    let mut dodge: bool;

    dodge = false;

    dx = abs(ob.objlist[ob.objlist_i].tilex - ob.objlist[0].tilex);
    dy = abs(ob.objlist[ob.objlist_i].tiley - ob.objlist[0].tiley);

    if dx > dy {
        dist = dx;
    } else {
        dist = dy;
    }

    let obj = ob.objlist[ob.objlist_i];
    if CheckLine(w3d, ob, obj)
    // got a shot at player?
    {
        ob.objlist[ob.objlist_i].hidden = 0;

        if US_RndT(w3d) < (w3d.wl_play.tics << 3)
        //&& objfreelist
        {
            //
            // go into attack frame
            //
            NewState(w3d, ob, ob.objlist_i, s_schabbshoot1);
            return;
        }
        dodge = true;
    } else {
        ob.objlist[ob.objlist_i].hidden = 1;
    }

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if ob.objlist[ob.objlist_i].distance < 0 {
            //
            // waiting for a door to open
            //
            OpenDoor(w3d, ob, -ob.objlist[ob.objlist_i].distance - 1);
            if w3d.wl_act1.doorobjlist[(-ob.objlist[ob.objlist_i].distance - 1) as usize].action
                != doortype::dr_open
            {
                return;
            }
            ob.objlist[ob.objlist_i].distance = TILEGLOBAL; // go ahead, the door is now open
            TryWalk(w3d, ob);
        }

        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        if dist < 4 {
            SelectRunDir(w3d, ob);
        } else if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
=================
=
= T_Gift
=
=================
*/

pub fn T_Gift(w3d: &mut modules, ob: &mut object) {
    //println!("T_Gift");

    let mut Move: i32;
    let dx: i32;
    let dy: i32;
    let dist: i32;
    let mut dodge: bool;

    dodge = false;

    dx = abs(ob.objlist[ob.objlist_i].tilex - ob.objlist[0].tilex);
    dy = abs(ob.objlist[ob.objlist_i].tiley - ob.objlist[0].tiley);

    if dx > dy {
        dist = dx;
    } else {
        dist = dy;
    }

    let obj = ob.objlist[ob.objlist_i];
    if CheckLine(w3d, ob, obj)
    // got a shot at player?
    {
        ob.objlist[ob.objlist_i].hidden = 0;

        if US_RndT(w3d) < (w3d.wl_play.tics << 3)
        //&& objfreelist
        {
            //
            // go into attack frame
            //
            NewState(w3d, ob, ob.objlist_i, s_giftshoot1);
            return;
        }
        dodge = true;
    } else {
        ob.objlist[ob.objlist_i].hidden = 1;
    }

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if ob.objlist[ob.objlist_i].distance < 0 {
            //
            // waiting for a door to open
            //
            OpenDoor(w3d, ob, -ob.objlist[ob.objlist_i].distance - 1);
            if w3d.wl_act1.doorobjlist[(-ob.objlist[ob.objlist_i].distance - 1) as usize].action
                != doortype::dr_open
            {
                return;
            }
            ob.objlist[ob.objlist_i].distance = TILEGLOBAL; // go ahead, the door is now open
            TryWalk(w3d, ob);
        }

        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        if dist < 4 {
            SelectRunDir(w3d, ob);
        } else if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
=================
=
= T_Fat
=
=================
*/

pub fn T_Fat(w3d: &mut modules, ob: &mut object) {
    //println!("T_Fat");

    let mut Move: i32;
    let dx: i32;
    let dy: i32;
    let dist: i32;
    let mut dodge: bool;

    dodge = false;

    dx = abs(ob.objlist[ob.objlist_i].tilex - ob.objlist[0].tilex);
    dy = abs(ob.objlist[ob.objlist_i].tiley - ob.objlist[0].tiley);

    if dx > dy {
        dist = dx;
    } else {
        dist = dy;
    }

    let obj = ob.objlist[ob.objlist_i];
    if CheckLine(w3d, ob, obj)
    // got a shot at player?
    {
        ob.objlist[ob.objlist_i].hidden = 0;

        if US_RndT(w3d) < (w3d.wl_play.tics << 3)
        //&& objfreelist
        {
            //
            // go into attack frame
            //
            NewState(w3d, ob, ob.objlist_i, s_fatshoot1);
            return;
        }
        dodge = true;
    } else {
        ob.objlist[ob.objlist_i].hidden = 1;
    }

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if ob.objlist[ob.objlist_i].distance < 0 {
            //
            // waiting for a door to open
            //
            OpenDoor(w3d, ob, -ob.objlist[ob.objlist_i].distance - 1);
            if w3d.wl_act1.doorobjlist[(-ob.objlist[ob.objlist_i].distance - 1) as usize].action
                != doortype::dr_open
            {
                return;
            }
            ob.objlist[ob.objlist_i].distance = TILEGLOBAL; // go ahead, the door is now open
            TryWalk(w3d, ob);
        }

        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        if dist < 4 {
            SelectRunDir(w3d, ob);
        } else if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
=============================================================================

                                    HITLERS

=============================================================================
*/

/*
===============
=
= SpawnFakeHitler
=
===============
*/

pub fn SpawnFakeHitler(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnFakeHitler");

    SpawnNewObj(w3d, ob, tilex, tiley, s_fakestand);
    ob.objlist[ob.objlist_i].speed = SPDPATROL;

    ob.objlist[ob.objlist_i].obclass = classtype::fakeobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_fake as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;
    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
}

/*
===============
=
= SpawnHitler
=
===============
*/

pub fn SpawnHitler(w3d: &mut modules, ob: &mut object, tilex: i32, tiley: i32) {
    //println!("SpawnHitler");
    //BUG
    if w3d.id_sd.DigiMode != SDSMode::sds_Off {
        //s_hitlerdie2.tictime = 140;
    } else {
        //s_hitlerdie2.tictime = 5;
    }

    SpawnNewObj(w3d, ob, tilex, tiley, s_mechastand);
    ob.objlist[ob.objlist_i].speed = SPDPATROL;

    ob.objlist[ob.objlist_i].obclass = classtype::mechahitlerobj;
    ob.objlist[ob.objlist_i].hitpoints =
        starthitpoints[w3d.wl_game.gamestate.difficulty as usize][enemy_t::en_hitler as usize];
    ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    ob.objlist[ob.objlist_i].flags |= objflag_t::FL_SHOOTABLE as i32 | objflag_t::FL_AMBUSH as i32;
    if !w3d.wl_main.loadedgame {
        w3d.wl_game.gamestate.killtotal += 1;
    }
    // get mechahitlerobj index
    w3d.wl_act2.mechahitlerobj_i = ob.objlist_i;
}

/*
===============
=
= A_HitlerMorph
=
===============
*/

pub fn A_HitlerMorph(w3d: &mut modules, ob: &mut object) {
    //println!("A_HitlerMorph");

    let hitpoints: [i32; 4] = [500, 700, 800, 900];

    //force last object
    let objlist_i_temp = ob.objlist_i;
    ob.objlist_i = ob.objlist.len() - 1;

    SpawnNewObj(
        w3d,
        ob,
        ob.objlist[w3d.wl_act2.mechahitlerobj_i].tilex,
        ob.objlist[w3d.wl_act2.mechahitlerobj_i].tiley,
        s_hitlerchase1,
    );
    ob.objlist[ob.objlist_i].speed = SPDPATROL * 5;

    ob.objlist[ob.objlist_i].x = ob.objlist[w3d.wl_act2.mechahitlerobj_i].x;
    ob.objlist[ob.objlist_i].y = ob.objlist[w3d.wl_act2.mechahitlerobj_i].y;

    ob.objlist[ob.objlist_i].distance = ob.objlist[w3d.wl_act2.mechahitlerobj_i].distance;
    ob.objlist[ob.objlist_i].dir = ob.objlist[w3d.wl_act2.mechahitlerobj_i].dir;
    ob.objlist[ob.objlist_i].flags =
        ob.objlist[w3d.wl_act2.mechahitlerobj_i].flags | objflag_t::FL_SHOOTABLE as i32;
    ob.objlist[ob.objlist_i].flags &= !(objflag_t::FL_NONMARK as i32); // hitler stuck with nodir fix

    ob.objlist[ob.objlist_i].obclass = classtype::realhitlerobj;
    ob.objlist[ob.objlist_i].hitpoints = hitpoints[w3d.wl_game.gamestate.difficulty as usize];

    if !w3d.wl_main.loadedgame {
        // Count real hitler for correct kill ratios
        w3d.wl_game.gamestate.killtotal += 1;
    }

    //get back counter
    ob.objlist_i = objlist_i_temp;
}

////////////////////////////////////////////////////////
//
// A_MechaSound
// A_Slurpie
//
////////////////////////////////////////////////////////

pub fn A_MechaSound(w3d: &mut modules, ob: &mut object) {
    //println!("A_MechaSound");

    if ob.objlist[ob.objlist_i].areanumber >= NUMAREAS
        || w3d.wl_act1.areabyplayer[ob.objlist[ob.objlist_i].areanumber as usize]
    {
        PlaySoundLocActor(w3d, ob, soundnames::MECHSTEPSND);
    }
}

pub fn A_Slurpie(w3d: &mut modules) {
    //println!("A_Slurpie");

    SD_PlaySound(w3d, soundnames::SLURPIESND);
}

/*
=================
=
= T_FakeFire
=
=================
*/

pub fn T_FakeFire(w3d: &mut modules, ob: &mut object) {
    //println!("T_FakeFire");

    let deltax: i32;
    let deltay: i32;
    let mut angle: f64;
    let iangle: i32;

    if ob.objlist.len() == MAXACTORS as usize
    // stop shooting if over MAXACTORS
    {
        NewState(w3d, ob, ob.objlist_i, s_fakechase1);
        return;
    }

    deltax = ob.objlist[0].x - ob.objlist[ob.objlist_i].x;
    deltay = ob.objlist[ob.objlist_i].y - ob.objlist[0].y;

    angle = libm::atan2(deltay as f64, deltax as f64);

    if angle < 0.0 {
        angle = M_PI * 2.0 + angle;
    }
    iangle = (angle / (M_PI * 2.0) * ANGLES as f64) as i32;

    GetNewActor(ob);

    ob.newobj.state = s_fire1;
    ob.newobj.ticcount = 1;

    ob.newobj.tilex = ob.objlist[ob.objlist_i].tilex;
    ob.newobj.tiley = ob.objlist[ob.objlist_i].tiley;
    ob.newobj.x = ob.objlist[ob.objlist_i].x;
    ob.newobj.y = ob.objlist[ob.objlist_i].y;
    ob.newobj.dir = dirtype::nodir;
    ob.newobj.angle = iangle;
    ob.newobj.obclass = classtype::fireobj;
    ob.newobj.speed = 0x1200;
    ob.newobj.flags = objflag_t::FL_NEVERMARK as i32;
    ob.newobj.active = activetype::ac_yes;

    //
    ob.objlist.push(ob.newobj);

    PlaySoundLocActor(w3d, ob, soundnames::FLAMETHROWERSND);
}

/*
=================
=
= T_Fake
=
=================
*/

pub fn T_Fake(w3d: &mut modules, ob: &mut object) {
    //println!("T_Fake");

    let mut Move: i32;

    let obj = ob.objlist[ob.objlist_i];
    if CheckLine(w3d, ob, obj)
    // got a shot at player?
    {
        ob.objlist[ob.objlist_i].hidden = 0;
        if US_RndT(w3d) < (w3d.wl_play.tics << 1)
        //&& objfreelist)
        {
            //
            // go into attack frame
            //
            NewState(w3d, ob, ob.objlist_i, s_fakeshoot1);
            return;
        }
    } else {
        ob.objlist[ob.objlist_i].hidden = 1;
    }

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        SelectDodgeDir(w3d, ob);
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        SelectDodgeDir(w3d, ob);

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
============================================================================

STAND

============================================================================
*/

/*
===============
=
= T_Stand
=
===============
*/

pub fn T_Stand(w3d: &mut modules, ob: &mut object) {
    //println!("T_Stand");

    SightPlayer(w3d, ob);
}

/*
============================================================================

CHASE

============================================================================
*/

/*
=================
=
= T_Chase
=
=================
*/

pub fn T_Chase(w3d: &mut modules, ob: &mut object) {
    //println!("T_Chase");

    let mut Move: i32;
    let mut target: i32;
    let dx: i32;
    let dy: i32;
    let dist: i32;
    let mut chance: i32;
    let mut dodge: bool;

    if w3d.wl_game.gamestate.victoryflag {
        return;
    }

    dodge = false;
    let obj = ob.objlist[ob.objlist_i];
    if CheckLine(w3d, ob, obj)
    // got a shot at player?
    {
        //BUG?
        ob.objlist[ob.objlist_i].hidden = 0;
        dx = abs(ob.objlist[ob.objlist_i].tilex - ob.objlist[0].tilex);
        dy = abs(ob.objlist[ob.objlist_i].tiley - ob.objlist[0].tiley);

        if dx > dy {
            dist = dx;
        } else {
            dist = dy;
        }

        if DEMOCOND_ORIG(w3d) {
            if dist == 0 || (dist == 1 && ob.objlist[ob.objlist_i].distance < 0x4000) {
                chance = 300;
            } else {
                chance = (w3d.wl_play.tics << 4) / dist;
            }
        } else {
            if dist != 0 {
                chance = (w3d.wl_play.tics << 4) / dist;
            } else {
                chance = 300;
            }

            if dist == 1 {
                target = abs(ob.objlist[ob.objlist_i].x - ob.objlist[0].x);
                if target < 0x14000 {
                    target = abs(ob.objlist[ob.objlist_i].y - ob.objlist[0].y);
                    if target < 0x14000 {
                        chance = 300;
                    }
                }
            }
        }

        if US_RndT(w3d) < chance {
            //
            // go into attack frame
            //
            match ob.objlist[ob.objlist_i].obclass {
                classtype::guardobj => {
                    NewState(w3d, ob, ob.objlist_i, s_grdshoot1);
                }
                classtype::officerobj => {
                    NewState(w3d, ob, ob.objlist_i, s_ofcshoot1);
                }
                classtype::mutantobj => {
                    NewState(w3d, ob, ob.objlist_i, s_mutshoot1);
                }
                classtype::ssobj => {
                    NewState(w3d, ob, ob.objlist_i, s_ssshoot1);
                }
                classtype::bossobj => {
                    NewState(w3d, ob, ob.objlist_i, s_bossshoot1);
                }
                classtype::gretelobj => {
                    NewState(w3d, ob, ob.objlist_i, s_gretelshoot1);
                }
                classtype::mechahitlerobj => {
                    NewState(w3d, ob, ob.objlist_i, s_mechashoot1);
                }
                classtype::realhitlerobj => {
                    NewState(w3d, ob, ob.objlist_i, s_hitlershoot1);
                }
                _ => (),
            }
            return;
        }
        dodge = true;
    } else {
        //BUG?
        ob.objlist[ob.objlist_i].hidden = 1;
    }

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if ob.objlist[ob.objlist_i].distance < 0 {
            //
            // waiting for a door to open
            //
            OpenDoor(w3d, ob, -ob.objlist[ob.objlist_i].distance - 1);
            if w3d.wl_act1.doorobjlist[(-ob.objlist[ob.objlist_i].distance - 1) as usize].action
                != doortype::dr_open
            {
                return;
            }
            ob.objlist[ob.objlist_i].distance = TILEGLOBAL; // go ahead, the door is now open
            if DEMOIF_SDL(w3d) {
                TryWalk(w3d, ob);
            }
        }

        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        if dodge {
            SelectDodgeDir(w3d, ob);
        } else {
            SelectChaseDir(w3d, ob);
        }

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
=================
=
= T_Ghosts
=
=================
*/

pub fn T_Ghosts(w3d: &mut modules, ob: &mut object) {
    //println!("T_Ghosts");

    let mut Move: i32;

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        SelectChaseDir(w3d, ob);
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        SelectChaseDir(w3d, ob);

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
=================
=
= T_DogChase
=
=================
*/

pub fn T_DogChase(w3d: &mut modules, ob: &mut object) {
    //println!("T_DogChase");

    let mut Move: i32;
    let mut dx: i32;
    let mut dy: i32;

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        SelectDodgeDir(w3d, ob);
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        //
        // check for byte range
        //
        dx = ob.objlist[0].x - ob.objlist[ob.objlist_i].x;
        if dx < 0 {
            dx = -dx;
        }
        dx -= Move;
        if dx <= MINACTORDIST {
            dy = ob.objlist[0].y - ob.objlist[ob.objlist_i].y;
            if dy < 0 {
                dy = -dy;
            }
            dy -= Move;
            if dy <= MINACTORDIST {
                NewState(w3d, ob, ob.objlist_i, s_dogjump1);
                return;
            }
        }

        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        //
        // reached goal tile, so select another one
        //

        //
        // fix position to account for round off during moving
        //
        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        SelectDodgeDir(w3d, ob);

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // object is blocked in
        }
    }
}

/*
============================================================================

                                    PATH

============================================================================
*/

/*
===============
=
= SelectPathDir
=
===============
*/

pub fn SelectPathDir(w3d: &mut modules, ob: &mut object) {
    //println!("SelectPathDir");

    let spot: i32;

    spot = MAPSPOT(
        w3d,
        ob.objlist[ob.objlist_i].tilex,
        ob.objlist[ob.objlist_i].tiley,
        1,
    ) - ICONARROWS;

    if (spot as u32) < 8 {
        // new direction
        ob.objlist[ob.objlist_i].dir = dirtype::from_u8(spot as u8);
    }
    ob.objlist[ob.objlist_i].distance = TILEGLOBAL; //65536

    if !TryWalk(w3d, ob) {
        ob.objlist[ob.objlist_i].dir = dirtype::nodir;
    }
}

/*
===============
=
= T_Path
=
===============
*/

pub fn T_Path(w3d: &mut modules, ob: &mut object) {
    //println!("T_Path");

    let mut Move: i32;

    if SightPlayer(w3d, ob) {
        return;
    }

    if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
        SelectPathDir(w3d, ob);
        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // all movement is blocked
        }
    }

    Move = ob.objlist[ob.objlist_i].speed * w3d.wl_play.tics;

    while Move != 0 {
        if ob.objlist[ob.objlist_i].distance < 0 {
            //
            // waiting for a door to open
            //
            OpenDoor(w3d, ob, -ob.objlist[ob.objlist_i].distance - 1);
            if w3d.wl_act1.doorobjlist[(-ob.objlist[ob.objlist_i].distance - 1) as usize].action
                != doortype::dr_open
            {
                return;
            }
            ob.objlist[ob.objlist_i].distance = TILEGLOBAL; // go ahead, the door is now open

            //BUG
            if DEMOIF_SDL(w3d) {
                TryWalk(w3d, ob);
            }
        }
        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        if ob.objlist[ob.objlist_i].tilex > MAPSIZE as i32
            || ob.objlist[ob.objlist_i].tiley > MAPSIZE as i32
        {
            //BUG
            //format! (str, "T_Path hit a wall at %u,%u, dir %u", obj.tilex,obj.tiley,obj.dir);
            Quit("T_Path hit a wall at %u,%u, dir %u");
        }

        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;

        Move -= ob.objlist[ob.objlist_i].distance;

        SelectPathDir(w3d, ob);

        if ob.objlist[ob.objlist_i].dir == dirtype::nodir {
            return; // all movement is blocked
        }
    }
}

/*
=============================================================================

                                    FIGHT

=============================================================================
*/

/*
===============
=
= T_Shoot
=
= Try to damage the player, based on skill level and player's speed
=
===============
*/

pub fn T_Shoot(w3d: &mut modules, ob: &mut object) {
    //println!("T_Shoot");

    let dx: i32;
    let dy: i32;
    let mut dist: i32;
    let hitchance: i32;
    let damage: i32;

    if ob.objlist[ob.objlist_i].areanumber < NUMAREAS
        && !w3d.wl_act1.areabyplayer[ob.objlist[ob.objlist_i].areanumber as usize]
    {
        return;
    }
    let obj = ob.objlist[ob.objlist_i];
    if CheckLine(w3d, ob, obj)
    // player is not behind a wall
    {
        dx = abs(ob.objlist[ob.objlist_i].tilex - ob.objlist[0].tilex);
        dy = abs(ob.objlist[ob.objlist_i].tiley - ob.objlist[0].tiley);

        if dx > dy {
            dist = dx;
        } else {
            dist = dy;
        }

        if ob.objlist[ob.objlist_i].obclass == classtype::ssobj
            || ob.objlist[ob.objlist_i].obclass == classtype::bossobj
        {
            dist = dist * 2 / 3; // ss are better shots
        }

        if w3d.wl_agent.thrustspeed >= RUNSPEED {
            if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_VISABLE as i32) != 0 {
                hitchance = 160 - dist * 16; // player can see to dodge
            } else {
                hitchance = 160 - dist * 8;
            }
        } else {
            if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_VISABLE as i32) != 0 {
                hitchance = 256 - dist * 16; // player can see to dodge
            } else {
                hitchance = 256 - dist * 8;
            }
        }

        // see if the shot was a hit
        if US_RndT(w3d) < hitchance {
            if dist < 2 {
                damage = US_RndT(w3d) >> 2;
            } else if dist < 4 {
                damage = US_RndT(w3d) >> 3;
            } else {
                damage = US_RndT(w3d) >> 4;
            }

            TakeDamage(w3d, ob, damage);
        }
    }

    match ob.objlist[ob.objlist_i].obclass {
        classtype::ssobj => {
            PlaySoundLocActor(w3d, ob, soundnames::SSFIRESND);
        }
        classtype::giftobj | classtype::fatobj => {
            PlaySoundLocActor(w3d, ob, soundnames::MISSILEFIRESND);
        }
        classtype::mechahitlerobj | classtype::realhitlerobj | classtype::bossobj => {
            PlaySoundLocActor(w3d, ob, soundnames::BOSSFIRESND);
        }
        classtype::schabbobj => {
            PlaySoundLocActor(w3d, ob, soundnames::SCHABBSTHROWSND);
        }
        classtype::fakeobj => {
            PlaySoundLocActor(w3d, ob, soundnames::FLAMETHROWERSND);
        }
        _ => {
            PlaySoundLocActor(w3d, ob, soundnames::NAZIFIRESND);
        }
    }
}

/*
===============
=
= T_Bite
=
===============
*/

pub fn T_Bite(w3d: &mut modules, ob: &mut object) {
    //println!("T_Bite");

    let mut dx: i32;
    let mut dy: i32;

    PlaySoundLocActor(w3d, ob, soundnames::DOGATTACKSND); // JAB

    dx = ob.objlist[0].x - ob.objlist[ob.objlist_i].x;
    if dx < 0 {
        dx = -dx;
    }
    dx -= TILEGLOBAL;
    if dx <= MINACTORDIST {
        dy = ob.objlist[0].y - ob.objlist[ob.objlist_i].y;
        if dy < 0 {
            dy = -dy;
        }
        dy -= TILEGLOBAL;
        if dy <= MINACTORDIST {
            if US_RndT(w3d) < 180 {
                let damage = US_RndT(w3d) >> 4;
                TakeDamage(w3d, ob, damage);
                return;
            }
        }
    }
}

/*
============================================================================

                                    BJ VICTORY

============================================================================
*/

/*
===============
=
= SpawnBJVictory
=
===============
*/

pub fn SpawnBJVictory(w3d: &mut modules, ob: &mut object) {
    //println!("SpawnBJVictory");

    //force last object
    let objlist_i_temp = ob.objlist_i;
    ob.objlist_i = ob.objlist.len() - 1;

    SpawnNewObj(
        w3d,
        ob,
        ob.objlist[0].tilex,
        ob.objlist[0].tiley + 1,
        s_bjrun1,
    );
    ob.objlist[ob.objlist_i].x = ob.objlist[0].x;
    ob.objlist[ob.objlist_i].y = ob.objlist[0].y;
    ob.objlist[ob.objlist_i].obclass = classtype::bjobj;
    ob.objlist[ob.objlist_i].dir = dirtype::north;
    ob.objlist[ob.objlist_i].temp1 = 6;

    //get back counter
    ob.objlist_i = objlist_i_temp;
}

/*
===============
=
= T_BJRun
=
===============
*/

pub fn T_BJRun(w3d: &mut modules, ob: &mut object) {
    //println!("T_BJRun");

    let mut Move: i32;

    Move = BJRUNSPEED * w3d.wl_play.tics;

    while Move != 0 {
        if Move < ob.objlist[ob.objlist_i].distance {
            MoveObj(w3d, ob, Move);
            break;
        }

        ob.objlist[ob.objlist_i].x = (ob.objlist[ob.objlist_i].tilex << TILESHIFT) + TILEGLOBAL / 2;
        ob.objlist[ob.objlist_i].y = (ob.objlist[ob.objlist_i].tiley << TILESHIFT) + TILEGLOBAL / 2;
        Move -= ob.objlist[ob.objlist_i].distance;

        SelectPathDir(w3d, ob);

        ob.objlist[ob.objlist_i].temp1 -= 1;
        let value = ob.objlist[ob.objlist_i].temp1;

        if value == 0 {
            //BUG ?
            NewState(w3d, ob, ob.objlist_i, s_bjjump1);
            return;
        }
    }
}

/*
===============
=
= T_BJJump
=
===============
*/

pub fn T_BJJump(w3d: &mut modules, ob: &mut object) {
    //println!("T_BJJump");

    let Move: i32;

    Move = BJJUMPSPEED * w3d.wl_play.tics;
    MoveObj(w3d, ob, Move);
}
/*
===============
=
= T_BJYell
=
===============
*/

pub fn T_BJYell(w3d: &mut modules, ob: &mut object) {
    //println!("T_BJYell");

    PlaySoundLocActor(w3d, ob, soundnames::YEAHSND); // JAB
}

/*
===============
=
= T_BJDone
=
===============
*/

pub fn T_BJDone(w3d: &mut modules) {
    //println!("T_BJDone");

    w3d.wl_play.playstate = exit_t::ex_victorious; // exit castle tile
}

//===========================================================================

/*
===============
=
= CheckPosition
=
===============
*/

pub fn CheckPosition(_w3d: &mut modules, ob: &mut object) -> bool {
    //println!("CheckPosition");

    let xl: i32;
    let yl: i32;
    let xh: i32;
    let yh: i32;
    let mut check: *mut objtype;

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

            if !check.is_null() {
                return false;
            }
        }
    }

    return true;
}

/*
===============
=
= A_StartDeathCam
=
===============
*/

pub fn A_StartDeathCam(w3d: &mut modules, ob: &mut object) {
    //println!("A_StartDeathCam");

    let dx: i32;
    let dy: i32;
    let mut fangle: f64;
    let mut xmove: i32;
    let mut ymove: i32;
    let mut dist: i32;

    FinishPaletteShifts(w3d);

    VW_WaitVBL(w3d, 100);

    if w3d.wl_game.gamestate.victoryflag {
        w3d.wl_play.playstate = exit_t::ex_victorious; // exit castle tile
        return;
    }

    if w3d.id_vl.usedoublebuffering {
        VW_UpdateScreen(w3d);
    }

    w3d.wl_game.gamestate.victoryflag = true;

    let fadeheight: i32;
    if w3d.wl_play.viewsize != 21 {
        fadeheight = w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * STATUSLINES;
    } else {
        fadeheight = w3d.id_vl.screenHeight;
    }

    VL_BarScaledCoord(
        w3d,
        0,
        0,
        w3d.id_vl.screenWidth,
        fadeheight,
        w3d.wl_game.bordercol,
    );

    FizzleFade(w3d, 0, 0, w3d.id_vl.screenWidth, fadeheight, 70, false);

    if w3d.wl_game.bordercol != VIEWCOLOR {
        w3d.id_vh.fontnumber = 1;
        SETFONTCOLOR(w3d, 15, w3d.wl_game.bordercol);
        w3d.id_us.PrintX = 68;
        w3d.id_us.PrintY = 45;
        US_Print(w3d, STR_SEEAGAIN.to_string());
    } else {
        Write(w3d, 0, 7, STR_SEEAGAIN.to_string());
    }

    VW_UpdateScreen(w3d);
    if w3d.id_vl.usedoublebuffering {
        VW_UpdateScreen(w3d);
    }

    IN_UserInput(w3d, 300);
    //
    // line angle up exactly
    //
    NewState(w3d, ob, 0, s_deathcam);

    ob.objlist[0].x = w3d.wl_game.gamestate.killx;
    ob.objlist[0].y = w3d.wl_game.gamestate.killy;

    dx = ob.objlist[ob.objlist_i].x - ob.objlist[0].x;
    dy = ob.objlist[0].y - ob.objlist[ob.objlist_i].y;

    fangle = libm::atan2(dy as f64, dx as f64); // returns -pi to pi
    if fangle < 0.0 {
        fangle = M_PI * 2.0 + fangle;
    }

    ob.objlist[0].angle = (fangle / (M_PI * 2.0) * ANGLES as f64) as i32;
    //
    // try to position as close as possible without being in a wall
    //
    dist = 0x14000;

    loop {
        xmove = FixedMul(dist, w3d.wl_draw.costable[ob.objlist[0].angle as usize]);
        ymove = -FixedMul(dist, w3d.wl_draw.sintable[ob.objlist[0].angle as usize]);

        ob.objlist[0].x = ob.objlist[ob.objlist_i].x - xmove;
        ob.objlist[0].y = ob.objlist[ob.objlist_i].y - ymove;
        dist += 0x1000;

        if CheckPosition(w3d, ob) {
            break;
        }
    }
    w3d.wl_agent.plux = (ob.objlist[0].x >> UNSIGNEDSHIFT) as i16; // scale to fit in unsigned
    w3d.wl_agent.pluy = (ob.objlist[0].y >> UNSIGNEDSHIFT) as i16;
    ob.objlist[0].tilex = ob.objlist[0].x >> TILESHIFT; // scale to tile values
    ob.objlist[0].tiley = ob.objlist[0].y >> TILESHIFT;

    //
    // go back to the game
    //

    DrawPlayBorder(w3d);

    w3d.wl_game.fizzlein = true;

    match ob.objlist[ob.objlist_i].obclass {
        classtype::schabbobj => {
            NewState(w3d, ob, ob.objlist_i, s_schabbdeathcam);
        }
        classtype::realhitlerobj => {
            NewState(w3d, ob, ob.objlist_i, s_hitlerdeathcam);
        }
        classtype::giftobj => {
            NewState(w3d, ob, ob.objlist_i, s_giftdeathcam);
        }
        classtype::fatobj => {
            NewState(w3d, ob, ob.objlist_i, s_fatdeathcam);
        }
        _ => (),
    }
}
