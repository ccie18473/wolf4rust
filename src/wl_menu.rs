#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_menu
//
//===========================================================================

pub struct wl_menu {
    pub EpisodeSelect: [u8; 6],

    pub SaveGamesAvail: [i32; 10],
    pub StartGame: i32,
    pub SoundStatus: i32,
    pub pickquick: i32,
    pub SaveGameNames: [String; 10],
    pub SaveName: String,

    pub lastgameon: i32,

    pub lastmusic: i32,
    pub color_hlite: [i32; 4],
    pub color_norml: [i32; 4],
    pub totalMousex: i32,
    pub totalMousey: i32,
}

impl wl_menu {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "UPLOAD")]
            EpisodeSelect: [1, 0, 0, 0, 0, 0],
            #[cfg(feature = "GOODTIMES")]
            EpisodeSelect: [1, 1, 1, 1, 1, 1],

            SaveGamesAvail: [0; 10],
            StartGame: 0,
            SoundStatus: 1,
            pickquick: 0,
            SaveGameNames: [EMPTY_STRING; 10],
            SaveName: String::from("savegam?."),

            lastgameon: 0,

            lastmusic: 0,
            color_hlite: [DEACTIVE, HIGHLIGHT, READHCOLOR, 0x67],
            color_norml: [DEACTIVE, TEXTCOLOR, READCOLOR, 0x6b],
            totalMousex: 0,
            totalMousey: 0,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const EMPTY_STRING: String = String::new();

#[cfg(feature = "UPLOAD")]
pub const STARTITEM: i32 = menuitems::readthis as i32;

#[cfg(feature = "GOODTIMES")]
pub const STARTITEM: i32 = menuitems::newgame as i32;

// ENDSTRx constants are defined in foreign.h

pub const endStrings: [&str; 9] = [
    ENDSTR1, ENDSTR2, ENDSTR3, ENDSTR4, ENDSTR5, ENDSTR6, ENDSTR7, ENDSTR8, ENDSTR9,
];

pub const BORDCOLOR: i32 = 0x29;
pub const BORD2COLOR: i32 = 0x23;
pub const DEACTIVE: i32 = 0x2b;
pub const BKGDCOLOR: i32 = 0x2d;
pub const STRIPE: i32 = 0x2c;

pub const READCOLOR: i32 = 0x4a;
pub const READHCOLOR: i32 = 0x47;
pub const VIEWCOLOR: i32 = 0x7f;
pub const TEXTCOLOR: i32 = 0x17;
pub const HIGHLIGHT: i32 = 0x13;

pub const MENUSONG: i32 = musicnames::WONDERIN_MUS as i32;
pub const INTROSONG: i32 = musicnames::NAZI_NOR_MUS as i32;

pub const SENSITIVE: i32 = 60;

pub const MENU_X: i32 = 76;
pub const MENU_Y: i32 = 55;
pub const MENU_W: i32 = 178;

#[cfg(any(feature = "UPLOAD"))]
pub const MENU_H: i32 = 13 * 10 + 6;
#[cfg(any(feature = "GOODTIMES"))]
pub const MENU_H: i32 = 13 * 9 + 6;

pub const SM_X: i32 = 48;
pub const SM_W: i32 = 250;

pub const SM_Y1: i32 = 20;
pub const SM_H1: i32 = 4 * 13 - 7;
pub const SM_Y2: i32 = SM_Y1 + 5 * 13;
pub const SM_H2: i32 = 4 * 13 - 7;
pub const SM_Y3: i32 = SM_Y2 + 5 * 13;
pub const SM_H3: i32 = 3 * 13 - 7;

pub const CTL_X: i32 = 24;
pub const CTL_Y: i32 = 86;

pub const CTL_W: i32 = 284;
pub const CTL_H: i32 = 60;

pub const LSM_X: i32 = 85;
pub const LSM_Y: i32 = 55;
pub const LSM_W: i32 = 175;
pub const LSM_H: i32 = 10 * 13 + 10;

pub const NM_X: i32 = 50;
pub const NM_Y: i32 = 100;
pub const NM_W: i32 = 225;
pub const NM_H: i32 = 13 * 4 + 15;

pub const NE_X: i32 = 10;
pub const NE_Y: i32 = 23;
pub const NE_W: i32 = 320 - NE_X * 2;
pub const NE_H: i32 = 200 - NE_Y * 2;

pub const CST_Y: i32 = 48;
pub const CST_START: i32 = 60;
pub const CST_SPC: i32 = 60;

pub struct CP_itemtype<'a> {
    pub active: i32,
    pub string: &'a str,
    pub routine: *mut routine_int,
}
pub struct CP_iteminfo {
    pub x: i32,
    pub y: i32,
    pub amount: i32,
    pub curpos: i32,
    pub indent: i32,
}

//BUG should have 10 entries
pub static mut MainMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: STR_NG,
        routine: CP_NewGame as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_SD,
        routine: CP_Sound as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_CL,
        routine: CP_Control as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_LG,
        routine: CP_LoadGame as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: STR_SG,
        routine: CP_SaveGame as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_CV,
        routine: CP_ChangeView as *mut routine_int,
    },
    #[cfg(feature = "UPLOAD")]
    CP_itemtype {
        active: 2,
        string: "Read This!",
        routine: CP_ReadThis as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_VS,
        routine: CP_ViewScores as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_BD,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_QT,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "GOODTIMES")]
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut MainItems: CP_iteminfo = CP_iteminfo {
    x: MENU_X,
    y: MENU_Y,
    #[cfg(feature = "UPLOAD")]
    amount: 10,
    #[cfg(feature = "GOODTIMES")]
    amount: 9,
    curpos: STARTITEM,
    indent: 24,
};

//BUG should have 12 entries
pub static mut SndMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: STR_NONE,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_PC,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_ALSB,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_NONE,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: STR_DISNEY,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_SB,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_NONE,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_ALSB,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut SndItems: CP_iteminfo = CP_iteminfo {
    x: SM_X,
    y: SM_Y1,
    amount: 12,
    curpos: 0,
    indent: 52,
};

//BUG should have 10 entries
pub static mut LSMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut LSItems: CP_iteminfo = CP_iteminfo {
    x: LSM_X,
    y: LSM_Y,
    amount: 10,
    curpos: 0,
    indent: 24,
};

pub enum CTL {
    CTL_MOUSEENABLE,
    CTL_MOUSESENS,
    CTL_JOYENABLE,
}

//BUG should have 4 entries
pub static mut CtlMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 0,
        string: STR_MOUSEEN,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: STR_SENS,
        routine: MouseSensitivity as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: STR_JOYEN,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_CUSTOM,
        routine: CustomControls as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_QT,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut CtlItems: CP_iteminfo = CP_iteminfo {
    x: CTL_X,
    y: CTL_Y,
    amount: 4,
    curpos: -1,
    indent: 56,
};

//BUG should have 9 entries
pub static mut CusMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut CusItems: CP_iteminfo = CP_iteminfo {
    x: 8,
    y: CST_Y + 13 * 2,
    amount: 9,
    curpos: -1,
    indent: 0,
};

