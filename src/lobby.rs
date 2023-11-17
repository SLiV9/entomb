//
// Part of entomb
// Copyright (c) 2023 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

use fastrand;

const NUM_NOUNS: usize = 2485;
const NUM_BYTES_PER_NOUN: usize = 16;
const NOUN_BYTES_LEN: usize = NUM_NOUNS * NUM_BYTES_PER_NOUN;
const NOUN_BYTES: &[u8; NOUN_BYTES_LEN] = include_bytes!("../assets/nouns.txt");

const NUM_KEYWORDS: usize = 4;
const NUM_DIGITS: usize = 3;

pub struct Lobby
{
	rng: fastrand::Rng,
	word_indices: [usize; 1 + NUM_KEYWORDS],
	prev_gamepad: u8,
	is_revealed: bool,
	secret_code: [u8; NUM_DIGITS],
}

impl Lobby
{
	pub fn from_seed(seed: u64) -> Self
	{
		let rng = fastrand::Rng::with_seed(seed);
		let mut word_indices = [0; 1 + NUM_KEYWORDS];
		for i in 0..(1 + NUM_KEYWORDS)
		{
			let mut is_bad = true;
			while is_bad
			{
				word_indices[i] = rng.usize(0..NUM_NOUNS);
				is_bad = (0..i).any(|j| word_indices[j] == word_indices[i]);
			}
		}
		Self {
			rng,
			word_indices,
			prev_gamepad: 0,
			is_revealed: false,
			secret_code: [0, 0, 0],
		}
	}

	pub fn update(&mut self)
	{
		let gamepad = unsafe { *GAMEPAD1 };

		if self.prev_gamepad == 0
		{
			if gamepad & BUTTON_2 != 0 && self.is_revealed
			{
				if self.secret_code == [0, 0, 0]
				{
					for i in 0..NUM_DIGITS
					{
						let mut is_bad = true;
						while is_bad
						{
							self.secret_code[i] = self.rng.u8(1..=4);
							is_bad = (0..i).any(|j| {
								self.secret_code[j] == self.secret_code[i]
							});
						}
					}
				}
				else
				{
					self.secret_code = [0, 0, 0];
				}
			}

			if gamepad & BUTTON_1 != 0
			{
				self.is_revealed = !self.is_revealed;
			}
		}

		self.prev_gamepad = gamepad;
	}

	pub fn draw(&self)
	{
		unsafe { *DRAW_COLORS = 0x2 };
		text("Public checksum:", 8, 8);
		let safe_word = self.word(0);
		text(safe_word, 8, 18);

		text("Secret keywords:", 8, 40);
		for i in 0..NUM_KEYWORDS
		{
			let y = 50 + 10 * i as i32;
			text(&[b'1' + i as u8], 8, y);
			text(".", 16, y);
			text(self.word(1 + i), 32, y);
		}

		text("Secret code:", 8, 110);
		if self.is_revealed
		{
			text(&[b'1' + self.secret_code[0] - 1], 112, 110);
			text("-", 120, 110);
			text(&[b'1' + self.secret_code[1] - 1], 128, 110);
			text("-", 136, 110);
			text(&[b'1' + self.secret_code[2] - 1], 144, 110);

			if self.secret_code == [0, 0, 0]
			{
				text(b"Press \x81 to set.", 8, 136);
			}
			else
			{
				text(b"Press \x81 to clear.", 8, 136);
			}
			text(b"Press \x80 to hide.", 8, 144);
		}
		else
		{
			text("?????", 112, 110);
			text(b"Press \x80 to reveal.", 8, 144);
		}
	}

	fn word(&self, number: usize) -> &'static [u8]
	{
		let index = self.word_indices[number];
		let offset = index * NUM_BYTES_PER_NOUN;
		&NOUN_BYTES[offset..(offset + NUM_BYTES_PER_NOUN)]
	}
}
