#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_main
//
//===========================================================================

pub struct wl_main {
    pub str: String,
    pub dirangle: [i32; 9],

    pub focallength: i32,
    pub screenofs: i32,
    pub viewscreenx: i32,
    pub viewscreeny: i32,
    pub viewwidth: i32,
    pub viewheight: i32,
    pub centerx: i32,
    pub centery: i32,
    pub shootdelta: i32, // pixels away from centerx a target can be
    pub scale: i32,
    pub heightnumerator: i32,

    pub startgame: bool,
    pub loadedgame: bool,
    pub mouseadjustment: i32,

    pub configdir: String,
    pub configname: String,

    pub param_debugmode: bool,
    pub param_nowait: bool,
    pub param_difficulty: i32,
    pub param_tedlevel: i32,
    pub param_joystickindex: i32,

    pub param_joystickhat: i32,
    pub param_samplerate: i32,
    pub param_audiobuffer: i32,

    pub param_mission: i32,
    pub param_goodtimes: bool,
    pub param_ignorenumchunks: bool,
}

impl wl_main {
    pub fn new() -> Self {
        Self {
            str: String::new(),
            dirangle: [
                0,
                ANGLES / 8,
                2 * ANGLES / 8,
                3 * ANGLES / 8,
                4 * ANGLES / 8,
                5 * ANGLES / 8,
                6 * ANGLES / 8,
                7 * ANGLES / 8,
                ANGLES,
            ],

            focallength: 0,
            screenofs: 0,
            viewscreenx: 0,
            viewscreeny: 0,
            viewwidth: 0,
            viewheight: 0,
            centerx: 0,
            centery: 0,
            shootdelta: 0, // pixels away from centerx a target can be
            scale: 0,
            heightnumerator: 0,

            startgame: false,
            loadedgame: false,
            mouseadjustment: 0,

            configdir: String::from(""),
            configname: String::from("config."),

            param_debugmode: false,
            param_nowait: false,
            param_difficulty: 1, // default is "normal"
            param_tedlevel: -1,  // default is not to start a level
            param_joystickindex: 0,

            param_joystickhat: -1,
            param_samplerate: 44100,
            param_audiobuffer: 2048,

            param_mission: 0,
            param_goodtimes: false,
            param_ignorenumchunks: false,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const FOCALLENGTH: i32 = 0x5700; // in global coordinates
pub const VIEWGLOBAL: i32 = 0x10000; // globals visable flush to wall

pub const radtoint: f32 = (FINEANGLES as f64 / 2.0 / PI) as f32;

#[cfg(any(feature = "UPLOAD"))]
pub static wolfdigimap: [i32; 64] = [
    // These first sounds are in the upload version
    soundnames::HALTSND as i32,
    0,
    -1,
    soundnames::DOGBARKSND as i32,
    1,
    -1,
    soundnames::CLOSEDOORSND as i32,
    2,
    -1,
    soundnames::OPENDOORSND as i32,
    3,
    -1,
    soundnames::ATKMACHINEGUNSND as i32,
    4,
    0,
    soundnames::ATKPISTOLSND as i32,
    5,
    0,
    soundnames::ATKGATLINGSND as i32,
    6,
    0,
    soundnames::SCHUTZADSND as i32,
    7,
    -1,
    soundnames::GUTENTAGSND as i32,
    8,
    -1,
    soundnames::MUTTISND as i32,
    9,
    -1,
    soundnames::BOSSFIRESND as i32,
    10,
    1,
    soundnames::SSFIRESND as i32,
    11,
    -1,
    soundnames::DEATHSCREAM1SND as i32,
    12,
    -1,
    soundnames::DEATHSCREAM2SND as i32,
    13,
    -1,
    soundnames::DEATHSCREAM3SND as i32,
    13,
    -1,
    soundnames::TAKEDAMAGESND as i32,
    14,
    -1,
    soundnames::PUSHWALLSND as i32,
    15,
    -1,
    soundnames::LEBENSND as i32,
    20,
    -1,
    soundnames::NAZIFIRESND as i32,
    21,
    -1,
    soundnames::SLURPIESND as i32,
    22,
    -1,
    soundnames::YEAHSND as i32,
    32,
    -1,
    soundnames::LASTSOUND as i32,
];

#[cfg(any(feature = "GOODTIMES"))]
pub static wolfdigimap: [i32; 142] = [
    // These first sounds are in the upload version
    soundnames::HALTSND as i32,
    0,
    -1,
    soundnames::DOGBARKSND as i32,
    1,
    -1,
    soundnames::CLOSEDOORSND as i32,
    2,
    -1,
    soundnames::OPENDOORSND as i32,
    3,
    -1,
    soundnames::ATKMACHINEGUNSND as i32,
    4,
    0,
    soundnames::ATKPISTOLSND as i32,
    5,
    0,
    soundnames::ATKGATLINGSND as i32,
    6,
    0,
    soundnames::SCHUTZADSND as i32,
    7,
    -1,
    soundnames::GUTENTAGSND as i32,
    8,
    -1,
    soundnames::MUTTISND as i32,
    9,
    -1,
    soundnames::BOSSFIRESND as i32,
    10,
    1,
    soundnames::SSFIRESND as i32,
    11,
    -1,
    soundnames::DEATHSCREAM1SND as i32,
    12,
    -1,
    soundnames::DEATHSCREAM2SND as i32,
    13,
    -1,
    soundnames::DEATHSCREAM3SND as i32,
    13,
    -1,
    soundnames::TAKEDAMAGESND as i32,
    14,
    -1,
    soundnames::PUSHWALLSND as i32,
    15,
    -1,
    soundnames::LEBENSND as i32,
    20,
    -1,
    soundnames::NAZIFIRESND as i32,
    21,
    -1,
    soundnames::SLURPIESND as i32,
    22,
    -1,
    soundnames::YEAHSND as i32,
    32,
    -1,
    // These are in all other episodes
    soundnames::DOGDEATHSND as i32,
    16,
    -1,
    soundnames::AHHHGSND as i32,
    17,
    -1,
    soundnames::DIESND as i32,
    18,
    -1,
    soundnames::EVASND as i32,
    19,
    -1,
    soundnames::TOT_HUNDSND as i32,
    23,
    -1,
    soundnames::MEINGOTTSND as i32,
    24,
    -1,
    soundnames::SCHABBSHASND as i32,
    25,
    -1,
    soundnames::HITLERHASND as i32,
    26,
    -1,
    soundnames::SPIONSND as i32,
    27,
    -1,
    soundnames::NEINSOVASSND as i32,
    28,
    -1,
    soundnames::DOGATTACKSND as i32,
    29,
    -1,
    soundnames::LEVELDONESND as i32,
    30,
    -1,
    soundnames::MECHSTEPSND as i32,
    31,
    -1,
    soundnames::SCHEISTSND as i32,
    33,
    -1,
    soundnames::DEATHSCREAM4SND as i32,
    34,
    -1, // AIIEEE
    soundnames::DEATHSCREAM5SND as i32,
    35,
    -1, // DEE-DEE
    soundnames::DONNERSND as i32,
    36,
    -1, // EPISODE 4 BOSS DIE
    soundnames::EINESND as i32,
    37,
    -1, // EPISODE 4 BOSS SIGHTING
    soundnames::ERLAUBENSND as i32,
    38,
    -1, // EPISODE 6 BOSS SIGHTING
    soundnames::DEATHSCREAM6SND as i32,
    39,
    -1, // FART
    soundnames::DEATHSCREAM7SND as i32,
    40,
    -1, // GASP
    soundnames::DEATHSCREAM8SND as i32,
    41,
    -1, // GUH-BOY!
    soundnames::DEATHSCREAM9SND as i32,
    42,
    -1, // AH GEEZ!
    soundnames::KEINSND as i32,
    43,
    -1, // EPISODE 5 BOSS SIGHTING
    soundnames::MEINSND as i32,
    44,
    -1, // EPISODE 6 BOSS DIE
    soundnames::ROSESND as i32,
    45,
    -1, // EPISODE 5 BOSS DIE
    soundnames::LASTSOUND as i32,
];

pub static mut MusicItems: CP_iteminfo = CP_iteminfo {
    x: CTL_X,
    y: CTL_Y,
    amount: 6,
    curpos: 0,
    indent: 32,
};

pub static mut MusicMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: "Get Them!",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Searching",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "P.O.W.",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Suspense",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "War March",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Around The Corner!",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Nazi Anthem",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Lurking...",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Going After Hitler",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Pounding Headache",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Into the Dungeons",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Ultimate Conquest",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Kill the S.O.B.",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "The Nazi Rap",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Twelfth Hour",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Zero Hour",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Ultimate Conquest",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "Wolfpack",
        routine: call_routine_int as *mut routine_int,
    },
];

/*
====================
=
= ReadConfig
=
====================
*/

fn ReadConfig(w3d: &mut modules) {
    //println!("ReadConfig");

    let sd: SDMode;
    let sm: SMMode;
    let sds: SDSMode;

    //
    // no config file, so select by hardware
    //

    if w3d.id_sd.SoundBlasterPresent || w3d.id_sd.AdLibPresent {
        sd = SDMode::sdm_AdLib;
        sm = SMMode::smm_AdLib;
    } else {
        sd = SDMode::sdm_PC;
        sm = SMMode::smm_Off;
    }

    if w3d.id_sd.SoundBlasterPresent {
        sds = SDSMode::sds_SoundBlaster;
    } else {
        sds = SDSMode::sds_Off;
    }

    if w3d.id_in.MousePresent {
        w3d.wl_play.mouseenabled = 1;
    }

    if IN_JoyPresent(w3d) {
        w3d.wl_play.joystickenabled = 1;
    }

    w3d.wl_play.viewsize = 19; // start with a good size
    w3d.wl_main.mouseadjustment = 5;

    SD_SetMusicMode(w3d, sm);
    SD_SetSoundMode(w3d, sd);
    SD_SetDigiDevice(w3d, sds);
}

/*
=====================
=
= NewGame
=
= Set up new game to start from the beginning
=
=====================
*/

pub fn NewGame(w3d: &mut modules, difficulty: i32, episode: i32) {
    //println!("NewGame");

    w3d.wl_game.gamestate = gametype::new();

    w3d.wl_game.gamestate.difficulty = difficulty;
    w3d.wl_game.gamestate.weapon = weapontype::wp_pistol;
    w3d.wl_game.gamestate.bestweapon = weapontype::wp_pistol;
    w3d.wl_game.gamestate.chosenweapon = weapontype::wp_pistol;

    w3d.wl_game.gamestate.health = 100;
    w3d.wl_game.gamestate.ammo = STARTAMMO as i32;
    w3d.wl_game.gamestate.lives = 3;
    w3d.wl_game.gamestate.nextextra = EXTRAPOINTS as i32;
    w3d.wl_game.gamestate.episode = episode as i32;

    w3d.wl_main.startgame = true;
}

/*
==================
=
= BuildTables
=
= Calculates:
=
= scale                 projection constant
= sintable/costable     overlapping fractional tables
=
==================
*/

pub fn BuildTables(w3d: &mut modules) {
    //println!("BuildTables");

    //
    // calculate fine tangents
    //

    for i in 0..FINEANGLES / 8 {
        let tang: f64 = libm::tan((i as f64 + 0.5) / radtoint as f64);

        w3d.wl_draw.finetangent[i as usize] = (tang * GLOBAL1 as f64) as i32;
        w3d.wl_draw.finetangent[(FINEANGLES / 4 - 1 - i) as usize] =
            ((1.0 / tang) * GLOBAL1 as f64) as i32;
    }

    //
    // costable overlays sintable with a quarter phase shift
    // ANGLES is assumed to be divisable by four
    //

    let mut angle: f32 = 0.0;
    let anglestep: f32 = (PI / 2.0 / ANGLEQUAD as f64) as f32;

    for i in 0..ANGLEQUAD {
        let value: i32 = (GLOBAL1 as f64 * (libm::sin(angle as f64))) as i32;
        w3d.wl_draw.sintable[i as usize] = value;
        w3d.wl_draw.sintable[(i + ANGLES) as usize] = value;
        w3d.wl_draw.sintable[(ANGLES / 2 - i) as usize] = value;
        w3d.wl_draw.sintable[(ANGLES - i) as usize] = -value;
        w3d.wl_draw.sintable[(ANGLES / 2 + i) as usize] = -value;
        angle += anglestep;
    }
    w3d.wl_draw.sintable[ANGLEQUAD as usize] = 65536;
    w3d.wl_draw.sintable[3 * ANGLEQUAD as usize] = -65536;

    let mut angle: f32 = 0.0;
    let anglestep: f32 = (PI / 2.0 / ANGLEQUAD as f64) as f32;

    for i in 0..ANGLEQUAD {
        let value: i32 = (GLOBAL1 as f64 * (libm::cos(angle as f64))) as i32;
        w3d.wl_draw.costable[i as usize] = value;
        w3d.wl_draw.costable[(i + ANGLES) as usize] = value;
        w3d.wl_draw.costable[(ANGLES / 2 - i) as usize] = -value;
        w3d.wl_draw.costable[(ANGLES - i) as usize] = value;
        w3d.wl_draw.costable[(ANGLES / 2 + i) as usize] = -value;
        angle += anglestep;
    }

    w3d.wl_draw.costable[ANGLEQUAD as usize] = 0;
    w3d.wl_draw.costable[3 * ANGLEQUAD as usize] = 0;
}

//===========================================================================

/*
====================
=
= CalcProjection
=
= Uses focallength
=
====================
*/

pub fn CalcProjection(w3d: &mut modules, focal: i32) {
    //println!("CalcProjection");

    let mut intang: i32;
    let mut angle: f32;
    let mut tang: f64;
    let halfview: i32;
    let facedist: f64;

    w3d.wl_main.focallength = focal;
    facedist = focal as f64 + MINDIST as f64;
    halfview = w3d.wl_main.viewwidth as i32 / 2; // half view in pixels

    //
    // calculate scale value for vertical height calculations
    // and sprite x calculations
    //
    w3d.wl_main.scale = halfview * facedist as i32 / (VIEWGLOBAL as i32 / 2);

    //
    // divide heightnumerator by a posts distance to get the posts height for
    // the heightbuffer.  The pixel height is height>>2
    //
    w3d.wl_main.heightnumerator = (TILEGLOBAL * w3d.wl_main.scale) >> 6;

    //
    // calculate the angle offset from view angle of each pixel's ray
    //

    for i in 0..halfview {
        // start 1/2 pixel over, so viewangle bisects two middle pixels
        tang = (i * VIEWGLOBAL as i32 / w3d.wl_main.viewwidth) as f64 / facedist;
        angle = libm::atan(tang) as f32;
        intang = (angle * radtoint) as i32;
        w3d.wl_draw.pixelangle[(halfview - 1 - i) as usize] = intang;
        w3d.wl_draw.pixelangle[(halfview + i) as usize] = -intang;
    }
}

/*
===================
=
= SetupWalls
=
= Map tile values to scaled pics
=
===================
*/

pub fn SetupWalls(w3d: &mut modules) {
    //println!("SetupWalls");

    w3d.wl_draw.horizwall[0] = 0;
    w3d.wl_draw.vertwall[0] = 0;

    for i in 1..MAXWALLTILES as i32 {
        w3d.wl_draw.horizwall[i as usize] = (i - 1) * 2;
        w3d.wl_draw.vertwall[i as usize] = (i - 1) * 2 + 1;
    }
}

/*
==========================
=
= SignonScreen
=
==========================
*/

pub fn SignonScreen(w3d: &mut modules) {
    // VGA version
    //println!("SignonScreen");

    VL_SetVGAPlaneMode(w3d);

    VL_MemToScreen(w3d, &signon, 320, 200, 0, 0);
}

/*
==========================
=
= FinishSignon
=
==========================
*/

pub fn FinishSignon(w3d: &mut modules) {
    //println!("FinishSignon");

    {
        let color = VL_GetPixel(w3d, 0, 0);
        VW_Bar(w3d, 0, 189, 300, 11, color as i32);
        w3d.id_us.WindowX = 0;
        w3d.id_us.WindowW = 320;
        w3d.id_us.PrintY = 190;

        SETFONTCOLOR(w3d, 14, 4);
        US_CPrint(w3d, "Press a key".to_string());
        VW_UpdateScreen(w3d);

        if !w3d.wl_main.param_nowait {
            IN_Ack(w3d);
        }

        let color = VL_GetPixel(w3d, 0, 0);
        VW_Bar(w3d, 0, 189, 300, 11, color as i32);
        w3d.id_us.PrintY = 190;

        SETFONTCOLOR(w3d, 10, 4);
        US_CPrint(w3d, "Working...".to_string());
        VW_UpdateScreen(w3d);

        SETFONTCOLOR(w3d, 0, 15);
    }
}

/*
=====================
=
= InitDigiMap
=
=====================
*/

// channel mapping:
//  -1: any non reserved channel
//   0: player weapons
//   1: boss weapons

pub fn InitDigiMap(w3d: &mut modules) {
    //println!("InitDigiMap");

    //let map: Vec<i32> = wolfdigimap.to_vec();

    //for map = 0; map != LASTSOUND; map += 3
    let len = wolfdigimap.len();

    for map_index in (0..len - 1).step_by(3) {
        w3d.id_sd.DigiMap[wolfdigimap[map_index] as usize] = wolfdigimap[map_index + 1];
        w3d.id_sd.DigiChannel[wolfdigimap[map_index + 1] as usize] = wolfdigimap[map_index + 2];

        SD_PrepareSound(w3d, wolfdigimap[map_index + 1] as i32);
    }
}
pub fn DoJukebox(w3d: &mut modules) {
    //println!("DoJukebox");

    let mut which: i32;
    let mut lastsong: i32 = -1;
    let start: i32;
    let songs_jb: [i32; 18];

    songs_jb = [
        musicnames::GETTHEM_MUS as i32,
        musicnames::SEARCHN_MUS as i32,
        musicnames::POW_MUS as i32,
        musicnames::SUSPENSE_MUS as i32,
        musicnames::WARMARCH_MUS as i32,
        musicnames::CORNER_MUS as i32,
        musicnames::NAZI_OMI_MUS as i32,
        musicnames::PREGNANT_MUS as i32,
        musicnames::GOINGAFT_MUS as i32,
        musicnames::HEADACHE_MUS as i32,
        musicnames::DUNGEON_MUS as i32,
        musicnames::ULTIMATE_MUS as i32,
        musicnames::INTROCW3_MUS as i32,
        musicnames::NAZI_RAP_MUS as i32,
        musicnames::TWELFTH_MUS as i32,
        musicnames::ZEROHOUR_MUS as i32,
        musicnames::ULTIMATE_MUS as i32,
        musicnames::PACMAN_MUS as i32,
    ];

    IN_ClearKeysDown(w3d);
    if !w3d.id_sd.AdLibPresent && !w3d.id_sd.SoundBlasterPresent {
        return;
    }

    MenuFadeOut(w3d);

    start = 0;

    CA_LoadAllSounds(w3d);

    w3d.id_vh.fontnumber = 1;
    ClearMScreen(w3d);
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);
    DrawStripes(w3d, 10);
    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);

    DrawWindow(
        w3d,
        (CTL_X - 2) as i32,
        (CTL_Y - 6) as i32,
        280,
        13 * 7,
        BKGDCOLOR,
    );

    unsafe {
        //DrawMenu(w3d, &MusicItems, &MusicMenu[start as usize]);
        DrawMenu(w3d, &MusicItems, &MusicMenu);
    }

    SETFONTCOLOR(w3d, READHCOLOR, BKGDCOLOR);
    w3d.id_us.PrintY = 15;
    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowY = 320;
    US_CPrint(w3d, "Robert's Jukebox".to_string());

    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);
    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);

    loop {
        which = HandleMenu(
            w3d,
            unsafe { &mut MusicItems },
            unsafe { &mut MusicMenu },
            call_routine_void,
        );

        if which >= 0 {
            if lastsong >= 0 {
                unsafe {
                    MusicMenu[(start + lastsong) as usize].active = 1;
                }
            }

            StartCPMusic(w3d, songs_jb[(start + which) as usize]);

            unsafe {
                MusicMenu[(start + which) as usize].active = 2;
            }
            unsafe {
                //DrawMenu(w3d, &MusicItems, &MusicMenu[start as usize]);
                DrawMenu(w3d, &MusicItems, &MusicMenu);
            }
            VW_UpdateScreen(w3d);
            lastsong = which;
        }
        if which < 0 {
            break;
        }
    }

    MenuFadeOut(w3d);
    IN_ClearKeysDown(w3d);
}