//BUG should have 11 entries
pub static mut NewEmenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: "Episode 1\nEscape from Wolfenstein",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "GOODTIMES")]
    CP_itemtype {
        active: 1,
        string: "Episode 2\nOperation: Eisenfaust",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "UPLOAD")]
    CP_itemtype {
        active: 3,
        string: "Episode 2\nOperation: Eisenfaust",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "GOODTIMES")]
    CP_itemtype {
        active: 1,
        string: "Episode 3\nDie, Fuhrer, Die!",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "UPLOAD")]
    CP_itemtype {
        active: 3,
        string: "Episode 3\nDie, Fuhrer, Die!",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "GOODTIMES")]
    CP_itemtype {
        active: 1,
        string: "Episode 4\nA Dark Secret",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "UPLOAD")]
    CP_itemtype {
        active: 3,
        string: "Episode 4\nA Dark Secret",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "GOODTIMES")]
    CP_itemtype {
        active: 1,
        string: "Episode 5\nTrail of the Madman",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "UPLOAD")]
    CP_itemtype {
        active: 3,
        string: "Episode 5\nTrail of the Madman",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "GOODTIMES")]
    CP_itemtype {
        active: 1,
        string: "Episode 6\nConfrontation",
        routine: call_routine_int as *mut routine_int,
    },
    #[cfg(feature = "UPLOAD")]
    CP_itemtype {
        active: 3,
        string: "Episode 6\nConfrontation",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut NewEitems: CP_iteminfo = CP_iteminfo {
    x: NE_X,
    y: NE_Y,
    amount: 11,
    curpos: 0,
    indent: 88,
};

//BUG should have 4 entries
pub static mut NewMenu: [CP_itemtype; 18] = [
    CP_itemtype {
        active: 1,
        string: STR_DADDY,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_HURTME,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_BRINGEM,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 1,
        string: STR_DEATH,
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
    CP_itemtype {
        active: 0,
        string: "",
        routine: call_routine_int as *mut routine_int,
    },
];

pub static mut NewItems: CP_iteminfo = CP_iteminfo {
    x: NM_X,
    y: NM_Y,
    amount: 4,
    curpos: 2,
    indent: 24,
};

pub enum ORDER {
    FIRE,
    STRAFE,
    RUN,
    OPEN,
}

pub const mbarray: [&str; 4] = ["b0", "b1", "b2", "b3"];

pub const order: [u8; 4] = [
    ORDER::RUN as u8,
    ORDER::OPEN as u8,
    ORDER::FIRE as u8,
    ORDER::STRAFE as u8,
];

enum MOVE {
    FWRD,
    RIGHT,
    BKWD,
    LEFT,
}

pub const moveorder: [u8; 4] = [
    MOVE::LEFT as u8,
    MOVE::RIGHT as u8,
    MOVE::FWRD as u8,
    MOVE::BKWD as u8,
];

// static int

pub static mut redrawitem: i32 = 1;
pub static mut lastitem: i32 = -1;

pub static mut lastwhich: i32 = -1;

//===========================================================================
//  Function pointers
//===========================================================================

pub type routine_void = fn(w3d: &mut modules, which: i32);
pub type routine_int = fn(w3d: &mut modules, which: i32) -> i32;

pub fn call_routine_void(_w3d: &mut modules, _which: i32) {
    //
}

pub fn call_routine_int(_w3d: &mut modules, _which: i32) -> i32 {
    return 0;
}

pub enum menuitems {
    newgame,
    soundmenu,
    control,
    loadgame,
    savegame,
    changeview,
    #[cfg(feature = "UPLOAD")]
    readthis,
    viewscores,
    backtodemo,
    quit,
}

impl menuitems {
    pub fn from_i32(value: i32) -> menuitems {
        match value {
            -1 => menuitems::quit,
            0 => menuitems::newgame,
            1 => menuitems::soundmenu,
            2 => menuitems::control,
            3 => menuitems::loadgame,
            4 => menuitems::savegame,
            5 => menuitems::changeview,
            #[cfg(feature = "UPLOAD")]
            6 => menuitems::readthis,
            #[cfg(feature = "UPLOAD")]
            7 => menuitems::viewscores,
            #[cfg(feature = "UPLOAD")]
            8 => menuitems::backtodemo,
            #[cfg(feature = "UPLOAD")]
            9 => menuitems::quit,
            #[cfg(feature = "GOODTIMES")]
            6 => menuitems::viewscores,
            #[cfg(feature = "GOODTIMES")]
            7 => menuitems::backtodemo,
            #[cfg(feature = "GOODTIMES")]
            8 => menuitems::quit,

            _ => menuitems::backtodemo,
        }
    }
}

//
// WL_INTER
//
#[derive(Clone, Copy)]
pub struct LRstruct {
    pub kill: usize,
    pub secret: usize,
    pub treasure: usize,
    pub time: usize,
}

pub fn MenuFadeOut(w3d: &mut modules) {
    //println!("MenuFadeOut");

    VL_FadeOut(w3d, 0, 255, 0, 0, 0, 10);
}

pub fn MenuFadeIn(w3d: &mut modules) {
    //println!("MenuFadeIn");

    VL_FadeIn(w3d, 0, 255, w3d.id_vl.gamepal, 10);
}

////////////////////////////////////////////////////////////////////
//
// Wolfenstein Control Panel!  Ta Da!
//
////////////////////////////////////////////////////////////////////

pub fn US_ControlPanel(w3d: &mut modules, ob: &mut object, scancode: Scancode) {
    //println!("US_ControlPanel");

    let mut which: i32;

    if w3d.wl_game.ingame {
        if CP_CheckQuick(scancode) != 0 {
            return;
        }
        w3d.wl_play.lastgamemusicoffset = StartCPMusic(w3d, MENUSONG);
    } else {
        StartCPMusic(w3d, MENUSONG as i32);
    }
    SetupControlPanel(w3d);

    //
    // F-KEYS FROM WITHIN GAME
    //
    let mut finishup: bool = false;
    match scancode {
        Scancode::F1 => {
            #[cfg(feature = "GOODTIMES")]
            BossKey();
            #[cfg(feature = "UPLOAD")]
            HelpScreens(w3d);
            //goto finishup;
            finishup = true;
        }

        Scancode::F2 => {
            CP_SaveGame(w3d, 0);
            //goto finishup;
            finishup = true;
        }

        Scancode::F3 => {
            CP_LoadGame(w3d, 0);
            //goto finishup;
            finishup = true;
        }

        Scancode::F4 => {
            CP_Sound(w3d, 0);
            //goto finishup;
            finishup = true;
        }

        Scancode::F5 => {
            CP_ChangeView(w3d, 0);
            //goto finishup;
            finishup = true;
        }

        Scancode::F6 => {
            CP_Control(w3d, 0);
            //goto finishup;
            finishup = true;
        }

        _ => (),
    }

    if finishup {
        CleanupControlPanel(w3d);
        return;
    }

    DrawMainMenu(w3d);
    MenuFadeIn(w3d);
    w3d.wl_menu.StartGame = 0;

    //
    // MAIN MENU LOOP
    //
    loop {
        which = HandleMenu(
            w3d,
            unsafe { &mut MainItems },
            unsafe { &MainMenu },
            call_routine_void,
        );

        let menuitem = menuitems::from_i32(which);

        match menuitem {
            menuitems::viewscores => {
                if unsafe { MainMenu[menuitems::viewscores as usize].routine == ptr::null_mut() } {
                    if CP_EndGame(w3d, ob, 0) {
                        w3d.wl_menu.StartGame = 1;
                    }
                } else {
                    DrawMainMenu(w3d);
                    MenuFadeIn(w3d);
                }
            }

            menuitems::backtodemo => {
                w3d.wl_menu.StartGame = 1;
                if !w3d.wl_game.ingame {
                    StartCPMusic(w3d, INTROSONG);
                }
                VL_FadeOut(w3d, 0, 255, 0, 0, 0, 10);
            }

            menuitems::quit => {
                CP_Quit(w3d, 0);
            }
            _ => {
                if w3d.wl_menu.StartGame == 0 {
                    DrawMainMenu(w3d);
                    MenuFadeIn(w3d);
                }
            }
        }

        //
        // "EXIT OPTIONS" OR "NEW GAME" EXITS
        //
        if w3d.wl_menu.StartGame != 0 {
            break;
        }
    }

    //
    // DEALLOCATE EVERYTHING
    //
    CleanupControlPanel(w3d);

    //
    // CHANGE MAINMENU ITEM
    //
    if w3d.wl_main.startgame || w3d.wl_main.loadedgame {
        EnableEndGameMenuItem(w3d);
    }
}

pub fn EnableEndGameMenuItem(_w3d: &mut modules) {
    //println!("EnableEndGameMenuItem");

    unsafe { MainMenu[menuitems::viewscores as usize].routine = ptr::null_mut() };

    unsafe { MainMenu[menuitems::viewscores as usize].string = STR_EG };
}

////////////////////////
//
// DRAW MAIN MENU SCREEN
//

pub fn DrawMainMenu(w3d: &mut modules) {
    //println!("DrawMainMenu");

    {
        ClearMScreen(w3d);

        VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);
        DrawStripes(w3d, 10);
        VWB_DrawPic(w3d, 84, 0, graphicnums::C_OPTIONSPIC as i32);

        DrawWindow(w3d, MENU_X - 8, MENU_Y - 3, MENU_W, MENU_H, BKGDCOLOR);

        //
        // CHANGE "GAME" AND "DEMO"
        //
        if w3d.wl_game.ingame {
            unsafe { MainMenu[menuitems::backtodemo as usize].string = STR_GAME };
            unsafe { MainMenu[menuitems::backtodemo as usize].active = 2 };
        } else {
            unsafe { MainMenu[menuitems::backtodemo as usize].string = STR_DEMO };
            unsafe { MainMenu[menuitems::backtodemo as usize].active = 1 };
        }

        unsafe { DrawMenu(w3d, &MainItems, &MainMenu) };
        VW_UpdateScreen(w3d);
    }
}

////////////////////////////////////////////////////////////////////
//
// READ THIS!
//
////////////////////////////////////////////////////////////////////

#[cfg(feature = "UPLOAD")]
pub fn CP_ReadThis(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_ReadThis");

    StartCPMusic(w3d, musicnames::CORNER_MUS as i32);
    HelpScreens(w3d);
    StartCPMusic(w3d, MENUSONG);

    return 1;
}

////////////////////////////////////////////////////////////////////
//
// BOSS KEY
//
////////////////////////////////////////////////////////////////////

#[cfg(feature = "GOODTIMES")]
pub fn BossKey() {
    //println!("BossKey");
}

////////////////////////////////////////////////////////////////////
//
// CHECK QUICK-KEYS & QUIT (WHILE IN A GAME)
//
////////////////////////////////////////////////////////////////////

pub fn CP_CheckQuick(_scancode: Scancode) -> i32 {
    //println!("CP_CheckQuick");

    return 0;
}

////////////////////////////////////////////////////////////////////
//
// END THE CURRENT GAME
//
////////////////////////////////////////////////////////////////////

pub fn CP_EndGame(w3d: &mut modules, ob: &mut object, _blank: i32) -> bool {
    //println!("CP_EndGame");

    let res: i32;

    res = Confirm(w3d, ENDGAMESTR.to_string());

    DrawMainMenu(w3d);
    if res == 0 {
        return false;
    }

    w3d.wl_menu.pickquick = 0;
    w3d.wl_game.gamestate.lives = 0;
    w3d.wl_play.playstate = exit_t::ex_died;
    ob.killerobj = objtype::new();

    unsafe { MainMenu[menuitems::savegame as usize].active = 0 };
    unsafe { MainMenu[menuitems::viewscores as usize].routine = CP_ViewScores as *mut routine_int };

    unsafe { MainMenu[menuitems::viewscores as usize].string = STR_VS };

    return true;
}

////////////////////////////////////////////////////////////////////
//
// VIEW THE HIGH SCORES
//
////////////////////////////////////////////////////////////////////

pub fn CP_ViewScores(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_ViewScores");

    w3d.id_vh.fontnumber = 0;

    StartCPMusic(w3d, musicnames::ROSTER_MUS as i32);

    DrawHighScores(w3d);
    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);
    w3d.id_vh.fontnumber = 1;

    IN_Ack(w3d);

    StartCPMusic(w3d, MENUSONG);
    MenuFadeOut(w3d);

    return 0;
}

////////////////////////////////////////////////////////////////////
//
// START A NEW GAME
//
////////////////////////////////////////////////////////////////////

pub fn CP_NewGame(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_NewGame");

    let mut which: i32;
    let mut episode: i32 = 0;

    'outer: loop {
        //firstpart:

        DrawNewEpisode(w3d);

        'inner: loop {
            which = HandleMenu(
                w3d,
                unsafe { &mut NewEitems },
                unsafe { &NewEmenu },
                call_routine_void,
            );
            match which {
                -1 => {
                    MenuFadeOut(w3d);
                    return 0;
                }

                _ => {
                    if w3d.wl_menu.EpisodeSelect[(which / 2) as usize] == 0 {
                        SD_PlaySound(w3d, soundnames::NOWAYSND);
                        Message (w3d,"Please select \"Read This!\"\nfrom the Options menu to\nfind out how to order this\nepisode from Apogee.".to_string());
                        IN_ClearKeysDown(w3d);
                        IN_Ack(w3d);
                        DrawNewEpisode(w3d);
                        which = 0;
                    } else {
                        episode = which / 2;
                        which = 1;
                    }
                }
            }
            if which != 0 {
                break 'inner;
            }
        }

        ShootSnd(w3d);

        //
        // ALREADY IN A GAME?
        //
        if w3d.wl_game.ingame {
            if Confirm(w3d, CURGAME.to_string()) == 0 {
                MenuFadeOut(w3d);
                return 0;
            }
        }

        MenuFadeOut(w3d);

        DrawNewGame(w3d);
        which = HandleMenu(
            w3d,
            unsafe { &mut NewItems },
            unsafe { &NewMenu },
            DrawNewGameDiff,
        );
        if which < 0 {
            MenuFadeOut(w3d);
            //goto firstpart;
        } else {
            break 'outer;
        }
    }

    ShootSnd(w3d);

    NewGame(w3d, which, episode);
    w3d.wl_menu.StartGame = 1;
    MenuFadeOut(w3d);

    //
    // CHANGE "READ THIS!" TO NORMAL COLOR
    //
    #[cfg(feature = "UPLOAD")]
    unsafe {
        MainMenu[menuitems::readthis as usize].active = 1
    };

    w3d.wl_menu.pickquick = 0;

    return 0;
}

/////////////////////
//
// DRAW NEW EPISODE MENU
//

pub fn DrawNewEpisode(w3d: &mut modules) {
    //println!("DrawNewEpisode");

    ClearMScreen(w3d);
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);

    DrawWindow(w3d, NE_X - 4, NE_Y - 4, NE_W + 8, NE_H + 8, BKGDCOLOR);
    SETFONTCOLOR(w3d, READHCOLOR, BKGDCOLOR);
    w3d.id_us.PrintY = 2;
    w3d.id_us.WindowX = 0;

    US_CPrint(w3d, "Which episode to play?".to_string());

    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);
    DrawMenu(w3d, unsafe { &mut NewEitems }, unsafe { &NewEmenu });

    for i in 0..6 {
        VWB_DrawPic(
            w3d,
            NE_X + 32,
            NE_Y + i * 26,
            graphicnums::C_EPISODE1PIC as i32 + i,
        );
    }

    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);
    WaitKeyUp(w3d);
}

/////////////////////
//
// DRAW NEW GAME MENU
//

pub fn DrawNewGame(w3d: &mut modules) {
    //println!("DrawNewGame");

    ClearMScreen(w3d);
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);

    SETFONTCOLOR(w3d, READHCOLOR, BKGDCOLOR);
    w3d.id_us.PrintX = NM_X + 20;
    w3d.id_us.PrintY = NM_Y - 32;

    US_Print(w3d, "How tough are you?".to_string());

    DrawWindow(w3d, NM_X - 5, NM_Y - 10, NM_W, NM_H, BKGDCOLOR);

    DrawMenu(w3d, unsafe { &mut NewItems }, unsafe { &NewMenu });
    DrawNewGameDiff(w3d, unsafe { NewItems.curpos });
    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);
    WaitKeyUp(w3d);
}

