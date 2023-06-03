#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_play
//
//===========================================================================

pub struct wl_play {
    pub madenoise: bool,

    pub playstate: exit_t,

    pub DebugOk: i32,

    pub singlestep: bool,
    pub godmode: u8,
    pub noclip: bool,
    pub ammocheat: bool,
    pub mapreveal: bool,

    pub extravbls: i32,

    pub tilemap: [[tiletype; MAPSIZE as usize]; MAPSIZE as usize], // wall values only
    pub spotvis: [[bool; MAPSIZE as usize]; MAPSIZE as usize],

    //
    // replacing refresh manager
    //
    pub mapwidth: i32,
    pub mapheight: i32,
    pub tics: i32,
    //
    // control info
    //
    pub mouseenabled: u8,
    pub joystickenabled: u8,
    pub dirscan: [Scancode; 4],
    pub buttonscan: [Scancode; buttontype::NUMBUTTONS as usize],
    pub buttonmouse: [buttontype; 4],
    pub buttonjoy: [buttontype; 32],

    pub viewsize: i32,

    pub buttonheld: [bool; buttontype::NUMBUTTONS as usize],

    pub demorecord: bool,
    pub demoplayback: bool,
    pub demoptr: Vec<i8>,
    pub demoptr_i: usize,
    pub lastdemoptr: Vec<i8>,
    pub lastdemoptr_i: usize,
    pub demobuffer: Vec<u8>,
    //
    // current user input
    //
    pub controlx: i32, // range from -100 to 100 per tic
    pub controly: i32,
    pub buttonstate: [bool; buttontype::NUMBUTTONS as usize],

    pub lastgamemusicoffset: i32,

    pub lastmusicchunk: musicnames,

    pub redshifts: [[Color; 256]; NUMREDSHIFTS as usize],
    pub whiteshifts: [[Color; 256]; NUMWHITESHIFTS as usize],

    pub damagecount: i32,
    pub bonuscount: i32,
    pub palshifted: bool,

    pub funnyticount: i32,
}

impl wl_play {
    pub fn new() -> Self {
        Self {
            madenoise: false,

            playstate: exit_t::ex_stillplaying,

            DebugOk: 0,

            singlestep: false,
            godmode: 0,
            noclip: false,
            ammocheat: false,
            mapreveal: false,

            extravbls: 0,

            tilemap: [[0; MAPSIZE as usize]; MAPSIZE as usize],
            spotvis: [[false; MAPSIZE as usize]; MAPSIZE as usize],

            mapwidth: 0,
            mapheight: 0,
            tics: 0,

            mouseenabled: 0,
            joystickenabled: 0,
            dirscan: [
                Scancode::Up,
                Scancode::Right,
                Scancode::Down,
                Scancode::Left,
            ],
            buttonscan: [
                Scancode::LCtrl,
                Scancode::LAlt,
                Scancode::LShift,
                Scancode::Space,
                Scancode::Num1, //wp_knife
                Scancode::Num2, //wp_pistol
                Scancode::Num3, //wp_machinegun
                Scancode::Num4, //wp_chaingun
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
                Scancode::F24,
            ],
            buttonmouse: [
                buttontype::bt_attack,
                buttontype::bt_strafe,
                buttontype::bt_use,
                buttontype::bt_nobutton,
            ],
            buttonjoy: [
                buttontype::bt_attack,
                buttontype::bt_strafe,
                buttontype::bt_use,
                buttontype::bt_run,
                buttontype::bt_strafeleft,
                buttontype::bt_straferight,
                buttontype::bt_esc,
                buttontype::bt_pause,
                buttontype::bt_prevweapon,
                buttontype::bt_nextweapon,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
                buttontype::bt_nobutton,
            ],

            viewsize: 0,

            buttonheld: [false; buttontype::NUMBUTTONS as usize],

            demorecord: false,
            demoplayback: false,
            demoptr: Vec::new(),
            demoptr_i: 0,
            lastdemoptr: Vec::new(),
            lastdemoptr_i: 0,
            demobuffer: Vec::new(),

            controlx: 0,
            controly: 0,
            buttonstate: [false; buttontype::NUMBUTTONS as usize],

            lastgamemusicoffset: 0,

            lastmusicchunk: musicnames::LASTMUSIC,

            redshifts: [[Color::RED; 256]; NUMREDSHIFTS as usize],
            whiteshifts: [[Color::WHITE; 256]; NUMWHITESHIFTS as usize],

            damagecount: 0,
            bonuscount: 0,
            palshifted: false,

            funnyticount: 0,
        }
    }
    pub fn clear(&mut self) {
        self.madenoise = false;
        self.playstate = exit_t::ex_stillplaying;
        self.DebugOk = 0;
        self.singlestep = false;
        self.godmode = 0;
        self.noclip = false;
        self.ammocheat = false;
        self.mapreveal = false;
        self.extravbls = 0;
        self.tilemap = [[0; MAPSIZE as usize]; MAPSIZE as usize];
        self.spotvis = [[false; MAPSIZE as usize]; MAPSIZE as usize];
        self.mapwidth = 0;
        self.mapheight = 0;
        self.tics = 0;
        //self.mouseenabled = 0;
        //self.joystickenabled = 0;
        self.viewsize = 0;
        self.buttonheld = [false; buttontype::NUMBUTTONS as usize];
        self.demorecord = false;
        self.demoplayback = false;
        self.demoptr = Vec::new();
        self.demoptr_i = 0;
        self.lastdemoptr = Vec::new();
        self.lastdemoptr_i = 0;
        self.demobuffer = Vec::new();
        self.controlx = 0;
        self.controly = 0;
        self.buttonstate = [false; buttontype::NUMBUTTONS as usize];
        self.lastgamemusicoffset = 0;
        self.lastmusicchunk = musicnames::LASTMUSIC;
        //self.redshifts = [[Color::RED; 256]; NUMREDSHIFTS as usize];
        //self.whiteshifts = [[Color::WHITE; 256]; NUMWHITESHIFTS as usize];
        self.damagecount = 0;
        self.bonuscount = 0;
        self.palshifted = false;
        self.funnyticount = 0;
    }
}

pub struct object {
    pub objlist: Vec<objtype>, //MAXACTORS
    pub objlist_i: usize,
    pub newobj: objtype,
    pub killerobj: objtype,
    pub objcount: i32,
    pub actorat: Vec<Vec<*mut objtype>>,
}