/*
==========================
=
= InitGame
=
= Load a few things right away
=
==========================
*/

pub fn InitGame(w3d: &mut modules) {
    //println!("InitGame");

    let mut didjukebox: bool = false;

    // initialize SDL
    //if(SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_JOYSTICK) < 0)
    //{
    //    printf("Unable to init SDL: %s\n", SDL_GetError());
    //    exit(1);
    //}
    //atexit(SDL_Quit);

    let numJoysticks = w3d.id_vl.joystick.num_joysticks().unwrap();

    if w3d.wl_main.param_joystickindex != 0
        && (w3d.wl_main.param_joystickindex < -1
            || w3d.wl_main.param_joystickindex >= numJoysticks as i32)
    {
        if numJoysticks == 0 {
            println!("No joysticks are available to SDL!");
        } else {
            println!(
                "The joystick index must be between -1 and {}!",
                numJoysticks - 1
            );
        }
        exit(1);
    }

    SignonScreen(w3d);

    VW_UpdateScreen(w3d);

    VH_Startup(w3d);
    IN_Startup(w3d);
    PM_Startup(w3d);
    SD_Startup(w3d);
    CA_Startup(w3d);
    US_Startup();

    //
    // build some tables
    //
    InitDigiMap(w3d);

    ReadConfig(w3d);

    SetupSaveGames();

    //
    // HOLDING DOWN 'M' KEY?
    //
    IN_ProcessEvents(w3d);

    if Keyboard(w3d, Scancode::M) {
        DoJukebox(w3d);
        didjukebox = true;
    }

    //
    // draw intro screen stuff
    //
    IntroScreen(w3d);

    //
    // load in and lock down some basic chunks
    //
    BuildTables(w3d); // trig tables
    SetupWalls(w3d);

    NewViewSize(w3d, w3d.wl_play.viewsize);

    //
    // initialize variables
    //
    InitRedShifts(w3d);
    if !didjukebox {
        FinishSignon(w3d);
    }
}

