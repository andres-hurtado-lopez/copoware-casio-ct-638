use defmt::Format;

#[derive(Format)]
pub enum Pitch{
    C1,
    CS1,
    D1,
    Eb1,
    E1,
    F1,
    FS1,
    G1,
    GS1,
    A1,
    Bb1,
    B1,
    C2,
    CS2,
    D2,
    Eb2,
    E2,
    F2,
    FS2,
    G2,
    GS2,
    A2,
    Bb2,
    B2,
    C3,
    CS3,
    D3,
    Eb3,
    E3,
    F3,
    FS3,
    G3,
    GS3,
    A3,
    Bb3,
    B3,
    C4,
    CS4,
    D4,
    Eb4,
    E4,
    F4,
    FS4,
    G4,
    GS4,
    A4,
    Bb4,
    B4,
    C5,
    CS5,
    D5,
    Eb5,
    E5,
    F5,
    FS5,
    G5,
    GS5,
    A5,
    Bb5,
    B5,
    C6,
}

impl Pitch{
    pub fn relative(&self) -> u8 {
	match self {
	    Pitch::C1 => 0,
	    Pitch::CS1 => 1,
	    Pitch::D1 => 2,
	    Pitch::Eb1 => 3,
	    Pitch::E1 => 4,
	    Pitch::F1 => 5,
	    Pitch::FS1 => 6,
	    Pitch::G1 => 7,
	    Pitch::GS1 => 8,
	    Pitch::A1 => 9,
	    Pitch::Bb1 => 10,
	    Pitch::B1 => 11,
	    Pitch::C2 => 12,
	    Pitch::CS2 => 13,
	    Pitch::D2 => 14,
	    Pitch::Eb2 => 15,
	    Pitch::E2 => 16,
	    Pitch::F2 => 17,
	    Pitch::FS2 => 18,
	    Pitch::G2 => 19,
	    Pitch::GS2 => 20,
	    Pitch::A2 => 21,
	    Pitch::Bb2 => 22,
	    Pitch::B2 => 23,
	    Pitch::C3 => 24,
	    Pitch::CS3 => 25,
	    Pitch::D3 => 26,
	    Pitch::Eb3 => 27,
	    Pitch::E3 => 28,
	    Pitch::F3 => 29,
	    Pitch::FS3 => 30,
	    Pitch::G3 => 31,
	    Pitch::GS3 => 32,
	    Pitch::A3 => 33,
	    Pitch::Bb3 => 34,
	    Pitch::B3 => 35,
	    Pitch::C4 => 36,
	    Pitch::CS4 => 37,
	    Pitch::D4 => 38,
	    Pitch::Eb4 => 39,
	    Pitch::E4 => 40,
	    Pitch::F4 => 41,
	    Pitch::FS4 => 42,
	    Pitch::G4 => 43,
	    Pitch::GS4 => 44,
	    Pitch::A4 => 45,
	    Pitch::Bb4 => 46,
	    Pitch::B4 => 47,
	    Pitch::C5 => 48,
	    Pitch::CS5 => 49,
	    Pitch::D5 => 50,
	    Pitch::Eb5 => 51,
	    Pitch::E5 => 52,
	    Pitch::F5 => 53,
	    Pitch::FS5 => 54,
	    Pitch::G5 => 55,
	    Pitch::GS5 => 56,
	    Pitch::A5 => 57,
	    Pitch::Bb5 => 58,
	    Pitch::B5 => 59,
	    Pitch::C6 => 60,
	}
    }