impl object {
    pub fn new() -> Self {
        Self {
            objlist: Vec::new(),
            objlist_i: 0,
            newobj: objtype::new(),
            killerobj: objtype::new(),
            objcount: 0,
            actorat: vec![vec![ptr::null_mut(); MAPSIZE as usize]; MAPSIZE as usize],
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

//
// LIST OF SONGS FOR EACH VERSION
//
pub const songs: [musicnames; 60] = [
    //
    // Episode One
    //
    musicnames::GETTHEM_MUS,
    musicnames::SEARCHN_MUS,
    musicnames::POW_MUS,
    musicnames::SUSPENSE_MUS,
    musicnames::GETTHEM_MUS,
    musicnames::SEARCHN_MUS,
    musicnames::POW_MUS,
    musicnames::SUSPENSE_MUS,
    musicnames::WARMARCH_MUS, // Boss level
    musicnames::CORNER_MUS,   // Secret level
    //
    // Episode Two
    //
    musicnames::NAZI_OMI_MUS,
    musicnames::PREGNANT_MUS,
    musicnames::GOINGAFT_MUS,
    musicnames::HEADACHE_MUS,
    musicnames::NAZI_OMI_MUS,
    musicnames::PREGNANT_MUS,
    musicnames::HEADACHE_MUS,
    musicnames::GOINGAFT_MUS,
    musicnames::WARMARCH_MUS, // Boss level
    musicnames::DUNGEON_MUS,  // Secret level
    //
    // Episode Three
    //
    musicnames::INTROCW3_MUS,
    musicnames::NAZI_RAP_MUS,
    musicnames::TWELFTH_MUS,
    musicnames::ZEROHOUR_MUS,
    musicnames::INTROCW3_MUS,
    musicnames::NAZI_RAP_MUS,
    musicnames::TWELFTH_MUS,
    musicnames::ZEROHOUR_MUS,
    musicnames::ULTIMATE_MUS, // Boss level
    musicnames::PACMAN_MUS,   // Secret level
    //
    // Episode Four
    //
    musicnames::GETTHEM_MUS,
    musicnames::SEARCHN_MUS,
    musicnames::POW_MUS,
    musicnames::SUSPENSE_MUS,
    musicnames::GETTHEM_MUS,
    musicnames::SEARCHN_MUS,
    musicnames::POW_MUS,
    musicnames::SUSPENSE_MUS,
    musicnames::WARMARCH_MUS, // Boss level
    musicnames::CORNER_MUS,   // Secret level
    //
    // Episode Five
    //
    musicnames::NAZI_OMI_MUS,
    musicnames::PREGNANT_MUS,
    musicnames::GOINGAFT_MUS,
    musicnames::HEADACHE_MUS,
    musicnames::NAZI_OMI_MUS,
    musicnames::PREGNANT_MUS,
    musicnames::HEADACHE_MUS,
    musicnames::GOINGAFT_MUS,
    musicnames::WARMARCH_MUS, // Boss level
    musicnames::DUNGEON_MUS,  // Secret level
    //
    // Episode Six
    //
    musicnames::INTROCW3_MUS,
    musicnames::NAZI_RAP_MUS,
    musicnames::TWELFTH_MUS,
    musicnames::ZEROHOUR_MUS,
    musicnames::INTROCW3_MUS,
    musicnames::NAZI_RAP_MUS,
    musicnames::TWELFTH_MUS,
    musicnames::ZEROHOUR_MUS,
    musicnames::ULTIMATE_MUS, // Boss level
    musicnames::FUNKYOU_MUS,  // Secret level
];

pub const NUMREDSHIFTS: i32 = 6;
pub const REDSTEPS: i32 = 8;

pub const NUMWHITESHIFTS: i32 = 3;
pub const WHITESTEPS: i32 = 20;
pub const WHITETICS: i32 = 6;

/*
=============================================================================

                               USER CONTROL

=============================================================================
*/

/*
===================
=
= PollKeyboardButtons
=
===================
*/

pub fn PollKeyboardButtons(w3d: &mut modules) {
    //println!("PollKeyboardButtons");

    for i in 0..buttontype::NUMBUTTONS as usize {
        if Keyboard(w3d, w3d.wl_play.buttonscan[i]) {
            w3d.wl_play.buttonstate[i] = true;
        }
    }
}

/*
===================
=
= PollMouseButtons
=
===================
*/

pub fn PollMouseButtons(w3d: &mut modules) {
    //println!("PollMouseButtons");

    let buttons = IN_MouseButtons(w3d);

    if buttons & 1 != 0 {
        w3d.wl_play.buttonstate[w3d.wl_play.buttonmouse[0] as usize] = true;
    }
    if buttons & 2 != 0 {
        w3d.wl_play.buttonstate[w3d.wl_play.buttonmouse[1] as usize] = true;
    }
    if buttons & 4 != 0 {
        w3d.wl_play.buttonstate[w3d.wl_play.buttonmouse[2] as usize] = true;
    }
}

/*
===================
=
= PollJoystickButtons
=
===================
*/

pub fn PollJoystickButtons(w3d: &mut modules) {
    //println!("PollJoystickButtons");

    let mut val = 1;
    let buttons = IN_JoyButtons(w3d);

    for i in 0..w3d.id_in.JoyNumButtons {
        if buttons & val != 0 {
            w3d.wl_play.buttonstate[w3d.wl_play.buttonjoy[i as usize] as usize] = true;
        }
        val <<= 1;
    }
}

/*
===================
=
= PollKeyboardMove
=
===================
*/

pub fn PollKeyboardMove(w3d: &mut modules) {
    //println!("PollKeyboardMove");
    let delta: i32;

    if w3d.wl_play.buttonstate[buttontype::bt_run as usize] {
        delta = RUNMOVE * w3d.wl_play.tics
    } else {
        delta = BASEMOVE * w3d.wl_play.tics;
    }

    if Keyboard(w3d, w3d.wl_play.dirscan[controldir_t::di_north as usize]) {
        w3d.wl_play.controly -= delta;
    }
    if Keyboard(w3d, w3d.wl_play.dirscan[controldir_t::di_south as usize]) {
        w3d.wl_play.controly += delta;
    }
    if Keyboard(w3d, w3d.wl_play.dirscan[controldir_t::di_west as usize]) {
        w3d.wl_play.controlx -= delta;
    }
    if Keyboard(w3d, w3d.wl_play.dirscan[controldir_t::di_east as usize]) {
        w3d.wl_play.controlx += delta;
    }
}

/*
===================
=
= PollMouseMove
=
===================
*/

pub fn PollMouseMove(w3d: &mut modules) {
    //println!("PollMouseMove");

    let mousexmove: i32;
    let mouseymove: i32;

    let event_pump = w3d.id_vl.sdl_context.event_pump().unwrap();

    let buttons = event_pump.relative_mouse_state();
    mousexmove = buttons.x();
    mouseymove = buttons.y();

    w3d.wl_play.controlx += mousexmove * 10 / (13 - w3d.wl_main.mouseadjustment);
    w3d.wl_play.controly += mouseymove * 20 / (13 - w3d.wl_main.mouseadjustment);
}

/*
===================
=
= PollJoystickMove
=
===================
*/

pub fn PollJoystickMove(w3d: &mut modules) {
    //println!("PollJoystickMove");

    let mut joyx: i16 = 0;
    let mut joyy: i16 = 0;
    let delta: i32;

    IN_GetJoyDelta(w3d, &mut joyx, &mut joyy);

    if w3d.wl_play.buttonstate[buttontype::bt_run as usize] {
        delta = RUNMOVE * w3d.wl_play.tics
    } else {
        delta = BASEMOVE * w3d.wl_play.tics;
    }

    if joyx > 64 || w3d.wl_play.buttonstate[buttontype::bt_turnright as usize] {
        w3d.wl_play.controlx += delta;
    } else if joyx < -64 || w3d.wl_play.buttonstate[buttontype::bt_turnleft as usize] {
        w3d.wl_play.controlx -= delta;
    }

    if joyy > 64 || w3d.wl_play.buttonstate[buttontype::bt_movebackward as usize] {
        w3d.wl_play.controly += delta;
    } else if joyy < -64 || w3d.wl_play.buttonstate[buttontype::bt_moveforward as usize] {
        w3d.wl_play.controly -= delta;
    }
}

/*
===================
=
= PollControls
=
= Gets user or demo input, call once each frame
=
= controlx              set between -100 and 100 per tic
= controly
= buttonheld[]  the state of the buttons LAST frame
= buttonstate[] the state of the buttons THIS frame
=
===================
*/

pub fn PollControls(w3d: &mut modules) {
    //println!("PollControls");

    let max: i32;
    let min: i32;
    let mut buttonbits: i32;

    IN_ProcessEvents(w3d);

    //
    // get timing info for last frame
    //
    if w3d.wl_play.demoplayback || w3d.wl_play.demorecord
    // demo recording and playback needs to be constant
    {
        // wait up to DEMOTICS Wolf tics
        let curtime = w3d.id_vl.timer.ticks() as i32;
        w3d.wl_draw.lasttimecount += DEMOTICS;
        let timediff = (w3d.wl_draw.lasttimecount * 100) / 7 - curtime;

        if timediff > 0 {
            SDL_Delay(w3d, timediff);
        }

        if timediff < -2 * DEMOTICS {
            // more than 2-times DEMOTICS behind?
            w3d.wl_draw.lasttimecount = (curtime * 7) / 100; // yes, set to current timecount
        }

        w3d.wl_play.tics = DEMOTICS;
    } else {
        CalcTics(w3d);
    }

    w3d.wl_play.controlx = 0;
    w3d.wl_play.controly = 0;
    //memcpy (buttonheld, buttonstate, sizeof (buttonstate));
    w3d.wl_play.buttonheld = w3d.wl_play.buttonstate;
    //memset (buttonstate, 0, sizeof (buttonstate));
    w3d.wl_play.buttonstate = [false; buttontype::NUMBUTTONS as usize];

    if w3d.wl_play.demoplayback {
        //
        // read commands from demo buffer
        //

        buttonbits = w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] as i32;
        w3d.wl_play.demoptr_i += 1;
        for i in 0..buttontype::NUMBUTTONS as usize {
            if buttonbits & 1 == 1 {
                w3d.wl_play.buttonstate[i] = true;
            } else {
                w3d.wl_play.buttonstate[i] = false;
            }

            buttonbits >>= 1;
        }

        w3d.wl_play.controlx = w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] as i32;
        w3d.wl_play.demoptr_i += 1;
        w3d.wl_play.controly = w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] as i32;
        w3d.wl_play.demoptr_i += 1;

        if w3d.wl_play.demoptr_i == w3d.wl_play.lastdemoptr_i {
            w3d.wl_play.playstate = exit_t::ex_completed; // demo is done
        }

        w3d.wl_play.controlx *= w3d.wl_play.tics;
        w3d.wl_play.controly *= w3d.wl_play.tics;

        return;
    }

