#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
use embassy_executor::Spawner;
use embassy_futures::{join::join, yield_now};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{
    USB,
    PIN_2,
    PIN_3,
    PIN_4,
    PIN_5,
    PIN_6,
    PIN_7,
    PIN_8,
    PIN_9,
    PIN_10,
    PIN_11,
    PIN_12,
    PIN_13,
    PIN_14,
    PIN_15,
    PIN_16,
    PIN_17,
    PIN_18,
    PIN_19,
    PIN_20,
};
use embassy_rp::gpio;
use gpio::{Level, Output, Input, Pull, Pin};
use defmt::{info,error};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::midi::MidiClass;
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Config};
use {defmt_rtt as _, panic_probe as _};
use embassy_sync::{
    channel::{Channel},
    blocking_mutex::raw::CriticalSectionRawMutex
};

mod midi;
use midi::*;

static CHANNEL: Channel<CriticalSectionRawMutex, Note, 10> = Channel::new();


bind_interrupts!(
    
    struct Irqs {
	USBCTRL_IRQ => InterruptHandler<USB>;
    }
    
);



#[embassy_executor::main]
async fn main(spawner: Spawner) {


    info!("Inicializando embassy");
    let p = embassy_rp::init(Default::default());


    info!("Iniciando read matrix");
    if let Err(why) = spawner.spawn(read_matrix(
	p.PIN_2,
	p.PIN_3,
	p.PIN_4,
	p.PIN_5,
	p.PIN_6,
	p.PIN_7,
	p.PIN_8,
	p.PIN_9,
	p.PIN_10,
	p.PIN_11,
	p.PIN_12,
	p.PIN_13,
	p.PIN_14,
	p.PIN_15,
	p.PIN_16,
	p.PIN_17,
	p.PIN_18,
	p.PIN_19,
	p.PIN_20,
	
    ))  {
	error!("Failed spawning 'midi_talk' task: {:?}",why);
    }

    info!("Iniciando midi_talk");
    if let Err(why) = spawner.spawn(midi_talk(p.USB)) {
	info!("Failed spawning 'midi_talk' task: {:?}",why);
    }



}


struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

