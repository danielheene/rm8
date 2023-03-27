use std::{io, time::Duration};

use serialport::{available_ports, ErrorKind, SerialPort, SerialPortType};

use crate::{
	audio::{self, Audio},
	slip::Slip,
	value::Value,
};

const VENDOR_ID: u16 = 0x16c0;
const PRODUCT_ID: u16 = 0x048a;
const JOYPYAD_CMD: u8 = 0xfb;
const WAVEFORM_CMD: u8 = 0xfc;
const CHARACTER_CMD: u8 = 0xfd;
const RECTANGLE_CMD: u8 = 0xfe;
pub const SCREEN_WIDTH: u32 = 320;
pub const SCREEN_HEIGHT: u32 = 240;
pub const WAVEFORM_HEIGHT: u32 = 22;
const MIN_OCTAVE: u8 = 0;
const MAX_OCTAVE: u8 = 10;
const MIN_VELOCITY: u8 = 0;
const MAX_VELOCITY: u8 = 127;
pub const KEY_EDIT: u8 = 1;
pub const KEY_OPTION: u8 = 1 << 1;
pub const KEY_RIGHT: u8 = 1 << 2;
pub const KEY_PLAY: u8 = 1 << 3;
pub const KEY_SHIFT: u8 = 1 << 4;
pub const KEY_DOWN: u8 = 1 << 5;
pub const KEY_UP: u8 = 1 << 6;
pub const KEY_LEFT: u8 = 1 << 7;
pub const KEY_DIR: u8 = KEY_UP | KEY_DOWN | KEY_LEFT | KEY_RIGHT;

pub enum Command<'a> {
	#[allow(dead_code)]
	Joypad(u8),
	Waveform((u8, u8, u8), &'a [u8]),
	Character(u8, u16, u16, (u8, u8, u8), (u8, u8, u8)),
	Rectangle(u16, u16, u16, u16, (u8, u8, u8)),
}

pub struct M8 {
	port: Box<dyn SerialPort>,
	buf: [u8; 324],
	reconnect: bool,
	lost: bool,
	slip: Slip<1024>,
	pub keyjazz: Value<bool>,
	pub note: Value<u8>,
	pub octave: Value<u8>,
	pub velocity: Value<u8>,
	pub keys: Value<u8>,
	audio: Option<Audio>,
	loopback: bool,
}

impl Drop for M8 {
	fn drop(&mut self) {
		let _ = self.disconnect();
	}
}

impl M8 {
	pub fn list_capture_devices() -> Result<Vec<String>, String> {
		// TODO:
		Ok(vec!["M8 Analog Stereo".to_string()])
	}

	pub fn capture_device_name(&self) -> Option<String> {
		self.audio.as_ref().map(|a| a.name())
	}

	pub fn connect_audio(&mut self, audio: Audio) {
		let mut audio = audio;
		audio.resume();
		self.audio.replace(audio);
		self.loopback = true;
	}

	pub fn toggle_audio(&mut self) {
		if let Some(ref mut audio) = self.audio {
			audio.toggle();
		}
	}

	pub fn reopen_audio(&mut self, name: String) -> Result<(), String> {
		if !self.loopback {
			return Ok(());
		}
		if let Some(ref mut audio) = self.audio {
			audio.reopen(todo!(), name, todo!())
		} else {
			self.audio.replace(audio::Audio::open(todo!(), Some(name), todo!())?);
			Ok(())
		}
	}

	fn open_serial(p: &serialport::SerialPortInfo) -> serialport::Result<Self> {
		if let SerialPortType::UsbPort(ref info) = p.port_type {
			if info.vid == VENDOR_ID && info.pid == PRODUCT_ID {
				return Ok(Self {
					audio: None,
					loopback: false,
					port: serialport::new(&p.port_name, 115200)
						.timeout(Duration::from_millis(1))
						.open()?,
					reconnect: false,
					lost: false,
					buf: [0; 324],
					slip: Slip::new(),
					keyjazz: Value::new(false),
					note: Value::new(255),
					octave: Value::new(3),
					velocity: Value::new(100),
					keys: Value::new(0),
				});
			}
		}
		Err(serialport::Error {
			kind: ErrorKind::NoDevice,
			description: "M8 not found".to_string(),
		})
	}

	pub fn device_name(&self) -> Option<String> {
		self.port.name()
	}

	pub fn set_reconnect(&mut self, reconnect: bool) {
		self.reconnect = reconnect;
	}

	pub fn disconnected(&self) -> bool {
		self.lost
	}

	fn try_reconnect(&mut self) -> Result<(), String> {
		if self.reconnect && self.lost {
			if let Ok(mut new_self) = M8::detect() {
				std::mem::swap(&mut self.port, &mut new_self.port);
				self.lost = false;
				self.reset(true)?;
				if *self.keyjazz {
					self.keyjazz.set_changed();
				}
			}
		}
		Ok(())
	}

	pub fn open<T: AsRef<str>>(device: T) -> serialport::Result<Self> {
		for p in available_ports()? {
			if p.port_name != device.as_ref() {
				continue;
			}
			return Self::open_serial(&p);
		}
		Err(serialport::Error {
			kind: ErrorKind::NoDevice,
			description: "Device not found".to_string(),
		})
	}

