#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  wl_text
//
//===========================================================================

pub struct wl_text {
    pub pagenum: i32,
    pub numpages: i32,

    pub leftmargin: [i32; TEXTROWS as usize],
    pub rightmargin: [i32; TEXTROWS as usize],
    pub text: Vec<u8>,
    pub text_i: usize,
    pub rowon: i32,

    pub picx: i32,
    pub picy: i32,
    pub picnum: i32,
    pub picdelay: i32,
    pub layoutdone: bool,

    pub endextern: i32,
    pub helpextern: i32,
}

impl wl_text {
    pub fn new() -> Self {
        Self {
            pagenum: 0,
            numpages: 0,

            leftmargin: [0; TEXTROWS as usize],
            rightmargin: [0; TEXTROWS as usize],
            text: Vec::new(),
            text_i: 0,
            rowon: 0,

            picx: 0,
            picy: 0,
            picnum: 0,
            picdelay: 0,
            layoutdone: false,

            endextern: graphicnums::T_ENDART1 as i32,
            helpextern: graphicnums::T_HELPART as i32,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const BACKCOLOR: i32 = 0x11;

pub const WORDLIMIT: i32 = 80;
pub const FONTHEIGHT: i32 = 10;
pub const TOPMARGIN: i32 = 16;
pub const BOTTOMMARGIN: i32 = 32;
pub const LEFTMARGIN: i32 = 16;
pub const RIGHTMARGIN: i32 = 16;
pub const PICMARGIN: i32 = 8;
pub const TEXTROWS: i32 = (200 - TOPMARGIN - BOTTOMMARGIN) / FONTHEIGHT;
pub const SPACEWIDTH: i32 = 7;
pub const SCREENPIXWIDTH: i32 = 320;
pub const SCREENMID: i32 = SCREENPIXWIDTH / 2;

/*
=====================
=
= RipToEOL
=
=====================
*/

pub fn RipToEOL(w3d: &mut modules) {
    //println!("RipToEOL");

    while w3d.wl_text.text[w3d.wl_text.text_i] as char != '\n' {
        w3d.wl_text.text_i += 1;
    }
    //move to char after /n
    w3d.wl_text.text_i += 1;
}

/*
=====================
=
= ParseNumber
=
=====================
*/

pub fn ParseNumber(w3d: &mut modules) -> i32 {
    //println!("ParseNumber");

    let mut ch: char;
    let mut num: String = String::new();

    //
    // scan until a number is found
    //
    ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;

    while ch < '0' || ch > '9' {
        w3d.wl_text.text_i += 1;
        ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;
    }

    //
    // copy the number out
    //

    loop {
        num.push(ch);
        w3d.wl_text.text_i += 1;
        ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;

        if ch < '0' || ch > '9' {
            break;
        }
    }

    let i = match num.parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    return i;
}

/*
=====================
=
= ParsePicCommand
=
= Call with text pointing just after a ^P
= Upon exit text points to the start of next line
=
=====================
*/

pub fn ParsePicCommand(w3d: &mut modules) {
    //println!("ParsePicCommand");

    w3d.wl_text.picy = ParseNumber(w3d);
    w3d.wl_text.picx = ParseNumber(w3d);
    w3d.wl_text.picnum = ParseNumber(w3d);
    RipToEOL(w3d);
}

pub fn ParseTimedCommand(w3d: &mut modules) {
    //println!("ParseTimedCommand");

    w3d.wl_text.picy = ParseNumber(w3d);
    w3d.wl_text.picx = ParseNumber(w3d);
    w3d.wl_text.picnum = ParseNumber(w3d);
    w3d.wl_text.picdelay = ParseNumber(w3d);
    RipToEOL(w3d);
}

/*
=====================
=
= TimedPicCommand
=
= Call with text pointing just after a ^P
= Upon exit text points to the start of next line
=
=====================
*/

pub fn TimedPicCommand(w3d: &mut modules) {
    //println!("TimedPicCommand");

    ParseTimedCommand(w3d);

    //
    // update the screen, and wait for time delay
    //
    VW_UpdateScreen(w3d);

    //
    // wait for time
    //
    Delay(w3d, w3d.wl_text.picdelay);

    //
    // draw pic
    //
    VWB_DrawPic(
        w3d,
        w3d.wl_text.picx & !7,
        w3d.wl_text.picy,
        w3d.wl_text.picnum,
    );
}

/*
=====================
=
= HandleCommand
=
=====================
*/

pub fn HandleCommand(w3d: &mut modules) {
    //println!("HandleCommand");

    let mut i: char;
    let margin: i32;
    let mut top: i32;
    let mut bottom: i32;
    let picwidth: i32;
    let picheight: i32;
    let picmid: i32;

    w3d.wl_text.text_i += 1;
    let ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;
    let ch = ch.to_ascii_uppercase();

    match ch
    {
        'B' => {
            w3d.wl_text.picy=ParseNumber(w3d);
            w3d.wl_text.picx=ParseNumber(w3d);
            picwidth=ParseNumber(w3d);
            picheight=ParseNumber(w3d);
            VWB_Bar(w3d,w3d.wl_text.picx,w3d.wl_text.picy,picwidth,picheight,BACKCOLOR);
            RipToEOL(w3d);
        }
        ';' => {               // comment
            RipToEOL(w3d);
        }
        'P' |               // ^P is start of next page, ^E is end of file
        'E' => {
            w3d.wl_text.layoutdone = true;
            w3d.wl_text.text_i -=1;             // back up to the '^'
        }

        'C' => {               // ^c<hex digit> changes text color
            w3d.wl_text.text_i +=1;
            i = w3d.wl_text.text[w3d.wl_text.text_i] as char;
            i = i.to_ascii_uppercase();

            if i>='0' && i<='9' {
                w3d.id_vh.fontcolor = i as i32 -'0' as i32;
            }
            else if i>='A' && i<='F' {
                w3d.id_vh.fontcolor = i as i32 -'A' as i32+10;
            }

            w3d.id_vh.fontcolor *= 16;

            w3d.wl_text.text_i +=1;
            i = w3d.wl_text.text[w3d.wl_text.text_i] as char;
            i = i.to_ascii_uppercase();

            if i>='0' && i<='9' {
                w3d.id_vh.fontcolor += i as i32-'0' as i32;
            }
            else if i>='A' && i<='F' {
                w3d.id_vh.fontcolor += i as i32-'A' as i32 +10;
            }
            w3d.wl_text.text_i +=1;
        }
        '>' => {
            w3d.id_vh.px = 160;
            w3d.wl_text.text_i +=1;
        }
        'L' => {
            w3d.id_vh.py=ParseNumber(w3d);
            w3d.wl_text.rowon = (w3d.id_vh.py-TOPMARGIN)/FONTHEIGHT;
            w3d.id_vh.py = TOPMARGIN+w3d.wl_text.rowon*FONTHEIGHT;
            w3d.id_vh.px=ParseNumber(w3d);

            // scan to end of line
            while w3d.wl_text.text[w3d.wl_text.text_i] as char != '\n' {
                w3d.wl_text.text_i += 1;
            }
            //move to char after /n
            w3d.wl_text.text_i += 1;
        }
        'T' => {               // ^Tyyy,xxx,ppp,ttt waits ttt tics, then draws pic
            TimedPicCommand (w3d);
        }
        'G' => {               // ^Gyyy,xxx,ppp draws graphic
            ParsePicCommand (w3d);
            VWB_DrawPic (w3d,w3d.wl_text.picx&!7,w3d.wl_text.picy,w3d.wl_text.picnum);
            picwidth = w3d.id_vh.pictable[(w3d.wl_text.picnum-STARTPICS) as usize].width as i32;
            picheight = w3d.id_vh.pictable[(w3d.wl_text.picnum-STARTPICS) as usize].height as i32;
            //
            // adjust margins
            //
            picmid = w3d.wl_text.picx + picwidth/2;
            if picmid > SCREENMID {
                margin = w3d.wl_text.picx-PICMARGIN;                        // new right margin
            }
            else {
                margin = w3d.wl_text.picx+picwidth+PICMARGIN;       // new left margin
            }

            top = (w3d.wl_text.picy-TOPMARGIN)/FONTHEIGHT;
            if top<0 {
                top = 0;
            }
            bottom = (w3d.wl_text.picy+picheight-TOPMARGIN)/FONTHEIGHT;
            if bottom>=TEXTROWS {
                bottom = TEXTROWS-1;
            }

            for i in top..=bottom{
                if picmid > SCREENMID {
                    w3d.wl_text.rightmargin[i as usize] = margin;
                }
                else {
                    w3d.wl_text.leftmargin[i as usize] = margin;
                }

            //
            // adjust this line if needed
            //
            if w3d.id_vh.px < w3d.wl_text.leftmargin[w3d.wl_text.rowon as usize] {
                w3d.id_vh.px = w3d.wl_text.leftmargin[w3d.wl_text.rowon as usize];
            }
        }
    }
    _ => (),
}
}

/*
=====================
=
= NewLine
=
=====================
*/

pub fn NewLine(w3d: &mut modules) {
    //println!("NewLine");

    let mut ch: char;

    w3d.wl_text.rowon += 1;

    if w3d.wl_text.rowon == TEXTROWS {
        //
        // overflowed the page, so skip until next page break
        //
        w3d.wl_text.layoutdone = true;

        loop {
            ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;

            if ch == '^' {
                ch = w3d.wl_text.text[w3d.wl_text.text_i + 1] as char;
                ch = ch.to_ascii_uppercase();

                if ch == 'E' || ch == 'P' {
                    w3d.wl_text.layoutdone = true;
                    return;
                }
            }
            w3d.wl_text.text_i += 1;
        }
    }
    w3d.id_vh.px = w3d.wl_text.leftmargin[w3d.wl_text.rowon as usize];
    w3d.id_vh.py += FONTHEIGHT;
}

/*
=====================
=
= HandleCtrls
=
=====================
*/

pub fn HandleCtrls(w3d: &mut modules) {
    //println!("HandleCtrls");

    let ch: char;

    // get the character and advance
    ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;
    w3d.wl_text.text_i += 1;

    if ch == '\n' {
        NewLine(w3d);
        return;
    }
}

/*
=====================
=
= HandleWord
=
=====================
*/

pub fn HandleWord(w3d: &mut modules) {
    //println!("HandleWord");

    let mut wword: String = String::new();
    let mut wordindex: i32;
    let mut wwidth: i32 = 0;
    let mut wheight: i32 = 0;
    let newpos: i32;

    let mut ch: char;

    //
    // copy the next word into [word]
    //
    ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;

    wordindex = 1;

    while ch as i32 > 32 {
        wword.push(ch);
        w3d.wl_text.text_i += 1;
        ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;
        wordindex += 1;
        if wordindex == WORDLIMIT {
            Quit("PageLayout: Word limit exceeded");
        }
    }
    wword.push(0 as char); // stick a null at end for C

    //
    // see if it fits on this line
    //
    let word = wword.clone();
    VW_MeasurePropString(w3d, word, &mut wwidth, &mut wheight);

    while w3d.id_vh.px + wwidth > w3d.wl_text.rightmargin[w3d.wl_text.rowon as usize] {
        NewLine(w3d);
        if w3d.wl_text.layoutdone {
            return; // overflowed page
        }
    }

    //
    // print it
    //
    newpos = w3d.id_vh.px + wwidth;
    let word = wword.clone();
    VWB_DrawPropString(w3d, word);
    w3d.id_vh.px = newpos;

    //
    // suck up any extra spaces
    //
    ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;

    while ch == ' ' {
        w3d.id_vh.px += SPACEWIDTH;
        w3d.wl_text.text_i += 1;
        ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;
    }
}

/*
=====================
=
= PageLayout
=
= Clears the screen, draws the pics on the page, and word wraps the text.
= Returns a pointer to the terminating command
=
=====================
*/

pub fn PageLayout(w3d: &mut modules, shownumber: bool) {
    //println!("PageLayout");

    let oldfontcolor: i32;
    let mut ch: char;

    oldfontcolor = w3d.id_vh.fontcolor;

    w3d.id_vh.fontcolor = 0;

    //
    // clear the screen
    //
    VWB_Bar(w3d, 0, 0, 320, 200, BACKCOLOR);
    VWB_DrawPic(w3d, 0, 0, graphicnums::H_TOPWINDOWPIC as i32);
    VWB_DrawPic(w3d, 0, 8, graphicnums::H_LEFTWINDOWPIC as i32);
    VWB_DrawPic(w3d, 312, 8, graphicnums::H_RIGHTWINDOWPIC as i32);
    VWB_DrawPic(w3d, 8, 176, graphicnums::H_BOTTOMINFOPIC as i32);

    for i in 0..TEXTROWS as usize {
        w3d.wl_text.leftmargin[i] = LEFTMARGIN;
        w3d.wl_text.rightmargin[i] = SCREENPIXWIDTH - RIGHTMARGIN;
    }

    w3d.id_vh.px = LEFTMARGIN;
    w3d.id_vh.py = TOPMARGIN;
    w3d.wl_text.rowon = 0;
    w3d.wl_text.layoutdone = false;

    //
    // make sure we are starting layout text (^P first command)
    //
    while w3d.wl_text.text[w3d.wl_text.text_i] as char <= ' ' {
        w3d.wl_text.text_i += 1;
    }

    if w3d.wl_text.text[w3d.wl_text.text_i] as char != '^'
        || w3d.wl_text.text[w3d.wl_text.text_i + 1] as char != 'P'
    {
        Quit("PageLayout: Text not headed with ^P");
    }

    while w3d.wl_text.text[w3d.wl_text.text_i] as char != '\n' {
        w3d.wl_text.text_i += 1;
    }
    //move to char after /n
    w3d.wl_text.text_i += 1;

    //
    // process text stream
    //
    loop {
        ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;

        if ch == '^' {
            HandleCommand(w3d);
        } else if ch == '\t' {
            w3d.id_vh.px = (w3d.id_vh.px + 8) & 0xf8;
            w3d.wl_text.text_i += 1;
        } else if ch <= ' ' {
            HandleCtrls(w3d);
        } else {
            HandleWord(w3d);
        }

        if w3d.wl_text.layoutdone {
            break;
        }
    }

    w3d.wl_text.pagenum += 1;

    if shownumber {
        let new_string = format!("pg {} of {}", w3d.wl_text.pagenum, w3d.wl_text.numpages);
        w3d.id_vh.px = 213;

        w3d.id_vh.py = 183;
        w3d.id_vh.fontcolor = 0x4f; //12^BACKCOLOR;

        VWB_DrawPropString(w3d, new_string);
    }

    w3d.id_vh.fontcolor = oldfontcolor;
}

//===========================================================================

/*
=====================
=
= BackPage
=
= Scans for a previous ^P
=
=====================
*/

pub fn BackPage(w3d: &mut modules) {
    //println!("BackPage");

    w3d.wl_text.pagenum -= 1;

    loop {
        w3d.wl_text.text_i -= 1;
        let ch1 = w3d.wl_text.text[w3d.wl_text.text_i] as char;

        let ch2 = w3d.wl_text.text[w3d.wl_text.text_i + 1] as char;
        let ch2 = ch2.to_ascii_uppercase();

        if ch1 == '^' && ch2 == 'P' {
            return;
        }
    }
}

//===========================================================================

/*
=====================
=
= CacheLayout
=
= Scans an entire layout file (until a ^E), counting pages
=
=====================
*/

pub fn CacheLayout(w3d: &mut modules) {
    //println!("CacheLayout");

    let mut ch: char;

    w3d.wl_text.numpages = 0;
    w3d.wl_text.pagenum = 0;
    w3d.wl_text.text_i = 0;

    loop {
        ch = w3d.wl_text.text[w3d.wl_text.text_i] as char;
        if ch == '^' {
            ch = w3d.wl_text.text[w3d.wl_text.text_i + 1] as char;
            ch = ch.to_ascii_uppercase();
            w3d.wl_text.text_i += 1;

            if ch == 'P' {
                // start of a page
                w3d.wl_text.numpages += 1;
            }
            if ch == 'E'
            // end of file, so return
            {
                w3d.wl_text.text_i = 0;
                return;
            }

            if ch == 'G' {
                // draw graphic command
                ParsePicCommand(w3d);
            }

            if ch == 'T' {
                // timed draw graphic command
                ParseTimedCommand(w3d);
            }
        } else {
            w3d.wl_text.text_i += 1;
        }

        if w3d.wl_text.text_i >= w3d.wl_text.text.len() {
            break;
        }
    }
    Quit("CacheLayout: No ^E to terminate file!");
}

/*
=====================
=
= ShowArticle
=
=====================
*/

pub fn ShowArticle(w3d: &mut modules, article: Vec<u8>) {
    //println!("ShowArticle");

    let oldfontnumber: i32;
    let mut newpage: bool;
    let mut firstpage: bool;
    let mut ci: ControlInfo = ControlInfo::new();

    w3d.wl_text.text = article;
    oldfontnumber = w3d.id_vh.fontnumber;
    w3d.id_vh.fontnumber = 0;
    VWB_Bar(w3d, 0, 0, 320, 200, BACKCOLOR);
    CacheLayout(w3d);

    newpage = true;
    firstpage = true;

    loop {
        if newpage {
            newpage = false;

            PageLayout(w3d, true);

            VW_UpdateScreen(w3d);
            if firstpage {
                VL_FadeIn(w3d, 0, 255, w3d.id_vl.gamepal, 10);
                firstpage = false;
            }
        }
        SDL_Delay(w3d, 5);
        //BUG
        w3d.id_in.LastScan = Scancode::F24;
        ReadAnyControl(w3d, &mut ci);
        let mut dir: Direction = ci.dir;

        match dir {
            Direction::dir_North | Direction::dir_South => {
                break;
            }

            _ => {
                if ci.button0 != 0 {
                    dir = Direction::dir_South;
                }
                match w3d.id_in.LastScan {
                    Scancode::Up | Scancode::PageUp | Scancode::Left => {
                        dir = Direction::dir_North;
                    }

                    Scancode::Return | Scancode::Down | Scancode::PageDown | Scancode::Right => {
                        dir = Direction::dir_South;
                    }
                    _ => (),
                }
            }
        }

        match dir {
            Direction::dir_North | Direction::dir_West => {
                if w3d.wl_text.pagenum > 1 {
                    BackPage(w3d);
                    BackPage(w3d);
                    newpage = true;
                }
                TicDelay(w3d, 20);
            }

            Direction::dir_South | Direction::dir_East => {
                if w3d.wl_text.pagenum < w3d.wl_text.numpages {
                    newpage = true;
                }
                TicDelay(w3d, 20);
            }
            _ => (),
        }

        if w3d.id_in.LastScan == Scancode::Escape || ci.button1 != 0 {
            break;
        }
        //BUG
        IN_ClearKeysDown(w3d);
    }

    IN_ClearKeysDown(w3d);
    w3d.id_vh.fontnumber = oldfontnumber;
}

/*
=================
=
= HelpScreens
=
=================
*/

#[cfg(feature = "UPLOAD")]
pub fn HelpScreens(w3d: &mut modules) {
    //println!("HelpScreens");

    let artnum: i32;
    let text: Vec<u8>;

    artnum = w3d.wl_text.helpextern;
    text = w3d.id_ca.grsegs[artnum as usize].clone();

    ShowArticle(w3d, text);

    VW_FadeOut(w3d);

    FreeMusic(w3d);
}

//
// END ARTICLES
//
pub fn EndText(w3d: &mut modules) {
    //println!("EndText");

    let artnum: i32;
    let text: Vec<u8>;

    SD_StopDigitized(w3d);

    artnum = w3d.wl_text.endextern + w3d.wl_game.gamestate.episode;
    text = w3d.id_ca.grsegs[artnum as usize].clone();

    ShowArticle(w3d, text);

    VW_FadeOut(w3d);
    SETFONTCOLOR(w3d, 0, 15);
    IN_ClearKeysDown(w3d);
    if w3d.id_in.MousePresent && IN_IsInputGrabbed(w3d) {
        IN_CenterMouse(w3d); // Clear accumulated mouse movement
    }

    FreeMusic(w3d);
}