////////////////////////
//
// DRAW NEW GAME GRAPHIC
//

pub fn DrawNewGameDiff(w3d: &mut modules, w: i32) {
    //println!("DrawNewGameDiff");

    VWB_DrawPic(
        w3d,
        NM_X + 185,
        NM_Y + 7,
        w + graphicnums::C_BABYMODEPIC as i32,
    );
}

////////////////////////////////////////////////////////////////////
//
// HANDLE SOUND MENU
//
////////////////////////////////////////////////////////////////////

pub fn CP_Sound(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_Sound");

    let mut which: i32;

    DrawSoundMenu(w3d);
    MenuFadeIn(w3d);
    WaitKeyUp(w3d);

    loop {
        which = HandleMenu(
            w3d,
            unsafe { &mut SndItems },
            unsafe { &SndMenu },
            call_routine_void,
        );
        //
        // HANDLE MENU CHOICES
        //
        match which {
            //
            // SOUND EFFECTS
            //
            0 => {
                if w3d.id_sd.SoundMode != SDMode::sdm_Off {
                    SD_WaitSoundDone(w3d);
                    SD_SetSoundMode(w3d, SDMode::sdm_Off);
                    DrawSoundMenu(w3d);
                }
            }
            1 => {
                if w3d.id_sd.SoundMode != SDMode::sdm_PC {
                    SD_WaitSoundDone(w3d);
                    SD_SetSoundMode(w3d, SDMode::sdm_PC);
                    CA_LoadAllSounds(w3d);
                    DrawSoundMenu(w3d);
                    ShootSnd(w3d);
                }
            }
            2 => {
                if w3d.id_sd.SoundMode != SDMode::sdm_AdLib {
                    SD_WaitSoundDone(w3d);
                    SD_SetSoundMode(w3d, SDMode::sdm_AdLib);
                    CA_LoadAllSounds(w3d);
                    DrawSoundMenu(w3d);
                    ShootSnd(w3d);
                }
            }

            //
            // DIGITIZED SOUND
            //
            5 => {
                if w3d.id_sd.DigiMode != SDSMode::sds_Off {
                    SD_SetDigiDevice(w3d, SDSMode::sds_Off);
                    DrawSoundMenu(w3d);
                }
            }
            6 => {
                /*
                if (DigiMode != sds_SoundSource)
                {
                    SD_SetDigiDevice (sds_SoundSource);
                    DrawSoundMenu ();
                    ShootSnd ();
                }
                */
            }
            7 => {
                if w3d.id_sd.DigiMode != SDSMode::sds_SoundBlaster {
                    SD_SetDigiDevice(w3d, SDSMode::sds_SoundBlaster);
                    DrawSoundMenu(w3d);
                    ShootSnd(w3d);
                }
            }

            //
            // MUSIC
            //
            10 => {
                if w3d.id_sd.MusicMode != SMMode::smm_Off {
                    SD_SetMusicMode(w3d, SMMode::smm_Off);
                    DrawSoundMenu(w3d);
                    ShootSnd(w3d);
                }
            }
            11 => {
                if w3d.id_sd.MusicMode != SMMode::smm_AdLib {
                    SD_SetMusicMode(w3d, SMMode::smm_AdLib);
                    DrawSoundMenu(w3d);
                    ShootSnd(w3d);
                    StartCPMusic(w3d, MENUSONG);
                }
            }
            _ => (),
        }

        if which < 0 {
            break;
        }
    }

    MenuFadeOut(w3d);

    return 0;
}

//////////////////////
//
// DRAW THE SOUND MENU
//

pub fn DrawSoundMenu(w3d: &mut modules) {
    //println!("DrawSoundMenu");

    let mut on: i32;

    //
    // DRAW SOUND MENU
    //
    ClearMScreen(w3d);
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);

    DrawWindow(w3d, SM_X - 8, SM_Y1 - 3, SM_W, SM_H1, BKGDCOLOR);
    DrawWindow(w3d, SM_X - 8, SM_Y2 - 3, SM_W, SM_H2, BKGDCOLOR);
    DrawWindow(w3d, SM_X - 8, SM_Y3 - 3, SM_W, SM_H3, BKGDCOLOR);

    //
    // IF NO ADLIB, NON-CHOOSENESS!
    //
    if !w3d.id_sd.AdLibPresent && !w3d.id_sd.SoundBlasterPresent {
        unsafe { SndMenu[2].active = 0 };
        unsafe { SndMenu[10].active = 0 };
        unsafe { SndMenu[11].active = 0 };
    }

    if !w3d.id_sd.SoundBlasterPresent {
        unsafe {
            SndMenu[7].active = 0;
        }
    }

    if !w3d.id_sd.SoundBlasterPresent {
        unsafe {
            SndMenu[5].active = 0;
        }
    }

    DrawMenu(w3d, unsafe { &SndItems }, unsafe { &SndMenu });

    VWB_DrawPic(w3d, 100, SM_Y1 - 20, graphicnums::C_FXTITLEPIC as i32);
    VWB_DrawPic(w3d, 100, SM_Y2 - 20, graphicnums::C_DIGITITLEPIC as i32);
    VWB_DrawPic(w3d, 100, SM_Y3 - 20, graphicnums::C_MUSICTITLEPIC as i32);

    for i in 0..unsafe { SndItems.amount } {
        if unsafe { SndMenu[i as usize].string != "" } {
            //
            // DRAW SELECTED/NOT SELECTED GRAPHIC BUTTONS
            //
            on = 0;
            match i {
                //
                // SOUND EFFECTS
                //
                0 => {
                    if w3d.id_sd.SoundMode == SDMode::sdm_Off {
                        on = 1;
                    }
                }
                1 => {
                    if w3d.id_sd.SoundMode == SDMode::sdm_PC {
                        on = 1;
                    }
                }
                2 => {
                    if w3d.id_sd.SoundMode == SDMode::sdm_AdLib {
                        on = 1;
                    }
                }

                //
                // DIGITIZED SOUND
                //
                5 => {
                    if w3d.id_sd.DigiMode == SDSMode::sds_Off {
                        on = 1;
                    }
                }
                6 => {
                    /*
                    if w3d.id_sd.DigiMode == SDSMode::sds_SoundSource {
                        on = 1;
                    }
                    */
                }
                7 => {
                    if w3d.id_sd.DigiMode == SDSMode::sds_SoundBlaster {
                        on = 1;
                    }
                }

                //
                // MUSIC
                //
                10 => {
                    if w3d.id_sd.MusicMode == SMMode::smm_Off {
                        on = 1;
                    }
                }
                11 => {
                    if w3d.id_sd.MusicMode == SMMode::smm_AdLib {
                        on = 1;
                    }
                }
                _ => (),
            }

            if on != 0 {
                VWB_DrawPic(
                    w3d,
                    SM_X + 24,
                    SM_Y1 + i * 13 + 2,
                    graphicnums::C_SELECTEDPIC as i32,
                );
            } else {
                VWB_DrawPic(
                    w3d,
                    SM_X + 24,
                    SM_Y1 + i * 13 + 2,
                    graphicnums::C_NOTSELECTEDPIC as i32,
                );
            }
        }
    }

    DrawMenuGun(w3d, unsafe { &SndItems });
    VW_UpdateScreen(w3d);
}

//
// DRAW LOAD/SAVE IN PROGRESS
//

pub fn DrawLSAction(w3d: &mut modules, which: i32) {
    //println!("DrawLSAction");

    let LSA_X: i32 = 96;
    let LSA_Y: i32 = 80;
    let LSA_W: i32 = 130;
    let LSA_H: i32 = 42;

    DrawWindow(w3d, LSA_X, LSA_Y, LSA_W, LSA_H, TEXTCOLOR);
    DrawOutline(w3d, LSA_X, LSA_Y, LSA_W, LSA_H, 0, HIGHLIGHT);
    VWB_DrawPic(
        w3d,
        LSA_X + 8,
        LSA_Y + 5,
        graphicnums::C_DISKLOADING1PIC as i32,
    );

    w3d.id_vh.fontnumber = 1;
    SETFONTCOLOR(w3d, 0, TEXTCOLOR);
    w3d.id_us.PrintX = LSA_X + 46;
    w3d.id_us.PrintY = LSA_Y + 13;

    if which != 0 {
        let mut s = STR_LOADING.to_string();
        s.push_str("...");
        US_Print(w3d, s);
    } else {
        let mut s = STR_SAVING.to_string();
        s.push_str("...");
        US_Print(w3d, s);
    }

    VW_UpdateScreen(w3d);
}

////////////////////////////////////////////////////////////////////
//
// LOAD SAVED GAMES
//
////////////////////////////////////////////////////////////////////

pub fn CP_LoadGame(w3d: &mut modules, quick: i32) -> i32 {
    //println!("CP_LoadGame");

    //FILE *file;
    let mut which: i32;
    let mut exit: i32 = 0;
    //char name[13];
    //char loadpath[300];

    //strcpy (name, SaveName);

    //
    // QUICKLOAD?
    //
    if quick != 0 {
        which = unsafe { LSItems.curpos };

        if w3d.wl_menu.SaveGamesAvail[which as usize] != 0 {
            /*
            name[7] = which + '0';

            if(configdir[0])
                snprintf(loadpath, sizeof(loadpath), "%s/%s", configdir, name);
            else
                strcpy(loadpath, name);

            file = fopen (loadpath, "rb");
            fseek (file, 32, SEEK_SET);
            loadedgame = true;
            LoadTheGame (file, 0, 0);
            loadedgame = false;
            fclose (file);
            */

            DrawFace(w3d);
            DrawHealth(w3d);
            DrawLives(w3d);
            DrawLevel(w3d);
            DrawAmmo(w3d);
            DrawKeys(w3d);
            DrawWeapon(w3d);
            DrawScore(w3d);
            ContinueMusic(w3d, w3d.wl_play.lastgamemusicoffset);
            return 1;
        }
    }

    DrawLoadSaveScreen(w3d, 0);

    loop {
        which = HandleMenu(
            w3d,
            unsafe { &mut LSItems },
            unsafe { &LSMenu },
            TrackWhichGame,
        );

        if which >= 0 && w3d.wl_menu.SaveGamesAvail[which as usize] != 0 {
            ShootSnd(w3d);
            //name[7] = which + '0';

            if w3d.wl_main.configdir != "" {
                //snprintf(loadpath, sizeof(loadpath), "%s/%s", configdir, name);
            } else {
                //strcpy(loadpath, name);
            }

            //file = fopen (loadpath, "rb");
            //fseek (file, 32, SEEK_SET);

            DrawLSAction(w3d, 0);
            w3d.wl_main.loadedgame = true;

            //LoadTheGame (file, LSA_X + 8, LSA_Y + 5);
            //fclose (file);

            w3d.wl_menu.StartGame = 1;
            ShootSnd(w3d);
            //
            // CHANGE "READ THIS!" TO NORMAL COLOR
            //
            #[cfg(feature = "UPLOAD")]
            unsafe {
                MainMenu[menuitems::readthis as usize].active = 1
            };

            exit = 1;
            break;
        }
        if which < 0 {
            break;
        }
    }

    MenuFadeOut(w3d);

    return exit;
}

///////////////////////////////////
//
// HIGHLIGHT CURRENT SELECTED ENTRY
//