    pub fn from_relative(i: u8) -> Option<Self>{
	match i {
	    0 => Some(Pitch::C1),
	    1 => Some(Pitch::CS1),
	    2 => Some(Pitch::D1),
	    3 => Some(Pitch::Eb1),
	    4 => Some(Pitch::E1),
	    5 => Some(Pitch::F1),
	    6 => Some(Pitch::FS1),
	    7 => Some(Pitch::G1),
	    8 => Some(Pitch::GS1),
	    9 => Some(Pitch::A1),
	    10 => Some(Pitch::Bb1),
	    11 => Some(Pitch::B1),
	    12 => Some(Pitch::C2),
	    13 => Some(Pitch::CS2),
	    14 => Some(Pitch::D2),
	    15 => Some(Pitch::Eb2),
	    16 => Some(Pitch::E2),
	    17 => Some(Pitch::F2),
	    18 => Some(Pitch::FS2),
	    19 => Some(Pitch::G2),
	    20 => Some(Pitch::GS2),
	    21 => Some(Pitch::A2),
	    22 => Some(Pitch::Bb2),
	    23 => Some(Pitch::B2),
	    24 => Some(Pitch::C3),
	    25 => Some(Pitch::CS3),
	    26 => Some(Pitch::D3),
	    27 => Some(Pitch::Eb3),
	    28 => Some(Pitch::E3),
	    29 => Some(Pitch::F3),
	    30 => Some(Pitch::FS3),
	    31 => Some(Pitch::G3),
	    32 => Some(Pitch::GS3),
	    33 => Some(Pitch::A3),
	    34 => Some(Pitch::Bb3),
	    35 => Some(Pitch::B3),
	    36 => Some(Pitch::C4),
	    37 => Some(Pitch::CS4),
	    38 => Some(Pitch::D4),
	    39 => Some(Pitch::Eb4),
	    40 => Some(Pitch::E4),
	    41 => Some(Pitch::F4),
	    42 => Some(Pitch::FS4),
	    43 => Some(Pitch::G4),
	    44 => Some(Pitch::GS4),
	    45 => Some(Pitch::A4),
	    46 => Some(Pitch::Bb4),
	    47 => Some(Pitch::B4),
	    48 => Some(Pitch::C5),
	    49 => Some(Pitch::CS5),
	    50 => Some(Pitch::D5),
	    51 => Some(Pitch::Eb5),
	    52 => Some(Pitch::E5),
	    53 => Some(Pitch::F5),
	    54 => Some(Pitch::FS5),
	    55 => Some(Pitch::G5),
	    56 => Some(Pitch::GS5),
	    57 => Some(Pitch::A5),
	    58 => Some(Pitch::Bb5),
	    59 => Some(Pitch::B5),
	    60 => Some(Pitch::C6),
	    _ => None,
	}
    }

    pub fn iter() -> PitchIterator {
	PitchIterator{
	    last_note: 0,
	}
    }
}


pub struct PitchIterator{
    last_note: u8
}

impl Iterator for PitchIterator{
    type Item  = Pitch;

    fn next(&mut self) -> Option<Self::Item>{
	let result;
	if self.last_note < 61{
	    result = Pitch::from_relative(self.last_note);
	    self.last_note += 1;
	}else{
	    result = None;
	}

	result
    }
}

#[derive(Format)]
pub struct Note{
    pub pitch: Pitch,
    pub speed: u8,
    pub pressed: bool
}

pub type MidiEncodedBytes = [u8;4];

impl From<Note> for MidiEncodedBytes {

    fn from(i: Note) -> Self {
	// Status byte : 1001 CCCC
        // Data byte 1 : 0PPP PPPP
        // Data byte 2 : 0VVV VVVV

	// where:
        //      "CCCC" is the MIDI channel (from 0 to 15)
        //      "PPP PPPP" is the pitch value (from 0 to 127)
        //      "VVV VVVV" is the velocity value (from 0 to 127)  8 = Pianisimo  127 = molto forte
	
	//       C#7 On
	//      09       90       61       2a
	//00001001 10010000 01100001 00101010

	//       C#7 off
	//      08       80       61       00
	//00001000 00001000 01100001 00000000

	//      C#6 on
	//      09       90       54       29
	//00001001 10010000 01010100 00101001

	//      C#6 off
	//      08       80       54       00
	//00001000 00001000 01010100 00000000

	// http://www.music-software-development.com/midi-tutorial.html


	let ntype;
	let channel;
	let speed = i.speed;
	
	match i.pressed{
	    true => {
		ntype = 0x09u8;
		channel = 0x90u8;
	    },
	    false => {
		ntype = 0x08u8;
		channel = 0x80u8;
	    }
	};

	//const BASE_C3 :u8 = 0x30u8;
	const BASE_C1 :u8 = 0x18u8;

	let note = BASE_C1 + i.pitch.relative();
		
	[ntype,channel,note,speed]
    }
    
}