//===========================================================================

/*
==========================
=
= SetViewSize
=
==========================
*/

pub fn SetViewSize(w3d: &mut modules, width: i32, height: i32) -> bool {
    //println!("SetViewSize");

    w3d.wl_main.viewwidth = width & !15; // must be divisable by 16
    w3d.wl_main.viewheight = height & !1; // must be even
    w3d.wl_main.centerx = w3d.wl_main.viewwidth / 2 - 1;
    w3d.wl_main.centery = w3d.wl_main.viewheight / 2;
    w3d.wl_main.shootdelta = w3d.wl_main.viewwidth / 10;

    if w3d.wl_main.viewheight == w3d.id_vl.screenHeight {
        w3d.wl_main.viewscreenx = 0;
        w3d.wl_main.viewscreeny = 0;
        w3d.wl_main.screenofs = 0;
    } else {
        w3d.wl_main.viewscreenx = (w3d.id_vl.screenWidth - w3d.wl_main.viewwidth) / 2;
        w3d.wl_main.viewscreeny = (w3d.id_vl.screenHeight
            - w3d.id_vl.scaleFactor * STATUSLINES as i32
            - w3d.wl_main.viewheight)
            / 2;
        w3d.wl_main.screenofs =
            w3d.wl_main.viewscreeny * w3d.id_vl.screenWidth + w3d.wl_main.viewscreenx;
    }

    //
    // calculate trace angles and projection constants
    //
    CalcProjection(w3d, FOCALLENGTH as i32);

    return true;
}

