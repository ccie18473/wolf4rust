#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_inter
//
//===========================================================================

pub struct wl_inter {
    pub LevelRatios: [LRstruct; LRpack],
    pub lastBreathTime: i32,
    pub which: i32,
    pub max: i32,
}

impl wl_inter {
    pub fn new() -> Self {
        Self {
            LevelRatios: [LRstruct {
                kill: 0,
                secret: 0,
                treasure: 0,
                time: 0,
            }; LRpack],
            lastBreathTime: 0,
            which: 0,
            max: 10,
        }
    }
    pub fn clear(&mut self) {
        self.LevelRatios = [LRstruct {
            kill: 0,
            secret: 0,
            treasure: 0,
            time: 0,
        }; LRpack];
        self.lastBreathTime = 0;
    }
}
//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

// None

/*
==================
=
= CLearSplitVWB
=
==================
*/

pub fn ClearSplitVWB(w3d: &mut modules) {
    //println!("ClearSplitVWB");

    w3d.id_us.WindowX = 0;
    w3d.id_us.WindowY = 0;
    w3d.id_us.WindowW = 320;
    w3d.id_us.WindowH = 160;
}

/*
==================
=
= Victory
=
==================
*/

pub fn Victory(w3d: &mut modules) {
    //println!("Victory");

    let mut i: i32;
    let mut sec: i32 = 0;
    let mut min: i32;
    let mut kr: i32 = 0;
    let mut sr: i32 = 0;
    let mut tr: i32 = 0;
    let mut x: i32;
    let mut tempstr: String;

    let RATIOX: i32 = 6;
    let RATIOY: i32 = 14;
    let TIMEX: i32 = 14;
    let TIMEY: i32 = 8;

    StartCPMusic(w3d, musicnames::URAHERO_MUS as i32);
    ClearSplitVWB(w3d);

    VWB_Bar(
        w3d,
        0,
        0,
        320,
        w3d.id_vl.screenHeight / w3d.id_vl.scaleFactor - STATUSLINES + 1,
        VIEWCOLOR,
    );
    if w3d.wl_game.bordercol != VIEWCOLOR {
        DrawStatusBorder(w3d, VIEWCOLOR);
    }

    Write(w3d, 18, 2, STR_YOUWIN.to_string());

    Write(w3d, TIMEX, TIMEY - 2, STR_TOTALTIME.to_string());

    Write(w3d, 12, RATIOY - 2, "averages".to_string());

    Write(w3d, RATIOX + 8, RATIOY, STR_RATKILL.to_string());
    Write(w3d, RATIOX + 4, RATIOY + 2, STR_RATSECRET.to_string());
    Write(w3d, RATIOX, RATIOY + 4, STR_RATTREASURE.to_string());

    VWB_DrawPic(w3d, 8, 4, graphicnums::L_BJWINSPIC as i32);

    for i in 0..LRpack {
        sec += w3d.wl_inter.LevelRatios[i].time as i32;
        kr += w3d.wl_inter.LevelRatios[i].kill as i32;
        sr += w3d.wl_inter.LevelRatios[i].secret as i32;
        tr += w3d.wl_inter.LevelRatios[i].treasure as i32;
    }

    kr /= LRpack as i32;
    sr /= LRpack as i32;
    tr /= LRpack as i32;

    min = sec / 60;
    sec %= 60;

    if min > 99 {
        min = 99;
        sec = 99;
    }

    i = TIMEX * 8 + 1;

    VWB_DrawPic(
        w3d,
        i,
        TIMEY * 8,
        graphicnums::L_NUM0PIC as i32 + (min / 10),
    );
    i += 2 * 8;
    VWB_DrawPic(
        w3d,
        i,
        TIMEY * 8,
        graphicnums::L_NUM0PIC as i32 + (min % 10),
    );
    i += 2 * 8;
    Write(w3d, i / 8, TIMEY, ":".to_string());
    i += 1 * 8;
    VWB_DrawPic(
        w3d,
        i,
        TIMEY * 8,
        graphicnums::L_NUM0PIC as i32 + (sec / 10),
    );
    i += 2 * 8;
    VWB_DrawPic(
        w3d,
        i,
        TIMEY * 8,
        graphicnums::L_NUM0PIC as i32 + (sec % 10),
    );
    VW_UpdateScreen(w3d);

    //itoa (kr, tempstr, 10);
    tempstr = kr.to_string();
    tempstr.truncate(10);

    x = RATIOX + 24 - tempstr.len() as i32 * 2;
    Write(w3d, x, RATIOY, tempstr);

    //itoa (sr, tempstr, 10);
    tempstr = sr.to_string();
    tempstr.truncate(10);

    x = RATIOX + 24 - tempstr.len() as i32 * 2;
    Write(w3d, x, RATIOY + 2, tempstr);

    //itoa (tr, tempstr, 10);
    tempstr = tr.to_string();
    tempstr.truncate(10);

    x = RATIOX + 24 - tempstr.len() as i32 * 2;
    Write(w3d, x, RATIOY + 4, tempstr);

    #[cfg(feature = "GOODTIMES")]
    //
    // TOTAL TIME VERIFICATION CODE
    //
    //BUG
    if w3d.wl_game.gamestate.difficulty >= gd::gd_medium as i32 {
        VWB_DrawPic(w3d, 30 * 8, TIMEY * 8, graphicnums::C_TIMECODEPIC as i32);
        w3d.id_vh.fontnumber = 0;
        w3d.id_vh.fontcolor = READHCOLOR;
        w3d.id_us.PrintX = 30 * 8 - 3;
        w3d.id_us.PrintY = TIMEY * 8 + 8;
        w3d.id_us.PrintX += 4;

        tempstr = String::new();
        let char0 = char::from_u32((((min / 10) ^ (min % 10)) ^ 0xa) as u32 + 65).unwrap();
        tempstr.push(char0);
        let char1 = char::from_u32((((sec / 10) ^ (sec % 10)) ^ 0xa) as u32 + 65).unwrap();
        tempstr.push(char1);
        let char2 = char::from_u32(char0 as u32 ^ char1 as u32 + 65).unwrap();
        tempstr.push(char2);
        tempstr.push('0');
        US_Print(w3d, tempstr);
    }

    w3d.id_vh.fontnumber = 1;

    VW_UpdateScreen(w3d);
    VW_FadeIn(w3d);

    IN_Ack(w3d);

    VW_FadeOut(w3d);
    if w3d.id_vl.screenHeight % 200 != 0 {
        VL_ClearScreen(w3d, Color::BLACK);
    }

    unsafe { MainMenu[menuitems::savegame as usize].active = 0 }; // ADDEDFIX 3 - Tricob

    EndText(w3d);
}

