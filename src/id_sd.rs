#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

//===========================================================================
//
//  id_sd
//
//===========================================================================

pub struct id_sd {
    pub AdLibPresent: bool,
    pub SoundBlasterPresent: bool,
    pub SBProPresent: bool,
    pub SoundPositioned: bool,
    pub SoundChunks: Vec<Vec<Chunk>>,
    pub channelSoundPos: [globalsoundpos; MIX_CHANNELS as usize],
    pub SoundMode: SDMode,
    pub MusicMode: SMMode,
    pub DigiMode: SDSMode,
    pub SoundTable: *mut Vec<u8>,
    pub tableoffset: i32,
    pub DigiMap: [i32; soundnames::LASTSOUND as usize],
    pub DigiChannel: [i32; (STARTMUSIC - STARTDIGISOUNDS) as usize],
    //      Internal variables
    pub SD_Started: bool,
    pub nextsoundpos: bool,
    pub SoundNumber: soundnames,
    pub DigiNumber: soundnames,
    pub SoundPriority: u16,
    pub DigiPriority: u16,
    pub LeftPosition: i32,
    pub RightPosition: i32,

    pub NumDigi: u16,
    pub DigiList: Vec<digiinfo>,
    pub DigiPlaying: bool,

    //      PC Sound variables
    pub pcLastSample: i8,
    pub pcSound: *mut u8,
    pub pcLengthLeft: i32,

    //      AdLib variables
    pub alSound: *mut u8,
    pub alBlock: u8,
    pub alLengthLeft: i32,
    pub alTimeCount: i32,
    pub alZeroInst: Instrument,

    //      Sequencer variables
    pub sqActive: bool,
    pub sqHack: *mut u16,
    pub sqHackPtr: *mut u16,
    pub sqHackLen: i32,
    pub sqHackSeqLen: i32,
    pub sqHackTime: i32,

    //
    pub numreadysamples: i32,
    pub curAlSound: *mut u8,
    pub curAlSoundPtr: *mut u8,
    pub curAlLengthLeft: i32,
    pub soundTimeCounter: i32,
    pub samplesPerMusicTick: i32,
}

impl id_sd {
    pub fn new() -> Self {
        let SoundMode: SDMode = SDMode::sdm_Off;
        let MusicMode: SMMode = SMMode::smm_Off;
        let DigiMode: SDSMode = SDSMode::sds_Off;
        let digiinfo: digiinfo = digiinfo {
            startpage: 0,
            length: 0,
        };

        let alZeroInst: Instrument = Instrument {
            mChar: 0,
            cChar: 0,
            mScale: 0,
            cScale: 0,
            mAttack: 0,
            cAttack: 0,
            mSus: 0,
            cSus: 0,
            mWave: 0,
            cWave: 0,
            nConn: 0,
            voice: 0,
            mode: 0,
            unused: [0; 3],
        };

        let gsp: globalsoundpos = globalsoundpos {
            valid: 0,
            globalsoundx: 0,
            globalsoundy: 0,
        };

        let mut SoundChunks: Vec<Vec<Chunk>> = Vec::new();

        for _i in 0..STARTMUSIC - STARTDIGISOUNDS {
            let chunk: Vec<Chunk> = Vec::new();
            SoundChunks.push(chunk);
        }

        //
        // Init sdl2::mixer structures
        //

        match sdl2::mixer::open_audio(44100, AUDIO_S16, 2, 2048) {
            Ok(_) => (),
            Err(e) => println!("Unable to open audio: {}", e),
        }

        sdl2::mixer::reserve_channels(2); // reserve player and boss weapon channels

        sdl2::mixer::Group(1).add_channels_range(2, MIX_CHANNELS - 1); // group remaining channels

        Self {
            AdLibPresent: false,
            SoundBlasterPresent: false,
            SBProPresent: false,
            SoundPositioned: false,
            SoundChunks,
            channelSoundPos: [gsp; MIX_CHANNELS as usize],
            SoundMode,
            MusicMode,
            DigiMode,
            SoundTable: ptr::null_mut(),
            tableoffset: 0,
            DigiMap: [0; soundnames::LASTSOUND as usize],
            DigiChannel: [0; (STARTMUSIC - STARTDIGISOUNDS) as usize],
            //      Internal variables
            SD_Started: false,
            nextsoundpos: false,
            SoundNumber: soundnames::HITWALLSND,
            DigiNumber: soundnames::HITWALLSND,
            SoundPriority: 0,
            DigiPriority: 0,
            LeftPosition: 0,
            RightPosition: 0,

            NumDigi: 0,
            DigiList: vec![digiinfo],
            DigiPlaying: false,

            //      PC Sound variables
            pcLastSample: 0,
            pcSound: ptr::null_mut(),
            pcLengthLeft: 0,

            //      AdLib variables
            alSound: ptr::null_mut(),
            alBlock: 0,
            alLengthLeft: 0,
            alTimeCount: 0,
            alZeroInst,

            //      Sequencer variables
            sqActive: false,
            sqHack: ptr::null_mut(),
            sqHackPtr: ptr::null_mut(),
            sqHackLen: 0,
            sqHackSeqLen: 0,
            sqHackTime: 0,

            //
            numreadysamples: 0,
            curAlSound: ptr::null_mut(),
            curAlSoundPtr: ptr::null_mut(),
            curAlLengthLeft: 0,
            soundTimeCounter: 5,
            samplesPerMusicTick: 0,
        }
    }
}

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SDMode {
    sdm_Off,
    sdm_PC,
    sdm_AdLib,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SMMode {
    smm_Off,
    smm_AdLib,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SDSMode {
    sds_Off,
    sds_PC,
    sds_SoundBlaster,
}

#[repr(C)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct SoundCommon {
    pub length: i32,
    pub priority: u16,
}

#[repr(C)]
#[derive(Default, Serialize, Deserialize)]
pub struct PCSound {
    pub common: SoundCommon,
    pub data: [Vec<u8>; 1],
}

#[repr(C)]
#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Instrument {
    pub mChar: u8,
    pub cChar: u8,
    pub mScale: u8,
    pub cScale: u8,
    pub mAttack: u8,
    pub cAttack: u8,
    pub mSus: u8,
    pub cSus: u8,
    pub mWave: u8,
    pub cWave: u8,
    pub nConn: u8,

    // These are only for Muse - these bytes are really unused
    pub voice: u8,
    pub mode: u8,
    pub unused: [u8; 3],
}