pub fn TrackWhichGame(w3d: &mut modules, w: i32) {
    //println!("TrackWhichGame");

    //static int lastgameon = 0;

    PrintLSEntry(w3d, w3d.wl_menu.lastgameon, TEXTCOLOR);
    PrintLSEntry(w3d, w, HIGHLIGHT);

    w3d.wl_menu.lastgameon = w;
}

////////////////////////////
//
// DRAW THE LOAD/SAVE SCREEN
//

pub fn DrawLoadSaveScreen(w3d: &mut modules, loadsave: i32) {
    //println!("DrawLoadSaveScreen");

    ClearMScreen(w3d);
    w3d.id_vh.fontnumber = 1;
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);
    DrawWindow(w3d, LSM_X - 10, LSM_Y - 5, LSM_W, LSM_H, BKGDCOLOR);
    DrawStripes(w3d, 10);

    if loadsave == 0 {
        VWB_DrawPic(w3d, 60, 0, graphicnums::C_LOADGAMEPIC as i32);
    } else {
        VWB_DrawPic(w3d, 60, 0, graphicnums::C_SAVEGAMEPIC as i32);
    }

    for i in 0..10 {
        PrintLSEntry(w3d, i, TEXTCOLOR);
    }

    DrawMenu(w3d, unsafe { &mut LSItems }, unsafe { &LSMenu });
    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);
    WaitKeyUp(w3d);
}

///////////////////////////////////////////
//
// PRINT LOAD/SAVE GAME ENTRY W/BOX OUTLINE
//

pub fn PrintLSEntry(w3d: &mut modules, w: i32, color: i32) {
    //println!("PrintLSEntry");

    SETFONTCOLOR(w3d, color, BKGDCOLOR);
    DrawOutline(
        w3d,
        LSM_X + unsafe { LSItems.indent },
        LSM_Y + w * 13,
        LSM_W - unsafe { LSItems.indent } - 15,
        11,
        color,
        color,
    );
    w3d.id_us.PrintX = LSM_X + unsafe { LSItems.indent } + 2;
    w3d.id_us.PrintY = LSM_Y + w * 13 + 1;
    w3d.id_vh.fontnumber = 0;

    if w3d.wl_menu.SaveGamesAvail[w as usize] != 0 {
        let s = w3d.wl_menu.SaveGameNames[w as usize].clone();
        US_Print(w3d, s);
    } else {
        let mut s = "      - ".to_string();
        s.push_str(STR_EMPTY);
        s.push_str(" -");
        US_Print(w3d, s);
    }

    w3d.id_vh.fontnumber = 1;
}

////////////////////////////////////////////////////////////////////
//
// SAVE CURRENT GAME
//
////////////////////////////////////////////////////////////////////

pub fn CP_SaveGame(_w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_SaveGame");

    return 0;
}

////////////////////////////////////////////////////////////////////
//
// DEFINE CONTROLS
//
////////////////////////////////////////////////////////////////////

pub fn CP_Control(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_Control");

    let mut which: i32;

    DrawCtlScreen(w3d);
    MenuFadeIn(w3d);
    WaitKeyUp(w3d);

    loop {
        which = HandleMenu(
            w3d,
            unsafe { &mut CtlItems },
            unsafe { &CtlMenu },
            call_routine_void,
        );
        match which {
            //CTL::CTL_MOUSEENABLE
            0 => {
                w3d.wl_play.mouseenabled ^= 1;
                if IN_IsInputGrabbed(w3d) {
                    IN_CenterMouse(w3d);
                }
                DrawCtlScreen(w3d);
                unsafe { CusItems.curpos = -1 };
                ShootSnd(w3d);
            }

            //CTL::CTL_JOYENABLE
            2 => {
                w3d.wl_play.joystickenabled ^= 1;
                DrawCtlScreen(w3d);
                unsafe { CusItems.curpos = -1 };
                ShootSnd(w3d);
            }

            //CTL::CTL_MOUSESENS | CTL::CTL_CUSTOMIZE
            1 | 3 => {
                DrawCtlScreen(w3d);
                MenuFadeIn(w3d);
                WaitKeyUp(w3d);
            }
            _ => (),
        }
        if which < 0 {
            break;
        }
    }

    MenuFadeOut(w3d);

    return 0;
}

////////////////////////////////
//
// DRAW MOUSE SENSITIVITY SCREEN
//

pub fn DrawMouseSens(w3d: &mut modules) {
    //println!("DrawMouseSens");

    ClearMScreen(w3d);
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);
    DrawWindow(w3d, 10, 80, 300, 30, BKGDCOLOR);

    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowW = 320;
    w3d.id_us.PrintY = 82;
    SETFONTCOLOR(w3d, READCOLOR, BKGDCOLOR);
    US_CPrint(w3d, STR_MOUSEADJ.to_string());

    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);
    w3d.id_us.PrintX = 14;
    w3d.id_us.PrintY = 95;
    US_Print(w3d, STR_SLOW.to_string());
    w3d.id_us.PrintX = 269;
    US_Print(w3d, STR_FAST.to_string());

    VWB_Bar(w3d, 60, 97, 200, 10, TEXTCOLOR);
    DrawOutline(w3d, 60, 97, 200, 10, 0, HIGHLIGHT);
    DrawOutline(
        w3d,
        60 + 20 * w3d.wl_main.mouseadjustment,
        97,
        20,
        10,
        0,
        READCOLOR,
    );
    VWB_Bar(
        w3d,
        61 + 20 * w3d.wl_main.mouseadjustment,
        98,
        19,
        9,
        READHCOLOR,
    );

    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);
}

///////////////////////////
//
// ADJUST MOUSE SENSITIVITY
//

pub fn MouseSensitivity(w3d: &mut modules) -> i32 {
    //println!("MouseSensitivity");

    let mut ci: ControlInfo = ControlInfo::new();
    let mut exit: i32 = 0;
    let oldMA: i32;

    oldMA = w3d.wl_main.mouseadjustment;
    DrawMouseSens(w3d);

    loop {
        SDL_Delay(w3d, 5);
        ReadAnyControl(w3d, &mut ci);

        match ci.dir {
            Direction::dir_North | Direction::dir_West => {
                if w3d.wl_main.mouseadjustment != 0 {
                    w3d.wl_main.mouseadjustment -= 1;
                    VWB_Bar(w3d, 60, 97, 200, 10, TEXTCOLOR);
                    DrawOutline(w3d, 60, 97, 200, 10, 0, HIGHLIGHT);
                    DrawOutline(
                        w3d,
                        60 + 20 * w3d.wl_main.mouseadjustment,
                        97,
                        20,
                        10,
                        0,
                        READCOLOR,
                    );
                    VWB_Bar(
                        w3d,
                        61 + 20 * w3d.wl_main.mouseadjustment,
                        98,
                        19,
                        9,
                        READHCOLOR,
                    );
                    VW_UpdateScreen(w3d);
                    SD_PlaySound(w3d, soundnames::MOVEGUN1SND);
                    TicDelay(w3d, 20);
                }
            }

            Direction::dir_South | Direction::dir_East => {
                if w3d.wl_main.mouseadjustment < 9 {
                    w3d.wl_main.mouseadjustment += 1;
                    VWB_Bar(w3d, 60, 97, 200, 10, TEXTCOLOR);
                    DrawOutline(w3d, 60, 97, 200, 10, 0, HIGHLIGHT);
                    DrawOutline(
                        w3d,
                        60 + 20 * w3d.wl_main.mouseadjustment,
                        97,
                        20,
                        10,
                        0,
                        READCOLOR,
                    );
                    VWB_Bar(
                        w3d,
                        61 + 20 * w3d.wl_main.mouseadjustment,
                        98,
                        19,
                        9,
                        READHCOLOR,
                    );
                    VW_UpdateScreen(w3d);
                    SD_PlaySound(w3d, soundnames::MOVEGUN1SND);
                    TicDelay(w3d, 20);
                }
            }
            _ => (),
        }

        if ci.button0 != 0 || Keyboard(w3d, Scancode::Space) || Keyboard(w3d, Scancode::Return) {
            exit = 1;
        } else if ci.button1 != 0 || Keyboard(w3d, Scancode::Escape) {
            exit = 2;
        }

        if exit != 0 {
            break;
        }
    }

    if exit == 2 {
        w3d.wl_main.mouseadjustment = oldMA;
        SD_PlaySound(w3d, soundnames::ESCPRESSEDSND);
    } else {
        SD_PlaySound(w3d, soundnames::SHOOTSND);
    }
    WaitKeyUp(w3d);
    MenuFadeOut(w3d);

    return 0;
}

///////////////////////////
//
// DRAW CONTROL MENU SCREEN
//

pub fn DrawCtlScreen(w3d: &mut modules) {
    //println!("DrawCtlScreen");

    let x: i32;
    let mut y: i32;

    ClearMScreen(w3d);
    DrawStripes(w3d, 10);
    VWB_DrawPic(w3d, 80, 0, graphicnums::C_CONTROLPIC as i32);
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);
    DrawWindow(w3d, CTL_X - 8, CTL_Y - 5, CTL_W, CTL_H, BKGDCOLOR);

    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowW = 320;
    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);

    if IN_JoyPresent(w3d) {
        unsafe { CtlMenu[CTL::CTL_JOYENABLE as usize].active = 1 };
    }

    if w3d.id_in.MousePresent {
        unsafe { CtlMenu[CTL::CTL_MOUSESENS as usize].active = 1 };
        unsafe { CtlMenu[CTL::CTL_MOUSEENABLE as usize].active = 1 };
    }

    unsafe { CtlMenu[CTL::CTL_MOUSESENS as usize].active = w3d.wl_play.mouseenabled as i32 };

    DrawMenu(w3d, unsafe { &CtlItems }, unsafe { &CtlMenu });

    x = CTL_X + unsafe { CtlItems.indent } - 24;
    y = CTL_Y + 3;
    if w3d.wl_play.mouseenabled != 0 {
        VWB_DrawPic(w3d, x, y, graphicnums::C_SELECTEDPIC as i32);
    } else {
        VWB_DrawPic(w3d, x, y, graphicnums::C_NOTSELECTEDPIC as i32);
    }

    y = CTL_Y + 29;
    if w3d.wl_play.joystickenabled != 0 {
        VWB_DrawPic(w3d, x, y, graphicnums::C_SELECTEDPIC as i32);
    } else {
        VWB_DrawPic(w3d, x, y, graphicnums::C_NOTSELECTEDPIC as i32);
    }

    //
    // PICK FIRST AVAILABLE SPOT
    //
    if unsafe { CtlItems.curpos } < 0 || unsafe { CtlMenu[CtlItems.curpos as usize].active } == 0 {
        for i in 0..unsafe { CtlItems.amount } {
            if unsafe { CtlMenu[i as usize].active != 0 } {
                unsafe { CtlItems.curpos = i };
                break;
            }
        }
    }

    DrawMenuGun(w3d, unsafe { &CtlItems });
    VW_UpdateScreen(w3d);
}

////////////////////////////////////////////////////////////////////
//
// CUSTOMIZE CONTROLS
//
////////////////////////////////////////////////////////////////////

pub fn CustomControls(w3d: &mut modules) -> i32 {
    //println!("CustomControls");

    let mut which: i32;

    DrawCustomScreen(w3d);

    loop {
        which = HandleMenu(
            w3d,
            unsafe { &mut CusItems },
            unsafe { &CusMenu },
            FixupCustom,
        );

        match which {
            0 => {
                DefineMouseBtns();
                DrawCustMouse(w3d, 1);
            }
            3 => {
                DefineJoyBtns();
                DrawCustJoy(w3d, 0);
            }
            6 => {
                DefineKeyBtns();
                DrawCustKeybd(w3d, 0);
            }
            8 => {
                DefineKeyMove();
                DrawCustKeys(w3d, 0);
            }
            _ => (),
        }
        if which < 0 {
            break;
        }
    }

    MenuFadeOut(w3d);

    return 0;
}