#[embassy_executor::task]
async fn read_matrix(
    pin2: PIN_2,
    pin3: PIN_3,
    pin4: PIN_4,
    pin5: PIN_5,
    pin6: PIN_6,
    pin7: PIN_7,
    pin8: PIN_8,
    pin9: PIN_9,
    pin10: PIN_10,
    pin11: PIN_11,
    pin12: PIN_12,
    pin13: PIN_13,
    pin14: PIN_14,
    pin15: PIN_15,
    pin16: PIN_16,
    pin17: PIN_17,
    pin18: PIN_18,
    pin19: PIN_19,
    pin20: PIN_20,
) {


    macro_rules! to_pin{
	( L1 ) => {
	    pin2
	};
	( L2 ) => {
	    pin4
	};
	( L3 ) => {
	    pin6
	};
	( L4 ) => {
	    pin8
	};
	( L5 ) => {
	    pin10
	};
	( L6 ) => {
	    pin3
	};
	( L7 ) => {
	    pin5
	};
	( L8 ) => {
	    pin7
	};
	( L9 ) => {
	    pin9
	};
	( L10 ) => {
	    pin11
	};
	( L11 ) => {
	    pin12
	};
	( L12 ) => {
	    pin13
	};
	( L13 ) => {
	    pin14
	};
	( L14 ) => {
	    pin15
	};
	( L15 ) => {
	    pin16
	};
	( L16 ) => {
	    pin17
	};
	( L17 ) => {
	    pin20
	};
	( LSUS_IN ) => {
	    pin18
	};
	( LSUS_OUT ) => {
	    pin19
	};
    }

    
    info!("Iniciando pines...");
    let sender = CHANNEL.sender();

    
    //11 9 10 8 7 6 13 14 15 16 17
    // Onput only pins
    let mut l1 = Output::new(to_pin!(L1), Level::Low);
    let mut l2 = Output::new(to_pin!(L2), Level::Low);
    let mut l3 = Output::new(to_pin!(L3), Level::Low);
    let mut l4 = Output::new(to_pin!(L4), Level::Low);
    let mut l5 = Output::new(to_pin!(L5), Level::Low);
    let mut l12 = Output::new(to_pin!(L12), Level::Low);
    let mut l13 = Output::new(to_pin!(L13), Level::Low);
    let mut l14 = Output::new(to_pin!(L14), Level::Low);
    let mut l15 = Output::new(to_pin!(L15), Level::Low);
    let mut l16 = Output::new(to_pin!(L16), Level::Low);
    let mut l17 = Output::new(to_pin!(L17), Level::Low);
    let mut lsus_out = Output::new(to_pin!(LSUS_OUT), Level::Low);

    
    
    // Input only pins
    let l6 = Input::new(to_pin!(L6), Pull::Down);
    let l7 = Input::new(to_pin!(L7), Pull::Down);
    let l8 = Input::new(to_pin!(L8), Pull::Down);
    let l9 = Input::new(to_pin!(L9), Pull::Down);
    let l10 = Input::new(to_pin!(L10), Pull::Down);
    let l11 = Input::new(to_pin!(L11), Pull::Down);
    let lsus_in = Input::new(to_pin!(LSUS_IN), Pull::Down);

    let mut state : [bool;62] = [false; 62];

    #[inline]
    fn check_pressed_io_output_input<'d,TO: Pin, TI: Pin>(activated: &mut Output<'d,TO>, read: &Input<'d,TI>) -> bool{
	activated.set_high();
	let state = read.is_high();
	activated.set_low();

	state
    }

    #[inline]
    fn update_state_and_generate_note_if_any(state: &mut [bool],pitch: Pitch,  new_state: bool) -> Option<Note> {

	let index = pitch.relative() as usize;
	
	let Some(read) = state.get(index) else {
	    return None;
	};

	let speed = 127u8;
	let pressed : bool;

	match (read, new_state){
	    (true, false) => {
		state.get_mut(index).map(|x| *x = false);
		pressed = false;
	    },
	    (false, true) => {
		state.get_mut(index).map(|x| *x = true);
		pressed = true;
	    },
	    (true,true) | (false, false) => {
		return None;
	    }
	}

	let new_note = Note{
	    pitch,
	    speed,
	    pressed,
		
	};

	info!("NOta detectada {}",&new_note);

	return Some(new_note)
	
    }
    

    loop {

	for checked_pitch in Pitch::iter() {

	    let new_state = match checked_pitch{
		Pitch::C1 => {
		    check_pressed_io_output_input(&mut l1, &l6)
		},
		Pitch::CS1 => {
		    check_pressed_io_output_input(&mut l1, &l7)
		},
		Pitch::D1 => {
		    check_pressed_io_output_input(&mut l1, &l8)
		},
		Pitch::Eb1 => {
		    check_pressed_io_output_input(&mut l1, &l9)
		},
		Pitch::E1 => {
		    check_pressed_io_output_input(&mut l1, &l10)
		},
		Pitch::F1 => {
		    check_pressed_io_output_input(&mut l1, &l11)
		},
		Pitch::FS1 => {
		    check_pressed_io_output_input(&mut l2, &l6)
		},
		Pitch::G1 => {
		    check_pressed_io_output_input(&mut l2, &l7)
		},
		Pitch::GS1 => {
		    check_pressed_io_output_input(&mut l2, &l8)
		},
		Pitch::A1 => {
		    check_pressed_io_output_input(&mut l2, &l9)
		},
		Pitch::Bb1 => {
		    check_pressed_io_output_input(&mut l2, &l10)
		},
		Pitch::B1 => {
		    check_pressed_io_output_input(&mut l2, &l11)
		},
		Pitch::C2 => {
		    check_pressed_io_output_input(&mut l3, &l6)
		},
		Pitch::CS2 => {
		    check_pressed_io_output_input(&mut l3, &l7)
		},
		Pitch::D2 => {
		    check_pressed_io_output_input(&mut l3, &l8)
		},
		Pitch::Eb2 => {
		    check_pressed_io_output_input(&mut l3, &l9)
		},
		Pitch::E2 => {
		    check_pressed_io_output_input(&mut l3, &l10)
		},
		Pitch::F2 => {
		    check_pressed_io_output_input(&mut l3, &l11)
		},
		Pitch::FS2 => {
		    check_pressed_io_output_input(&mut l4, &l6)
		},
		Pitch::G2 => {
		    check_pressed_io_output_input(&mut l4, &l7)
		},
		Pitch::GS2 => {
		    check_pressed_io_output_input(&mut l4, &l8)
		},
		Pitch::A2 => {
		    check_pressed_io_output_input(&mut l4, &l9)
		},
		Pitch::Bb2 => {
		    check_pressed_io_output_input(&mut l4, &l10)
		},
		Pitch::B2 => {
		    check_pressed_io_output_input(&mut l4, &l11)
		},
		Pitch::C3 => {
		    check_pressed_io_output_input(&mut l5, &l6)
		},
		Pitch::CS3 => {
		    check_pressed_io_output_input(&mut l5, &l7)
		},
		Pitch::D3 => {
		    check_pressed_io_output_input(&mut l5, &l8)
		},
		Pitch::Eb3 => {
		    check_pressed_io_output_input(&mut l5, &l9)
		},
		Pitch::E3 => {
		    check_pressed_io_output_input(&mut l5, &l10)
		},
		Pitch::F3 => {
		    check_pressed_io_output_input(&mut l5, &l11)
		},
		Pitch::FS3 => {
		    check_pressed_io_output_input(&mut l12, &l6)
		},
		Pitch::G3 => {
		    check_pressed_io_output_input(&mut l12, &l7)
		},
		Pitch::GS3 => {
		    check_pressed_io_output_input(&mut l12, &l8)
		},
		Pitch::A3 => {
		    check_pressed_io_output_input(&mut l12, &l9)
		},
		Pitch::Bb3 => {
		    check_pressed_io_output_input(&mut l12, &l10)
		},
		Pitch::B3 => {
		    check_pressed_io_output_input(&mut l12, &l11)
		},
		Pitch::C4 => {
		    check_pressed_io_output_input(&mut l13, &l6)
		},
		Pitch::CS4 => {
		    check_pressed_io_output_input(&mut l13, &l7)
		},
		Pitch::D4 => {
		    check_pressed_io_output_input(&mut l13, &l8)
		},
		Pitch::Eb4 => {
		    check_pressed_io_output_input(&mut l13, &l9)
		},
		Pitch::E4 => {
		    check_pressed_io_output_input(&mut l13, &l10)
		},
		Pitch::F4 => {
		    check_pressed_io_output_input(&mut l13, &l11)
		},
		Pitch::FS4 => {
		    check_pressed_io_output_input(&mut l14, &l6)
		},
		Pitch::G4 => {
		    check_pressed_io_output_input(&mut l14, &l7)
		},
		Pitch::GS4 => {
		    check_pressed_io_output_input(&mut l14, &l8)
		},
		Pitch::A4 => {
		    check_pressed_io_output_input(&mut l14, &l9)
		},
		Pitch::Bb4 => {
		    check_pressed_io_output_input(&mut l14, &l10)
		},
		Pitch::B4 => {
		    check_pressed_io_output_input(&mut l14, &l11)
		},
		Pitch::C5 => {
		    check_pressed_io_output_input(&mut l15, &l6)
		},
		Pitch::CS5 => {
		    check_pressed_io_output_input(&mut l15, &l7)
		},
		Pitch::D5 => {
		    check_pressed_io_output_input(&mut l15, &l8)
		},
		Pitch::Eb5 => {
		    check_pressed_io_output_input(&mut l15, &l9)
		},
		Pitch::E5 => {
		    check_pressed_io_output_input(&mut l15, &l10)
		},
		Pitch::F5 => {
		    check_pressed_io_output_input(&mut l15, &l11)
		},
		Pitch::FS5 => {
		    check_pressed_io_output_input(&mut l16, &l6)
		},
		Pitch::G5 => {
		    check_pressed_io_output_input(&mut l16, &l7)
		},
		Pitch::GS5 => {
		    check_pressed_io_output_input(&mut l16, &l8)
		},
		Pitch::A5 => {
		    check_pressed_io_output_input(&mut l16, &l9)
		},
		Pitch::Bb5 => {
		    check_pressed_io_output_input(&mut l16, &l10)
		},
		Pitch::B5 => {
		    check_pressed_io_output_input(&mut l16, &l11)
		},
		Pitch::C6 => {
		    check_pressed_io_output_input(&mut l17, &l6)
		},
	    };

	    
	    state[61] = check_pressed_io_output_input(&mut lsus_out, &lsus_in);
	    
	    if let Some(new_note) = update_state_and_generate_note_if_any(&mut state, checked_pitch, new_state){
		
		sender.send(new_note).await;
		    
	    }
	    
	    yield_now().await;

	    
	}
	
    }
}

