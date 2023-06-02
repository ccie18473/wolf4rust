#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  id_pm
//
//===========================================================================
pub struct id_pm {
    pub ChunksInFile: i32,
    pub PMSpriteStart: i32,
    pub PMSoundStart: i32,

    pub PMSoundInfoPagePadded: bool,

    pub pageLengths: Vec<i32>,

    pub PMPageData: Vec<i32>,
    pub PMPages: Vec<Vec<u8>>,
}

impl id_pm {
    pub fn new() -> Self {
        Self {
            ChunksInFile: 0,
            PMSpriteStart: 0,
            PMSoundStart: 0,

            PMSoundInfoPagePadded: false,

            pageLengths: Vec::new(),

            PMPageData: Vec::new(),
            PMPages: Vec::new(),
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub fn PM_GetSpritePage(w3d: &mut modules, v: i32) -> Vec<u8> {
    //println!("PM_GetSpritePage");

    PM_GetPage(w3d, w3d.id_pm.PMSpriteStart as i32 + v)
}

pub fn PM_GetSoundPage(w3d: &mut modules, v: i32) -> Vec<u8> {
    //println!("PM_GetSoundPage");

    PM_GetPage(w3d, w3d.id_pm.PMSoundStart as i32 + v)
}

/*
==================
=
= PM_Startup
=
==================
*/

pub fn PM_Startup(w3d: &mut modules) {
    //println!("PM_Startup");

    let mut padding: i32;
    let mut page: Vec<u8> = Vec::new();
    let mut pageOffsets: Vec<i32> = Vec::new();
    let mut pagesize: i32;
    let filesize: i32;
    let datasize: i32;

    let mut fname: String;
    let mut handle: File;

    let _temp_buf: Vec<i32> = Vec::new();

    fname = String::from("vswap.");
    fname = fname + &w3d.id_ca.audioext;
    handle = File::open(&fname).unwrap();

    //
    // read in header variables
    //

    let mut buf: Vec<u8> = Vec::new();
    let bytes = handle.read_to_end(&mut buf).unwrap();

    w3d.id_pm.ChunksInFile = (buf.remove(0) as i32) + ((buf.remove(0) as i32) << 8);
    w3d.id_pm.PMSpriteStart = (buf.remove(0) as i32) + ((buf.remove(0) as i32) << 8);
    w3d.id_pm.PMSoundStart = (buf.remove(0) as i32) + ((buf.remove(0) as i32) << 8);

    //
    // read in the chunk offsets
    //

    for _i in 0..w3d.id_pm.ChunksInFile {
        let value1 = buf.remove(0) as i32;
        let value2 = (buf.remove(0) as i32) << 8;
        let value3 = (buf.remove(0) as i32) << 16;
        let value4 = (buf.remove(0) as i32) << 24;

        let value = value1 + value2 + value3 + value4;

        pageOffsets.push(value);
    }

    //
    // read in the chunk lengths
    //

    for _i in 0..w3d.id_pm.ChunksInFile {
        let value1 = buf.remove(0) as i32;
        let value2 = (buf.remove(0) as i32) << 8;

        let value = value1 + value2;

        w3d.id_pm.pageLengths.push(value);
    }

    filesize = bytes as i32;
    datasize = filesize - pageOffsets[0];

    if datasize < 0 {
        Quit("PM_Startup: The page file \"{fname}\" is too large!");
    }

    pageOffsets.push(filesize);

    //
    // check that all chunk offsets are valid
    //

    for i in 0..w3d.id_pm.ChunksInFile {
        if !pageOffsets[i as usize] != 0 {
            continue; // sparse page
        }

        if pageOffsets[i as usize] < pageOffsets[0] || pageOffsets[i as usize] >= filesize {
            Quit ("PM_Startup: Illegal page offset for page {i}: {pageOffsets[i]} (filesize: {filesize})");
        }
    }

    //
    // calculate total amount of padding needed for sprites and sound info page
    //

    padding = 0;

    for i in w3d.id_pm.PMSpriteStart..w3d.id_pm.PMSoundStart {
        if pageOffsets[i as usize] == 0 {
            continue; // sparse page
        }

        if (((pageOffsets[i as usize] - pageOffsets[0]) + padding) & 1) != 0 {
            padding += 1;
        }
    }

    //
    // allocate enough memory to hold the whole page file
    //

    //
    // [ChunksInFile + 1] pointers to page starts
    // the last pointer points one byte after the last page
    //

    //
    // load pages and initialize PMPages pointers
    //

    let mut page_index: usize = 0;
    let PMPageData_index: usize = 0;

    for i in 0..w3d.id_pm.ChunksInFile {
        if (i >= w3d.id_pm.PMSpriteStart && i < w3d.id_pm.PMSoundStart)
            || i == w3d.id_pm.ChunksInFile - 1
        {
            //
            // pad with zeros to make it 2-byte aligned
            //

            if ((page_index - PMPageData_index) & 1) != 0 {
                {
                    page.push(0);
                    page_index += 1;
                }

                if i == w3d.id_pm.ChunksInFile - 1 {
                    w3d.id_pm.PMSoundInfoPagePadded = true;
                }
            }
        }

        if pageOffsets[i as usize] == 0 {
            w3d.id_pm.PMPages.push(vec![0]);
            continue; // sparse page
        }

        //
        // use specified page length when next page is sparse
        // otherwise, calculate size from the offset difference between this and the next page
        //
        if pageOffsets[(i + 1) as usize] == 0 {
            pagesize = w3d.id_pm.pageLengths[i as usize];
        } else {
            pagesize = pageOffsets[(i + 1) as usize] - pageOffsets[i as usize];
        }

        page.resize(pagesize as usize, 0);

        handle
            .seek(SeekFrom::Start(pageOffsets[i as usize] as u64))
            .unwrap();
        let _bytes = handle.read(&mut page).unwrap();
        w3d.id_pm.PMPages.push(page.clone());

        page_index += pagesize as usize;
    }
    //
    // last page points after page buffer
    //
    w3d.id_pm.PMPages.push(page);
}

/*
==================
=
= PM_GetPageSize
=
==================
*/

pub fn PM_GetPageSize(w3d: &mut modules, page: i32) -> u16 {
    //println!("PM_GetPageSize");

    if page < 0 || page >= w3d.id_pm.ChunksInFile as i32 {
        Quit("PM_GetPageSize: Invalid page request: {page}");
    }

    let value = w3d.id_pm.PMPages[page as usize].len() as u16;

    return value;
}

/*
==================
=
= PM_GetPage
=
= Returns the address of the page
=
==================
*/

pub fn PM_GetPage(w3d: &mut modules, page: i32) -> Vec<u8> {
    //println!("PM_GetPage");

    if page < 0 || page >= w3d.id_pm.ChunksInFile {
        Quit("PM_GetPage: Invalid page request: {page}");
    }

    return w3d.id_pm.PMPages[page as usize].clone();
}

/*
==================
=
= PM_GetPageEnd
=
= Returns the address of the last page
=
==================
*/

pub fn PM_GetPageEnd(w3d: &mut modules) -> Vec<u8> {
    //println!("PM_GetPageEnd");

    return w3d.id_pm.PMPages[w3d.id_pm.ChunksInFile as usize].clone();
}