////////////////////////
//
// DEFINE THE MOUSE BUTTONS
//

pub fn DefineMouseBtns() {
    //println!("DefineMouseBtns");
}

////////////////////////
//
// DEFINE THE JOYSTICK BUTTONS
//

pub fn DefineJoyBtns() {
    //println!("DefineJoyBtns");
}

////////////////////////
//
// DEFINE THE KEYBOARD BUTTONS
//

pub fn DefineKeyBtns() {
    //println!("DefineKeyBtns");
}

////////////////////////
//
// DEFINE THE KEYBOARD BUTTONS
//

pub fn DefineKeyMove() {
    //println!("DefineKeyMove");
}

////////////////////////
//
// FIXUP GUN CURSOR OVERDRAW SHIT
//

pub fn FixupCustom(w3d: &mut modules, w: i32) {
    //println!("FixupCustom");

    //static int lastwhich = -1;

    let mut y: i32 = CST_Y + 26 + w * 13;

    VWB_Hlin(w3d, 7, 32, y - 1, DEACTIVE);
    VWB_Hlin(w3d, 7, 32, y + 12, BORD2COLOR);

    VWB_Hlin(w3d, 7, 32, y - 2, BORDCOLOR);
    VWB_Hlin(w3d, 7, 32, y + 13, BORDCOLOR);

    match w {
        0 => {
            DrawCustMouse(w3d, 1);
        }
        3 => {
            DrawCustJoy(w3d, 1);
        }
        6 => {
            DrawCustKeybd(w3d, 1);
        }
        8 => {
            DrawCustKeys(w3d, 1);
        }
        _ => (),
    }

    if unsafe { lastwhich >= 0 } {
        y = CST_Y + 26 + unsafe { lastwhich } * 13;

        VWB_Hlin(w3d, 7, 32, y - 1, DEACTIVE);
        VWB_Hlin(w3d, 7, 32, y + 12, BORD2COLOR);

        VWB_Hlin(w3d, 7, 32, y - 2, BORDCOLOR);
        VWB_Hlin(w3d, 7, 32, y + 13, BORDCOLOR);

        if unsafe { lastwhich != w } {
            match unsafe { lastwhich } {
                0 => {
                    DrawCustMouse(w3d, 0);
                }
                3 => {
                    DrawCustJoy(w3d, 0);
                }
                6 => {
                    DrawCustKeybd(w3d, 0);
                }
                8 => {
                    DrawCustKeys(w3d, 0);
                }
                _ => (),
            }
        }
    }

    unsafe { lastwhich = w };
}

////////////////////////
//
// DRAW CUSTOMIZE SCREEN
//