#[embassy_executor::task]
async fn midi_talk(usb: USB)  {

    info!("Iniciando Driver de USB");
    //Create the driver, from the HAL.
    let usb_driver = Driver::new(usb, Irqs);

    let mut usb_config = Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("CopoWare");
    usb_config.product = Some("Casio CT-638");
    usb_config.serial_number = Some("00000001");
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = 64;

    usb_config.device_class = 0xEF;
    usb_config.device_sub_class = 0x02;
    usb_config.device_protocol = 0x01;
    usb_config.composite_with_iads = true;

    let mut usb_device_descriptor = [0; 256];
    let mut usb_config_descriptor = [0; 256];
    let mut usb_bos_descriptor = [0; 256];
    let mut usb_control_buf = [0; 64];

    let mut usb_builder = Builder::new(
        usb_driver,
        usb_config,
        &mut usb_device_descriptor,
        &mut usb_config_descriptor,
        &mut usb_bos_descriptor,
        &mut [], // no msos descriptors
        &mut usb_control_buf,
    );

    let n_in_jacks = 1;
    let n_out_jacks = 1;
    let max_packet_size = 64;

    // Create classes on the builder.
    let mut class = MidiClass::new(&mut usb_builder, n_in_jacks, n_out_jacks, max_packet_size);

    // The `MidiClass` can be split into `Sender` and `Receiver`, to be used in separate tasks.
    //let (sender, receiver) = class.split();


    // Run the USB device.
    info!("Iniciando USB");
    let mut usb = usb_builder.build();
    let usb_fut = usb.run();

    
    let midi_fut = async {
	loop {
	    info!("Esperando conexion MIDI.....");
            class.wait_connection().await;
            info!("MIDI Conectado");

	    let receiver = CHANNEL.receiver();
	    
	    loop {
		let incomming_note = receiver.receive().await;
		let encoded_midi_note : MidiEncodedBytes = incomming_note.into();
		info!("datos para el midi: {:x}", &encoded_midi_note);
		if let Err(why) = class.write_packet(&encoded_midi_note).await {
		    error!("Falla al escribir paquete midi hacia el PC: {:?}",why);
		};
	    }

	}
    };

    join(usb_fut, midi_fut).await;

}