	pub fn list_ports() -> serialport::Result<Vec<String>> {
		let mut v = Vec::new();
		for p in available_ports()? {
			if let SerialPortType::UsbPort(ref info) = p.port_type {
				if info.vid == VENDOR_ID && info.pid == PRODUCT_ID {
					v.push(p.port_name.clone())
				}
			}
		}
		Ok(v)
	}

	pub fn detect() -> serialport::Result<Self> {
		for p in available_ports()? {
			if let Ok(p) = Self::open_serial(&p) {
				return Ok(p);
			}
		}
		Err(serialport::Error {
			kind: ErrorKind::NoDevice,
			description: "No port found".to_string(),
		})
	}

	pub fn read(&mut self) -> Result<Option<Command<'_>>, String> {
		self.try_reconnect()?;
		match self.slip.read(&mut self.port, &mut self.buf) {
			Ok(Some(bytes)) if !bytes.is_empty() => match bytes[0] {
				JOYPYAD_CMD if bytes.len() == 3 => Ok(None),
				JOYPYAD_CMD => Err("invalid joypad command".to_string()),
				WAVEFORM_CMD if bytes.len() == 4 || bytes.len() == 324 => {
					Ok(Some(Command::Waveform((bytes[1], bytes[2], bytes[3]), &bytes[4..])))
				}
				WAVEFORM_CMD => Err("invalid waveform command".to_string()),
				CHARACTER_CMD if bytes.len() == 12 => Ok(Some(Command::Character(
					bytes[1],
					read16(&bytes[2..4]),
					read16(&bytes[4..6]),
					(bytes[6], bytes[7], bytes[8]),
					(bytes[9], bytes[10], bytes[11]),
				))),
				CHARACTER_CMD => Err("invalid character command".to_string()),
				RECTANGLE_CMD if bytes.len() == 12 => Ok(Some(Command::Rectangle(
					read16(&bytes[1..3]),
					read16(&bytes[3..5]),
					read16(&bytes[5..7]),
					read16(&bytes[7..9]),
					(bytes[9], bytes[10], bytes[11]),
				))),
				RECTANGLE_CMD => Err("invalid rectangle command".to_string()),
				_ => Err(format!("unknown command {:02X}", bytes[0])),
			},
			Ok(None) => Ok(None),
			Ok(_) => Err("empty command".to_string()),
			Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {
				self.lost = true;
				Ok(None)
			}
			Err(e) => Err(format!("read failed: {}", e)),
		}
	}

	fn write(&mut self, buf: &[u8]) -> Result<(), String> {
		match self.port.write(buf) {
			Ok(n) if n != buf.len() => Err("failed to write command".to_string()),
			Ok(_) => Ok(()),
			Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
			Err(e) => {
				// Err(format!("write failed: {}", e)),
				println!("write failed: {}", e);
				Ok(())
			}
		}
	}

	pub fn enable_and_reset_display(&mut self) -> Result<(), String> {
		self.write("ER".as_bytes())
	}

	pub fn reset_display(&mut self) -> Result<(), String> {
		self.refresh_keyjazz();
		self.write("R".as_bytes())
	}

	pub fn disconnect(&mut self) -> Result<(), String> {
		self.write("D".as_bytes())
	}

	pub fn reset(&mut self, disconnect: bool) -> Result<(), String> {
		if disconnect {
			let _ = self.disconnect()?;
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
		self.refresh_keyjazz();
		self.enable_and_reset_display()
	}

	fn refresh_keyjazz(&mut self) {
		if *self.keyjazz {
			self.velocity.set_changed();
			self.octave.set_changed();
		}
	}

	pub fn refresh(&mut self) {
		self.refresh_keyjazz();
		self.keys.set(0);
	}

	pub fn send_keyjazz(&mut self) -> Result<(), String> {
		if *self.note == 255 {
			self.write(&[b'K', *self.note])
		} else {
			self.write(&[b'K', *self.note, *self.velocity])
		}
	}

	pub fn send_keys(&mut self) -> Result<(), String> {
		self.write(&[b'C', *self.keys])
	}

	pub fn dec_octave(&mut self) {
		self.octave.sub(1, MIN_OCTAVE)
	}

	pub fn inc_octave(&mut self) {
		self.octave.add(1, MAX_OCTAVE)
	}

	pub fn dec_velocity(&mut self, fast: bool) {
		self.velocity.sub(if fast { 16 } else { 1 }, MIN_VELOCITY)
	}

	pub fn inc_velocity(&mut self, fast: bool) {
		self.velocity.add(if fast { 16 } else { 1 }, MAX_VELOCITY)
	}

	pub fn set_note_off(&mut self) {
		self.note.set(255)
	}

	pub fn set_note(&mut self, note: u8) {
		self.note.set(note + *self.octave * 12)
	}
}

fn read16(bytes: &[u8]) -> u16 {
	assert!(bytes.len() == 2);
	u16::from_le_bytes(bytes.try_into().unwrap())
}