pub fn ShowViewSize(w3d: &mut modules, width: i32) {
    //println!("ShowViewSize");

    let oldwidth: i32;
    let oldheight: i32;

    oldwidth = w3d.wl_main.viewwidth;
    oldheight = w3d.wl_main.viewheight;

    if width == 21 {
        w3d.wl_main.viewwidth = w3d.id_vl.screenWidth;
        w3d.wl_main.viewheight = w3d.id_vl.screenHeight;
        VWB_BarScaledCoord(w3d, 0, 0, w3d.id_vl.screenWidth, w3d.id_vl.screenHeight, 0);
    } else if width == 20 {
        w3d.wl_main.viewwidth = w3d.id_vl.screenWidth;
        w3d.wl_main.viewheight =
            w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * STATUSLINES as i32;
        DrawPlayBorder(w3d);
    } else {
        w3d.wl_main.viewwidth = width * 16 * w3d.id_vl.screenWidth / 320;
        w3d.wl_main.viewheight =
            width * (16 as f32 * HEIGHTRATIO) as i32 * w3d.id_vl.screenHeight / 200;
        DrawPlayBorder(w3d);
    }

    w3d.wl_main.viewwidth = oldwidth;
    w3d.wl_main.viewheight = oldheight;
}

pub fn NewViewSize(w3d: &mut modules, width: i32) {
    //println!("NewViewSize");

    w3d.wl_play.viewsize = width;
    if w3d.wl_play.viewsize == 21 {
        SetViewSize(w3d, w3d.id_vl.screenWidth, w3d.id_vl.screenHeight);
    } else if w3d.wl_play.viewsize == 20 {
        SetViewSize(
            w3d,
            w3d.id_vl.screenWidth,
            w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * STATUSLINES as i32,
        );
    } else {
        SetViewSize(
            w3d,
            width * 16 * w3d.id_vl.screenWidth / 320,
            width * (16 as f32 * HEIGHTRATIO) as i32 * w3d.id_vl.screenHeight / 200,
        );
    }
}