    //
    // get button states
    //
    PollKeyboardButtons(w3d);

    if w3d.wl_play.mouseenabled != 0 && IN_IsInputGrabbed(w3d) {
        PollMouseButtons(w3d);
    }

    if w3d.wl_play.joystickenabled != 0 {
        PollJoystickButtons(w3d);
    }

    //
    // get movements
    //
    PollKeyboardMove(w3d);

    if w3d.wl_play.mouseenabled != 0 && IN_IsInputGrabbed(w3d) {
        PollMouseMove(w3d);
    }

    if w3d.wl_play.joystickenabled != 0 {
        PollJoystickMove(w3d);
    }

    //
    // bound movement to a maximum
    //
    max = 100 * w3d.wl_play.tics;
    min = -max;

    if w3d.wl_play.controlx > max {
        w3d.wl_play.controlx = max;
    } else if w3d.wl_play.controlx < min {
        w3d.wl_play.controlx = min;
    }

    if w3d.wl_play.controly > max {
        w3d.wl_play.controly = max;
    } else if w3d.wl_play.controly < min {
        w3d.wl_play.controly = min;
    }

    if w3d.wl_play.demorecord {
        //
        // save info out to demo buffer
        //
        w3d.wl_play.controlx /= w3d.wl_play.tics;
        w3d.wl_play.controly /= w3d.wl_play.tics;

        buttonbits = 0;

        // TODO: Support 32-bit buttonbits
        for i in (0..buttontype::NUMBUTTONS as usize - 1).rev() {
            buttonbits <<= 1;
            if w3d.wl_play.buttonstate[i] {
                buttonbits |= 1;
            }
        }

        w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] = buttonbits as i8;
        w3d.wl_play.demoptr_i += 1;
        w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] = w3d.wl_play.controlx as i8;
        w3d.wl_play.demoptr_i += 1;
        w3d.wl_play.demoptr[w3d.wl_play.demoptr_i] = w3d.wl_play.controly as i8;

        if w3d.wl_play.demoptr_i >= w3d.wl_play.lastdemoptr_i - 8 {
            w3d.wl_play.playstate = exit_t::ex_completed;
        } else {
            w3d.wl_play.controlx *= w3d.wl_play.tics;
            w3d.wl_play.controly *= w3d.wl_play.tics;
        }
    }
}

//==========================================================================

/*
=====================
=
= CheckKeys
=
=====================
*/

pub fn CheckKeys(w3d: &mut modules, ob: &mut object) {
    //println!("CheckKeys");

    let scan: Scancode;

    if w3d.id_vl.screenfaded || w3d.wl_play.demoplayback {
        // don't do anything with a faded screen
        return;
    }

    scan = w3d.id_in.LastScan;

    //
    // SECRET CHEAT CODE: 'MLI'
    //
    if Keyboard(w3d, Scancode::M) && Keyboard(w3d, Scancode::L) && Keyboard(w3d, Scancode::I) {
        w3d.wl_game.gamestate.health = 100;
        w3d.wl_game.gamestate.ammo = 99;
        w3d.wl_game.gamestate.keys = 3;
        w3d.wl_game.gamestate.score = 0;
        w3d.wl_game.gamestate.TimeCount += 42000;
        GiveWeapon(w3d, weapontype::wp_chaingun);
        DrawWeapon(w3d);
        DrawHealth(w3d);
        DrawKeys(w3d);
        DrawAmmo(w3d);
        DrawScore(w3d);

        ClearMemory(w3d, ob);
        ClearSplitVWB(w3d);

        //Message (STR_CHEATER1 "\n" STR_CHEATER2 "\n\n" STR_CHEATER3 "\n" STR_CHEATER4 "\n" STR_CHEATER5);

        IN_ClearKeysDown(w3d);
        IN_Ack(w3d);

        if w3d.wl_play.viewsize < 17 {
            DrawPlayBorder(w3d);
        }
    }

    //
    // OPEN UP DEBUG KEYS
    //

    if Keyboard(w3d, Scancode::Backspace)
        && Keyboard(w3d, Scancode::LShift)
        && Keyboard(w3d, Scancode::LAlt)
        && w3d.wl_main.param_debugmode
    {
        ClearMemory(w3d, ob);
        ClearSplitVWB(w3d);

        //Message ("Debugging keys are\nnow available!");
        IN_ClearKeysDown(w3d);
        IN_Ack(w3d);

        DrawPlayBorderSides(w3d);
        w3d.wl_play.DebugOk = 1;
    }

    //
    // TRYING THE KEEN CHEAT CODE!
    //
    if Keyboard(w3d, Scancode::B) && Keyboard(w3d, Scancode::A) && Keyboard(w3d, Scancode::T) {
        ClearMemory(w3d, ob);
        ClearSplitVWB(w3d);

        //Message ("Commander Keen is also\n" "available from Apogee, but\n" "then, you already know\n" "that - right, Cheatmeister?!");

        IN_ClearKeysDown(w3d);
        IN_Ack(w3d);

        if w3d.wl_play.viewsize < 18 {
            DrawPlayBorder(w3d);
        }
    }

    //
    // pause key weirdness can't be checked as a scan code
    //
    if w3d.wl_play.buttonstate[buttontype::bt_pause as usize] {
        w3d.id_in.Paused = true;
    }

    if w3d.id_in.Paused {
        let lastoffs = StopMusic(w3d);
        VWB_DrawPic(w3d, 16 * 8, 80 - 2 * 8, graphicnums::PAUSEDPIC as i32);
        VW_UpdateScreen(w3d);
        IN_Ack(w3d);
        w3d.id_in.Paused = false;
        ContinueMusic(w3d, lastoffs);
        if w3d.id_in.MousePresent && IN_IsInputGrabbed(w3d) {
            IN_CenterMouse(w3d); // Clear accumulated mouse movement
        }
        w3d.wl_draw.lasttimecount = GetTimeCount(w3d);
        return;
    }

    //
    // F1-F7/ESC to enter control panel
    //
    if scan == Scancode::F10 || scan == Scancode::F9 || scan == Scancode::F7 || scan == Scancode::F8
    // pop up quit dialog
    {
        ClearMemory(w3d, ob);
        ClearSplitVWB(w3d);
        US_ControlPanel(w3d, ob, scan);

        DrawPlayBorderSides(w3d);

        SETFONTCOLOR(w3d, 0, 15);
        IN_ClearKeysDown(w3d);
        return;
    }

    if (scan as i32 >= Scancode::F1 as i32 && scan as i32 <= Scancode::F9 as i32)
        || scan == Scancode::Escape
        || w3d.wl_play.buttonstate[buttontype::bt_esc as usize]
    {
        let lastoffs = StopMusic(w3d);

        ClearMemory(w3d, ob);
        VW_FadeOut(w3d);

        if w3d.wl_play.buttonstate[buttontype::bt_esc as usize] {
            US_ControlPanel(w3d, ob, Scancode::Escape);
        } else {
            US_ControlPanel(w3d, ob, scan);
        }

        SETFONTCOLOR(w3d, 0, 15);
        IN_ClearKeysDown(w3d);
        VW_FadeOut(w3d);

        if w3d.wl_play.viewsize != 21 {
            DrawPlayScreen(w3d);
        }

        if !w3d.wl_main.startgame && !w3d.wl_main.loadedgame {
            ContinueMusic(w3d, lastoffs);
        }

        if w3d.wl_main.loadedgame {
            w3d.wl_play.playstate = exit_t::ex_abort;
        }

        w3d.wl_draw.lasttimecount = GetTimeCount(w3d);

        if w3d.id_in.MousePresent && IN_IsInputGrabbed(w3d) {
            IN_CenterMouse(w3d); // Clear accumulated mouse movement
        }
        return;
    }

    //
    // TAB-? debug keys
    //

    if Keyboard(w3d, Scancode::Tab) && w3d.wl_play.DebugOk != 0 {
        w3d.id_vh.fontnumber = 0;
        SETFONTCOLOR(w3d, 0, 15);
        if DebugKeys() && w3d.wl_play.viewsize < 20 {
            DrawPlayBorder(w3d); // dont let the blue borders flash

            if w3d.id_in.MousePresent && IN_IsInputGrabbed(w3d) {
                IN_CenterMouse(w3d); // Clear accumulated mouse movement
            }

            w3d.wl_draw.lasttimecount = GetTimeCount(w3d);
        }
        return;
    }
}