pub fn DrawCustomScreen(w3d: &mut modules) {
    //println!("DrawCustomScreen");

    ClearMScreen(w3d);
    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowW = 320;
    VWB_DrawPic(w3d, 112, 184, graphicnums::C_MOUSELBACKPIC as i32);
    DrawStripes(w3d, 10);
    VWB_DrawPic(w3d, 80, 0, graphicnums::C_CUSTOMIZEPIC as i32);

    //
    // MOUSE
    //
    SETFONTCOLOR(w3d, READCOLOR, BKGDCOLOR);
    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowW = 320;

    w3d.id_us.PrintY = CST_Y;
    US_CPrint(w3d, "Mouse\n".to_string());

    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);

    w3d.id_us.PrintX = CST_START;
    US_Print(w3d, STR_CRUN.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 1;
    US_Print(w3d, STR_COPEN.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 2;
    US_Print(w3d, STR_CFIRE.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 3;
    US_Print(w3d, STR_CSTRAFE.to_string());
    US_Print(w3d, "\n".to_string());

    DrawWindow(w3d, 5, w3d.id_us.PrintY - 1, 310, 13, BKGDCOLOR);
    DrawCustMouse(w3d, 0);
    US_Print(w3d, "\n".to_string());

    //
    // JOYSTICK/PAD
    //

    SETFONTCOLOR(w3d, READCOLOR, BKGDCOLOR);
    US_CPrint(w3d, "Joystick/Gravis GamePad\n".to_string());

    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);

    w3d.id_us.PrintX = CST_START;
    US_Print(w3d, STR_CRUN.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 1;
    US_Print(w3d, STR_COPEN.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 2;
    US_Print(w3d, STR_CFIRE.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 3;
    US_Print(w3d, STR_CSTRAFE.to_string());
    US_Print(w3d, "\n".to_string());

    DrawWindow(w3d, 5, w3d.id_us.PrintY - 1, 310, 13, BKGDCOLOR);
    DrawCustJoy(w3d, 0);
    US_Print(w3d, "\n".to_string());

    //
    // KEYBOARD
    //

    SETFONTCOLOR(w3d, READCOLOR, BKGDCOLOR);
    US_CPrint(w3d, "Keyboard\n".to_string());

    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);

    w3d.id_us.PrintX = CST_START;
    US_Print(w3d, STR_CRUN.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 1;
    US_Print(w3d, STR_COPEN.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 2;
    US_Print(w3d, STR_CFIRE.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 3;
    US_Print(w3d, STR_CSTRAFE.to_string());
    US_Print(w3d, "\n".to_string());

    DrawWindow(w3d, 5, w3d.id_us.PrintY - 1, 310, 13, BKGDCOLOR);
    DrawCustKeybd(w3d, 0);
    US_Print(w3d, "\n".to_string());

    //
    // KEYBOARD MOVE KEYS
    //
    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);

    w3d.id_us.PrintX = CST_START;
    US_Print(w3d, STR_LEFT.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 1;
    US_Print(w3d, STR_RIGHT.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 2;
    US_Print(w3d, STR_FRWD.to_string());
    w3d.id_us.PrintX = CST_START + CST_SPC * 3;
    US_Print(w3d, STR_BKWD.to_string());
    US_Print(w3d, "\n".to_string());

    DrawWindow(w3d, 5, w3d.id_us.PrintY - 1, 310, 13, BKGDCOLOR);
    DrawCustKeys(w3d, 0);

    //
    // PICK STARTING POINT IN MENU
    //
    if unsafe { CusItems.curpos < 0 } {
        for i in 0..unsafe { CusItems.amount } {
            if unsafe { CusMenu[i as usize].active != 0 } {
                unsafe { CusItems.curpos = i };
                break;
            }
        }
    }

    VW_UpdateScreen(w3d);
    MenuFadeIn(w3d);
}

pub fn PrintCustMouse(w3d: &mut modules, i: i32) {
    //println!("PrintCustMouse");

    for j in 0..4 {
        if order[i as usize] == w3d.wl_play.buttonmouse[j as usize] as u8 {
            w3d.id_us.PrintX = CST_START + CST_SPC * i;
            US_Print(w3d, mbarray[j as usize].to_string());
            break;
        }
    }
}

pub fn DrawCustMouse(w3d: &mut modules, hilight: i32) {
    //println!("DrawCustMouse");

    let mut color: i32;

    color = TEXTCOLOR;
    if hilight != 0 {
        color = HIGHLIGHT;
    }
    SETFONTCOLOR(w3d, color, BKGDCOLOR);

    if w3d.wl_play.mouseenabled == 0 {
        SETFONTCOLOR(w3d, DEACTIVE, BKGDCOLOR);
        unsafe { CusMenu[0].active = 0 };
    } else {
        unsafe { CusMenu[0].active = 1 };
    }

    w3d.id_us.PrintY = CST_Y + 13 * 2;
    for i in 0..4 {
        PrintCustMouse(w3d, i);
    }
}

pub fn PrintCustJoy(w3d: &mut modules, i: i32) {
    //println!("PrintCustJoy");

    for j in 0..4 {
        if order[i as usize] == w3d.wl_play.buttonjoy[j as usize] as u8 {
            w3d.id_us.PrintX = CST_START + CST_SPC * i;
            US_Print(w3d, mbarray[j as usize].to_string());
            break;
        }
    }
}

pub fn DrawCustJoy(w3d: &mut modules, hilight: i32) {
    //println!("DrawCustJoy");

    let mut color: i32;

    color = TEXTCOLOR;
    if hilight != 0 {
        color = HIGHLIGHT;
    }
    SETFONTCOLOR(w3d, color, BKGDCOLOR);

    if w3d.wl_play.joystickenabled == 0 {
        SETFONTCOLOR(w3d, DEACTIVE, BKGDCOLOR);
        unsafe { CusMenu[3].active = 0 };
    } else {
        unsafe { CusMenu[3].active = 1 };
    }

    w3d.id_us.PrintY = CST_Y + 13 * 5;

    for i in 0..4 {
        PrintCustJoy(w3d, i);
    }
}

pub fn PrintCustKeybd(w3d: &mut modules, i: i32) {
    //println!("PrintCustKeybd");

    w3d.id_us.PrintX = CST_START + CST_SPC * i;
    let s = IN_GetScanName(w3d, w3d.wl_play.buttonscan[order[i as usize] as usize]);
    US_Print(w3d, s.clone());
}

pub fn DrawCustKeybd(w3d: &mut modules, hilight: i32) {
    //println!("DrawCustKeybd");

    let mut color: i32;

    color = TEXTCOLOR;
    if hilight != 0 {
        color = HIGHLIGHT;
    }
    SETFONTCOLOR(w3d, color, BKGDCOLOR);

    w3d.id_us.PrintY = CST_Y + 13 * 8;
    for i in 0..4 {
        PrintCustKeybd(w3d, i);
    }
}

pub fn PrintCustKeys(w3d: &mut modules, i: i32) {
    //println!("PrintCustKeys");

    w3d.id_us.PrintX = CST_START + CST_SPC * i;
    let s = IN_GetScanName(w3d, w3d.wl_play.dirscan[moveorder[i as usize] as usize]);
    US_Print(w3d, s.clone());
}

pub fn DrawCustKeys(w3d: &mut modules, hilight: i32) {
    //println!("DrawCustKeys");

    let mut color: i32;

    color = TEXTCOLOR;
    if hilight != 0 {
        color = HIGHLIGHT;
    }
    SETFONTCOLOR(w3d, color, BKGDCOLOR);

    w3d.id_us.PrintY = CST_Y + 13 * 10;
    for i in 0..4 {
        PrintCustKeys(w3d, i);
    }
}

////////////////////////////////////////////////////////////////////
//
// CHANGE SCREEN VIEWING SIZE
//
////////////////////////////////////////////////////////////////////

pub fn CP_ChangeView(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_ChangeView");

    let mut exit: i32 = 0;
    let oldview: i32;
    let mut newview: i32;
    let mut ci: ControlInfo = ControlInfo::new();

    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowY = 0;
    w3d.id_us.WindowW = 320;
    w3d.id_us.WindowH = 200;
    newview = w3d.wl_play.viewsize;
    oldview = w3d.wl_play.viewsize;

    DrawChangeView(w3d, oldview);
    MenuFadeIn(w3d);

    loop {
        CheckPause(w3d);
        SDL_Delay(w3d, 5);
        ReadAnyControl(w3d, &mut ci);

        match ci.dir {
            Direction::dir_South | Direction::dir_West => {
                newview -= 1;
                if newview < 4 {
                    newview = 4;
                }
                if newview >= 19 {
                    DrawChangeView(w3d, newview);
                } else {
                    ShowViewSize(w3d, newview);
                }
                VW_UpdateScreen(w3d);
                SD_PlaySound(w3d, soundnames::HITWALLSND);
                TicDelay(w3d, 10);
            }

            Direction::dir_North | Direction::dir_East => {
                newview += 1;
                if newview >= 21 {
                    newview = 21;
                    DrawChangeView(w3d, newview);
                } else {
                    ShowViewSize(w3d, newview);
                }
                VW_UpdateScreen(w3d);
                SD_PlaySound(w3d, soundnames::HITWALLSND);
                TicDelay(w3d, 10);
            }
            _ => (),
        }

        if ci.button0 != 0 || Keyboard(w3d, Scancode::Return) {
            exit = 1;
        } else if ci.button1 != 0 || Keyboard(w3d, Scancode::Escape) {
            SD_PlaySound(w3d, soundnames::ESCPRESSEDSND);
            MenuFadeOut(w3d);
            if w3d.id_vl.screenHeight % 200 != 0 {
                VL_ClearScreen(w3d, Color::BLACK);
            }
            return 0;
        }
        if exit != 0 {
            break;
        }
    }

    if oldview != newview {
        SD_PlaySound(w3d, soundnames::SHOOTSND);
        let mut s = STR_THINK.to_string();
        s.push_str("...");
        Message(w3d, s);
        NewViewSize(w3d, newview);
    }

    ShootSnd(w3d);
    MenuFadeOut(w3d);
    if w3d.id_vl.screenHeight % 200 != 0 {
        VL_ClearScreen(w3d, Color::BLACK);
    }

    return 0;
}

/////////////////////////////
//
// DRAW THE CHANGEVIEW SCREEN
//

pub fn DrawChangeView(w3d: &mut modules, view: i32) {
    //println!("DrawChangeView");

    let rescaledHeight: i32 = w3d.id_vl.screenHeight / w3d.id_vl.scaleFactor;

    if view != 21 {
        VWB_Bar(w3d, 0, rescaledHeight - 40, 320, 40, w3d.wl_game.bordercol);
    }

    ShowViewSize(w3d, view);

    w3d.id_us.PrintY = (w3d.id_vl.screenHeight / w3d.id_vl.scaleFactor) - 39;
    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowY = 320; // TODO: Check this!
    SETFONTCOLOR(w3d, HIGHLIGHT, BKGDCOLOR);

    let mut s = STR_SIZE1.to_string();
    s.push('\n');

    US_CPrint(w3d, s);

    let mut s = STR_SIZE2.to_string();
    s.push('\n');

    US_CPrint(w3d, s);

    US_CPrint(w3d, STR_SIZE3.to_string());

    VW_UpdateScreen(w3d);
}

////////////////////////////////////////////////////////////////////
//
// QUIT THIS INFERNAL GAME!
//
////////////////////////////////////////////////////////////////////

pub fn CP_Quit(w3d: &mut modules, _blank: i32) -> i32 {
    //println!("CP_Quit");

    let string = endStrings[(US_RndT(w3d) & 0x7 + (US_RndT(w3d) & 1)) as usize];

    if Confirm(w3d, string.to_string()) != 0 {
        VW_UpdateScreen(w3d);
        SD_MusicOff(w3d);
        SD_StopSound(w3d);
        MenuFadeOut(w3d);
        Quit("Achtung !!!");
    }

    DrawMainMenu(w3d);
    return 0;
}

////////////////////////////////////////////////////////////////////
//
// HANDLE INTRO SCREEN (SYSTEM CONFIG)
//
////////////////////////////////////////////////////////////////////

pub fn IntroScreen(w3d: &mut modules) {
    //println!("IntroScreen");

    let MAINCOLOR: i32 = 0x6c;
    let EMSCOLOR: i32 = 0x6c; // 0x4f
    let XMSCOLOR: i32 = 0x6c; // 0x7f

    let FILLCOLOR: i32 = 14;

    for i in 0..10 {
        VWB_Bar(w3d, 49, 163 - 8 * i as i32, 6, 5, MAINCOLOR - i);
    }
    for i in 0..10 {
        VWB_Bar(w3d, 89, 163 - 8 * i as i32, 6, 5, EMSCOLOR - i);
    }
    for i in 0..10 {
        VWB_Bar(w3d, 129, 163 - 8 * i as i32, 6, 5, XMSCOLOR - i);
    }

    //
    // FILL BOXES
    //
    if w3d.id_in.MousePresent {
        VWB_Bar(w3d, 164, 82, 12, 2, FILLCOLOR);
    }

    if IN_JoyPresent(w3d) {
        VWB_Bar(w3d, 164, 105, 12, 2, FILLCOLOR);
    }

    if w3d.id_sd.AdLibPresent && !w3d.id_sd.SoundBlasterPresent {
        VWB_Bar(w3d, 164, 128, 12, 2, FILLCOLOR);
    }

    if w3d.id_sd.SoundBlasterPresent {
        VWB_Bar(w3d, 164, 151, 12, 2, FILLCOLOR);
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////
//
// SUPPORT ROUTINES
//
////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////
//
// Clear Menu screens to dark red
//
////////////////////////////////////////////////////////////////////

pub fn ClearMScreen(w3d: &mut modules) {
    //println!("ClearMScreen");

    VWB_Bar(w3d, 0, 0, 320, 200, BORDCOLOR);
}

////////////////////////////////////////////////////////////////////
//
// Draw a window for a menu
//
////////////////////////////////////////////////////////////////////

pub fn DrawWindow(w3d: &mut modules, x: i32, y: i32, w: i32, h: i32, wcolor: i32) {
    //println!("DrawWindow");

    VWB_Bar(w3d, x, y, w, h, wcolor);
    DrawOutline(w3d, x, y, w, h, BORD2COLOR, DEACTIVE);
}

pub fn DrawOutline(w3d: &mut modules, x: i32, y: i32, w: i32, h: i32, color1: i32, color2: i32) {
    //println!("DrawOutline");

    VWB_Hlin(w3d, x, x + w, y, color2);
    VWB_Vlin(w3d, y, y + h, x, color2);
    VWB_Hlin(w3d, x, x + w, y + h, color1);
    VWB_Vlin(w3d, y, y + h, x + w, color1);
}

////////////////////////////////////////////////////////////////////
//
// Setup Control Panel stuff - graphics, etc.
//
////////////////////////////////////////////////////////////////////

pub fn SetupControlPanel(w3d: &mut modules) {
    //println!("SetupControlPanel");

    //
    // CACHE SOUNDS
    //
    SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);
    w3d.id_vh.fontnumber = 1;
    w3d.id_us.WindowH = 200;
    if w3d.id_vl.screenHeight % 200 != 0 {
        VL_ClearScreen(w3d, Color::BLACK);
    }

    if !w3d.wl_game.ingame {
        CA_LoadAllSounds(w3d);
    } else {
        unsafe { MainMenu[menuitems::savegame as usize].active = 1 };
    }

    //
    // CENTER MOUSE
    //
    if IN_IsInputGrabbed(w3d) {
        IN_CenterMouse(w3d);
    }
}

////////////////////////////////////////////////////////////////////
//
// SEE WHICH SAVE GAME FILES ARE AVAILABLE & READ STRING IN
//
////////////////////////////////////////////////////////////////////

pub fn SetupSaveGames() {
    //println!("SetupSaveGames");
}

////////////////////////////////////////////////////////////////////
//
// Clean up all the Control Panel stuff
//
////////////////////////////////////////////////////////////////////

pub fn CleanupControlPanel(w3d: &mut modules) {
    //println!("CleanupControlPanel");

    w3d.id_vh.fontnumber = 0;
}

////////////////////////////////////////////////////////////////////
//
// Handle moving gun around a menu
//
////////////////////////////////////////////////////////////////////

pub fn HandleMenu(
    w3d: &mut modules,
    item_i: &mut CP_iteminfo,
    items: &[CP_itemtype; 18],
    routine: routine_void,
) -> i32 {
    //println!("HandleMenu");

    let mut key: String;
    //static int redrawitem = 1, lastitem = -1;
    let x: i32;
    let mut y: i32;
    let basey: i32;
    let mut exit: i32;
    let mut which: i32;
    let mut shape: i32;
    let mut lastBlinkTime: i32;
    let mut timer: i32;
    let mut ci: ControlInfo = ControlInfo::new();

    which = item_i.curpos;
    x = item_i.x & -8;
    basey = item_i.y - 2;
    y = basey + which * 13;

    VWB_DrawPic(w3d, x as i32, y as i32, graphicnums::C_CURSOR1PIC as i32);
    SetTextColor(w3d, &items[which as usize], 1);

    if unsafe { redrawitem != 0 } {
        w3d.id_us.PrintX = (item_i.x + item_i.indent) as i32;
        w3d.id_us.PrintY = (item_i.y + which * 13) as i32;
        US_Print(w3d, items[which as usize].string.to_string());
    }

    //
    // CALL CUSTOM ROUTINE IF IT IS NEEDED
    //
    //if routine {
    routine(w3d, which);
    //}
    VW_UpdateScreen(w3d);

    shape = graphicnums::C_CURSOR1PIC as i32;
    timer = 8;
    exit = 0;
    lastBlinkTime = GetTimeCount(w3d);
    IN_ClearKeysDown(w3d);

    loop {
        //
        // CHANGE GUN SHAPE
        //
        if GetTimeCount(w3d) - lastBlinkTime > timer as i32 {
            lastBlinkTime = GetTimeCount(w3d);

            if shape == graphicnums::C_CURSOR1PIC as i32 {
                shape = graphicnums::C_CURSOR2PIC as i32;
                timer = 8;
            } else {
                shape = graphicnums::C_CURSOR1PIC as i32;
                timer = 70;
            }
            VWB_DrawPic(w3d, x as i32, y as i32, shape as i32);
            //if routine {
            routine(w3d, which);
            //}
            VW_UpdateScreen(w3d);
        } else {
            SDL_Delay(w3d, 5);
        }

        CheckPause(w3d);

        //
        // SEE IF ANY KEYS ARE PRESSED FOR INITIAL CHAR FINDING
        //
        key = w3d.id_in.LastASCII.to_string();

        if key != Scancode::F24.to_string() {
            let mut ok = 0;
            let key_letter = key.chars().nth(0).unwrap();

            for i in which + 1..item_i.amount {
                let key_menu_letter = items[i as usize].string.to_string().chars().nth(0);

                match key_menu_letter {
                    Some(value) => {
                        if items[i as usize].active != 0 && (value == key_letter) {
                            EraseGun(w3d, item_i, items, x as i32, y as i32, which);
                            which = i;
                            DrawGun(w3d, item_i, items, x as i32, &mut y, which, basey, routine);
                            ok = 1;
                            IN_ClearKeysDown(w3d);
                            break;
                        }
                    }
                    None => continue,
                }
            }
            //
            // DIDN'T FIND A MATCH FIRST TIME THRU. CHECK AGAIN.
            //
            if ok == 0 {
                for i in 0..which {
                    let key_menu_letter = items[i as usize].string.to_string().chars().nth(0);

                    match key_menu_letter {
                        Some(value) => {
                            if items[i as usize].active != 0 && (value == key_letter) {
                                EraseGun(w3d, item_i, items, x as i32, y as i32, which);
                                which = i;
                                DrawGun(
                                    w3d, item_i, items, x as i32, &mut y, which, basey, routine,
                                );
                                IN_ClearKeysDown(w3d);
                                break;
                            }
                        }
                        None => continue,
                    }
                }
            }
        }

        //
        // GET INPUT
        //
        ReadAnyControl(w3d, &mut ci);

        match ci.dir {
            ////////////////////////////////////////////////
            //
            // MOVE UP
            //
            Direction::dir_North => {
                EraseGun(w3d, item_i, items, x as i32, y as i32, which);

                //
                // ANIMATE HALF-STEP
                //
                if which != 0 && items[(which - 1) as usize].active != 0 {
                    y -= 6;
                    DrawHalfStep(w3d, x as i32, y as i32);
                }

                //
                // MOVE TO NEXT AVAILABLE SPOT
                //
                loop {
                    if which == 0 {
                        which = item_i.amount - 1;
                    } else {
                        which -= 1;
                    }
                    if (items[which as usize]).active != 0 {
                        break;
                    }
                }

                DrawGun(w3d, item_i, items, x as i32, &mut y, which, basey, routine);
                //
                // WAIT FOR BUTTON-UP OR DELAY NEXT MOVE
                //
                TicDelay(w3d, 20);
            }
            ////////////////////////////////////////////////
            //
            // MOVE DOWN
            //
            Direction::dir_South => {
                EraseGun(w3d, item_i, items, x as i32, y as i32, which);
                //
                // ANIMATE HALF-STEP
                //
                if which != item_i.amount - 1 && items[(which + 1) as usize].active != 0 {
                    y += 6;
                    DrawHalfStep(w3d, x as i32, y as i32);
                }

                loop {
                    if which == item_i.amount - 1 {
                        which = 0;
                    } else {
                        which += 1;
                    }
                    if (items[which as usize]).active != 0 {
                        break;
                    }
                }

                DrawGun(w3d, item_i, items, x as i32, &mut y, which, basey, routine);

                //
                // WAIT FOR BUTTON-UP OR DELAY NEXT MOVE
                //
                TicDelay(w3d, 20);
            }
            _ => {}
        }

        if ci.button0 != 0 || Keyboard(w3d, Scancode::Space) || Keyboard(w3d, Scancode::Return) {
            exit = 1;
        }

        if ci.button1 != 0 && !Keyboard(w3d, Scancode::LAlt) || Keyboard(w3d, Scancode::Escape) {
            exit = 2;
        }

        if exit != 0 {
            break;
        }
    }

    IN_ClearKeysDown(w3d);

    //
    // ERASE EVERYTHING
    //
    if unsafe { lastitem != which as i32 } {
        VWB_Bar(w3d, (x - 1) as i32, y as i32, 25, 16, BKGDCOLOR);
        w3d.id_us.PrintX = item_i.x as i32 + item_i.indent as i32;
        w3d.id_us.PrintY = (item_i.y + which * 13) as i32;
        US_Print(w3d, (items[which as usize]).string.to_string());
        unsafe { redrawitem = 1 };
    } else {
        unsafe { redrawitem = 0 };
    }

    //if routine {
    routine(w3d, which);
    //}
    VW_UpdateScreen(w3d);

    item_i.curpos = which;

    unsafe { lastitem = which as i32 };

    match exit {
        1 => {
            //
            // CALL THE ROUTINE
            //
            if (items[which as usize]).routine != ptr::null_mut() {
                //BUG causes noise
                //ShootSnd(w3d);
                //BUG when choosing unavailable episodes
                //MenuFadeOut(w3d);

                let routine = items[which as usize].routine;
                unsafe {
                    //https://users.rust-lang.org/t/function-pointers-and-raw-function-pointers/15152/9
                    //Insane...
                    //
                    let routine = (&(routine as *const routine_int) as *const *const routine_int)
                        as *const routine_int;
                    (*routine)(w3d, 0);
                }
            }

            return which as i32;
        }
        2 => {
            SD_PlaySound(w3d, soundnames::ESCPRESSEDSND);
            return -1;
        }
        _ => (),
    }
    return 0; // JUST TO SHUT UP THE ERROR MESSAGES!
}

//
// ERASE GUN & DE-HIGHLIGHT STRING
//

pub fn EraseGun(
    w3d: &mut modules,
    item_i: &CP_iteminfo,
    items: &[CP_itemtype; 18],
    x: i32,
    y: i32,
    which: i32,
) {
    //println!("EraseGun");

    VWB_Bar(w3d, (x - 1) as i32, y as i32, 25, 16, BKGDCOLOR);

    SetTextColor(w3d, &items[which as usize], 0);

    w3d.id_us.PrintX = item_i.x + item_i.indent;
    w3d.id_us.PrintY = item_i.y + which * 13;

    US_Print(w3d, items[which as usize].string.to_string());
    VW_UpdateScreen(w3d);
}

//
// DRAW HALF STEP OF GUN TO NEXT POSITION
//

pub fn DrawHalfStep(w3d: &mut modules, x: i32, y: i32) {
    //println!("DrawHalfStep");

    VWB_DrawPic(w3d, x, y, graphicnums::C_CURSOR1PIC as i32);
    VW_UpdateScreen(w3d);
    SD_PlaySound(w3d, soundnames::MOVEGUN1SND);
    SDL_Delay(w3d, 8 * 100 / 7);
}

//
// DRAW GUN AT NEW POSITION
//

pub fn DrawGun(
    w3d: &mut modules,
    item_i: &CP_iteminfo,
    items: &[CP_itemtype; 18],
    x: i32,
    y: &mut i32,
    which: i32,
    basey: i32,
    routine: fn(w3d: &mut modules, which: i32),
) {
    //println!("DrawGun");

    VWB_Bar(w3d, (x - 1) as i32, (*y) as i32, 25, 16, BKGDCOLOR);

    *y = (basey + which * 13) as i32;

    VWB_DrawPic(w3d, x as i32, (*y) as i32, graphicnums::C_CURSOR1PIC as i32);

    SetTextColor(w3d, &items[which as usize], 1);

    w3d.id_us.PrintX = (item_i.x + item_i.indent) as i32;

    w3d.id_us.PrintY = (item_i.y + which * 13) as i32;

    US_Print(w3d, items[which as usize].string.to_string());

    //
    // CALL CUSTOM ROUTINE IF IT IS NEEDED
    //
    //if routine {
    routine(w3d, which);
    //}
    VW_UpdateScreen(w3d);

    SD_PlaySound(w3d, soundnames::MOVEGUN2SND);
}

////////////////////////////////////////////////////////////////////
//
// DELAY FOR AN AMOUNT OF TICS OR UNTIL CONTROLS ARE INACTIVE
//
////////////////////////////////////////////////////////////////////

pub fn TicDelay(w3d: &mut modules, count: i32) {
    //println!("TicDelay");

    let mut ci: ControlInfo = ControlInfo::new();

    let startTime = GetTimeCount(w3d);

    loop {
        SDL_Delay(w3d, 5);
        ReadAnyControl(w3d, &mut ci);

        if (GetTimeCount(w3d) - startTime > count) || ci.dir == Direction::dir_None {
            break;
        }
    }
}

////////////////////////////////////////////////////////////////////
//
// Draw a menu
//
////////////////////////////////////////////////////////////////////

pub fn DrawMenu(w3d: &mut modules, item_i: &CP_iteminfo, items: &[CP_itemtype; 18]) {
    //println!("DrawMenu");

    let which = item_i.curpos;

    w3d.id_us.WindowX = item_i.x as i32 + item_i.indent as i32;
    w3d.id_us.PrintX = item_i.x as i32 + item_i.indent as i32;
    w3d.id_us.WindowY = item_i.y as i32;
    w3d.id_us.PrintY = item_i.y as i32;

    w3d.id_us.WindowW = 320;
    w3d.id_us.WindowH = 200;

    for i in 0..item_i.amount {
        if which == i {
            SetTextColor(w3d, &items[i as usize], 1);
        } else {
            SetTextColor(w3d, &items[i as usize], 0);
        }

        w3d.id_us.PrintY = (item_i.y + i * 13) as i32;

        if items[i as usize].active != 0 {
            US_Print(w3d, items[i as usize].string.to_string());
        } else {
            SETFONTCOLOR(w3d, DEACTIVE, BKGDCOLOR);
            US_Print(w3d, items[i as usize].string.to_string());
            SETFONTCOLOR(w3d, TEXTCOLOR, BKGDCOLOR);
        }

        US_Print(w3d, "\n".to_string());
    }
}

////////////////////////////////////////////////////////////////////
//
// SET TEXT COLOR (HIGHLIGHT OR NO)
//
////////////////////////////////////////////////////////////////////

pub fn SetTextColor(w3d: &mut modules, items: &CP_itemtype, hlight: i32) {
    //println!("SetTextColor");

    if hlight != 0 {
        SETFONTCOLOR(
            w3d,
            w3d.wl_menu.color_hlite[items.active as usize],
            BKGDCOLOR,
        );
    } else {
        SETFONTCOLOR(
            w3d,
            w3d.wl_menu.color_norml[items.active as usize],
            BKGDCOLOR,
        );
    }
}

////////////////////////////////////////////////////////////////////
//
// WAIT FOR CTRLKEY-UP OR BUTTON-UP
//
////////////////////////////////////////////////////////////////////

pub fn WaitKeyUp(w3d: &mut modules) {
    //println!("WaitKeyUp");

    let mut ci: ControlInfo = ControlInfo::new();

    ReadAnyControl(w3d, &mut ci);

    while (ci.button0 != 0)
        | (ci.button1 != 0)
        | (ci.button2 != 0)
        | (ci.button3 != 0)
        | Keyboard(w3d, Scancode::Space)
        | Keyboard(w3d, Scancode::Return)
        | Keyboard(w3d, Scancode::Escape)
    {
        ReadAnyControl(w3d, &mut ci);
        IN_WaitAndProcessEvents(w3d);
    }
}

////////////////////////////////////////////////////////////////////
//
// READ KEYBOARD, JOYSTICK AND MOUSE FOR INPUT
//
////////////////////////////////////////////////////////////////////

pub fn ReadAnyControl(w3d: &mut modules, ci: &mut ControlInfo) {
    //println!("ReadAnyControl");

    let mut mouseactive: i32 = 0;

    IN_ReadControl(w3d, 0, ci);

    if w3d.wl_play.mouseenabled != 0 {
        //&& IN_IsInputGrabbed(w3d) {
        let mousex: i32;
        let mousey: i32;

        //buttons = SDL_GetRelativeMouseState(&mousex, &mousey);
        let event_pump = w3d.id_vl.sdl_context.event_pump().unwrap();
        let buttons = event_pump.relative_mouse_state();

        mousex = buttons.x();
        mousey = buttons.y();

        let mut buttons = buttons.to_sdl_state();

        //int middlePressed = buttons & SDL_BUTTON(SDL_BUTTON_MIDDLE);
        let middlePressed = 0;
        //int rightPressed = buttons & SDL_BUTTON(SDL_BUTTON_RIGHT);
        let rightPressed = 0;

        //buttons &= ~(SDL_BUTTON(SDL_BUTTON_MIDDLE) | SDL_BUTTON(SDL_BUTTON_RIGHT));

        if middlePressed != 0 {
            buttons |= 1 << 2;
        }

        if rightPressed != 0 {
            buttons |= 1 << 1;
        }

        w3d.wl_menu.totalMousex += mousex;
        w3d.wl_menu.totalMousey += mousey;

        if w3d.wl_menu.totalMousey < -SENSITIVE {
            ci.dir = Direction::dir_North;
            mouseactive = 1;
        } else if w3d.wl_menu.totalMousey > SENSITIVE {
            ci.dir = Direction::dir_South;
            mouseactive = 1;
        }

        if w3d.wl_menu.totalMousex < -SENSITIVE {
            ci.dir = Direction::dir_West;
            mouseactive = 1;
        } else if w3d.wl_menu.totalMousex > SENSITIVE {
            ci.dir = Direction::dir_East;
            mouseactive = 1;
        }

        if mouseactive != 0 {
            w3d.wl_menu.totalMousex = 0;
            w3d.wl_menu.totalMousey = 0;
        }

        if buttons != 0 {
            ci.button0 = (buttons & 1) as u8;
            ci.button1 = (buttons & 2) as u8;
            ci.button2 = (buttons & 4) as u8;
            ci.button3 = 0;
            mouseactive = 1;
        }
    }

    if w3d.wl_play.joystickenabled != 0 && mouseactive == 0 {
        let mut jx: i16 = 0;
        let mut jy: i16 = 0;
        let jb: i32 ;

        IN_GetJoyDelta(w3d, &mut jx, &mut jy);

        if jy < -SENSITIVE as i16 {
            ci.dir = Direction::dir_North;
        } else if jy > SENSITIVE as i16 {
            ci.dir = Direction::dir_South;
        }

        if jx < -SENSITIVE as i16 {
            ci.dir = Direction::dir_West;
        } else if jx > SENSITIVE as i16 {
            ci.dir = Direction::dir_East;
        }

        jb = IN_JoyButtons(w3d);

        if jb != 0 {
            ci.button0 = (jb & 1) as u8;
            ci.button1 = (jb & 2) as u8;
            ci.button2 = (jb & 4) as u8;
            ci.button3 = (jb & 8) as u8;
        }
    }
}

////////////////////////////////////////////////////////////////////
//
// DRAW DIALOG AND CONFIRM YES OR NO TO QUESTION
//
////////////////////////////////////////////////////////////////////

pub fn Confirm(w3d: &mut modules, string: String) -> i32 {
    //println!("Confirm");

    let mut xit: i32 = 0;
    let x: i32;
    let y: i32;
    let mut tick: i32 = 0;
    let mut lastBlinkTime: i32;

    let whichsnd: [soundnames; 2] = [soundnames::ESCPRESSEDSND, soundnames::SHOOTSND];
    let mut ci: ControlInfo = ControlInfo::new();

    Message(w3d, string);
    IN_ClearKeysDown(w3d);
    WaitKeyUp(w3d);

    //
    // BLINK CURSOR
    //
    x = w3d.id_us.PrintX_cur;
    y = w3d.id_us.PrintY_cur;
    lastBlinkTime = GetTimeCount(w3d);

    loop {
        ReadAnyControl(w3d, &mut ci);

        if GetTimeCount(w3d) - lastBlinkTime >= 10 {
            match tick {
                0 => {
                    VWB_Bar(w3d, x, y, 8, 13, TEXTCOLOR);
                }
                1 => {
                    w3d.id_us.PrintX = x;
                    w3d.id_us.PrintY = y;
                    US_Print(w3d, "_".to_string());
                }
                _ => (),
            }
            VW_UpdateScreen(w3d);
            tick ^= 1;
            lastBlinkTime = GetTimeCount(w3d);
        } else {
            SDL_Delay(w3d, 5);
        }
        if Keyboard(w3d, Scancode::Y)
            || Keyboard(w3d, Scancode::N)
            || Keyboard(w3d, Scancode::Escape)
            || ci.button0 != 0
            || ci.button1 != 0
        {
            break;
        }
    }

    if Keyboard(w3d, Scancode::Y) || ci.button0 != 0 {
        xit = 1;
        ShootSnd(w3d);
    }

    IN_ClearKeysDown(w3d);
    WaitKeyUp(w3d);

    SD_PlaySound(w3d, whichsnd[xit as usize]);

    return xit;
}

////////////////////////////////////////////////////////////////////
//
// PRINT A MESSAGE IN A WINDOW
//
////////////////////////////////////////////////////////////////////

pub fn Message(w3d: &mut modules, string: String) {
    //println!("Message");

    let mut h: i32;
    let mut w: i32 = 0;
    let mut mw: i32 = 0;
    let len: usize = string.len();
    let font: fontstruct;

    w3d.id_vh.fontnumber = 1;

    let font_vec = w3d.id_ca.grsegs[(STARTFONT + w3d.id_vh.fontnumber) as usize].clone();
    font = bincode::deserialize(&font_vec).unwrap();

    h = font.height as i32;

    for i in 0..len {
        if string.chars().nth(i).unwrap() == '\n' {
            if w > mw {
                mw = w;
            }
            w = 0;
            h += font.height as i32;
        } else {
            w += font.width[string.chars().nth(i).unwrap() as usize] as i32;
        }
    }

    if w + 10 > mw {
        mw = w + 10;
    }

    w3d.id_us.PrintY = (w3d.id_us.WindowH / 2) - h / 2;
    w3d.id_us.PrintX = 160 - mw / 2;
    w3d.id_us.WindowX = 160 - mw / 2;

    DrawWindow(
        w3d,
        w3d.id_us.WindowX - 5,
        w3d.id_us.PrintY - 5,
        mw + 10,
        h + 10,
        TEXTCOLOR,
    );
    DrawOutline(
        w3d,
        w3d.id_us.WindowX - 5,
        w3d.id_us.PrintY - 5,
        mw + 10,
        h + 10,
        0,
        HIGHLIGHT,
    );
    SETFONTCOLOR(w3d, 0, TEXTCOLOR);
    US_Print(w3d, string);
    VW_UpdateScreen(w3d);
}

////////////////////////////////////////////////////////////////////
//
// THIS MAY BE FIXED A LITTLE LATER...
//
////////////////////////////////////////////////////////////////////

pub fn StartCPMusic(w3d: &mut modules, song: i32) -> i32 {
    //println!("StartCPMusic");

    let lastoffs: i32;

    w3d.wl_menu.lastmusic = song;
    lastoffs = SD_MusicOff(w3d);

    UNCACHEAUDIOCHUNK(w3d, STARTMUSIC as usize + w3d.wl_menu.lastmusic as usize);

    SD_StartMusic(w3d, STARTMUSIC as i32 + song);

    return lastoffs;
}

pub fn FreeMusic(w3d: &mut modules) {
    //println!("FreeMusic");

    UNCACHEAUDIOCHUNK(w3d, STARTMUSIC as usize + w3d.wl_menu.lastmusic as usize);
}

///////////////////////////////////////////////////////////////////////////
//
//      IN_GetScanName() - Returns a string containing the name of the
//              specified scan code
//
///////////////////////////////////////////////////////////////////////////

pub fn IN_GetScanName(_w3d: &mut modules, scan: Scancode) -> String {
    //println!("IN_GetScanName");

    if scan == Scancode::RShift {
        return "RShft".to_string();
    } else if scan == Scancode::LShift {
        return "Shift".to_string();
    } else if scan == Scancode::RCtrl {
        return "RCtrl".to_string();
    } else if scan == Scancode::LCtrl {
        return "Ctrl".to_string();
    } else if scan == Scancode::RAlt {
        return "RAlt".to_string();
    } else if scan == Scancode::LAlt {
        return "Alt".to_string();
    } else {
        return scan.name().to_string();
    };
}

///////////////////////////////////////////////////////////////////////////
//
// CHECK FOR PAUSE KEY (FOR MUSIC ONLY)
//
///////////////////////////////////////////////////////////////////////////

pub fn CheckPause(w3d: &mut modules) {
    //println!("CheckPause");

    if w3d.id_in.Paused {
        match w3d.wl_menu.SoundStatus {
            0 => {
                SD_MusicOn(w3d);
            }
            1 => {
                SD_MusicOff(w3d);
            }
            _ => (),
        }

        w3d.wl_menu.SoundStatus ^= 1;
        VW_WaitVBL(w3d, 3);
        IN_ClearKeysDown(w3d);
        w3d.id_in.Paused = false;
    }
}

pub fn DrawMenuGun(w3d: &mut modules, iteminfo: &CP_iteminfo) {
    //println!("DrawMenuGun");

    let x: i32;
    let y: i32;

    x = iteminfo.x;
    y = iteminfo.y + iteminfo.curpos * 13 - 2;

    VWB_DrawPic(w3d, x, y, graphicnums::C_CURSOR1PIC as i32);
}

///////////////////////////////////////////////////////////////////////////
//
// DRAW SCREEN TITLE STRIPES
//
///////////////////////////////////////////////////////////////////////////

pub fn DrawStripes(w3d: &mut modules, y: i32) {
    //println!("DrawStripes");

    {
        VWB_Bar(w3d, 0, y, 320, 24, 0);
        VWB_Hlin(w3d, 0, 319, y + 22, STRIPE);
    }
}

pub fn ShootSnd(w3d: &mut modules) {
    //println!("ShootSnd");

    SD_PlaySound(w3d, soundnames::SHOOTSND);
}

///////////////////////////////////////////////////////////////////////////
//
// CHECK FOR EPISODES
//
///////////////////////////////////////////////////////////////////////////

pub fn CheckForEpisodes(w3d: &mut modules) {
    //println!("CheckForEpisodes()");

    // On Linux like systems, the configdir defaults to $HOME/.wolf4rust
    if w3d.wl_main.configdir == "" {
        // Set config location to home directory for multi-user support

        let homedir = "HOME";
        match env::var(homedir) {
            Ok(_val) => (),
            Err(e) => println!("couldn't interpret {homedir}: {e}"),
        }

        /*homedir = getenv("HOME");
        if(homedir == NULL)
        {
            Quit("Your $HOME directory is not defined. You must set this before playing.");
        }
        */
        let WOLFDIR: String = "/.wolf4rust".to_string();
        if (homedir.len() + WOLFDIR.len()) > { w3d.wl_main.configdir.len() } {
            //Quit("Your $HOME directory path is too long. It cannot be used for saving games.");
        }

        w3d.wl_main.configdir.to_string().push_str(&homedir);

        w3d.wl_main.configdir.to_string().push_str(&WOLFDIR);
    }

    if w3d.wl_main.configdir != "" {
        // Ensure config directory exists and create if necessary
        if metadata(&w3d.wl_main.configdir).is_ok() {
            match { create_dir(&w3d.wl_main.configdir) } {
                Ok(_x) => (),
                //Err(x) => quit("The configuration directory \"{}\" could not be created.",  {configdir}),
                Err(_x) => println!(
                    "The configuration directory \"{}\" could not be created.",
                    { &w3d.wl_main.configdir }
                ),
            }
        }
    }

    //
    // ENGLISH
    //
    #[cfg(feature = "UPLOAD")]
    {
        if Path::new("vswap.wl1").exists() {
            w3d.id_ca.extension.push_str("wl1");
        } else {
            Quit("NO WOLFENSTEIN 3-D DATA FILES to be found!");
        }
    }
    #[cfg(feature = "GOODTIMES")]
    {
        if Path::new("vswap.wl6").exists() {
            w3d.id_ca.extension.push_str("wl6");
        } else {
            Quit("NO WOLFENSTEIN 3-D DATA FILES to be found!");
        }
    }

    w3d.id_ca.graphext.push_str(&w3d.id_ca.extension);
    w3d.id_ca.audioext.push_str(&w3d.id_ca.extension);

    w3d.wl_main.configname.push_str(&w3d.id_ca.extension);
    w3d.wl_menu.SaveName.push_str(&w3d.id_ca.extension);
    w3d.wl_game.demoname.push_str(&w3d.id_ca.extension);

    //w3d.wl_text.helpfilename.push_str(&w3d.id_ca.extension);

    //w3d.wl_text.endfilename.push_str(&w3d.id_ca.extension);
}
