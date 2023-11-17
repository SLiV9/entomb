//
// Part of entomb
// Copyright (c) 2023 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

use fastrand;

const NUM_STEPS: usize = 16;
const BITS_PER_STEP: u64 = 64 / (NUM_STEPS as u64);
const STEP_MASK: u64 = (1 << BITS_PER_STEP) - 1;

const NUM_KEYS: usize = 6;
const KEY_BUTTONS: [u8; NUM_KEYS] = [
	BUTTON_1,
	BUTTON_2,
	BUTTON_LEFT,
	BUTTON_RIGHT,
	BUTTON_UP,
	BUTTON_DOWN,
];
const KEY_CHARS: [u8; NUM_KEYS] = [0x80, 0x81, 0x84, 0x85, 0x86, 0x87];

#[derive(Default)]
pub struct Unlock
{
	sequence: [usize; NUM_STEPS],
	step: usize,
	prev_gamepad: u8,
	ticks: u64,
	seed_bits: u64,
}

impl Unlock
{
	pub fn from_seed(seed: u64) -> Self
	{
		let rng = fastrand::Rng::with_seed(seed);
		let mut sequence = [0; NUM_STEPS];
		for i in 0..NUM_STEPS
		{
			sequence[i] = rng.usize(0..NUM_KEYS);
		}
		Self {
			sequence,
			step: 0,
			prev_gamepad: 0xFF,
			ticks: 0,
			seed_bits: 0,
		}
	}

	pub fn update(&mut self) -> Option<u64>
	{
		let gamepad = unsafe { *GAMEPAD1 };

		self.ticks += 1;

		if self.prev_gamepad == 0
		{
			if gamepad & KEY_BUTTONS[self.sequence[self.step]] != 0
			{
				self.seed_bits <<= BITS_PER_STEP;
				self.seed_bits |= self.ticks & STEP_MASK;
				self.step += 1;
			}
		}

		self.prev_gamepad = gamepad;

		if self.step == NUM_STEPS
		{
			Some(self.seed_bits)
		}
		else
		{
			None
		}
	}

	pub fn draw(&self)
	{
		let key = KEY_CHARS[self.sequence[self.step]];

		unsafe { *DRAW_COLORS = 0x2 };
		text("Press  .", 80, 12 + 8 * (self.step as i32));
		text(&[key], 80 + 6 * 8, 12 + 8 * (self.step as i32));
	}
}