//===========================================================================

/*
#############################################################################

                                  The objlist data structure

#############################################################################

objlist containt structures for every actor currently playing.  The structure
is accessed as a linked list starting at *player, ending when ob->next ==
NULL.  GetNewObj inserts a new object at the end of the list, meaning that
if an actor spawn another actor, the new one WILL get to think and react the
same frame.  RemoveObj unlinks the given object and returns it to the free
list, but does not damage the objects ->next pointer, so if the current object
removes itself, a linked list following loop can still safely get to the
next element.

<backwardly linked free list>

#############################################################################
*/

/*
=========================
=
= InitActorList
=
= Call to clear out the actor object lists returning them all to the free
= list.  Allocates a special spot for the player.
=
=========================
*/

pub fn InitActorList(ob: &mut object) {
    //println!("InitActorList");

    //
    // init the actor lists
    //

    ob.objlist = Vec::new();
    ob.objcount = 0;

    //
    // give the player the first free spots
    //

    GetNewActor(ob);

    ob.objlist.push(ob.newobj); // player is the first object in the list
}

//===========================================================================

//===========================================================================

/*
=========================
=
= GetNewActor
=
= Sets the global variable new to point to a free spot in objlist.
= The free spot is inserted at the end of the liked list
=
= When the object list is full, the caller can either have it bomb out ot
= return a dummy object pointer that will never get used
=
=========================
*/

pub fn GetNewActor(ob: &mut object) {
    //println!("GetNewActor");

    if ob.objlist.len() == MAXACTORS as usize {
        Quit("GetNewActor: No free spots in objlist!");
    }

    ob.newobj.id = ob.objcount + 1; // agent id is '1'
    ob.newobj.active = activetype::ac_no;

    ob.objcount += 1;
}

//===========================================================================

/*
=========================
=
= RemoveObj
=
= Add the given object back into the free list, and unlink it from it's
= neighbors
=
=========================
*/

pub fn RemoveObj(ob: &mut object) {
    //println!("RemoveObj");

    let player = ob.objlist[0];
    let gone = ob.objlist[ob.objlist_i];

    if gone == player {
        Quit("RemoveObj: Tried to remove the player!");
    }
    ob.objlist.remove(ob.objlist_i);

    ob.objcount -= 1;
}

/*
=============================================================================

                                                MUSIC STUFF

=============================================================================
*/

/*
=================
=
= StopMusic
=
=================
*/

pub fn StopMusic(w3d: &mut modules) -> i32 {
    //println!("StopMusic");

    let lastoffs = SD_MusicOff(w3d);

    UNCACHEAUDIOCHUNK(
        w3d,
        (STARTMUSIC + w3d.wl_play.lastmusicchunk as i32) as usize,
    );

    return lastoffs;
}

//==========================================================================

/*
=================
=
= StartMusic
=
=================
*/

pub fn StartMusic(w3d: &mut modules) {
    //println!("StartMusic");

    SD_MusicOff(w3d);
    w3d.wl_play.lastmusicchunk =
        songs[(w3d.wl_game.gamestate.mapon + w3d.wl_game.gamestate.episode * 10) as usize];
    SD_StartMusic(w3d, STARTMUSIC as i32 + w3d.wl_play.lastmusicchunk as i32);
}

pub fn ContinueMusic(w3d: &mut modules, offs: i32) {
    //println!("ContinueMusic");

    let mut offs = offs;

    SD_MusicOff(w3d);
    w3d.wl_play.lastmusicchunk =
        songs[(w3d.wl_game.gamestate.mapon + w3d.wl_game.gamestate.episode * 10) as usize];

    SD_ContinueMusic(
        w3d,
        STARTMUSIC + w3d.wl_play.lastmusicchunk as i32,
        &mut offs,
    );
}

/*
=============================================================================

                                        PALETTE SHIFTING STUFF

=============================================================================
*/

/*
=====================
=
= InitRedShifts
=
=====================
*/

pub fn InitRedShifts(w3d: &mut modules) {
    //println!("InitRedShifts");

    let mut workptr: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    let mut baseptr: Color;
    let mut delta: i32;

    //
    // fade through intermediate frames
    //
    for i in 1..=NUMREDSHIFTS {
        //workptr = redshifts[i - 1];
        //baseptr = gamepal;

        for j in 0..=255 {
            baseptr = w3d.id_vl.gamepal[j];

            delta = 256 - baseptr.r as i32;
            workptr.r = (baseptr.r as i32 + delta * i / REDSTEPS) as u8;
            delta = -(baseptr.g as i32);
            workptr.g = (baseptr.g as i32 + delta * i / REDSTEPS) as u8;
            delta = -(baseptr.b as i32);
            workptr.b = (baseptr.b as i32 + delta * i / REDSTEPS) as u8;

            w3d.wl_play.redshifts[(i - 1) as usize][j] = workptr;
        }
    }

    for i in 1..=NUMWHITESHIFTS {
        //workptr = redshifts[i - 1];
        //baseptr = gamepal;

        for j in 0..=255 {
            baseptr = w3d.id_vl.gamepal[j];

            delta = 256 - baseptr.r as i32;
            workptr.r = (baseptr.r as i32 + delta * i / WHITESTEPS) as u8;
            delta = 248 - baseptr.g as i32;
            workptr.g = (baseptr.g as i32 + delta * i / WHITESTEPS) as u8;
            delta = -(baseptr.b as i32);
            workptr.b = (baseptr.b as i32 + delta * i / WHITESTEPS) as u8;

            w3d.wl_play.whiteshifts[(i - 1) as usize][j] = workptr;
        }
    }
}

/*
=====================
=
= ClearPaletteShifts
=
=====================
*/

pub fn ClearPaletteShifts(w3d: &mut modules) {
    //println!("ClearPaletteShifts");

    w3d.wl_play.bonuscount = 0;
    w3d.wl_play.damagecount = 0;
    w3d.wl_play.palshifted = false;
}

/*
=====================
=
= StartBonusFlash
=
=====================
*/

pub fn StartBonusFlash(w3d: &mut modules) {
    //println!("StartBonusFlash");

    w3d.wl_play.bonuscount = NUMWHITESHIFTS * WHITETICS; // white shift palette
}

/*
=====================
=
= StartDamageFlash
=
=====================
*/

pub fn StartDamageFlash(w3d: &mut modules, damage: i32) {
    //println!("StartDamageFlash");

    w3d.wl_play.damagecount += damage;
}

/*
=====================
=
= UpdatePaletteShifts
=
=====================
*/

pub fn UpdatePaletteShifts(w3d: &mut modules) {
    //println!("UpdatePaletteShifts");

    let mut red: i32;
    let mut white: i32;

    if w3d.wl_play.bonuscount != 0 {
        white = w3d.wl_play.bonuscount / WHITETICS as i32 + 1;
        if white > NUMWHITESHIFTS as i32 {
            white = NUMWHITESHIFTS as i32;
        }

        w3d.wl_play.bonuscount -= w3d.wl_play.tics;

        if w3d.wl_play.bonuscount < 0 {
            w3d.wl_play.bonuscount = 0;
        }
    } else {
        white = 0;
    }

    if w3d.wl_play.damagecount != 0 {
        red = w3d.wl_play.damagecount / 10 + 1;
        if red > NUMREDSHIFTS as i32 {
            red = NUMREDSHIFTS as i32;
        }

        w3d.wl_play.damagecount -= w3d.wl_play.tics;

        if w3d.wl_play.damagecount < 0 {
            w3d.wl_play.damagecount = 0;
        }
    } else {
        red = 0;
    }

    if red != 0 {
        VL_SetPalette(w3d, w3d.wl_play.redshifts[red as usize - 1], false);
        w3d.wl_play.palshifted = true;
    } else if white != 0 {
        VL_SetPalette(w3d, w3d.wl_play.whiteshifts[white as usize - 1], false);
        w3d.wl_play.palshifted = true;
    } else if w3d.wl_play.palshifted {
        VL_SetPalette(w3d, w3d.id_vl.gamepal, false); // back to normal
        w3d.wl_play.palshifted = false;
    }
}