/*
==========================
=
= Quit
=
==========================
*/

pub fn Quit(error_str: &str) {
    //println!("Quit");

    println!("{}", error_str);
    exit(0);
}

/*
=====================
=
= DemoLoop
=
=====================
*/

pub fn DemoLoop(w3d: &mut modules, ob: &mut object) {
    //println!("DemoLoop");

    let mut LastDemo: usize = 0;

    //
    // check for launch from ted
    //
    if w3d.wl_main.param_tedlevel != -1 {
        w3d.wl_main.param_nowait = true;
        EnableEndGameMenuItem(w3d);
        NewGame(w3d, w3d.wl_main.param_difficulty as i32, 0);

        {
            w3d.wl_game.gamestate.episode = w3d.wl_main.param_tedlevel / 10;
            w3d.wl_game.gamestate.mapon = w3d.wl_main.param_tedlevel % 10;
        }

        GameLoop(w3d, ob);
        Quit("");
    }

    //
    // main game cycle
    //

    StartCPMusic(w3d, INTROSONG);

    if !w3d.wl_main.param_nowait {
        PG13(w3d);
    }

    loop {
        while !w3d.wl_main.param_nowait {
            VWB_DrawPic(w3d, 0, 0, graphicnums::TITLEPIC as i32);
            VW_UpdateScreen(w3d);
            VW_FadeIn(w3d);

            if IN_UserInput(w3d, TickBase * 15) {
                break;
            }
            VW_FadeOut(w3d);
            //
            // credits page
            //
            VWB_DrawPic(w3d, 0, 0, graphicnums::CREDITSPIC as i32);
            VW_UpdateScreen(w3d);
            VW_FadeIn(w3d);

            if IN_UserInput(w3d, TickBase * 10) {
                break;
            }
            VW_FadeOut(w3d);
            //
            // high scores
            //
            DrawHighScores(w3d);
            VW_UpdateScreen(w3d);
            VW_FadeIn(w3d);

            if IN_UserInput(w3d, TickBase * 10) {
                break;
            }
            //
            // demo
            //
            let demonumber = LastDemo % 4;
            LastDemo += 1;
            PlayDemo(w3d, ob, demonumber);

            if w3d.wl_play.playstate == exit_t::ex_abort {
                break;
            }
            VW_FadeOut(w3d);
            if w3d.id_vl.screenHeight % 200 != 0 {
                VL_ClearScreen(w3d, Color::BLACK);
            }
            StartCPMusic(w3d, INTROSONG);
        }

        VW_FadeOut(w3d);

        if Keyboard(w3d, Scancode::Tab) && w3d.wl_main.param_debugmode {
            RecordDemo();
        } else {
            //BUG Scancode
            US_ControlPanel(w3d, ob, Scancode::F24);
        }

        if w3d.wl_main.startgame || w3d.wl_main.loadedgame {
            GameLoop(w3d, ob);
            if !w3d.wl_main.param_nowait {
                VW_FadeOut(w3d);
                StartCPMusic(w3d, INTROSONG);
            }
        }
    }
}