/*
==================
=
= PG13
=
==================
*/

pub fn PG13(w3d: &mut modules) {
    //println!("PG13");

    VW_FadeOut(w3d);
    VWB_Bar(w3d, 0, 0, 320, 200, 0x82); // background

    VWB_DrawPic(w3d, 216, 110, graphicnums::PG13PIC as i32);
    VW_UpdateScreen(w3d);

    VW_FadeIn(w3d);
    IN_UserInput(w3d, TickBase * 7);

    VW_FadeOut(w3d);
}

//==========================================================================

pub fn Write(w3d: &mut modules, x: i32, y: i32, string: String) {
    //println!("Write");

    let alpha: [i32; 43] = [
        graphicnums::L_NUM0PIC as i32,
        graphicnums::L_NUM1PIC as i32,
        graphicnums::L_NUM2PIC as i32,
        graphicnums::L_NUM3PIC as i32,
        graphicnums::L_NUM4PIC as i32,
        graphicnums::L_NUM5PIC as i32,
        graphicnums::L_NUM6PIC as i32,
        graphicnums::L_NUM7PIC as i32,
        graphicnums::L_NUM8PIC as i32,
        graphicnums::L_NUM9PIC as i32,
        graphicnums::L_COLONPIC as i32,
        0,
        0,
        0,
        0,
        0,
        0,
        graphicnums::L_APIC as i32,
        graphicnums::L_BPIC as i32,
        graphicnums::L_CPIC as i32,
        graphicnums::L_DPIC as i32,
        graphicnums::L_EPIC as i32,
        graphicnums::L_FPIC as i32,
        graphicnums::L_GPIC as i32,
        graphicnums::L_HPIC as i32,
        graphicnums::L_IPIC as i32,
        graphicnums::L_JPIC as i32,
        graphicnums::L_KPIC as i32,
        graphicnums::L_LPIC as i32,
        graphicnums::L_MPIC as i32,
        graphicnums::L_NPIC as i32,
        graphicnums::L_OPIC as i32,
        graphicnums::L_PPIC as i32,
        graphicnums::L_QPIC as i32,
        graphicnums::L_RPIC as i32,
        graphicnums::L_SPIC as i32,
        graphicnums::L_TPIC as i32,
        graphicnums::L_UPIC as i32,
        graphicnums::L_VPIC as i32,
        graphicnums::L_WPIC as i32,
        graphicnums::L_XPIC as i32,
        graphicnums::L_YPIC as i32,
        graphicnums::L_ZPIC as i32,
    ];

    let ox: i32;
    let mut nx: i32;
    let mut ny: i32;
    let len: usize = string.len();
    let mut ch: char;

    ox = x * 8;
    nx = x * 8;
    ny = y * 8;

    for i in 0..len {
        if string.chars().nth(i).unwrap() == '\n' {
            nx = ox;
            ny += 16;
        } else {
            ch = string.chars().nth(i).unwrap();

            if ch >= 'a' {
                let mut ch_u32 = ch as u32;
                ch_u32 -= 32; //('a' - 'A')
                ch = char::from_u32(ch_u32).unwrap();
            }
            if ch >= '0' {
                let mut ch_u32 = ch as u32;
                ch_u32 -= 48; //('0')
                ch = char::from_u32(ch_u32).unwrap();
            }

            match string.chars().nth(i).unwrap() {
                '!' => {
                    VWB_DrawPic(w3d, nx, ny, graphicnums::L_EXPOINTPIC as i32);
                    nx += 8;
                    continue;
                }

                '\'' => {
                    VWB_DrawPic(w3d, nx, ny, graphicnums::L_APOSTROPHEPIC as i32);
                    nx += 8;
                    continue;
                }

                ' ' => {}

                ':' => {
                    VWB_DrawPic(w3d, nx, ny, graphicnums::L_COLONPIC as i32);
                    nx += 8;
                    continue;
                }

                '%' => {
                    VWB_DrawPic(w3d, nx, ny, graphicnums::L_PERCENTPIC as i32);
                }

                _ => {
                    VWB_DrawPic(w3d, nx, ny, alpha[ch as usize]);
                }
            }
            nx += 16;
        }
    }
}