/*
=====================
=
= FinishPaletteShifts
=
= Resets palette to normal if needed
=
=====================
*/

pub fn FinishPaletteShifts(w3d: &mut modules) {
    //println!("FinishPaletteShifts");

    if w3d.wl_play.palshifted {
        w3d.wl_play.palshifted = false;
        VL_SetPalette(w3d, w3d.id_vl.gamepal, true);
    }
}

/*
=============================================================================

                                                CORE PLAYLOOP

=============================================================================
*/

/*
=====================
=
= DoActor
=
=====================
*/

pub fn DoActor(w3d: &mut modules, ob: &mut object) {
    //println!("DoActor");

    if ob.objlist[ob.objlist_i].active as i32 == 0
        && ob.objlist[ob.objlist_i].areanumber < NUMAREAS
        && !w3d.wl_act1.areabyplayer[ob.objlist[ob.objlist_i].areanumber as usize]
    {
        ob.objlist_i += 1;
        return;
    }

    if (ob.objlist[ob.objlist_i].flags
        & (objflag_t::FL_NONMARK as i32 | objflag_t::FL_NEVERMARK as i32))
        == 0
    {
        ob.actorat[ob.objlist[ob.objlist_i].tilex as usize]
            [ob.objlist[ob.objlist_i].tiley as usize] = ptr::null_mut();
    }

    //
    // non transitional object
    //
    if ob.objlist[ob.objlist_i].ticcount == 0 {
        //
        if ob.objlist[ob.objlist_i].state.think {
            //
            Think(w3d, ob, true);
            //
            if ob.objlist[ob.objlist_i].state == statetype::new() {
                RemoveObj(ob);

                ob.objlist_i += 1;
                return;
            }
        }

        if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_NEVERMARK as i32) != 0 {
            ob.objlist_i += 1;
            return;
        }

        if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_NONMARK as i32) != 0
            && !ob.actorat[ob.objlist[ob.objlist_i].tilex as usize]
                [ob.objlist[ob.objlist_i].tiley as usize]
                .is_null()
        {
            ob.objlist_i += 1;
            return;
        }

        ob.actorat[ob.objlist[ob.objlist_i].tilex as usize]
            [ob.objlist[ob.objlist_i].tiley as usize] =
            &mut ob.objlist[ob.objlist_i] as *mut objtype;

        ob.objlist_i += 1;
        return;
    }

    //
    // transitional object
    //
    ob.objlist[ob.objlist_i].ticcount -= w3d.wl_play.tics;

    while ob.objlist[ob.objlist_i].ticcount <= 0 {
        if ob.objlist[ob.objlist_i].state.action {
            //
            Think(w3d, ob, true);
            //
            if ob.objlist[ob.objlist_i].state == statetype::new() {
                RemoveObj(ob);

                ob.objlist_i += 1;
                return;
            }
        }

        //just update the state
        Think(w3d, ob, false);
        //
        if ob.objlist[ob.objlist_i].state == statetype::new() {
            RemoveObj(ob);

            ob.objlist_i += 1;
            return;
        }

        if ob.objlist[ob.objlist_i].state.tictime == 0 {
            ob.objlist[ob.objlist_i].ticcount = 0;
            //goto think;
            break;
        }
        ob.objlist[ob.objlist_i].ticcount += ob.objlist[ob.objlist_i].state.tictime;
    }

    //think:
    //
    // think
    //
    if ob.objlist[ob.objlist_i].state.think {
        //
        Think(w3d, ob, true);
        //
        if ob.objlist[ob.objlist_i].state == statetype::new() {
            RemoveObj(ob);

            ob.objlist_i += 1;
            return;
        }
    }

    if ob.objlist[ob.objlist_i].flags & objflag_t::FL_NEVERMARK as i32 != 0 {
        ob.objlist_i += 1;
        return;
    }

    if (ob.objlist[ob.objlist_i].flags & objflag_t::FL_NONMARK as i32 != 0)
        && !ob.actorat[ob.objlist[ob.objlist_i].tilex as usize]
            [ob.objlist[ob.objlist_i].tiley as usize]
            .is_null()
    {
        ob.objlist_i += 1;
        return;
    }

    ob.actorat[ob.objlist[ob.objlist_i].tilex as usize][ob.objlist[ob.objlist_i].tiley as usize] =
        &mut ob.objlist[ob.objlist_i] as *mut objtype;

    ob.objlist_i += 1;
}

//==========================================================================

/*
===================
=
= PlayLoop
=
===================
*/

pub fn PlayLoop(w3d: &mut modules, ob: &mut object) {
    //println!("PlayLoop");

    w3d.wl_play.playstate = exit_t::ex_stillplaying;
    w3d.wl_draw.lasttimecount = GetTimeCount(w3d);
    w3d.wl_draw.frameon = 0;
    w3d.wl_agent.anglefrac = 0;
    w3d.wl_agent.facecount = 0;
    w3d.wl_play.funnyticount = 0;
    w3d.wl_play.buttonstate = [false; buttontype::NUMBUTTONS as usize];

    ClearPaletteShifts(w3d);

    if w3d.id_in.MousePresent && IN_IsInputGrabbed(w3d) {
        IN_CenterMouse(w3d); // Clear accumulated mouse movement
    }

    if w3d.wl_play.demoplayback {
        IN_StartAck(w3d);
    }

    loop {
        PollControls(w3d);
        //
        // actor thinking
        //
        w3d.wl_play.madenoise = false;

        MoveDoors(w3d, ob);

        MovePWalls(w3d, ob);

        ob.objlist_i = 0;
        let mut object_i = 0;
        loop {
            DoActor(w3d, ob);
            // objlist can grow or shrink
            if object_i >= ob.objlist.len() - 1 {
                break;
            }
            object_i += 1;
        }
        ob.objlist_i = 0;

        UpdatePaletteShifts(w3d);

        ThreeDRefresh(w3d, ob);
        //
        // MAKE FUNNY FACE IF BJ DOESN'T MOVE FOR AWHILE
        //
        w3d.wl_game.gamestate.TimeCount += w3d.wl_play.tics;

        UpdateSoundLoc(w3d); // JAB
        if w3d.id_vl.screenfaded {
            VW_FadeIn(w3d);
        }

        CheckKeys(w3d, ob);

        //
        // debug aids
        //
        if w3d.wl_play.singlestep == true {
            VW_WaitVBL(w3d, 1);
        } else {
            VW_WaitVBL(w3d, 0);
        }
        w3d.wl_draw.lasttimecount = GetTimeCount(w3d);

        if w3d.wl_play.extravbls != 0 {
            VW_WaitVBL(w3d, w3d.wl_play.extravbls as i32);
        }

        if w3d.wl_play.demoplayback {
            if IN_CheckAck(w3d) {
                IN_ClearKeysDown(w3d);
                w3d.wl_play.playstate = exit_t::ex_abort;
            }
        }
        if w3d.wl_play.playstate != exit_t::ex_stillplaying || w3d.wl_main.startgame {
            break;
        }
    }

    if w3d.wl_play.playstate != exit_t::ex_died {
        FinishPaletteShifts(w3d);
    }
}

/*
===================
=
= Think (way to call object think and action)
=
===================
*/

