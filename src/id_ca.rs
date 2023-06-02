#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  id_ca
//
//===========================================================================

pub struct id_ca {
    pub mapsegs: Vec<Vec<u16>>,     // DON'T TOUCH TYPE
    pub mapheaderseg: Vec<maptype>, // DON'T TOUCH TYPE
    pub audiosegs: Vec<Vec<u8>>,    // DON'T TOUCH TYPE
    pub grsegs: Vec<Vec<u8>>,       // DON'T TOUCH TYPE
    pub tinf: mapfiletype,

    pub extension: String,
    pub graphext: String,
    pub audioext: String,

    pub grstarts: [i32; NUMCHUNKS as usize + 1],
    pub audiostarts: Vec<i32>, // array of offsets in audio / audiot
    pub grhuffman: [huffnode; 255],

    pub grhandle: File,    // handle to EGAGRAPH
    pub maphandle: File,   // handle to MAPTEMP / GAMEMAPS
    pub audiohandle: File, // handle to AUDIOT / AUDIO

    pub chunkcomplen: i32,
    pub chunkexplen: i32,

    pub oldsoundmode: SDMode,
}

impl id_ca {
    pub fn new() -> Self {
        let mut mapsegs = Vec::new();
        for _i in 0..MAPPLANES as usize {
            mapsegs.push(Vec::new());
        }
        let mut audiosegs = Vec::new();
        for _i in 0..NUMSNDCHUNKS as usize {
            audiosegs.push(Vec::new());
        }

        let mut grsegs = Vec::new();
        for _i in 0..NUMCHUNKS as usize {
            grsegs.push(Vec::new());
        }
        let pagefile: &str;

        #[cfg(feature = "UPLOAD")]
        {
            pagefile = "vswap.wl1";
        }
        #[cfg(feature = "GOODTIMES")]
        {
            pagefile = "vswap.wl6";
        }

        Self {
            mapsegs,
            mapheaderseg: Vec::new(),
            audiosegs,
            grsegs,
            tinf: mapfiletype {
                RLEWtag: 0,
                headeroffsets: [0; NUMMAPS as usize],
            },
            extension: String::new(),
            graphext: String::new(),
            audioext: String::new(),
            grstarts: [0; NUMCHUNKS as usize + 1],
            audiostarts: Vec::new(),
            grhuffman: [huffnode { bit0: 0, bit1: 0 }; 255],
            grhandle: File::open(pagefile).unwrap(), // handle to EGAGRAPH
            maphandle: File::open(pagefile).unwrap(), // handle to MAPTEMP / GAMEMAPS
            audiohandle: File::open(pagefile).unwrap(), // handle to AUDIOT / AUDIO
            chunkcomplen: 0,
            chunkexplen: 0,
            oldsoundmode: SDMode::sdm_Off,
        }
    }
    pub fn clear(&mut self) {
        self.mapsegs = Vec::new();
        for _i in 0..MAPPLANES as usize {
            self.mapsegs.push(Vec::new());
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

// Graphics
pub const gheadname: &str = "vgahead.";
pub const gfilename: &str = "vgagraph.";
pub const gdictname: &str = "vgadict.";
// Maps
pub const mheadname: &str = "maphead.";
pub const mfilename: &str = "gamemaps.";
// Audio
pub const aheadname: &str = "audiohed.";
pub const afilename: &str = "audiot.";

#[derive(Copy, Clone)]
pub struct huffnode {
    pub bit0: u16,
    pub bit1: u16, // 0-255 is a character, > is a pointer to a node
}

//#[derive(Serialize, Deserialize)] bincode limit T/32
pub struct mapfiletype {
    pub RLEWtag: u16,
    pub headeroffsets: [i32; NUMMAPS as usize],
}

#[derive(Serialize, Deserialize)]
pub struct maptype {
    pub planestart: [i32; MAPPLANES as usize],
    pub planelength: [u16; MAPPLANES as usize],
    pub width: u16,
    pub height: u16,
    pub name: [u8; 16],
}

pub const NEARTAG: u16 = 0xa7;
pub const FARTAG: u16 = 0xa8;
pub const BLOCK: i32 = 64;
pub const MASKBLOCK: i32 = 128;

pub const NUMMAPS: i32 = 60;
pub const MAPPLANES: i32 = 3;

pub fn UNCACHEAUDIOCHUNK(w3d: &mut modules, chunk: usize) {
    //println!("UNCACHEAUDIOCHUNK");

    if !w3d.id_ca.audiosegs[chunk].is_empty() {
        w3d.id_ca.audiosegs[chunk].clear();
    }
}

pub fn GRFILEPOS(w3d: &mut modules, idx: i32) -> i32 {
    //println!("GRFILEPOS");

    //assert(idx < lengthof(grstarts));
    w3d.id_ca.grstarts[idx as usize]
}

/*
=============================================================================

                            LOW LEVEL ROUTINES

=============================================================================
*/

/*
============================
=
= CAL_GetGrChunkLength
=
= Gets the length of an explicit length chunk (not tiles)
= The file pointer is positioned so the compressed data can be read in next.
=
============================
*/

pub fn CAL_GetGrChunkLength(w3d: &mut modules, chunk: i32) {
    //println!("CAL_GetGrChunkLength");

    let filepos = GRFILEPOS(w3d, chunk);
    let mut buf: Vec<u8> = vec![0; 4];

    w3d.id_ca
        .grhandle
        .seek(SeekFrom::Start(filepos as u64))
        .unwrap();
    let _bytes = w3d.id_ca.grhandle.read(&mut buf).unwrap();

    w3d.id_ca.chunkexplen = (buf[0] as i32)
        + ((buf[1] as i32) << 8)
        + ((buf[2] as i32) << 16)
        + ((buf[3] as i32) << 24);
    w3d.id_ca.chunkcomplen = GRFILEPOS(w3d, chunk + 1) - GRFILEPOS(w3d, chunk) - 4;
}

/*
============================================================================

                COMPRESSION routines, see JHUFF.C for more

============================================================================
*/

pub fn CAL_HuffExpand(source: Vec<u8>, dest: &mut Vec<u8>, length: i32, hufftable: Vec<huffnode>) {
    //println!("CAL_HuffExpand");

    //CAL_HuffExpand(compseg, (byte*)pictable, NUMPICS * sizeof(*pictable), grhuffman);

    let _end: Vec<i32>; //should be Vec<i32>
    let end_index: usize;
    let mut dest_index: usize = 0;

    let headptr: Vec<huffnode>;

    let mut huffptr: Vec<huffnode>;
    let mut huffptr_index: usize;

    /*
    if length == 0 {
        Quit("length or dest is null!");
        return;
    }
    */

    headptr = hufftable.clone(); // head node is always node 254

    //end = dest.clone();
    end_index = length as usize;

    let mut val: u8 = source[0];

    let mut mask: u8 = 1;
    let mut nodeval: u16;

    huffptr = headptr.clone();
    huffptr_index = 254;

    let mut source_index: usize = 1;

    loop {
        if val & mask == 0 {
            nodeval = huffptr[huffptr_index].bit0;
        } else {
            nodeval = huffptr[huffptr_index].bit1;
        }
        if mask == 0x80 {
            val = source[source_index];
            source_index += 1;
            mask = 1;
        } else {
            mask <<= 1;
        }

        if nodeval < 256 {
            dest.push(nodeval as u8);
            dest_index += 1;
            huffptr = headptr.clone();
            huffptr_index = 254;

            if dest_index >= end_index {
                break;
            }
        } else {
            huffptr = hufftable.clone();
            huffptr_index = (nodeval - 256) as usize;
        }
    }
}

/*
======================
=
= CAL_CarmackExpand
=
= Length is the length of the EXPANDED data
=
======================
*/

pub fn CAL_CarmackExpand(
    _w3d: &mut modules,
    source: &mut Vec<u8>,
    dest: &mut Vec<u16>,
    length: &mut i32,
) {
    //println!("CAL_CarmackExpand");

    let mut ch: u16;
    let mut chhigh: u16;
    let mut count: u16;
    let mut offset: usize;
    let inptr: Vec<u8>;
    let mut inptr_index: usize = 0;
    let mut copyptr: Vec<u16> = vec![0; 4096];
    let mut copyptr_index: usize = 0;
    let outptr: &mut Vec<u16>;
    let mut outptr_index: usize = 0;
    let mut dest_index: usize;

    *length /= 2;

    inptr = source.to_vec();
    outptr = dest;

    while *length > 0 {
        ch = inptr[inptr_index] as u16 + ((inptr[inptr_index + 1] as u16) << 8);

        inptr_index += 2;
        chhigh = ch >> 8;
        if chhigh == NEARTAG {
            count = ch & 0xff;
            if count == 0 {
                // have to insert a word containing the tag byte
                ch |= inptr[inptr_index] as u16;
                inptr_index += 1;
                outptr[outptr_index] = ch;
                outptr_index += 1;
                *length -= 1;
            } else {
                offset = inptr[inptr_index] as usize;
                inptr_index += 1;
                //copyptr = outptr - offset;

                *length -= count as i32;

                if *length < 0 {
                    return;
                }

                loop {
                    copyptr[copyptr_index] = outptr[outptr_index - offset];
                    outptr[outptr_index] = copyptr[copyptr_index];
                    outptr_index += 1;
                    copyptr_index += 1;
                    count -= 1;
                    if count <= 0 {
                        break;
                    }
                }
            }
        } else if chhigh == FARTAG {
            count = ch & 0xff;

            if count == 0 {
                // have to insert a word containing the tag byte
                ch |= inptr[inptr_index] as u16;
                outptr[outptr_index] = ch;
                outptr_index += 1;
                *length -= 1;
            } else {
                offset =
                    (inptr[inptr_index] as i32 + ((inptr[inptr_index + 1] as i32) << 8)) as usize;
                inptr_index += 2;
                //copyptr = dest + offset;

                *length -= count as i32;

                if *length < 0 {
                    return;
                }
                dest_index = 0;
                loop {
                    copyptr[copyptr_index] = outptr[dest_index + offset];
                    outptr[outptr_index] = copyptr[copyptr_index];
                    outptr_index += 1;
                    copyptr_index += 1;
                    dest_index += 1;
                    count -= 1;
                    if count <= 0 {
                        break;
                    }
                }
            }
        } else {
            outptr[outptr_index] = ch;
            outptr_index += 1;
            *length -= 1;
        }
    }
}

/*
======================
=
= CA_RLEWexpand
= length is EXPANDED length
=
======================
*/

pub fn CA_RLEWexpand(
    _w3d: &mut modules,
    source: Vec<u16>,
    dest: &mut Vec<u16>,
    length: i32,
    rlewtag: u16,
) {
    //println!("CA_RLEWexpand");

    let mut source_index: usize = 0;
    let mut dest_index: usize = 0;
    let mut value: u16;
    let mut count: u16;
    let end_index: usize = dest_index + length as usize / 2;
    //
    // expand it
    //
    loop {
        value = source[source_index];
        source_index += 1;

        if value != rlewtag {
            //
            // uncompressed
            //
            dest.push(value);
            dest_index += 1;
        } else {
            //
            // compressed string
            //
            count = source[source_index];
            source_index += 1;
            value = source[source_index];
            source_index += 1;
            for _i in 1..=count {
                dest.push(value);
                dest_index += 1;
            }
        }

        if dest_index >= end_index {
            break;
        }
    }
}

/*
=============================================================================

                                         CACHE MANAGER ROUTINES

=============================================================================
*/

/*
======================
=
= CAL_SetupGrFile
=
======================
*/

pub fn CAL_SetupGrFile(w3d: &mut modules) {
    //println!("CAL_SetupGrFile");

    let mut fname: String;
    let mut handle: File;
    let mut compseg: Vec<u8>;

    //
    // load ???dict.ext (huffman dictionary for graphics files)
    //

    fname = String::from(gdictname);
    fname = fname + &w3d.id_ca.graphext;
    handle = File::open(&fname).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let _bytes = handle.read_to_end(&mut buf).unwrap();

    for i in 0..255 {
        let bit0: u16 = buf[4 * i] as u16 + buf[4 * i + 1] as u16 * 256;
        let bit1: u16 = buf[4 * i + 2] as u16 + buf[4 * i + 3] as u16 * 256;
        let node: huffnode = huffnode { bit0, bit1 };
        w3d.id_ca.grhuffman[i] = node;
    }

    // load the data offsets from ???head.ext
    fname = String::from(gheadname);
    fname = fname + &w3d.id_ca.graphext;
    handle = File::open(&fname).unwrap();

    let headersize = handle.seek(SeekFrom::End(0)).unwrap();
    handle.seek(SeekFrom::Start(0)).unwrap();

    let expectedsize = w3d.id_ca.grstarts.len() as u64;

    if !w3d.wl_main.param_ignorenumchunks && headersize / 3 != expectedsize {
        Quit(
            "Wolf4SDL was not compiled for these data files:\n
                {fname} contains a wrong number of offsets ({}\n
                instead of {expectedsize})!\n\n
                Please check whether you are using the right executable!\n
                (For mod developers: perhaps you forgot to update NUMCHUNKS?) ,fname, headersize / 3, expectedsize,");
    }

    let mut data: Vec<u8> = Vec::new();
    let _bytes = handle.read_to_end(&mut data).unwrap();

    let mut count: usize = 0;
    // #transmute# (i32, i32, i32) to i32
    for i in &mut w3d.id_ca.grstarts {
        let val = (data[0 + 3 * count]) as i32
            | (data[1 + 3 * count] as i32) << 8
            | (data[2 + 3 * count] as i32) << 16;
        //*i = (val == 0x00FFFFFF ? -1 : val);
        if val == 0x00FFFFFF {
            *i = i32::MAX;
        } else {
            *i = val;
        }
        //d += 3;
        count += 1;
    }

    //
    // Open the graphics file
    //

    fname = String::from(gfilename);
    fname = fname + &w3d.id_ca.graphext;
    w3d.id_ca.grhandle = File::open(&fname).unwrap();

    //
    // load the pic and sprite headers into the arrays in the data segment
    //

    CAL_GetGrChunkLength(w3d, STRUCTPIC); // position file pointer
    compseg = vec![0; w3d.id_ca.chunkcomplen as usize];
    let _bytes = w3d.id_ca.grhandle.read(&mut compseg).unwrap();

    CAL_HuffExpand(
        compseg,
        &mut w3d.id_vh.pictable_bytes,
        NUMPICS * 4 as i32,
        w3d.id_ca.grhuffman.to_vec(),
    );

    //free(compseg);

    //
    // pictable ---> defines de picture size
    //

    let mut ptt = pictabletype {
        width: 0,
        height: 0,
    };

    for i in (0..w3d.id_vh.pictable_bytes.len()).step_by(4) {
        ptt.width =
            (w3d.id_vh.pictable_bytes[i] as u16) + ((w3d.id_vh.pictable_bytes[i + 1] as u16) << 8);
        ptt.height = (w3d.id_vh.pictable_bytes[i + 2] as u16)
            + ((w3d.id_vh.pictable_bytes[i + 3] as u16) << 8);

        w3d.id_vh.pictable.push(ptt);
    }

    CA_CacheGrChunks(w3d);
}

//==========================================================================

/*
======================
=
= CAL_SetupMapFile
=
======================
*/

pub fn CAL_SetupMapFile(w3d: &mut modules) {
    //println!("CAL_SetupMapFile");

    let mut fname: String;
    let mut handle: File;
    let mut pos: i32;

    //
    // load maphead.ext (offsets and tileinfo for map file)
    //

    fname = String::from(mheadname);
    fname = fname + &w3d.id_ca.extension;
    handle = File::open(&fname).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let _bytes = handle.read_to_end(&mut buf).unwrap();

    let mut mapft: mapfiletype = mapfiletype {
        RLEWtag: 0,
        headeroffsets: [0; NUMMAPS as usize],
    };

    let tag = buf[0] as u16 + ((buf[1] as u16) << 8);

    mapft.RLEWtag = tag;

    for i in 0..NUMMAPS as usize {
        let value1 = buf[4 * i + 2] as i32;
        let value2 = (buf[4 * i + 3] as i32) << 8;
        let value3 = (buf[4 * i + 4] as i32) << 16;
        let value4 = (buf[4 * i + 5] as i32) << 24;
        let value = value1 + value2 + value3 + value4;

        mapft.headeroffsets[i] = value;
    }

    w3d.id_ca.tinf = mapft;

    //
    // open the data file
    //

    fname = String::from(mfilename);
    fname = fname + &w3d.id_ca.extension;
    w3d.id_ca.maphandle = File::open(&fname).unwrap();

    //
    // load all map header
    //
    for i in 0..NUMMAPS as usize {
        pos = w3d.id_ca.tinf.headeroffsets[i];
        if pos < 0 {
            // $FFFFFFFF start is a sparse map
            continue;
        }

        w3d.id_ca
            .maphandle
            .seek(SeekFrom::Start(pos as u64))
            .unwrap();
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(38, 0); // sizeof maptype
        let _bytes = w3d.id_ca.maphandle.read(&mut buf).unwrap();

        let mapft: maptype = bincode::deserialize(&buf[..]).unwrap();

        w3d.id_ca.mapheaderseg.push(mapft);
    }

    //
    // allocate space for 3 64*64 planes
    //
    for i in 0..MAPPLANES {
        w3d.id_ca.mapsegs[i as usize] = Vec::new();
    }
}

//==========================================================================

/*
======================
=
= CAL_SetupAudioFile
=
======================
*/

pub fn CAL_SetupAudioFile(w3d: &mut modules) {
    //println!("CAL_SetupAudioFile");

    let mut fname: String;
    let mut handle: File;

    //
    // load audiohed.ext (offsets for audio file)
    //

    fname = String::from(aheadname);
    fname = fname + &w3d.id_ca.audioext;
    handle = File::open(&fname).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let _bytes = handle.read_to_end(&mut buf).unwrap();

    for i in (0..buf.len()).step_by(4) {
        let byte1 = buf[i] as i32;
        let byte2 = (buf[i + 1] as i32) << 8;
        let byte3 = (buf[i + 2] as i32) << 16;
        let byte4 = (buf[i + 3] as i32) << 24;

        let value = byte1 + byte2 + byte3 + byte4;

        w3d.id_ca.audiostarts.push(value);
    }

    //
    // open the data file
    //

    fname = String::from(afilename);
    fname = fname + &w3d.id_ca.audioext;
    w3d.id_ca.audiohandle = File::open(&fname).unwrap();
}

//==========================================================================

/*
======================
=
= CA_Startup
=
= Open all files and load in headers
=
======================
*/

pub fn CA_Startup(w3d: &mut modules) {
    //println!("CA_Startup");

    CAL_SetupMapFile(w3d);
    CAL_SetupGrFile(w3d);
    CAL_SetupAudioFile(w3d);
}

//==========================================================================


//===========================================================================

/*
======================
=
= CA_CacheAudioChunk
=
======================
*/

pub fn CA_CacheAudioChunk(w3d: &mut modules, chunk: i32) -> i32 {
    //println!("CA_CacheAudioChunk");

    let mut bufferseg: Vec<u8> = Vec::new();
    let mut sound: PCSound = PCSound::default();

    let ptr: Vec<u8>;

    let pos: i32 = w3d.id_ca.audiostarts[chunk as usize];
    let size: i32 = w3d.id_ca.audiostarts[(chunk + 1) as usize] - pos;

    if !w3d.id_ca.audiosegs[chunk as usize].is_empty() {
        return size; // already in memory
    }

    //lseek(audiohandle, pos, SEEK_SET);
    w3d.id_ca
        .audiohandle
        .seek(SeekFrom::Start(pos as u64))
        .unwrap();

    bufferseg.resize((ORIG_SOUNDCOMMON_SIZE) as usize, 0);

    let _bytes = w3d.id_ca.audiohandle.read(&mut bufferseg).unwrap();
    ptr = bufferseg.clone();

    //sound[ptr].common.length = READLONGWORD(ptr);
    let value1 = ptr[0] as u32;
    let value2 = (ptr[1] as u32) << 8;
    let value3 = (ptr[2] as u32) << 16;
    let value4 = (ptr[3] as u32) << 24;
    let value = value1 + value2 + value3 + value4;

    sound.common.length = value as i32;

    //sound[ptr].common.priority = READWORD(ptr);
    let value1 = ptr[4] as u16;
    let value2 = (ptr[5] as u16) << 8;
    let value = value1 + value2;

    sound.common.priority = value;

    bufferseg.resize((size - ORIG_SOUNDCOMMON_SIZE as i32 + 1) as usize, 0);
    let _bytes = w3d.id_ca.audiohandle.read(&mut bufferseg).unwrap();
    sound.data[0] = bufferseg.clone();

    let sound_u8 = bincode::serialize(&sound).unwrap();
    w3d.id_ca.audiosegs[chunk as usize] = sound_u8;

    return size;
}

pub fn CA_CacheAdlibSoundChunk(w3d: &mut modules, chunk: i32) {
    //println!("CA_CacheAdlibSoundChunk");

    let mut bufferseg: Vec<u8> = Vec::new();
    let mut sound: AdLibSound = AdLibSound::default();

    let ptr: Vec<u8>;
    let mut ptr_i: usize = 0;

    let pos: i32 = w3d.id_ca.audiostarts[chunk as usize];
    let size: i32 = w3d.id_ca.audiostarts[(chunk + 1) as usize] - pos;

    if !w3d.id_ca.audiosegs[chunk as usize].is_empty() {
        return; // already in memory
    }

    //lseek(audiohandle, pos, SEEK_SET);
    w3d.id_ca
        .audiohandle
        .seek(SeekFrom::Start(pos as u64))
        .unwrap();

    //bufferseg = SafeMalloc(ORIG_ADLIBSOUND_SIZE - 1);
    bufferseg.resize((ORIG_ADLIBSOUND_SIZE - 1) as usize, 0);

    //read(audiohandle, ptr, ORIG_ADLIBSOUND_SIZE - 1);   // without data[1]
    let _bytes = w3d.id_ca.audiohandle.read(&mut bufferseg).unwrap();
    ptr = bufferseg.clone();

    //AdLibSound *sound = SafeMalloc(size + sizeof(*sound) - ORIG_ADLIBSOUND_SIZE);

    //sound[ptr].common.length = READLONGWORD(ptr);
    let value1 = ptr[0] as u32;
    let value2 = (ptr[1] as u32) << 8;
    let value3 = (ptr[2] as u32) << 16;
    let value4 = (ptr[3] as u32) << 24;
    let value = value1 + value2 + value3 + value4;

    sound.common.length = value as i32;
    ptr_i += 4;

    //sound[ptr].common.priority = READWORD(ptr);
    let value1 = ptr[4] as u16;
    let value2 = (ptr[5] as u16) << 8;
    let value = value1 + value2;

    sound.common.priority = value;
    ptr_i += 2;

    sound.inst.mChar = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.cChar = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.mScale = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.cScale = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.mAttack = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.cAttack = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.mSus = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.cSus = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.mWave = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.cWave = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.nConn = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.voice = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.mode = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.unused[0] = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.unused[1] = bufferseg[ptr_i];
    ptr_i += 1;
    sound.inst.unused[2] = bufferseg[ptr_i];
    ptr_i += 1;
    sound.block = bufferseg[ptr_i];

    //read(audiohandle, sound->data, size - ORIG_ADLIBSOUND_SIZE + 1);  // + 1 because of byte data[1]
    bufferseg.resize((size - ORIG_ADLIBSOUND_SIZE as i32 + 1) as usize, 0);
    let _bytes = w3d.id_ca.audiohandle.read(&mut bufferseg).unwrap();
    sound.data[0] = bufferseg.clone();

    let sound_u8 = bincode::serialize(&sound).unwrap();
    w3d.id_ca.audiosegs[chunk as usize] = sound_u8;

    //free (bufferseg);
}

/*
======================
=
= CA_LoadAllSounds
=
= Purges all sounds, then loads all new ones (mode switch)
=
======================
*/

pub fn CA_LoadAllSounds(w3d: &mut modules) {
    //println!("CA_LoadAllSounds");

    let mut start: i32 = 0;
    let mut cachein: bool = false;

    match w3d.id_ca.oldsoundmode {
        SDMode::sdm_Off => {
            //goto cachein;
            cachein = true;
        }
        SDMode::sdm_PC => {
            start = STARTPCSOUNDS;
        }
        SDMode::sdm_AdLib => {
            start = STARTADLIBSOUNDS;
        }
    }

    if cachein == false {
        for _i in 0..NUMSOUNDS {
            UNCACHEAUDIOCHUNK(w3d, start as usize);
            start += 1;
        }
    }

    //cachein:

    w3d.id_ca.oldsoundmode = w3d.id_sd.SoundMode;

    match w3d.id_sd.SoundMode {
        SDMode::sdm_Off => {
            start = STARTADLIBSOUNDS; // needed for priorities...
        }
        SDMode::sdm_PC => {
            start = STARTPCSOUNDS;
        }
        SDMode::sdm_AdLib => {
            start = STARTADLIBSOUNDS;
        }
    }

    if start == STARTADLIBSOUNDS {
        for _i in 0..NUMSOUNDS {
            CA_CacheAdlibSoundChunk(w3d, start as i32);
            start += 1;
        }
    } else {
        for _i in 0..NUMSOUNDS {
            CA_CacheAudioChunk(w3d, start as i32);
            start += 1;
        }
    }
}

//===========================================================================

/*
======================
=
= CAL_ExpandGrChunk
=
= Does whatever is needed with a pointer to a compressed chunk
=
======================
*/

pub fn CAL_ExpandGrChunk(w3d: &mut modules, chunk: i32, mut source: Vec<u8>) {
    //println!("CAL_ExpandGrChunk");

    let expanded: i32;

    if chunk >= STARTTILE8 as i32 && chunk < STARTEXTERNS as i32 {
        //
        // expanded sizes of tile8/16/32 are implicit
        //

        if chunk < STARTTILE8M as i32
        // tile 8s are all in one chunk!
        {
            expanded = BLOCK * NUMTILE8;
        } else if chunk < STARTTILE16 as i32 {
            expanded = MASKBLOCK * NUMTILE8M;
        } else if chunk < STARTTILE16M as i32
        // all other tiles are one/chunk
        {
            expanded = BLOCK * 4;
        } else if chunk < STARTTILE32 as i32 {
            expanded = MASKBLOCK * 4;
        } else if chunk < STARTTILE32M as i32 {
            expanded = BLOCK * 16;
        } else {
            expanded = MASKBLOCK * 16;
        }

        CAL_HuffExpand(
            source,
            &mut w3d.id_ca.grsegs[chunk as usize],
            expanded,
            w3d.id_ca.grhuffman.to_vec(),
        );
    } else {
        //
        // everything else has an explicit size longword
        //

        //expanded = source[0] as i32;
        let byte1 = source[0] as i32;
        let byte2 = (source[1] as i32) << 8;
        let byte3 = (source[2] as i32) << 16;
        let byte4 = (source[3] as i32) << 24;

        expanded = byte1 + byte2 + byte3 + byte4;

        //
        // compensate slice source[4..]
        //
        source.push(0);
        source.push(0);
        source.push(0);
        source.push(0);

        CAL_HuffExpand(
            source[4..].to_vec(),
            &mut w3d.id_ca.grsegs[chunk as usize],
            expanded,
            w3d.id_ca.grhuffman.to_vec(),
        );
    }
}

/*
======================
=
= CAL_DeplaneGrChunk
=
======================
*/

pub fn CAL_DeplaneGrChunk(w3d: &mut modules, chunk: i32) {
    //println!("CAL_DeplaneGrChunk");

    let width: i32;
    let height: i32;

    if chunk == STARTTILE8 as i32 {
        width = 8;
        height = 8;

        for i in 0..NUMTILE8 {
            let start = (i * (width * height)) as usize;
            let end = ((i + 1) * (width * height)) as usize;

            VL_DePlaneVGA(
                //VL_DePlaneVGA (grsegs[chunk] + (i * (width * height)),width,height);
                &mut w3d.id_ca.grsegs[chunk as usize][start..end],
                width,
                height,
            );
        }
    } else {
        width = w3d.id_vh.pictable[(chunk - STARTPICS) as usize].width as i32;
        height = w3d.id_vh.pictable[(chunk - STARTPICS) as usize].height as i32;

        VL_DePlaneVGA(&mut w3d.id_ca.grsegs[chunk as usize], width, height);
    }
}

/*
======================
=
= CA_CacheGrChunks
=
= Load all graphics chunks into memory
=
======================
*/

pub fn CA_CacheGrChunks(w3d: &mut modules) {
    //println!("CA_CacheGrChunks");

    let mut pos: i32;
    let mut compressed: i32;
    let mut bufferseg: Vec<u8>;
    let mut source: Vec<u8>;
    //let mut chunk: i32;
    let mut next: i32;

    for chunk in (STRUCTPIC + 1)..NUMCHUNKS as i32 {
        if !w3d.id_ca.grsegs[chunk as usize].is_empty() {
            continue; // already in memory
        }

        //
        // load the chunk into a buffer
        //
        pos = GRFILEPOS(w3d, chunk);

        if pos == i32::MAX
        // $FFFFFFFF start is a sparse tile
        {
            continue;
        }

        next = chunk + 1;

        while GRFILEPOS(w3d, next) == i32::MAX
        // skip past any sparse tiles
        {
            next += 1;
        }

        compressed = GRFILEPOS(w3d, next) - pos;

        //lseek(grhandle,pos,SEEK_SET);

        //bufferseg = SafeMalloc(compressed);
        //source = bufferseg;
        bufferseg = vec![0; compressed as usize];
        source = bufferseg;

        //read(grhandle,source,compressed);

        let mut fname: String;
        fname = String::from(gfilename);
        fname = fname + &w3d.id_ca.graphext;
        w3d.id_ca.grhandle = File::open(&fname).unwrap();

        let _bytes = w3d.id_ca.grhandle.read_at(&mut source, pos as u64).unwrap();

        CAL_ExpandGrChunk(w3d, chunk, source.to_vec());

        if chunk >= STARTPICS as i32 && chunk < STARTEXTERNS as i32 {
            CAL_DeplaneGrChunk(w3d, chunk);
        }
    }
}

//==========================================================================

/*
======================
=
= CA_CacheMap
=
= WOLF: This is specialized for a 64*64 map size
=
======================
*/

pub fn CA_CacheMap(w3d: &mut modules, mapnum: i32) {
    //println!("CA_CacheMap");

    let mut pos: i32;
    let mut compressed: i32;
    let _plane: i32 = 0;
    let mut dest: Vec<u16>;
    let size: i32;
    let mut bufferseg: Vec<u16>;
    let mut source: Vec<u16>;
    let _source_index: usize = 0;
    let mut source_u8: Vec<u8> = Vec::new();
    let _source_u8_index: usize = 0;
    let mut buffer2seg: Vec<u16> = Vec::new();
    let mut expanded: i32;

    if w3d.id_ca.mapheaderseg[mapnum as usize].width != MAPSIZE
        || w3d.id_ca.mapheaderseg[mapnum as usize].height != MAPSIZE
    {
        Quit("CA_CacheMap: Map not MAPSIZE*MAPSIZE!");
    }

    //
    // load the planes into the allready allocated buffers
    //

    //size = MAPAREA * sizeof(*dest);
    size = MAPAREA as i32 * 2; // sizeof i32

    for plane in 0..MAPPLANES {
        pos = w3d.id_ca.mapheaderseg[mapnum as usize].planestart[plane as usize];
        compressed = w3d.id_ca.mapheaderseg[mapnum as usize].planelength[plane as usize] as i32;

        //BUG
        dest = w3d.id_ca.mapsegs[plane as usize].clone();

        w3d.id_ca
            .maphandle
            .seek(SeekFrom::Start(pos as u64))
            .unwrap();

        bufferseg = vec![0; compressed as usize];
        source = bufferseg;

        source_u8.resize(2 * compressed as usize, 0);

        let _bytes = w3d
            .id_ca
            .maphandle
            .read_at(&mut source_u8, pos as u64)
            .unwrap();

        for i in 0..source_u8.len() / 2 as usize {
            source[i] = source_u8[2 * i] as u16 + ((source_u8[2 * i + 1] as u16) << 8);
        }

        //
        // unhuffman, then unRLEW
        // The huffman'd chunk has a two byte expanded length first
        // The resulting RLEW chunk also does, even though it's not really
        // needed
        //
        expanded = source[0] as i32;
        buffer2seg.resize(expanded as usize, 0);

        CAL_CarmackExpand(
            w3d,
            &mut source_u8[2..].to_vec(),
            &mut buffer2seg,
            &mut expanded,
        );

        CA_RLEWexpand(
            w3d,
            //buffer2seg[1..].to_vec(),
            buffer2seg[1..].to_vec(),
            &mut dest,
            size,
            w3d.id_ca.tinf.RLEWtag,
        );

        //BUG
        w3d.id_ca.mapsegs[plane as usize] = dest;
    }
}