//===========================================================================

pub fn CheckParameters(w3d: &mut modules) {
    //println!("CheckParameters");

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mut hasError: bool = false;
    let mut showHelp: bool = false;
    let mut sampleRateGiven: bool = false;
    let mut audioBufferGiven: bool = false;
    let defaultSampleRate: i32 = w3d.wl_main.param_samplerate;
    let mut i: usize = 1;

    loop {
        if argc == 1 {
            break;
        }

        if argv[i] == "--goobers" {
            w3d.wl_main.param_debugmode = true;
        } else if argv[i] == "--baby" {
            w3d.wl_main.param_difficulty = 0;
        } else if argv[i] == "--easy" {
            w3d.wl_main.param_difficulty = 1;
        } else if argv[i] == "--normal" {
            w3d.wl_main.param_difficulty = 2;
        } else if argv[i] == "--hard" {
            w3d.wl_main.param_difficulty = 3;
        } else if argv[i] == "--nowait" {
            w3d.wl_main.param_nowait = true;
        } else if argv[i] == "--tedlevel" {
            i += 1;
            if i >= argc {
                println!("The tedlevel option is missing the level argument!");
                hasError = true;
            } else {
                w3d.wl_main.param_tedlevel = argv[i].parse().unwrap_or(-1);
            }
        } else if argv[i] == "--windowed" {
            w3d.id_vl.fullscreen = false;
        } else if argv[i] == "--windowed-mouse" {
            w3d.id_vl.fullscreen = false;
            w3d.id_in.forcegrabmouse = true;
        } else if argv[i] == "--res" {
            if i + 2 >= argc {
                println!("The res option needs the width and/or the height argument!");
                hasError = true;
            } else {
                w3d.id_vl.screenWidth = argv[i + 1].parse().unwrap_or(640);
                w3d.id_vl.screenHeight = argv[i + 1].parse().unwrap_or(400);
                let factor = { w3d.id_vl.screenWidth / 320 };
                if ({ w3d.id_vl.screenWidth % 320 >= 1 })
                    || ({ w3d.id_vl.screenHeight != 200 * factor })
                        && ({ w3d.id_vl.screenHeight != 240 * factor })
                {
                    println!("Screen size must be a multiple of 320x200 or 320x240!");
                    hasError = true;
                }
            }
        } else if argv[i] == "--resf" {
            if i + 2 >= argc {
                println!("The resf option needs the width and/or the height argument!");
                hasError = true;
            } else {
                w3d.id_vl.screenWidth = argv[i + 1].parse().unwrap_or(640);
                w3d.id_vl.screenHeight = argv[i + 1].parse().unwrap_or(400);
                if w3d.id_vl.screenWidth < 320 {
                    println!("Screen width must be at least 320!");
                    hasError = true;
                }
                if w3d.id_vl.screenHeight < 200 {
                    println!("Screen height must be at least 200!");
                    hasError = true;
                }
            }
        } else if argv[i] == "--bits" {
            i += 1;
            if i >= argc {
                println!("The bits option is missing the color depth argument!");
                hasError = true;
            } else {
                w3d.id_vl.screenBits = argv[i].parse().unwrap_or(-1);
                match w3d.id_vl.screenBits {
                    8 => (),
                    16 => (),
                    24 => (),
                    32 => (),
                    _ => {
                        println!("Screen color depth must be 8, 16, 24, or 32!");
                        hasError = true;
                        break;
                    }
                }
            }
        } else if argv[i] == "--nodblbuf" {
            w3d.id_vl.usedoublebuffering = false
        } else if argv[i] == "--w3d.wl_play.extravbls" {
            i += 1;
            if i >= argc {
                println!("The w3d.wl_play.extravbls option is missing the vbls argument!");
                hasError = true;
            } else {
                w3d.wl_play.extravbls = argv[i].parse().unwrap_or(0);

                if w3d.wl_play.extravbls < 0 {
                    println!("w3d.wl_play.extravbls must be positive!");
                    hasError = true;
                }
            }
        } else if argv[i] == "--joystick" {
            if i + 1 >= argc {
                println!("The joystick option is missing the index argument!");
                hasError = true;
            } else {
                w3d.wl_main.param_joystickindex = argv[i].parse().unwrap_or(0);
                // index is checked in InitGame
            }
        } else if argv[i] == "--joystickhat" {
            i += 1;
            if i >= argc {
                println!("The joystickhat option is missing the index argument!");
                hasError = true;
            } else {
                w3d.wl_main.param_joystickhat = argv[i].parse().unwrap_or(-1);
            }
        } else if argv[i] == "--samplerate" {
            i += 1;
            if i >= argc {
                println!("The samplerate option is missing the rate argument!");
                hasError = true;
            } else {
                w3d.wl_main.param_samplerate = argv[i].parse().unwrap_or(44100);
                sampleRateGiven = true;
            }
        } else if argv[i] == "--audiobuffer" {
            i += 1;
            if i >= argc {
                println!("The audiobuffer option is missing the size argument!");
                hasError = true;
            } else {
                w3d.wl_main.param_audiobuffer = argv[i].parse().unwrap_or(2048);
                audioBufferGiven = true;
            }
        } else if argv[i] == "--mission" {
            i += 1;
            if i >= argc {
                println!("The mission option is missing the mission argument!");
                hasError = true;
            } else {
                w3d.wl_main.param_mission = argv[i].parse().unwrap_or(0);

                if w3d.wl_main.param_mission < 0 || w3d.wl_main.param_mission > 3 {
                    println!("The mission option must be between 0 and 3!");
                    hasError = true;
                }
            }
        } else if argv[i] == "--configdir" {
            i += 1;
            if i >= argc {
                println!("The configdir option is missing the dir argument!");
                hasError = true;
            } else {
                let len = argv[i].len();
                if len + 2 > { w3d.wl_main.configdir.len() } {
                    println!("The config directory is too long!");
                    hasError = true;
                } else {
                    w3d.wl_main.configdir.to_string().push_str(&argv[i]);
                    if argv[i].chars().nth(len) != Some('/')
                        && argv[i].chars().nth(len) != Some('\\')
                    {
                        w3d.wl_main.configdir.to_string().push('/');
                    }
                }
            }
        } else if argv[i] == "--goodtimes" {
            w3d.wl_main.param_goodtimes = true;
        } else if argv[i] == "--ignorenumchunks" {
            w3d.wl_main.param_ignorenumchunks = true;
        } else if argv[i] == "--help" {
            showHelp = true;
        } else {
            hasError = true;
        }

        i += 1;
        if i >= argc {
            break;
        }
    }

    if hasError || showHelp {
        if hasError {
            print!("\n");
            print!("Wolf4Rust v1.0.0\n");
            print!("Ported to Rust by Antonio Soares\n");
            print!("Ported by Chaos-Software, additions by the community\n");
            print!("Original Wolfenstein 3D by id Software\n\n");
            print!("Usage: Wolf4Rust [options]\n");
            print!("Options:\n");
            print!(" --help                 This help page\n");
            print!(" --tedlevel <level>     Starts the game in the given level\n");
            print!(" --baby                 Sets the difficulty to baby for tedlevel\n");
            print!(" --easy                 Sets the difficulty to easy for tedlevel\n");
            print!(" --normal               Sets the difficulty to normal for tedlevel\n");
            print!(" --hard                 Sets the difficulty to hard for tedlevel\n");
            print!(" --nowait               Skips intro screens\n");
            print!(" --windowed[-mouse]     Starts the game in a window [and grabs mouse]\n");
            print!(" --res <width> <height> Sets the screen resolution\n");
            print!("                        (must be multiple of 320x200 or 320x240)\n");
            print!(" --resf <w> <h>         Sets any screen resolution >= 320x200\n");
            print!("                        (which may result in graphic errors)\n");
            print!(" --bits <b>             Sets the screen color depth\n");
            print!("                        (use this when you have palette/fading problems\n");
            print!("                        allowed: 8, 16, 24, 32, default: \"best\" depth)\n");
            print!(" --nodblbuf             Don't use SDL's double buffering\n");
            print!(" --w3d.wl_play.extravbls <vbls>     Sets a delay after each frame, which may help to\n");
            print!(
                "                        reduce flickering (unit is currently 8 ms, default: 0)\n"
            );
            print!(" --joystick <index>     Use the index-th joystick if available\n");
            print!("                        (-1 to disable joystick, default: 0)\n");
            print!(" --joystickhat <index>  Enables movement with the given coolie hat\n");
            print!(
                " --samplerate <rate>    Sets the sound sample rate (given in Hz, default: {})\n",
                defaultSampleRate
            );
            print!(
                " --audiobuffer <size>   Sets the size of the audio buffer (-> sound latency)\n"
            );
            print!(
                "                        (given in bytes, default: 2048 / (44100 / samplerate))\n"
            );
            print!(" --ignorenumchunks      Ignores the number of chunks in VGAHEAD.*\n");
            print!("                        (may be useful for some broken mods)\n");
            print!(
                " --configdir <dir>      Directory where config file and save games are stored\n"
            );
            print!("                        (default: $HOME/.wolf4rust)\n");
            exit(0);
        }

        if sampleRateGiven && !audioBufferGiven {
            {
                w3d.wl_main.param_audiobuffer = 2048 / (44100 / w3d.wl_main.param_samplerate)
            };
        }
    }
}