//
// Breathe Mr. BJ!!!
//

pub fn BJ_Breathe(w3d: &mut modules) {
    //println!("BJ_Breathe");

    //static int which = 0, max = 10;

    let pics: [i32; 2] = [graphicnums::L_GUYPIC as i32, graphicnums::L_GUY2PIC as i32];

    SDL_Delay(w3d, 5);

    if (GetTimeCount(w3d) - w3d.wl_inter.lastBreathTime) > w3d.wl_inter.max {
        w3d.wl_inter.which ^= 1;
        VWB_DrawPic(w3d, 0, 16, pics[w3d.wl_inter.which as usize]);
        VW_UpdateScreen(w3d);
        w3d.wl_inter.lastBreathTime = GetTimeCount(w3d);
        w3d.wl_inter.max = 35;
    }
}

/*
==================
=
= LevelCompleted
=
= Entered with the screen faded out
= Still in split screen mode with the status bar
=
= Exit with the screen faded out
=
==================
*/

pub fn LevelCompleted(w3d: &mut modules) {
    //println!("LevelCompleted");

    let VBLWAIT: i32 = 30;
    let PAR_AMOUNT: i32 = 500;
    let PERCENT100AMT: i32 = 10000;

    struct times<'a> {
        time: f64,
        timestr: &'a str,
    }

    let mut i: i32;
    let mut x: i32;
    let min: i32;
    let mut sec: i32;
    let mut ratio: i32;
    let mut kr: i32;
    let mut sr: i32;
    let mut tr: i32;

    let mut tempstr: String;
    let mut bonus: i32;
    let mut timeleft: i32 = 0;
    let parTimes: [times; 60] = [
        //
        // Episode One Par Times
        //
        times {
            time: 1.5,
            timestr: "01:30",
        },
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 3.5,
            timestr: "03:30",
        },
        times {
            time: 3.0,
            timestr: "03:00",
        },
        times {
            time: 3.0,
            timestr: "03:00",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        }, // Boss level
        times {
            time: 0.0,
            timestr: "??:??",
        }, // Secret level
        //
        // Episode Two Par Times
        //
        times {
            time: 1.5,
            timestr: "01:30",
        },
        times {
            time: 3.5,
            timestr: "03:30",
        },
        times {
            time: 3.0,
            timestr: "03:00",
        },
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 4.0,
            timestr: "04:00",
        },
        times {
            time: 6.0,
            timestr: "06:00",
        },
        times {
            time: 1.0,
            timestr: "01:00",
        },
        times {
            time: 3.0,
            timestr: "03:00",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        //
        // Episode Three Par Times
        //
        times {
            time: 1.5,
            timestr: "01:30",
        },
        times {
            time: 1.5,
            timestr: "01:30",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 3.5,
            timestr: "03:30",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 6.0,
            timestr: "06:00",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        //
        // Episode Four Par Times
        //
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 1.5,
            timestr: "01:30",
        },
        times {
            time: 1.0,
            timestr: "01:00",
        },
        times {
            time: 4.5,
            timestr: "04:30",
        },
        times {
            time: 3.5,
            timestr: "03:30",
        },
        times {
            time: 2.0,
            timestr: "02:00",
        },
        times {
            time: 4.5,
            timestr: "04:30",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        //
        // Episode Five Par Times
        //
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 1.5,
            timestr: "01:30",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 2.5,
            timestr: "02:30",
        },
        times {
            time: 4.0,
            timestr: "04:00",
        },
        times {
            time: 3.0,
            timestr: "03:00",
        },
        times {
            time: 4.5,
            timestr: "04:30",
        },
        times {
            time: 3.5,
            timestr: "03:30",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        //
        // Episode Six Par Times
        //
        times {
            time: 6.5,
            timestr: "06:30",
        },
        times {
            time: 4.0,
            timestr: "04:00",
        },
        times {
            time: 4.5,
            timestr: "04:30",
        },
        times {
            time: 6.0,
            timestr: "06:00",
        },
        times {
            time: 5.0,
            timestr: "05:00",
        },
        times {
            time: 5.5,
            timestr: "05:30",
        },
        times {
            time: 5.5,
            timestr: "05:30",
        },
        times {
            time: 8.5,
            timestr: "08:30",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
        times {
            time: 0.0,
            timestr: "??:??",
        },
    ];

    ClearSplitVWB(w3d); // set up for double buffering in split screen
    VWB_Bar(
        w3d,
        0,
        0,
        320,
        w3d.id_vl.screenHeight / w3d.id_vl.scaleFactor - STATUSLINES + 1,
        VIEWCOLOR,
    );

    if w3d.wl_game.bordercol != VIEWCOLOR {
        DrawStatusBorder(w3d, VIEWCOLOR);
    }

    StartCPMusic(w3d, musicnames::ENDLEVEL_MUS as i32);

    //
    // do the intermission
    //
    IN_ClearKeysDown(w3d);
    IN_StartAck(w3d);

    VWB_DrawPic(w3d, 0, 16, graphicnums::L_GUYPIC as i32);

    if w3d.wl_game.gamestate.mapon < 8 {
        Write(w3d, 14, 2, "floor\ncompleted".to_string());

        let mut s = STR_BONUS.to_string();
        s.push_str("     0");
        Write(w3d, 14, 7, s);
        Write(w3d, 16, 10, STR_TIME.to_string());
        Write(w3d, 16, 12, STR_PAR.to_string());

        Write(w3d, 9, 14, STR_RAT2KILL.to_string());
        Write(w3d, 5, 16, STR_RAT2SECRET.to_string());
        Write(w3d, 1, 18, STR_RAT2TREASURE.to_string());

        tempstr = (w3d.wl_game.gamestate.mapon + 1).to_string();
        tempstr.truncate(10);

        Write(w3d, 26, 2, tempstr);

        Write(
            w3d,
            26,
            12,
            (parTimes[(w3d.wl_game.gamestate.episode * 10 + w3d.wl_game.gamestate.mapon) as usize])
                .timestr
                .to_string(),
        );

        //
        // PRINT TIME
        //
        sec = w3d.wl_game.gamestate.TimeCount / 70;

        if sec > 99 * 60 {
            // 99 minutes max
            sec = 99 * 60;
        }

        if (w3d.wl_game.gamestate.TimeCount as f64)
            < (parTimes
                [(w3d.wl_game.gamestate.episode * 10 + w3d.wl_game.gamestate.mapon) as usize]
                .time
                * 4200.0)
        {
            timeleft = ((parTimes
                [(w3d.wl_game.gamestate.episode * 10 + w3d.wl_game.gamestate.mapon) as usize]
                .time
                * 4200.0)
                / 70.0) as i32
                - sec;
        }

        min = sec / 60;
        sec %= 60;

        i = 26 * 8;

        VWB_DrawPic(w3d, i, 10 * 8, graphicnums::L_NUM0PIC as i32 + (min / 10));
        i += 2 * 8;
        VWB_DrawPic(w3d, i, 10 * 8, graphicnums::L_NUM0PIC as i32 + (min % 10));
        i += 2 * 8;
        Write(w3d, i / 8, 10, ":".to_string());
        i += 1 * 8;
        VWB_DrawPic(w3d, i, 10 * 8, graphicnums::L_NUM0PIC as i32 + (sec / 10));
        i += 2 * 8;
        VWB_DrawPic(w3d, i, 10 * 8, graphicnums::L_NUM0PIC as i32 + (sec % 10));

        VW_UpdateScreen(w3d);
        VW_FadeIn(w3d);

        //
        // FIGURE RATIOS OUT BEFOREHAND
        //
        kr = 0;
        sr = 0;
        tr = 0;
        if w3d.wl_game.gamestate.killtotal != 0 {
            kr = (w3d.wl_game.gamestate.killcount * 100) / w3d.wl_game.gamestate.killtotal;
        }
        if w3d.wl_game.gamestate.secrettotal != 0 {
            sr = (w3d.wl_game.gamestate.secretcount * 100) / w3d.wl_game.gamestate.secrettotal;
        }
        if w3d.wl_game.gamestate.treasuretotal != 0 {
            tr = (w3d.wl_game.gamestate.treasurecount * 100) / w3d.wl_game.gamestate.treasuretotal;
        }

        //
        // PRINT TIME BONUS
        //
        bonus = timeleft * PAR_AMOUNT;

        if bonus != 0 {
            for i in 0..=timeleft {
                tempstr = (i * PAR_AMOUNT).to_string();
                tempstr.truncate(10);
                x = (36 - tempstr.len() * 2) as i32;
                Write(w3d, x, 7, tempstr);
                if (i % (PAR_AMOUNT / 10)) == 0 {
                    SD_PlaySound(w3d, soundnames::ENDBONUS1SND);
                }
                if !w3d.id_vl.usedoublebuffering || (i % (PAR_AMOUNT / 50)) == 0 {
                    VW_UpdateScreen(w3d);
                }
                while SD_SoundPlaying(w3d) != 0 {
                    BJ_Breathe(w3d);
                }
                if IN_CheckAck(w3d) {
                    //goto done;
                }
            }

            VW_UpdateScreen(w3d);

            SD_PlaySound(w3d, soundnames::ENDBONUS2SND);
            while SD_SoundPlaying(w3d) != 0 {
                BJ_Breathe(w3d);
            }
        }

        let RATIOXX: i32 = 37;

        //
        // KILL RATIO
        //
        ratio = kr;
        for i in 0..=ratio {
            tempstr = i.to_string();
            tempstr.truncate(10);
            x = RATIOXX - tempstr.len() as i32 * 2;
            Write(w3d, x, 14, tempstr);
            if (i % 10) == 0 {
                SD_PlaySound(w3d, soundnames::ENDBONUS1SND);
            }
            if !w3d.id_vl.usedoublebuffering || (i & 1) == 0 {
                VW_UpdateScreen(w3d);
            }
            while SD_SoundPlaying(w3d) != 0 {
                BJ_Breathe(w3d);
            }

            if IN_CheckAck(w3d) {
                //goto done;
            }
        }
        if ratio >= 100 {
            VW_WaitVBL(w3d, VBLWAIT);
            SD_StopSound(w3d);
            bonus += PERCENT100AMT;
            tempstr = bonus.to_string();
            tempstr.truncate(10);
            x = RATIOXX - 1 - tempstr.len() as i32 * 2;
            Write(w3d, x, 7, tempstr);
            VW_UpdateScreen(w3d);
            SD_PlaySound(w3d, soundnames::PERCENT100SND);
        } else if ratio == 0 {
            VW_WaitVBL(w3d, VBLWAIT);
            SD_StopSound(w3d);
            SD_PlaySound(w3d, soundnames::NOBONUSSND);
        } else {
            SD_PlaySound(w3d, soundnames::ENDBONUS2SND);
        }

        VW_UpdateScreen(w3d);
        while SD_SoundPlaying(w3d) != 0 {
            BJ_Breathe(w3d);
        }

        //
        // SECRET RATIO
        //
        ratio = sr;
        for i in 0..=ratio {
            tempstr = i.to_string();
            tempstr.truncate(10);
            x = RATIOXX - tempstr.len() as i32 * 2;
            Write(w3d, x, 16, tempstr);
            if (i % 10) == 0 {
                SD_PlaySound(w3d, soundnames::ENDBONUS1SND);
            }
            if !w3d.id_vl.usedoublebuffering || !(i & 1) != 0 {
                VW_UpdateScreen(w3d);
            }
            while SD_SoundPlaying(w3d) != 0 {
                BJ_Breathe(w3d);
            }

            if IN_CheckAck(w3d) {
                //goto done;
            }
        }
        if ratio >= 100 {
            VW_WaitVBL(w3d, VBLWAIT);
            SD_StopSound(w3d);
            bonus += PERCENT100AMT;
            tempstr = bonus.to_string();
            tempstr.truncate(10);
            x = RATIOXX - 1 - tempstr.len() as i32 * 2;
            Write(w3d, x, 7, tempstr);
            VW_UpdateScreen(w3d);
            SD_PlaySound(w3d, soundnames::PERCENT100SND);
        } else if ratio == 0 {
            VW_WaitVBL(w3d, VBLWAIT);
            SD_StopSound(w3d);
            SD_PlaySound(w3d, soundnames::NOBONUSSND);
        } else {
            SD_PlaySound(w3d, soundnames::ENDBONUS2SND);
        }
        VW_UpdateScreen(w3d);
        while SD_SoundPlaying(w3d) != 0 {
            BJ_Breathe(w3d);
        }

        //
        // TREASURE RATIO
        //
        ratio = tr;
        for i in 0..=ratio {
            tempstr = i.to_string();
            tempstr.truncate(10);
            x = RATIOXX - tempstr.len() as i32 * 2;
            Write(w3d, x, 18, tempstr);
            if (i % 10) != 0 {
                SD_PlaySound(w3d, soundnames::ENDBONUS1SND);
            }
            if !w3d.id_vl.usedoublebuffering || (i & 1) != 0 {
                VW_UpdateScreen(w3d);
            }
            while SD_SoundPlaying(w3d) != 0 {
                BJ_Breathe(w3d);
            }
            if IN_CheckAck(w3d) {
                //goto done;
            }
        }
        if ratio >= 100 {
            VW_WaitVBL(w3d, VBLWAIT);
            SD_StopSound(w3d);
            bonus += PERCENT100AMT;
            tempstr = bonus.to_string();
            tempstr.truncate(10);
            x = RATIOXX - 1 - tempstr.len() as i32 * 2;
            Write(w3d, x, 7, tempstr);
            VW_UpdateScreen(w3d);
            SD_PlaySound(w3d, soundnames::PERCENT100SND);
        } else if !ratio == 0 {
            VW_WaitVBL(w3d, VBLWAIT);
            SD_StopSound(w3d);
            SD_PlaySound(w3d, soundnames::NOBONUSSND);
        } else {
            SD_PlaySound(w3d, soundnames::ENDBONUS2SND);
        }
        VW_UpdateScreen(w3d);
        while SD_SoundPlaying(w3d) != 0 {
            BJ_Breathe(w3d);
        }

        //
        // JUMP STRAIGHT HERE IF KEY PRESSED
        //

        //done:

        tempstr = kr.to_string();
        tempstr.truncate(10);
        x = RATIOXX - tempstr.len() as i32 * 2;
        Write(w3d, x, 14, tempstr);

        tempstr = sr.to_string();
        tempstr.truncate(10);
        x = RATIOXX - tempstr.len() as i32 * 2;
        Write(w3d, x, 16, tempstr);

        tempstr = tr.to_string();
        tempstr.truncate(10);
        x = RATIOXX - tempstr.len() as i32 * 2;
        Write(w3d, x, 18, tempstr);

        let kr_bool: i32;
        let sr_bool: i32;
        let tr_bool: i32;

        if kr >= 100 {
            kr_bool = 1;
        } else {
            kr_bool = 0;
        }
        if sr >= 100 {
            sr_bool = 1;
        } else {
            sr_bool = 0;
        }
        if tr >= 100 {
            tr_bool = 1;
        } else {
            tr_bool = 0;
        }

        bonus = timeleft * PAR_AMOUNT
            + (PERCENT100AMT * (kr_bool))
            + (PERCENT100AMT * (sr_bool))
            + (PERCENT100AMT * (tr_bool));

        GivePoints(w3d, bonus);
        tempstr = bonus.to_string();
        tempstr.truncate(10);
        x = (36 - tempstr.len() * 2) as i32;
        Write(w3d, x, 7, tempstr);

        //
        // SAVE RATIO INFORMATION FOR ENDGAME
        //
        w3d.wl_inter.LevelRatios[w3d.wl_game.gamestate.mapon as usize].kill = kr as usize;
        w3d.wl_inter.LevelRatios[w3d.wl_game.gamestate.mapon as usize].secret = sr as usize;
        w3d.wl_inter.LevelRatios[w3d.wl_game.gamestate.mapon as usize].treasure = tr as usize;
        w3d.wl_inter.LevelRatios[w3d.wl_game.gamestate.mapon as usize].time =
            (min * 60 + sec) as usize;
    } else {
        Write(w3d, 14, 4, "secret floor\n completed!".to_string());

        Write(w3d, 10, 16, "15000 bonus!".to_string());

        VW_UpdateScreen(w3d);
        VW_FadeIn(w3d);

        GivePoints(w3d, 15000);
    }

    DrawScore(w3d);
    VW_UpdateScreen(w3d);

    w3d.wl_inter.lastBreathTime = GetTimeCount(w3d);
    IN_StartAck(w3d);
    while !IN_CheckAck(w3d) {
        BJ_Breathe(w3d);
    }

    //
    // done
    //

    VW_FadeOut(w3d);
    DrawPlayBorder(w3d);
}

/*
=================
=
= PreloadGraphics
=
= Fill the cache up
=
=================
*/

pub fn PreloadUpdate(w3d: &mut modules, current: i32, total: i32) -> bool {
    //println!("PreloadUpdate");

    let mut w: i32 = w3d.id_us.WindowW - w3d.id_vl.scaleFactor * 10;

    VWB_BarScaledCoord(
        w3d,
        w3d.id_us.WindowX + w3d.id_vl.scaleFactor * 5,
        w3d.id_us.WindowY + w3d.id_us.WindowH - w3d.id_vl.scaleFactor * 3,
        w,
        w3d.id_vl.scaleFactor * 2,
        BLACK,
    );

    w = (w * current) / total;
    if w != 0 {
        VWB_BarScaledCoord(
            w3d,
            w3d.id_us.WindowX + w3d.id_vl.scaleFactor * 5,
            w3d.id_us.WindowY + w3d.id_us.WindowH - w3d.id_vl.scaleFactor * 3,
            w,
            w3d.id_vl.scaleFactor * 2,
            0x37,
        ); //SECONDCOLOR);
        VWB_BarScaledCoord(
            w3d,
            w3d.id_us.WindowX + w3d.id_vl.scaleFactor * 5,
            w3d.id_us.WindowY + w3d.id_us.WindowH - w3d.id_vl.scaleFactor * 3,
            w - w3d.id_vl.scaleFactor * 1,
            w3d.id_vl.scaleFactor * 1,
            0x32,
        );
    }
    VW_UpdateScreen(w3d);

    return false;
}

pub fn PreloadGraphics(w3d: &mut modules) {
    //println!("PreloadGraphics");

    DrawLevel(w3d);
    ClearSplitVWB(w3d); // set up for double buffering in split screen

    VWB_BarScaledCoord(
        w3d,
        0,
        0,
        w3d.id_vl.screenWidth,
        w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * (STATUSLINES - 1),
        w3d.wl_game.bordercol,
    );
    VWB_DrawPicScaledCoord(
        w3d,
        ((w3d.id_vl.screenWidth - w3d.id_vl.scaleFactor * 224) / 16) * 8,
        (w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * (STATUSLINES + 48)) / 2,
        graphicnums::GETPSYCHEDPIC as i32,
    );

    w3d.id_us.WindowX = (w3d.id_vl.screenWidth - w3d.id_vl.scaleFactor * 224) / 2;
    w3d.id_us.WindowY = (w3d.id_vl.screenHeight - w3d.id_vl.scaleFactor * (STATUSLINES + 48)) / 2;
    w3d.id_us.WindowW = w3d.id_vl.scaleFactor * 28 * 8;
    w3d.id_us.WindowH = w3d.id_vl.scaleFactor * 48;

    VW_UpdateScreen(w3d);
    VW_FadeIn(w3d);

    //      PM_Preload (PreloadUpdate);
    PreloadUpdate(w3d, 10, 10);
    IN_UserInput(w3d, 70);
    VW_FadeOut(w3d);

    DrawPlayBorder(w3d);
    VW_UpdateScreen(w3d);
}

/*
==================
=
= DrawHighScores
=
==================
*/

pub fn DrawHighScores(w3d: &mut modules) {
    //println!("DrawHighScores");

    let mut _buffer: String;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut _s: Vec<HighScore>;

    {
        ClearMScreen(w3d);
        DrawStripes(w3d, 10);

        VWB_DrawPic(w3d, 48, 0, graphicnums::HIGHSCORESPIC as i32);

        VWB_DrawPic(w3d, 4 * 8, 68, graphicnums::C_NAMEPIC as i32);
        VWB_DrawPic(w3d, 20 * 8, 68, graphicnums::C_LEVELPIC as i32);
        VWB_DrawPic(w3d, 28 * 8, 68, graphicnums::C_SCOREPIC as i32);

        w3d.id_vh.fontnumber = 0;

        SETFONTCOLOR(w3d, 15, 0x29);
    }

    for i in 0..MaxScores {
        let name = &w3d.id_us.Scores[i].name;
        let completed = w3d.id_us.Scores[i].completed;
        let score = w3d.id_us.Scores[i].score;
        let episode = w3d.id_us.Scores[i].episode;

        w3d.id_us.PrintY = 76 + (16 * i as i32);

        //
        // name
        //
        w3d.id_us.PrintX = 4 * 8;

        US_Print(w3d, name.clone());

        //
        // level
        //
        let buffer1 = (episode + 1).to_string();
        let buffer2 = completed.to_string();
        let mut buffer = "E".to_string();
        buffer.push_str(&buffer1);
        buffer.push_str("/L");
        buffer.push_str(&buffer2);

        USL_MeasureString(w3d, buffer.clone(), &mut w, &mut h);
        w3d.id_us.PrintX = 20 * 8;

        US_Print(w3d, buffer);
        //
        // score
        //
        let buffer = score.to_string();

        USL_MeasureString(w3d, buffer.clone(), &mut w, &mut h);
        w3d.id_us.PrintX = (34 * 8) - 8 - w;

        US_Print(w3d, buffer);
    }
    VW_UpdateScreen(w3d);
}

/*
=======================
=
= CheckHighScore
=
=======================
*/

pub fn CheckHighScore(w3d: &mut modules, score: i32, other: i32) {
    //println!("CheckHighScore");

    let mut n: i32 = -1;
    let mut myscore: HighScore = HighScore::new();

    myscore.name = "".to_string();
    myscore.score = score;
    myscore.episode = w3d.wl_game.gamestate.episode;
    myscore.completed = other;

    for i in 0..MaxScores {
        if myscore.score > w3d.id_us.Scores[i].score
            || ((myscore.score == w3d.id_us.Scores[i].score)
                && (myscore.completed > w3d.id_us.Scores[i].completed))
        {
            for j in (MaxScores..i).rev() {
                w3d.id_us.Scores[j].name = w3d.id_us.Scores[j - 1].name.clone();
                w3d.id_us.Scores[j].score = w3d.id_us.Scores[j - 1].score;
                w3d.id_us.Scores[j].episode = w3d.id_us.Scores[j - 1].episode;
                w3d.id_us.Scores[j].completed = w3d.id_us.Scores[j - 1].completed;
            }
            w3d.id_us.Scores[i] = myscore;
            n = i as i32;
            break;
        }
    }

    StartCPMusic(w3d, musicnames::ROSTER_MUS as i32);

    DrawHighScores(w3d);

    VW_FadeIn(w3d);

    if n != -1 {
        //
        // got a high score
        //
        w3d.id_us.PrintY = 76 + (16 * n);
        w3d.id_us.PrintX = 4 * 8;
        w3d.id_vh.backcolor = BORDCOLOR;
        w3d.id_vh.fontcolor = 15;

        let mut name = w3d.id_us.Scores[n as usize].name.clone();
        US_LineInput(
            w3d,
            w3d.id_us.PrintX,
            w3d.id_us.PrintY,
            &mut name,
            "".to_string(),
            true,
            MaxHighName as i32,
            100,
        );
        w3d.id_us.Scores[n as usize].name = name.clone();
    } else {
        IN_ClearKeysDown(w3d);
        IN_UserInput(w3d, 500);
    }
}