#[repr(C)]
#[derive(Default, Serialize, Deserialize)]
pub struct AdLibSound {
    pub common: SoundCommon,
    pub inst: Instrument,
    pub block: u8,
    pub data: [Vec<u8>; 1],
}

#[derive(Copy, Clone)]
pub struct globalsoundpos {
    pub valid: i32,
    pub globalsoundx: i32,
    pub globalsoundy: i32,
}

#[derive(Default)]
pub struct digiinfo {
    pub startpage: i32,
    pub length: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct headchunk {
    pub RIFF: [char; 4],
    pub filelenminus8: i32,
    pub WAVE: [char; 4],
    pub fmt_: [char; 4],
    pub formatlen: i32,
    pub val0x0001: u16,
    pub channels: u16,
    pub samplerate: i32,
    pub bytespersec: i32,
    pub bytespersample: u16,
    pub bitspersample: u16,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct wavechunk {
    pub chunkid: [char; 4],
    pub chunklength: i32,
}

pub const ORIGSAMPLERATE: i16 = 7042;
pub const SQUARE_WAVE_AMP: i16 = 0x2000;

pub const oplChip: i32 = 0;

pub fn alOut(n: u8, b: u8) {
    unsafe {
        YM3812Write(oplChip, n as i32, b as i32);
    }
}

pub const TickBase: i32 = 70; // 70Hz per tick - used as a base for timer 0

pub const ORIG_SOUNDCOMMON_SIZE: i32 = 6;

pub const alChar: u8 = 0x20;
pub const alScale: u8 = 0x40;
pub const alAttack: u8 = 0x60;
pub const alSus: u8 = 0x80;
pub const alWave: u8 = 0xe0;
pub const alFreqL: u8 = 0xa0;
pub const alFreqH: u8 = 0xb0;
pub const alFeedCon: u8 = 0xc0;
pub const alEffects: u8 = 0xbd;

pub const ORIG_INSTRUMENT_SIZE: i32 = 16;

pub const ORIG_ADLIBSOUND_SIZE: i32 = ORIG_SOUNDCOMMON_SIZE + ORIG_INSTRUMENT_SIZE + 2;

pub const sqMaxTracks: u8 = 10;

pub const MIX_CHANNELS: i32 = 8;

pub fn GetTimeCount(w3d: &mut modules) -> i32 {
    //println!("GetTimeCount");

    SDL_GetTicks(w3d) * 7 / 100
}

pub fn Delay(w3d: &mut modules, wolfticks: i32) {
    //println!("Delay");

    if wolfticks > 0 {
        SDL_Delay(w3d, (wolfticks * 100) / 7);
    }
}

pub fn SDL_SoundFinished(w3d: &mut modules) {
    //println!("SDL_SoundFinished");

    w3d.id_sd.SoundNumber = soundnames::HITWALLSND;
    w3d.id_sd.SoundPriority = 0;
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_PCPlaySound() - Plays the specified sound on the PC speaker
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_PCPlaySound(w3d: &mut modules, sound: &mut PCSound) {
    //println!("SDL_PCPlaySound");

    let mut sound_data: Vec<u8>;

    sound_data = sound.data[0].clone();

    w3d.id_sd.pcLastSample = -1;
    w3d.id_sd.pcLengthLeft = sound.common.length;
    w3d.id_sd.pcSound = sound_data.as_mut_ptr();
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_PCStopSound() - Stops the current sound playing on the PC Speaker
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_PCStopSound(w3d: &mut modules) {
    //println!("SDL_PCStopSound");

    w3d.id_sd.pcSound = ptr::null_mut();
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_ShutPC() - Turns off the pc speaker
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_ShutPC(w3d: &mut modules) {
    //println!("SDL_ShutPC");

    w3d.id_sd.pcSound = ptr::null_mut();
}

// Adapted from Chocolate Doom (chocolate-doom/pcsound/pcsound_sdl.c)

pub unsafe extern "C" fn SDL_PCMixCallback(w3d: &mut modules, stream: *mut u8, len: i32) {
    //println!("SDL_PCMixCallback");

    let mut current_remaining = 0;
    let mut current_freq = 0;
    let mut phase_offset = 0;

    let mut leftptr: *mut i16;
    let mut rightptr: *mut i16;
    let mut this_value: i16;

    let nsamples: i32;

    // Number of samples is quadrupled, because of 16-bit and stereo

    nsamples = len as i32 / 4;

    leftptr = stream as *mut i16;
    rightptr = stream as *mut i16;
    rightptr = rightptr.add(1);

    // Fill the output buffer

    for _i in 0..nsamples {
        // Has this sound expired? If so, retrieve the next frequency

        while current_remaining == 0 {
            phase_offset = 0;

            // Get the next frequency to play

            if w3d.id_sd.pcSound != ptr::null_mut() {
                // The PC speaker sample rate is 140Hz (see SDL_t0SlowAsmService)
                current_remaining = w3d.wl_main.param_samplerate / 140;

                if w3d.id_sd.pcSound as i8 != w3d.id_sd.pcLastSample {
                    w3d.id_sd.pcLastSample = w3d.id_sd.pcSound as i8;

                    if w3d.id_sd.pcLastSample != 0 {
                        // The PC PIC counts down at 1.193180MHz
                        // So pwm_freq = counter_freq / reload_value
                        // reload_value = pcLastSample * 60 (see SDL_DoFX)
                        current_freq = 1193180 / (w3d.id_sd.pcLastSample as i32 * 60);
                    } else {
                        current_freq = 0;
                    }
                }
                w3d.id_sd.pcSound = w3d.id_sd.pcSound.add(1);
                w3d.id_sd.pcLengthLeft -= 1;
                if w3d.id_sd.pcLengthLeft == 0 {
                    w3d.id_sd.pcSound = ptr::null_mut();
                    w3d.id_sd.SoundNumber = soundnames::HITWALLSND;
                    w3d.id_sd.SoundPriority = 0;
                }
            } else {
                current_freq = 0;
                current_remaining = 1;
            }
        }

        // Set the value for this sample.

        if current_freq == 0 {
            // Silence

            this_value = 0;
        } else {
            let frac;

            // Determine whether we are at a peak or trough in the current
            // sound.  Multiply by 2 so that frac % 2 will give 0 or 1
            // depending on whether we are at a peak or trough.

            frac = (phase_offset * current_freq * 2) / w3d.wl_main.param_samplerate;

            if (frac % 2) == 0 {
                this_value = SQUARE_WAVE_AMP;
            } else {
                this_value = -SQUARE_WAVE_AMP;
            }

            phase_offset += 1;
        }

        current_remaining -= 1;

        // Use the same value for the left and right channels.

        *leftptr += this_value;
        *rightptr += this_value;

        leftptr = leftptr.add(2);
        rightptr = rightptr.add(2);
    }
}

pub fn SD_StopDigitized(w3d: &mut modules) {
    //println!("SD_StopDigitized");

    w3d.id_sd.DigiPlaying = false;
    w3d.id_sd.DigiNumber = soundnames::HITWALLSND;
    w3d.id_sd.DigiPriority = 0;
    w3d.id_sd.SoundPositioned = false;

    if w3d.id_sd.DigiMode == SDSMode::sds_PC && w3d.id_sd.SoundMode == SDMode::sdm_PC {
        SDL_SoundFinished(w3d);
    }

    match w3d.id_sd.DigiMode {
        SDSMode::sds_PC => {
            SDL_PCStopSound(w3d);
        }
        SDSMode::sds_SoundBlaster => {
            sdl2::mixer::Channel::all().halt();
        }
        _ => (),
    }
}

pub fn SD_GetChannelForDigi(w3d: &mut modules, which: i32) -> i32 {
    //println!("SD_GetChannelForDigi");

    if w3d.id_sd.DigiChannel[which as usize] != -1 {
        return w3d.id_sd.DigiChannel[which as usize];
    }

    let channel;

    let mut channel_opt = sdl2::mixer::Group(1).find_available();

    match channel_opt {
        Some(ch) => {
            channel = ch.0;
        }
        None => {
            channel_opt = sdl2::mixer::Group(1).find_oldest();
            match channel_opt {
                Some(ch) => {
                    channel = ch.0;
                }
                None => {
                    channel_opt = sdl2::mixer::Group(1).find_available();
                    match channel_opt {
                        Some(ch) => {
                            channel = ch.0;
                        }
                        None => {
                            channel = -1;
                        }
                    }
                }
            }
        }
    }

    return channel;
}

pub fn SD_SetPosition(w3d: &mut modules, channel: i32, leftpos: i32, rightpos: i32) {
    //println!("SD_SetPosition");

    if leftpos < 0
        || leftpos > 15
        || rightpos < 0
        || rightpos > 15
        || ((leftpos == 15) && (rightpos == 15))
    {
        Quit("SD_SetPosition: Illegal position");
    }

    match w3d.id_sd.DigiMode {
        SDSMode::sds_SoundBlaster => {
            //            SDL_PositionSBP(leftpos,rightpos);
            let mut channel_sdl = sdl2::mixer::Channel::all();
            channel_sdl.0 = channel;
            sdl2::mixer::Channel::set_panning(
                channel_sdl,
                ((15 - leftpos as u8) << 4) + 15,
                ((15 - rightpos as u8) << 4) + 15,
            )
            .unwrap();
        }
        _ => (),
    }
}

pub fn GetSample(_w3d: &mut modules, csample: f32, samples: Vec<u8>, size: i32) -> i16 {
    //println!("GetSample");

    let mut s0: f32 = 0.0;
    let s1: f32;
    let mut s2: f32 = 0.0;

    let cursample = csample as i32;
    let sf: f32 = csample - cursample as f32;

    if cursample - 1 >= 0 {
        s0 = (samples[(cursample - 1) as usize] as i16 - 128) as f32;
    }

    s1 = (samples[cursample as usize] as i16 - 128) as f32;

    if cursample + 1 < size {
        s2 = (samples[(cursample + 1) as usize] as i16 - 128) as f32;
    }

    let val: f32 = s0 * sf * (sf - 1.0) / 2.0 - s1 * (sf * sf - 1.0) + s2 * (sf + 1.0) * sf / 2.0;

    let mut intval: i32 = (val * 256.0) as i32;

    if intval < -32768 {
        intval = -32768;
    } else if intval > 32767 {
        intval = 32767;
    }

    return intval as i16;
}

pub fn SD_PrepareSound(w3d: &mut modules, which: i32) {
    //println!("SD_PrepareSound");

    if w3d.id_sd.DigiList.is_empty() {
        Quit("SD_PrepareSound({which}): DigiList not initialized!\n");
    }

    let page = w3d.id_sd.DigiList[which as usize].startpage;
    let size = w3d.id_sd.DigiList[which as usize].length;

    let mut origsamples: Vec<u8>;

    origsamples = PM_GetSoundPage(w3d, page);

    //BUG
    //truncate if needed
    origsamples.resize(size as usize, 0);

    if origsamples.len() + size as usize >= PM_GetPageEnd(w3d).len() {
        //Quit("SD_PrepareSound({which}): Sound reaches out of page file!\n");
    }

    let destsamples: i32 =
        ((size * w3d.wl_main.param_samplerate as i32) as f32 / ORIGSAMPLERATE as f32) as i32;

    //byte *wavebuffer = SafeMalloc(sizeof(headchunk) + sizeof(wavechunk) + destsamples * 2);     // dest are 16-bit samples
    let mut wavebuffer: Vec<u8> = Vec::new();

    let mut head: headchunk = headchunk {
        RIFF: ['R', 'I', 'F', 'F'],
        filelenminus8: 0,
        WAVE: ['W', 'A', 'V', 'E'],
        fmt_: ['f', 'm', 't', ' '],
        formatlen: 0x10,
        val0x0001: 0x0001,
        channels: 1,
        samplerate: w3d.wl_main.param_samplerate as i32,
        bytespersec: (w3d.wl_main.param_samplerate * 2) as i32,
        bytespersample: 2,
        bitspersample: 16,
    };

    let dhead: wavechunk = wavechunk {
        chunkid: ['d', 'a', 't', 'a'],
        chunklength: (destsamples * 2) as i32,
    };

    head.filelenminus8 = 36 + destsamples * 2; // (sizeof(dhead)-8 = 0)

    //memcpy(wavebuffer, &head, sizeof(head));
    //memcpy(wavebuffer+sizeof(head), &dhead, sizeof(dhead));

    let mut head_bytes = bincode::serialize(&head).unwrap();
    let mut dhead_bytes = bincode::serialize(&dhead).unwrap();

    wavebuffer.append(&mut head_bytes);
    wavebuffer.append(&mut dhead_bytes);

    // alignment is correct, as wavebuffer comes from malloc
    // and sizeof(headchunk) % 4 == 0 and sizeof(wavechunk) % 4 == 0

    for i in 0..destsamples {
        let csample = size as f32 * i as f32 / destsamples as f32;
        let sample = GetSample(w3d, csample, origsamples.clone(), size);

        // convert i16 to u8,u8 little endian
        let value = sample.to_le_bytes();
        wavebuffer.append(&mut value.to_vec());
    }

    let mut temp = sdl2::rwops::RWops::from_bytes_mut(&mut wavebuffer).unwrap();

    let chunk = sdl2::mixer::LoaderRWops::load_wav(&mut temp).unwrap();

    w3d.id_sd.SoundChunks[which as usize].push(chunk);
}

pub fn SD_PlayDigitized(w3d: &mut modules, which: u16, leftpos: i32, rightpos: i32) -> i32 {
    //println!("SD_PlayDigitized");

    if w3d.id_sd.DigiMode == SDSMode::sds_Off {
        return 0;
    }

    if which >= w3d.id_sd.NumDigi {
        Quit("SD_PlayDigitized: bad sound number which");
    }

    let channel = SD_GetChannelForDigi(w3d, which as i32);
    SD_SetPosition(w3d, channel, leftpos, rightpos);

    w3d.id_sd.DigiPlaying = true;

    let sample = &mut w3d.id_sd.SoundChunks[which as usize];

    if sample.is_empty() {
        println!("SD_PlayDigitized sample empty");
        return 0;
    }

    let mut channel_sdl = sdl2::mixer::Channel::all();
    channel_sdl.0 = channel;

    if sdl2::mixer::Channel::play(channel_sdl, &sample[0], 0)
        .unwrap()
        .0
        == -1
    {
        println!("Unable to play sound: {}", sdl2::get_error());
        return 0;
    };

    return channel;
}

pub fn SD_ChannelFinished(_channel: Channel) {
    //println!("SD_ChannelFinished");

    //BUG
    //w3d.id_sd.channelSoundPos[channel.0 as usize].valid = 0;
}

pub fn SD_SetDigiDevice(w3d: &mut modules, mode: SDSMode) {
    //println!("SD_SetDigiDevice");

    let mut devicenotpresent: bool;

    if mode == w3d.id_sd.DigiMode {
        return;
    }

    SD_StopDigitized(w3d);

    devicenotpresent = false;
    match mode {
        SDSMode::sds_SoundBlaster => {
            if !w3d.id_sd.SoundBlasterPresent {
                devicenotpresent = true;
            }
        }
        _ => (),
    }

    if !devicenotpresent {
        w3d.id_sd.DigiMode = mode;
    }
}

pub fn SDL_SetupDigi(w3d: &mut modules) {
    //println!("SDL_SetupDigi");

    // Correct padding enforced by PM_Startup()
    let mut soundInfoPage: Vec<u16> = Vec::new();

    let vec_i32 = PM_GetPage(w3d, (w3d.id_pm.ChunksInFile - 1) as i32);

    for i in (0..vec_i32.len()).step_by(2) {
        let value1 = vec_i32[i] as u16;
        let value2 = (vec_i32[i + 1] as u16) << 8;
        let value = value1 + value2;

        soundInfoPage.push(value);
    }

    w3d.id_sd.NumDigi = PM_GetPageSize(w3d, (w3d.id_pm.ChunksInFile - 1) as i32) / 4;

    //DigiList = SafeMalloc(NumDigi * sizeof(*DigiList));
    w3d.id_sd
        .DigiList
        .resize_with(w3d.id_sd.NumDigi as usize, Default::default);

    for i in 0..w3d.id_sd.NumDigi {
        // Calculate the size of the digi from the sizes of the pages between
        // the start page and the start page of the next sound

        w3d.id_sd.DigiList[i as usize].startpage = soundInfoPage[(i * 2) as usize] as i32;

        if w3d.id_sd.DigiList[i as usize].startpage >= (w3d.id_pm.ChunksInFile - 1) as i32 {
            w3d.id_sd.NumDigi = i;
            break;
        }

        let mut lastPage: i32;

        if i < w3d.id_sd.NumDigi - 1 {
            lastPage = soundInfoPage[(i * 2 + 2) as usize] as i32;
            if lastPage == 0
                || lastPage + w3d.id_pm.PMSoundStart as i32 > (w3d.id_pm.ChunksInFile - 1) as i32
            {
                lastPage = (w3d.id_pm.ChunksInFile - 1) as i32;
            } else {
                lastPage += w3d.id_pm.PMSoundStart as i32;
            }
        } else {
            lastPage = (w3d.id_pm.ChunksInFile - 1) as i32;
        }

        let mut size: u32 = 0;

        for page in
            (w3d.id_pm.PMSoundStart as i32 + w3d.id_sd.DigiList[i as usize].startpage)..lastPage
        {
            size += PM_GetPageSize(w3d, page) as u32;
        }

        // Don't include padding of sound info page, if padding was added
        if lastPage == (w3d.id_pm.ChunksInFile - 1) as i32 && w3d.id_pm.PMSoundInfoPagePadded {
            size -= 1;
        }

        // Patch lower 16-bit of size with size from sound info page.
        // The original VSWAP contains padding which is included in the page size,
        // but not included in the 16-bit size. So we use the more precise value.

        if (size & 0xffff0000) != 0 && (size & 0xffff) < soundInfoPage[(i * 2 + 1) as usize] as u32
        {
            size -= 0x10000;
        }
        size = (size & 0xffff0000) | soundInfoPage[(i * 2 + 1) as usize] as u32;

        w3d.id_sd.DigiList[i as usize].length = size as i32;
    }

    for i in 0..soundnames::LASTSOUND as usize {
        w3d.id_sd.DigiMap[i] = -1;
        w3d.id_sd.DigiChannel[i] = -1;
    }
}

//      AdLib Code

///////////////////////////////////////////////////////////////////////////
//
//      SDL_ALStopSound() - Turns off any sound effects playing through the
//              AdLib card
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_ALStopSound(w3d: &mut modules) {
    //println!("SDL_ALStopSound");

    w3d.id_sd.alSound = ptr::null_mut();
    alOut(alFreqH + 0, 0);
}

pub fn SDL_AlSetFXInst(_w3d: &mut modules, inst: Instrument) {
    //println!("SDL_AlSetFXInst");

    let c: u8;
    let m: u8;

    m = 0; // modulator cell for channel 0
    c = 3; // carrier cell for channel 0
    alOut(m + alChar, inst.mChar);
    alOut(m + alScale, inst.mScale);
    alOut(m + alAttack, inst.mAttack);
    alOut(m + alSus, inst.mSus);
    alOut(m + alWave, inst.mWave);
    alOut(c + alChar, inst.cChar);
    alOut(c + alScale, inst.cScale);
    alOut(c + alAttack, inst.cAttack);
    alOut(c + alSus, inst.cSus);
    alOut(c + alWave, inst.cWave);

    // Note: Switch commenting on these lines for old MUSE compatibility
    //    alOutInIRQ(alFeedCon,inst->nConn);
    alOut(alFeedCon, 0);
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_ALPlaySound() - Plays the specified sound on the AdLib card
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_ALPlaySound(w3d: &mut modules, sound: &mut AdLibSound) {
    //println!("SDL_ALPlaySound");

    let inst: Instrument;
    let mut sound_data: Vec<u8>;

    SDL_ALStopSound(w3d);

    w3d.id_sd.alLengthLeft = sound.common.length;
    sound_data = sound.data[0].clone();

    w3d.id_sd.alBlock = ((sound.block & 7) << 2) | 0x20;
    inst = sound.inst;

    if (inst.mSus | inst.cSus) == 0 {
        Quit("SDL_ALPlaySound() - Bad instrument");
    }
    SDL_AlSetFXInst(w3d, inst);
    w3d.id_sd.alSound = sound_data.as_mut_ptr();
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_ShutAL() - Shuts down the AdLib card for sound effects
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_ShutAL(w3d: &mut modules) {
    //println!("SDL_ShutAL");

    w3d.id_sd.alSound = ptr::null_mut();
    alOut(alEffects, 0);
    alOut(alFreqH + 0, 0);
    SDL_AlSetFXInst(w3d, w3d.id_sd.alZeroInst);
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_StartAL() - Starts up the AdLib card for sound effects
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_StartAL(w3d: &mut modules) {
    //println!("SDL_StartAL");

    alOut(alEffects, 0);
    SDL_AlSetFXInst(w3d, w3d.id_sd.alZeroInst);
}

////////////////////////////////////////////////////////////////////////////
//
//      SDL_ShutDevice() - turns off whatever device was being used for sound fx
//
////////////////////////////////////////////////////////////////////////////

pub fn SDL_ShutDevice(w3d: &mut modules) {
    //println!("SDL_ShutDevice");

    match w3d.id_sd.SoundMode {
        SDMode::sdm_PC => {
            SDL_ShutPC(w3d);
        }
        SDMode::sdm_AdLib => {
            SDL_ShutAL(w3d);
        }
        _ => (),
    }
    w3d.id_sd.SoundMode = SDMode::sdm_Off;
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_StartDevice() - turns on whatever device is to be used for sound fx
//
///////////////////////////////////////////////////////////////////////////

pub fn SDL_StartDevice(w3d: &mut modules) {
    //println!("SDL_StartDevice");

    match w3d.id_sd.SoundMode {
        SDMode::sdm_AdLib => {
            SDL_StartAL(w3d);
        }
        _ => (),
    }
    w3d.id_sd.SoundNumber = soundnames::HITWALLSND;
    w3d.id_sd.SoundPriority = 0;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_SetSoundMode() - Sets which sound hardware to use for sound effects
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_SetSoundMode(w3d: &mut modules, mut mode: SDMode) -> bool {
    //println!("SD_SetSoundMode");

    let mut result: bool = false;

    SD_StopSound(w3d);

    if mode == SDMode::sdm_AdLib && !w3d.id_sd.AdLibPresent {
        mode = SDMode::sdm_PC;
    }

    match mode {
        SDMode::sdm_Off => {
            w3d.id_sd.tableoffset = STARTADLIBSOUNDS;
            result = true;
        }
        SDMode::sdm_PC => {
            w3d.id_sd.tableoffset = STARTPCSOUNDS;
            result = true;
        }
        SDMode::sdm_AdLib => {
            w3d.id_sd.tableoffset = STARTADLIBSOUNDS;
            if w3d.id_sd.AdLibPresent {
                result = true;
            }
        }
    }

    w3d.id_sd.SoundTable = &mut w3d.id_ca.audiosegs[w3d.id_sd.tableoffset as usize];

    if result && (mode != w3d.id_sd.SoundMode) {
        SDL_ShutDevice(w3d);
        w3d.id_sd.SoundMode = mode;
        SDL_StartDevice(w3d);
    }

    return result;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_SetMusicMode() - sets the device to use for background music
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_SetMusicMode(w3d: &mut modules, mode: SMMode) -> bool {
    //println!("SD_SetMusicMode");

    let mut result: bool = false;

    SD_FadeOutMusic(w3d);
    while SD_MusicPlaying(w3d) {
        SDL_Delay(w3d, 5);
    }

    match mode {
        SMMode::smm_Off => {
            result = true;
        }

        SMMode::smm_AdLib => {
            if w3d.id_sd.AdLibPresent {
                result = true;
            }
        }
    }

    if result {
        w3d.id_sd.MusicMode = mode;
    }

    return result;
}

pub unsafe extern "C" fn SDL_IMFMusicPlayer(w3d: &mut modules, stream: *mut u8, len: i32) {
    //println!("SDL_IMFMusicPlayer");

    let stereolen: i32 = len >> 1;
    let mut sampleslen: i32 = stereolen >> 1;

    let mut stream16 = stream as *mut i16; // expect correct alignment

    loop {
        if w3d.id_sd.numreadysamples != 0 {
            if w3d.id_sd.numreadysamples < sampleslen {
                YM3812UpdateOne(oplChip, stream16, w3d.id_sd.numreadysamples);
                stream16 = stream16.add((w3d.id_sd.numreadysamples * 2) as usize);
                sampleslen -= w3d.id_sd.numreadysamples;
            } else {
                YM3812UpdateOne(oplChip, stream16, sampleslen);
                w3d.id_sd.numreadysamples -= sampleslen;
                return;
            }
        }

        w3d.id_sd.soundTimeCounter -= 1;

        if w3d.id_sd.soundTimeCounter == 0 {
            w3d.id_sd.soundTimeCounter = 5;

            if w3d.id_sd.curAlSound != w3d.id_sd.alSound {
                w3d.id_sd.curAlSound = w3d.id_sd.alSound;
                w3d.id_sd.curAlSoundPtr = w3d.id_sd.alSound;
                w3d.id_sd.curAlLengthLeft = w3d.id_sd.alLengthLeft;
            }
            if !w3d.id_sd.curAlSound.is_null() {
                if *w3d.id_sd.curAlSoundPtr != 0 {
                    alOut(alFreqL, *w3d.id_sd.curAlSoundPtr);
                    alOut(alFreqH, w3d.id_sd.alBlock);
                } else {
                    alOut(alFreqH, 0);
                }
                w3d.id_sd.curAlSoundPtr = w3d.id_sd.curAlSoundPtr.add(1);
                w3d.id_sd.curAlLengthLeft -= 1;

                if w3d.id_sd.curAlLengthLeft == 0 {
                    w3d.id_sd.curAlSound = ptr::null_mut();
                    w3d.id_sd.alSound = ptr::null_mut();
                    w3d.id_sd.SoundNumber = soundnames::HITWALLSND;
                    w3d.id_sd.SoundPriority = 0;
                    alOut(alFreqH, 0);
                }
            }
        }

        if w3d.id_sd.sqActive {
            loop {
                if w3d.id_sd.sqHackTime > w3d.id_sd.alTimeCount {
                    break;
                }
                w3d.id_sd.sqHackTime = w3d.id_sd.alTimeCount + *(w3d.id_sd.sqHackPtr.add(1)) as i32;
                alOut(
                    *(w3d.id_sd.sqHackPtr as *mut u8),
                    *((w3d.id_sd.sqHackPtr as *mut u8).add(1)),
                );
                w3d.id_sd.sqHackPtr = w3d.id_sd.sqHackPtr.add(2);
                w3d.id_sd.sqHackLen -= 4;

                if w3d.id_sd.sqHackLen <= 0 {
                    break;
                }
            }

            w3d.id_sd.alTimeCount += 1;

            if w3d.id_sd.sqHackLen == 0 {
                w3d.id_sd.sqHackPtr = w3d.id_sd.sqHack;
                w3d.id_sd.sqHackLen = w3d.id_sd.sqHackSeqLen;
                w3d.id_sd.sqHackTime = 0;
                w3d.id_sd.alTimeCount = 0;
            }
        }

        w3d.id_sd.numreadysamples = w3d.id_sd.samplesPerMusicTick;
    }
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_Startup() - starts up the Sound Mgr
//              Detects all additional sound hardware and installs my ISR
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_Startup(w3d: &mut modules) {
    //println!("SD_Startup");

    if w3d.id_sd.SD_Started {
        return;
    }

    // Init music

    w3d.id_sd.samplesPerMusicTick = w3d.wl_main.param_samplerate / 700; // SDL_t0FastAsmService played at 700Hz

    if unsafe { YM3812Init(1, 3579545, w3d.wl_main.param_samplerate) != 0 } {
        println!("Unable to create virtual OPL!!");
    }

    for i in 1..0xf6 {
        unsafe { YM3812Write(oplChip, i, 0) };
    }

    unsafe { YM3812Write(oplChip, 1, 0x20) }; // Set WSE=1
                                              // YM3812Write(0,8,0); // Set CSM=0 & SEL=0		 // already set in for statement

    //Mix_HookMusic(SDL_IMFMusicPlayer, 0);

    unsafe {
        Mix_HookMusic(
            Some(
                SDL_IMFMusicPlayer
                    as unsafe extern "C" fn(udata: &mut modules, stream: *mut u8, len: libc::c_int),
            ),
            w3d,
        )
    };

    //BUG
    //Mix_ChannelFinished(SD_ChannelFinished);
    sdl2::mixer::set_channel_finished(SD_ChannelFinished);

    w3d.id_sd.AdLibPresent = true;
    w3d.id_sd.SoundBlasterPresent = true;

    w3d.id_sd.alTimeCount = 0;

    // Add PC speaker sound mixer
    //Mix_SetPostMix(SDL_PCMixCallback, NULL);

    unsafe {
        Mix_SetPostMix(
            Some(
                SDL_PCMixCallback
                    as unsafe extern "C" fn(udata: &mut modules, stream: *mut u8, len: libc::c_int),
            ),
            w3d,
        )
    };

    SD_SetSoundMode(w3d, SDMode::sdm_Off);
    SD_SetMusicMode(w3d, SMMode::smm_Off);

    SDL_SetupDigi(w3d);

    w3d.id_sd.SD_Started = true;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_PositionSound() - Sets up a stereo imaging location for the next
//              sound to be played. Each channel ranges from 0 to 15.
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_PositionSound(w3d: &mut modules, leftvol: i32, rightvol: i32) {
    //println!("SD_PositionSound");

    w3d.id_sd.LeftPosition = leftvol;
    w3d.id_sd.RightPosition = rightvol;
    w3d.id_sd.nextsoundpos = true;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_PlaySound() - plays the specified sound on the appropriate hardware
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_PlaySound(w3d: &mut modules, sound: soundnames) -> u8 {
    //println!("SD_PlaySound");

    let ispos: bool;
    let lp: i32;
    let rp: i32;
    let soundbytes: *mut Vec<u8>;
    let mut s: SoundCommon = SoundCommon::default();
    let s_opt: Result<SoundCommon, Box<ErrorKind>>;

    lp = w3d.id_sd.LeftPosition;
    rp = w3d.id_sd.RightPosition;
    w3d.id_sd.LeftPosition = 0;
    w3d.id_sd.RightPosition = 0;

    ispos = w3d.id_sd.nextsoundpos;
    w3d.id_sd.nextsoundpos = false;

    if sound as i8 == -1
        || (w3d.id_sd.DigiMode == SDSMode::sds_Off && w3d.id_sd.SoundMode == SDMode::sdm_Off)
    {
        return 0;
    }

    unsafe {
        soundbytes = w3d.id_sd.SoundTable.add(sound as usize);
        s_opt = bincode::deserialize(&*soundbytes);
    }

    match s_opt {
        Ok(result) => {
            s = result;
        }
        Err(_e) => if w3d.id_sd.SoundMode != SDMode::sdm_Off {},
    }

    if (w3d.id_sd.DigiMode != SDSMode::sds_Off) && (w3d.id_sd.DigiMap[sound as usize] != -1) {
        if (w3d.id_sd.DigiMode == SDSMode::sds_PC) && (w3d.id_sd.SoundMode == SDMode::sdm_PC) {
            if s.priority < w3d.id_sd.SoundPriority {
                return 0;
            }
            SDL_PCStopSound(w3d);
            SD_PlayDigitized(w3d, w3d.id_sd.DigiMap[sound as usize] as u16, lp, rp);
            w3d.id_sd.SoundPositioned = ispos;
            w3d.id_sd.SoundNumber = sound;
            w3d.id_sd.SoundPriority = s.priority;
        } else {
            let channel = SD_PlayDigitized(w3d, w3d.id_sd.DigiMap[sound as usize] as u16, lp, rp);
            w3d.id_sd.SoundPositioned = ispos;
            w3d.id_sd.DigiNumber = sound;
            w3d.id_sd.DigiPriority = s.priority;

            return (channel + 1) as u8;
        }
        return 1;
    }

    if w3d.id_sd.SoundMode == SDMode::sdm_Off {
        return 0;
    }

    if s.length == 0 {
        Quit("SD_PlaySound() - Zero length sound");
    }

    if s.priority < w3d.id_sd.SoundPriority {
        return 0;
    }

    match w3d.id_sd.SoundMode {
        SDMode::sdm_PC => unsafe {
            let mut s: PCSound = bincode::deserialize(&*soundbytes).unwrap();
            SDL_PCPlaySound(w3d, &mut s);
        },
        SDMode::sdm_AdLib => {
            w3d.id_sd.curAlSound = ptr::null_mut();
            w3d.id_sd.alSound = ptr::null_mut(); // Tricob
            alOut(alFreqH, 0);
            unsafe {
                let mut s: AdLibSound = bincode::deserialize(&*soundbytes).unwrap();
                SDL_ALPlaySound(w3d, &mut s);
            }
        }
        _ => (),
    }

    w3d.id_sd.SoundNumber = sound;
    w3d.id_sd.SoundPriority = s.priority;

    return 0;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_SoundPlaying() - returns the sound number that's playing, or 0 if
//              no sound is playing
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_SoundPlaying(w3d: &mut modules) -> i32 {
    //println!("SD_SoundPlaying");

    let mut result: bool = false;

    match w3d.id_sd.SoundMode {
        SDMode::sdm_PC => {
            if w3d.id_sd.pcSound != ptr::null_mut() {
                result = true
            } else {
                result = false;
            }
        }
        SDMode::sdm_AdLib => {
            if w3d.id_sd.alSound != ptr::null_mut() {
                result = true
            } else {
                result = false;
            }
        }
        _ => (),
    }

    if result {
        return w3d.id_sd.SoundNumber as i32;
    } else {
        return 0;
    }
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_StopSound() - if a sound is playing, stops it
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_StopSound(w3d: &mut modules) {
    //println!("SD_StopSound");

    if w3d.id_sd.DigiPlaying == true {
        SD_StopDigitized(w3d);
    }

    match &w3d.id_sd.SoundMode {
        SDMode::sdm_PC => {
            SDL_PCStopSound(w3d);
        }

        SDMode::sdm_AdLib => {
            SDL_ALStopSound(w3d);
        }
        _ => (),
    }

    w3d.id_sd.SoundPositioned = false;

    SDL_SoundFinished(w3d);
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_WaitSoundDone() - waits until the current sound is done playing
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_WaitSoundDone(w3d: &mut modules) {
    //println!("SD_WaitSoundDone");

    while SD_SoundPlaying(w3d) != 0 {
        SDL_Delay(w3d, 5);
    }
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_MusicOn() - turns on the sequencer
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_MusicOn(w3d: &mut modules) {
    //println!("SD_MusicOn");

    w3d.id_sd.sqActive = true;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_MusicOff() - turns off the sequencer and any playing notes
//      returns the last music offset for music continue
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_MusicOff(w3d: &mut modules) -> i32 {
    //println!("SD_MusicOff");

    w3d.id_sd.sqActive = false;

    match w3d.id_sd.MusicMode {
        SMMode::smm_AdLib => {
            alOut(alEffects, 0);

            for i in 0..sqMaxTracks {
                alOut(alFreqH + i + 1, 0);
            }
        }

        _ => (),
    }

    return unsafe { w3d.id_sd.sqHackPtr.offset_from(w3d.id_sd.sqHack) } as i32;
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_StartMusic() - starts playing the music pointed to
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_StartMusic(w3d: &mut modules, chunk: i32) {
    //println!("SD_StartMusic");

    SD_MusicOff(w3d);

    if w3d.id_sd.MusicMode == SMMode::smm_AdLib {
        let chunkLen: i32 = CA_CacheAudioChunk(w3d, chunk);

        w3d.id_sd.sqHack = w3d.id_ca.audiosegs[chunk as usize].as_ptr() as *mut u16; // alignment is correct

        if unsafe { *w3d.id_sd.sqHack == 0 } {
            w3d.id_sd.sqHackLen = chunkLen;
            w3d.id_sd.sqHackSeqLen = chunkLen;
        } else {
            w3d.id_sd.sqHackLen = unsafe { *w3d.id_sd.sqHack as i32 };
            w3d.id_sd.sqHackSeqLen = unsafe { *w3d.id_sd.sqHack as i32 };
            unsafe { w3d.id_sd.sqHack = w3d.id_sd.sqHack.add(1) };
        }

        w3d.id_sd.sqHackPtr = w3d.id_sd.sqHack;
        w3d.id_sd.sqHackTime = 0;
        w3d.id_sd.alTimeCount = 0;

        SD_MusicOn(w3d);
    }
}

pub fn SD_ContinueMusic(w3d: &mut modules, chunk: i32, startoffs: &mut i32) {
    //println!("SD_ContinueMusic");

    SD_MusicOff(w3d);

    if w3d.id_sd.MusicMode == SMMode::smm_AdLib {
        let chunkLen: i32 = CA_CacheAudioChunk(w3d, chunk);
        w3d.id_sd.sqHack = w3d.id_ca.audiosegs[chunk as usize].as_ptr() as *mut u16; // alignment is correct

        if unsafe { *w3d.id_sd.sqHack == 0 } {
            w3d.id_sd.sqHackLen = chunkLen;
            w3d.id_sd.sqHackSeqLen = chunkLen;
        } else {
            w3d.id_sd.sqHackLen = unsafe { *w3d.id_sd.sqHack as i32 };
            w3d.id_sd.sqHackSeqLen = unsafe { *w3d.id_sd.sqHack as i32 };
            unsafe { w3d.id_sd.sqHack = w3d.id_sd.sqHack.add(1) };
        }
        w3d.id_sd.sqHackPtr = w3d.id_sd.sqHack;

        if startoffs >= &mut w3d.id_sd.sqHackLen {
            // 7                     // Andy, improved by Chris Chokan
            *startoffs = 0;
        }

        // fast forward to correct position
        // (needed to reconstruct the instruments)

        for _i in (0..*startoffs).step_by(2) {
            let reg: u8 = unsafe { *w3d.id_sd.sqHackPtr } as u8;
            let mut val: u8 = unsafe { *w3d.id_sd.sqHackPtr + 1 } as u8;
            if reg >= 0xb1 && reg <= 0xb8 {
                val &= 0xdf; // disable play note flag
            } else if reg == 0xbd {
                val &= 0xe0; // disable drum flags
            }

            alOut(reg, val);
            unsafe { w3d.id_sd.sqHackPtr.add(2) };
            w3d.id_sd.sqHackLen -= 4;
        }
        w3d.id_sd.sqHackTime = 0;
        w3d.id_sd.alTimeCount = 0;

        SD_MusicOn(w3d);
    }
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_FadeOutMusic() - starts fading out the music. Call SD_MusicPlaying()
//              to see if the fadeout is complete
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_FadeOutMusic(w3d: &mut modules) {
    //println!("SD_FadeOutMusic");

    match &w3d.id_sd.MusicMode {
        SMMode::smm_AdLib => {
            // DEBUG - quick hack to turn the music off
            SD_MusicOff(w3d);
        }
        _ => (),
    }
}

///////////////////////////////////////////////////////////////////////////
//
//      SD_MusicPlaying() - returns true if music is currently playing, false if
//              not
//
///////////////////////////////////////////////////////////////////////////

pub fn SD_MusicPlaying(w3d: &mut modules) -> bool {
    //println!("SD_MusicPlaying");

    let result: bool;

    match &w3d.id_sd.MusicMode {
        SMMode::smm_AdLib => {
            result = w3d.id_sd.sqActive;
        }
        _ => {
            result = false;
        }
    }

    return result;
}