pub fn Think(w3d: &mut modules, ob: &mut object, think: bool) {
    //println!("Think");

    match ob.objlist[ob.objlist_i].state {
        s_player => {
            if think {
                T_Player(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_player;
            }
        }
        s_attack => {
            if think {
                T_Attack(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_attack;
            }
        }
        s_rocket => {
            if think {
                T_Projectile(w3d, ob);
                A_Smoke(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_rocket;
            }
        }
        s_smoke1 => {
            ob.objlist[ob.objlist_i].state = s_smoke2;
        }
        s_smoke2 => {
            ob.objlist[ob.objlist_i].state = s_smoke3;
        }
        s_smoke3 => {
            ob.objlist[ob.objlist_i].state = s_smoke4;
        }
        s_smoke4 => {
            ob.objlist[ob.objlist_i].state = statetype::new();
        }
        s_boom1 => {
            ob.objlist[ob.objlist_i].state = s_boom2;
        }
        s_boom2 => {
            ob.objlist[ob.objlist_i].state = s_boom3;
        }
        s_boom3 => {
            ob.objlist[ob.objlist_i].state = statetype::new();
        }
        s_grdstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdstand;
            }
        }
        s_grdpath1 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdpath1s;
            }
        }
        s_grdpath1s => {
            ob.objlist[ob.objlist_i].state = s_grdpath2;
        }
        s_grdpath2 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdpath3;
            }
        }
        s_grdpath3 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdpath3s;
            }
        }
        s_grdpath3s => {
            ob.objlist[ob.objlist_i].state = s_grdpath4;
        }
        s_grdpath4 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdpath1;
            }
        }
        s_grdpain => {
            ob.objlist[ob.objlist_i].state = s_grdchase1;
        }
        s_grdpain1 => {
            ob.objlist[ob.objlist_i].state = s_grdchase1;
        }
        s_grdshoot1 => {
            ob.objlist[ob.objlist_i].state = s_grdshoot2;
        }
        s_grdshoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdshoot3;
            }
        }
        s_grdshoot3 => {
            ob.objlist[ob.objlist_i].state = s_grdchase1;
        }
        s_grdchase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdchase1s;
            }
        }
        s_grdchase1s => {
            ob.objlist[ob.objlist_i].state = s_grdchase2;
        }
        s_grdchase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdchase3;
            }
        }
        s_grdchase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdchase3s;
            }
        }
        s_grdchase3s => {
            ob.objlist[ob.objlist_i].state = s_grdchase4;
        }
        s_grdchase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grdchase1;
            }
        }
        s_grddie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_grddie2;
            }
        }
        s_grddie2 => {
            ob.objlist[ob.objlist_i].state = s_grddie3;
        }
        s_grddie3 => {
            ob.objlist[ob.objlist_i].state = s_grddie4;
        }
        s_grddie4 => {
            ob.objlist[ob.objlist_i].state = s_grddie4;
        }
        s_blinkychase1 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_blinkychase2;
            }
        }
        s_blinkychase2 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_blinkychase1;
            }
        }
        s_inkychase1 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_inkychase2;
            }
        }
        s_inkychase2 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_inkychase1;
            }
        }
        s_pinkychase1 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_pinkychase2;
            }
        }
        s_pinkychase2 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_pinkychase1;
            }
        }
        s_clydechase1 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_clydechase2;
            }
        }
        s_clydechase2 => {
            if think {
                T_Ghosts(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_clydechase1;
            }
        }
        s_dogpath1 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogpath1s;
            }
        }
        s_dogpath1s => {
            ob.objlist[ob.objlist_i].state = s_dogpath2;
        }
        s_dogpath2 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogpath3;
            }
        }
        s_dogpath3 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogpath3s;
            }
        }
        s_dogpath3s => {
            ob.objlist[ob.objlist_i].state = s_dogpath4;
        }
        s_dogpath4 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogpath1;
            }
        }
        s_dogjump1 => {
            ob.objlist[ob.objlist_i].state = s_dogjump2;
        }
        s_dogjump2 => {
            if think {
                T_Bite(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogjump3;
            }
        }
        s_dogjump3 => {
            ob.objlist[ob.objlist_i].state = s_dogjump4;
        }
        s_dogjump4 => {
            ob.objlist[ob.objlist_i].state = s_dogjump5;
        }
        s_dogjump5 => {
            ob.objlist[ob.objlist_i].state = s_dogchase1;
        }
        s_dogchase1 => {
            if think {
                T_DogChase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogchase1s;
            }
        }
        s_dogchase1s => {
            ob.objlist[ob.objlist_i].state = s_dogchase2;
        }
        s_dogchase2 => {
            if think {
                T_DogChase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogchase3;
            }
        }
        s_dogchase3 => {
            if think {
                T_DogChase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogchase3s;
            }
        }
        s_dogchase3s => {
            ob.objlist[ob.objlist_i].state = s_dogchase4;
        }
        s_dogchase4 => {
            if think {
                T_DogChase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogchase1;
            }
        }
        s_dogdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_dogdie2;
            }
        }
        s_dogdie2 => {
            ob.objlist[ob.objlist_i].state = s_dogdie3;
        }
        s_dogdie3 => {
            ob.objlist[ob.objlist_i].state = s_dogdead;
        }
        s_dogdead => {
            ob.objlist[ob.objlist_i].state = s_dogdead;
        }
        s_ofcstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcstand;
            }
        }
        s_ofcpath1 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcpath1s;
            }
        }
        s_ofcpath1s => {
            ob.objlist[ob.objlist_i].state = s_ofcpath2;
        }
        s_ofcpath2 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcpath3;
            }
        }
        s_ofcpath3 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcpath3s;
            }
        }
        s_ofcpath3s => {
            ob.objlist[ob.objlist_i].state = s_ofcpath4;
        }
        s_ofcpath4 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcpath1;
            }
        }
        s_ofcpain => {
            ob.objlist[ob.objlist_i].state = s_ofcchase1;
        }
        s_ofcpain1 => {
            ob.objlist[ob.objlist_i].state = s_ofcchase1;
        }
        s_ofcshoot1 => {
            ob.objlist[ob.objlist_i].state = s_ofcshoot2;
        }
        s_ofcshoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcshoot3;
            }
        }
        s_ofcshoot3 => {
            ob.objlist[ob.objlist_i].state = s_ofcchase1;
        }
        s_ofcchase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcchase1s;
            }
        }
        s_ofcchase1s => {
            ob.objlist[ob.objlist_i].state = s_ofcchase2;
        }
        s_ofcchase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcchase3;
            }
        }
        s_ofcchase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcchase3s;
            }
        }
        s_ofcchase3s => {
            ob.objlist[ob.objlist_i].state = s_ofcchase4;
        }
        s_ofcchase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcchase1;
            }
        }
        s_ofcdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ofcdie2;
            }
        }
        s_ofcdie2 => {
            ob.objlist[ob.objlist_i].state = s_ofcdie3;
        }
        s_ofcdie3 => {
            ob.objlist[ob.objlist_i].state = s_ofcdie4;
        }
        s_ofcdie4 => {
            ob.objlist[ob.objlist_i].state = s_ofcdie5;
        }
        s_ofcdie5 => {
            ob.objlist[ob.objlist_i].state = s_ofcdie5;
        }
        s_mutstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutstand;
            }
        }
        s_mutpath1 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutpath1s;
            }
        }
        s_mutpath1s => {
            ob.objlist[ob.objlist_i].state = s_mutpath2;
        }
        s_mutpath2 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutpath3;
            }
        }
        s_mutpath3 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutpath3s;
            }
        }
        s_mutpath3s => {
            ob.objlist[ob.objlist_i].state = s_mutpath4;
        }
        s_mutpath4 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutpath1;
            }
        }
        s_mutpain => {
            ob.objlist[ob.objlist_i].state = s_mutchase1;
        }
        s_mutpain1 => {
            ob.objlist[ob.objlist_i].state = s_mutchase1;
        }
        s_mutshoot1 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutshoot2;
            }
        }
        s_mutshoot2 => {
            ob.objlist[ob.objlist_i].state = s_mutshoot3;
        }
        s_mutshoot3 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutshoot4;
            }
        }
        s_mutshoot4 => {
            ob.objlist[ob.objlist_i].state = s_mutchase1;
        }
        s_mutchase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutchase1s;
            }
        }
        s_mutchase1s => {
            ob.objlist[ob.objlist_i].state = s_mutchase2;
        }
        s_mutchase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutchase3;
            }
        }
        s_mutchase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutchase3s;
            }
        }
        s_mutchase3s => {
            ob.objlist[ob.objlist_i].state = s_mutchase4;
        }
        s_mutchase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutchase1;
            }
        }
        s_mutdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mutdie2;
            }
        }
        s_mutdie2 => {
            ob.objlist[ob.objlist_i].state = s_mutdie3;
        }
        s_mutdie3 => {
            ob.objlist[ob.objlist_i].state = s_mutdie4;
        }
        s_mutdie4 => {
            ob.objlist[ob.objlist_i].state = s_mutdie5;
        }
        s_mutdie5 => {
            ob.objlist[ob.objlist_i].state = s_mutdie5;
        }
        s_ssstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ssstand;
            }
        }
        s_sspath1 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sspath1s;
            }
        }
        s_sspath1s => {
            ob.objlist[ob.objlist_i].state = s_sspath2;
        }
        s_sspath2 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sspath3;
            }
        }
        s_sspath3 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sspath3s;
            }
        }
        s_sspath3s => {
            ob.objlist[ob.objlist_i].state = s_sspath4;
        }
        s_sspath4 => {
            if think {
                T_Path(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sspath1;
            }
        }
        s_sspain => {
            ob.objlist[ob.objlist_i].state = s_sschase1;
        }
        s_sspain1 => {
            ob.objlist[ob.objlist_i].state = s_sschase1;
        }
        s_ssshoot1 => {
            ob.objlist[ob.objlist_i].state = s_ssshoot2;
        }
        s_ssshoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ssshoot3;
            }
        }
        s_ssshoot3 => {
            ob.objlist[ob.objlist_i].state = s_ssshoot4;
        }
        s_ssshoot4 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ssshoot5;
            }
        }
        s_ssshoot5 => {
            ob.objlist[ob.objlist_i].state = s_ssshoot6;
        }
        s_ssshoot6 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ssshoot7;
            }
        }
        s_ssshoot7 => {
            ob.objlist[ob.objlist_i].state = s_ssshoot8;
        }
        s_ssshoot8 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ssshoot9;
            }
        }
        s_ssshoot9 => {
            ob.objlist[ob.objlist_i].state = s_sschase1;
        }
        s_sschase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sschase1s;
            }
        }
        s_sschase1s => {
            ob.objlist[ob.objlist_i].state = s_sschase2;
        }
        s_sschase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sschase3;
            }
        }
        s_sschase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sschase3s;
            }
        }
        s_sschase3s => {
            ob.objlist[ob.objlist_i].state = s_sschase4;
        }
        s_sschase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_sschase1;
            }
        }
        s_ssdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_ssdie2;
            }
        }
        s_ssdie2 => {
            ob.objlist[ob.objlist_i].state = s_ssdie3;
        }
        s_ssdie3 => {
            ob.objlist[ob.objlist_i].state = s_ssdie4;
        }
        s_ssdie4 => {
            ob.objlist[ob.objlist_i].state = s_ssdie4;
        }
        s_bossstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossstand;
            }
        }
        s_bosschase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bosschase1s;
            }
        }
        s_bosschase1s => {
            ob.objlist[ob.objlist_i].state = s_bosschase2;
        }
        s_bosschase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bosschase3;
            }
        }
        s_bosschase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bosschase3s;
            }
        }
        s_bosschase3s => {
            ob.objlist[ob.objlist_i].state = s_bosschase4;
        }
        s_bosschase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bosschase1;
            }
        }
        s_bossdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossdie2;
            }
        }
        s_bossdie2 => {
            ob.objlist[ob.objlist_i].state = s_bossdie3;
        }
        s_bossdie3 => {
            ob.objlist[ob.objlist_i].state = s_bossdie4;
        }
        s_bossdie4 => {
            ob.objlist[ob.objlist_i].state = s_bossdie4;
        }
        s_bossshoot1 => {
            ob.objlist[ob.objlist_i].state = s_bossshoot2;
        }
        s_bossshoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossshoot3;
            }
        }
        s_bossshoot3 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossshoot4;
            }
        }
        s_bossshoot4 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossshoot5;
            }
        }
        s_bossshoot5 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossshoot6;
            }
        }
        s_bossshoot6 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossshoot7;
            }
        }
        s_bossshoot7 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bossshoot8;
            }
        }
        s_bossshoot8 => {
            ob.objlist[ob.objlist_i].state = s_bosschase1;
        }
        s_gretelstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelstand;
            }
        }
        s_gretelchase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelchase1s;
            }
        }
        s_gretelchase1s => {
            ob.objlist[ob.objlist_i].state = s_gretelchase2;
        }
        s_gretelchase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelchase3;
            }
        }
        s_gretelchase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelchase3s;
            }
        }
        s_gretelchase3s => {
            ob.objlist[ob.objlist_i].state = s_gretelchase4;
        }
        s_gretelchase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelchase1;
            }
        }
        s_greteldie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_greteldie2;
            }
        }
        s_greteldie2 => {
            ob.objlist[ob.objlist_i].state = s_greteldie3;
        }
        s_greteldie3 => {
            ob.objlist[ob.objlist_i].state = s_greteldie4;
        }
        s_greteldie4 => {
            ob.objlist[ob.objlist_i].state = s_greteldie4;
        }
        s_gretelshoot1 => {
            ob.objlist[ob.objlist_i].state = s_gretelshoot2;
        }
        s_gretelshoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelshoot3;
            }
        }
        s_gretelshoot3 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelshoot4;
            }
        }
        s_gretelshoot4 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelshoot5;
            }
        }
        s_gretelshoot5 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelshoot6;
            }
        }
        s_gretelshoot6 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelshoot7;
            }
        }
        s_gretelshoot7 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_gretelshoot8;
            }
        }
        s_gretelshoot8 => {
            ob.objlist[ob.objlist_i].state = s_gretelchase1;
        }
        s_schabbstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbstand;
            }
        }
        s_schabbchase1 => {
            if think {
                T_Schabb(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbchase1s;
            }
        }
        s_schabbchase1s => {
            ob.objlist[ob.objlist_i].state = s_schabbchase2;
        }
        s_schabbchase2 => {
            if think {
                T_Schabb(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbchase3;
            }
        }
        s_schabbchase3 => {
            if think {
                T_Schabb(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbchase3s;
            }
        }
        s_schabbchase3s => {
            ob.objlist[ob.objlist_i].state = s_schabbchase4;
        }
        s_schabbchase4 => {
            if think {
                T_Schabb(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbchase1;
            }
        }
        s_schabbdeathcam => {
            ob.objlist[ob.objlist_i].state = s_schabbdie1;
        }
        s_schabbdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbdie2;
            }
        }
        s_schabbdie2 => {
            ob.objlist[ob.objlist_i].state = s_schabbdie3;
        }
        s_schabbdie3 => {
            ob.objlist[ob.objlist_i].state = s_schabbdie4;
        }
        s_schabbdie4 => {
            ob.objlist[ob.objlist_i].state = s_schabbdie5;
        }
        s_schabbdie5 => {
            ob.objlist[ob.objlist_i].state = s_schabbdie6;
        }
        s_schabbdie6 => {
            if think {
                A_StartDeathCam(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbdie6;
            }
        }
        s_schabbshoot1 => {
            ob.objlist[ob.objlist_i].state = s_schabbshoot2;
        }
        s_schabbshoot2 => {
            if think {
                T_SchabbThrow(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_schabbchase1;
            }
        }
        s_needle1 => {
            if think {
                T_Projectile(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_needle2;
            }
        }
        s_needle2 => {
            if think {
                T_Projectile(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_needle3;
            }
        }
        s_needle3 => {
            if think {
                T_Projectile(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_needle4;
            }
        }
        s_needle4 => {
            if think {
                T_Projectile(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_needle1;
            }
        }
        s_giftstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftstand;
            }
        }
        s_giftchase1 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftchase1s;
            }
        }
        s_giftchase1s => {
            ob.objlist[ob.objlist_i].state = s_giftchase2;
        }
        s_giftchase2 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftchase3;
            }
        }
        s_giftchase3 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftchase3s;
            }
        }
        s_giftchase3s => {
            ob.objlist[ob.objlist_i].state = s_giftchase4;
        }
        s_giftchase4 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftchase1;
            }
        }
        s_giftdeathcam => {
            ob.objlist[ob.objlist_i].state = s_giftdie1;
        }
        s_giftdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftdie2;
            }
        }
        s_giftdie2 => {
            ob.objlist[ob.objlist_i].state = s_giftdie3;
        }
        s_giftdie3 => {
            ob.objlist[ob.objlist_i].state = s_giftdie4;
        }
        s_giftdie4 => {
            ob.objlist[ob.objlist_i].state = s_giftdie5;
        }
        s_giftdie5 => {
            ob.objlist[ob.objlist_i].state = s_giftdie6;
        }
        s_giftdie6 => {
            if think {
                A_StartDeathCam(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftdie6;
            }
        }
        s_giftshoot1 => {
            ob.objlist[ob.objlist_i].state = s_giftshoot2;
        }
        s_giftshoot2 => {
            if think {
                T_GiftThrow(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_giftchase1;
            }
        }
        s_fatstand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatstand;
            }
        }
        s_fatchase1 => {
            if think {
                T_Fat(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatchase1s;
            }
        }
        s_fatchase1s => {
            ob.objlist[ob.objlist_i].state = s_fatchase2;
        }
        s_fatchase2 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatchase3;
            }
        }
        s_fatchase3 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatchase3s;
            }
        }
        s_fatchase3s => {
            ob.objlist[ob.objlist_i].state = s_fatchase4;
        }
        s_fatchase4 => {
            if think {
                T_Gift(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatchase1;
            }
        }
        s_fatdeathcam => {
            ob.objlist[ob.objlist_i].state = s_fatdie1;
        }
        s_fatdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatdie2;
            }
        }
        s_fatdie2 => {
            ob.objlist[ob.objlist_i].state = s_fatdie3;
        }
        s_fatdie3 => {
            ob.objlist[ob.objlist_i].state = s_fatdie4;
        }
        s_fatdie4 => {
            ob.objlist[ob.objlist_i].state = s_fatdie5;
        }
        s_fatdie5 => {
            ob.objlist[ob.objlist_i].state = s_fatdie6;
        }
        s_fatdie6 => {
            if think {
                A_StartDeathCam(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatdie6;
            }
        }
        s_fatshoot1 => {
            ob.objlist[ob.objlist_i].state = s_fatshoot2;
        }
        s_fatshoot2 => {
            if think {
                T_GiftThrow(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatshoot3;
            }
        }
        s_fatshoot3 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatshoot4;
            }
        }
        s_fatshoot4 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatshoot5;
            }
        }
        s_fatshoot5 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatshoot6;
            }
        }
        s_fatshoot6 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fatchase1;
            }
        }
        s_fakestand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakestand;
            }
        }
        s_fakechase1 => {
            if think {
                T_Fake(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakechase1s;
            }
        }
        s_fakechase1s => {
            ob.objlist[ob.objlist_i].state = s_fakechase2;
        }
        s_fakechase2 => {
            if think {
                T_Fake(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakechase3;
            }
        }
        s_fakechase3 => {
            if think {
                T_Fake(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakechase3s;
            }
        }
        s_fakechase3s => {
            ob.objlist[ob.objlist_i].state = s_fakechase4;
        }
        s_fakechase4 => {
            if think {
                T_Fake(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakechase1;
            }
        }
        s_fakedie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakedie2;
            }
        }
        s_fakedie2 => {
            ob.objlist[ob.objlist_i].state = s_fakedie3;
        }
        s_fakedie3 => {
            ob.objlist[ob.objlist_i].state = s_fakedie4;
        }
        s_fakedie4 => {
            ob.objlist[ob.objlist_i].state = s_fakedie5;
        }
        s_fakedie5 => {
            ob.objlist[ob.objlist_i].state = s_fakedie6;
        }
        s_fakedie6 => {
            ob.objlist[ob.objlist_i].state = s_fakedie6;
        }
        s_fakeshoot1 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot2;
            }
        }
        s_fakeshoot2 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot3;
            }
        }
        s_fakeshoot3 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot4;
            }
        }
        s_fakeshoot4 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot5;
            }
        }
        s_fakeshoot5 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot6;
            }
        }
        s_fakeshoot6 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot7;
            }
        }
        s_fakeshoot7 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot8;
            }
        }
        s_fakeshoot8 => {
            if think {
                T_FakeFire(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fakeshoot9;
            }
        }
        s_fakeshoot9 => {
            ob.objlist[ob.objlist_i].state = s_fakechase1;
        }
        s_fire1 => {
            if think {
                T_Projectile(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fire2;
            }
        }
        s_fire2 => {
            if think {
                T_Projectile(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_fire1;
            }
        }
        s_mechastand => {
            if think {
                T_Stand(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechastand;
            }
        }
        s_mechachase1 => {
            if think {
                T_Chase(w3d, ob);
                A_MechaSound(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechachase1s;
            }
        }
        s_mechachase1s => {
            ob.objlist[ob.objlist_i].state = s_mechachase2;
        }
        s_mechachase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechachase3;
            }
        }
        s_mechachase3 => {
            if think {
                T_Chase(w3d, ob);
                A_MechaSound(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechachase3s;
            }
        }
        s_mechachase3s => {
            ob.objlist[ob.objlist_i].state = s_mechachase4;
        }
        s_mechachase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechachase1;
            }
        }
        s_mechadie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechadie2;
            }
        }
        s_mechadie2 => {
            ob.objlist[ob.objlist_i].state = s_mechadie3;
        }
        s_mechadie3 => {
            if think {
                A_HitlerMorph(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechadie4;
            }
        }
        s_mechadie4 => {
            ob.objlist[ob.objlist_i].state = s_mechadie4;
        }
        s_mechashoot1 => {
            ob.objlist[ob.objlist_i].state = s_mechashoot2;
        }
        s_mechashoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechashoot3;
            }
        }
        s_mechashoot3 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechashoot4;
            }
        }
        s_mechashoot4 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechashoot5;
            }
        }
        s_mechashoot5 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechashoot6;
            }
        }
        s_mechashoot6 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_mechachase1;
            }
        }
        s_hitlerchase1 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerchase1s;
            }
        }
        s_hitlerchase1s => {
            ob.objlist[ob.objlist_i].state = s_hitlerchase2;
        }
        s_hitlerchase2 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerchase3;
            }
        }
        s_hitlerchase3 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerchase3s;
            }
        }
        s_hitlerchase3s => {
            ob.objlist[ob.objlist_i].state = s_hitlerchase4;
        }
        s_hitlerchase4 => {
            if think {
                T_Chase(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerchase1;
            }
        }
        s_hitlerdeathcam => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie1;
        }
        s_hitlerdie1 => {
            if think {
                A_DeathScream(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerdie2;
            }
        }
        s_hitlerdie2 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie3;
        }
        s_hitlerdie3 => {
            if think {
                A_Slurpie(w3d);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerdie4;
            }
        }
        s_hitlerdie4 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie5;
        }
        s_hitlerdie5 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie6;
        }
        s_hitlerdie6 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie7;
        }
        s_hitlerdie7 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie8;
        }
        s_hitlerdie8 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie9;
        }
        s_hitlerdie9 => {
            ob.objlist[ob.objlist_i].state = s_hitlerdie10;
        }
        s_hitlerdie10 => {
            if think {
                A_StartDeathCam(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerdie10;
            }
        }
        s_hitlershoot1 => {
            ob.objlist[ob.objlist_i].state = s_hitlershoot2;
        }
        s_hitlershoot2 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlershoot3;
            }
        }
        s_hitlershoot3 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlershoot4;
            }
        }
        s_hitlershoot4 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlershoot5;
            }
        }
        s_hitlershoot5 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlershoot6;
            }
        }
        s_hitlershoot6 => {
            if think {
                T_Shoot(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_hitlerchase1;
            }
        }
        s_bjrun1 => {
            if think {
                T_BJRun(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjrun1s;
            }
        }
        s_bjrun1s => {
            ob.objlist[ob.objlist_i].state = s_bjrun2;
        }
        s_bjrun2 => {
            if think {
                T_BJRun(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjrun3;
            }
        }
        s_bjrun3 => {
            if think {
                T_BJRun(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjrun3s;
            }
        }
        s_bjrun3s => {
            ob.objlist[ob.objlist_i].state = s_bjrun4;
        }
        s_bjrun4 => {
            if think {
                T_BJRun(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjrun1;
            }
        }
        s_bjjump1 => {
            if think {
                T_BJJump(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjjump2;
            }
        }
        s_bjjump2 => {
            if think {
                T_BJJump(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjjump3;
            }
        }
        s_bjjump3 => {
            if think {
                T_BJJump(w3d, ob);
                T_BJYell(w3d, ob);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjjump4;
            }
        }
        s_bjjump4 => {
            if think {
                T_BJDone(w3d);
            } else {
                ob.objlist[ob.objlist_i].state = s_bjjump4;
            }
        }
        s_deathcam => {
            ob.objlist[ob.objlist_i].state = statetype::new();
        }
        _ => (),
    }
}
