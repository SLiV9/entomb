//
// Part of entomb
// Copyright (c) 2023 Sander in 't Veld
// License: MIT
//

mod wasm4;

use wasm4::*;

mod global_state;

use global_state::Wrapper;

mod lobby;
mod unlock;

use lobby::*;
use unlock::*;

static GAME: Wrapper<Game> = Wrapper::new(Game::Loading(0));

enum Game
{
	Loading(u64),
	Unlock(Unlock),
	Lobby(Lobby),
}

enum Transition
{
	Start
	{
		seed: u64
	},
	Unlocked
	{
		seed: u64
	},
}

#[no_mangle]
fn update()
{
	let game = GAME.get_mut();
	let transition = match game
	{
		Game::Loading(ticks) =>
		{
			*ticks += 1;
			if unsafe { *GAMEPAD1 } & BUTTON_1 != 0
			{
				Some(Transition::Start { seed: *ticks })
			}
			else
			{
				None
			}
		}
		Game::Unlock(unlock) => match unlock.update()
		{
			Some(seed) => Some(Transition::Unlocked { seed }),
			None => None,
		},
		Game::Lobby(lobby) =>
		{
			lobby.update();
			None
		}
	};
	match transition
	{
		Some(Transition::Start { seed }) =>
		{
			*game = Game::Unlock(Unlock::from_seed(seed));
		}
		Some(Transition::Unlocked { seed }) =>
		{
			*game = Game::Lobby(Lobby::from_seed(seed));
		}
		None => (),
	}

	match game
	{
		Game::Loading(ticks) => draw_loading_screen(*ticks),
		Game::Unlock(unlock) => unlock.draw(),
		Game::Lobby(lobby) => lobby.draw(),
	}
}

#[no_mangle]
fn start()
{
	const BLACK: u32 = 0x000000;
	const YELLOW: u32 = 0xffff02;
	unsafe {
		*PALETTE = [BLACK, YELLOW, YELLOW, YELLOW];
	}
}

fn draw_loading_screen(ticks: u64)
{
	if ticks < 35
	{
		unsafe { *DRAW_COLORS = 0x2 };
		text("Loading...", 8, 144);
	}
	else
	{
		unsafe { *DRAW_COLORS = 0x2 };
		text(b"Press \x80 to start.", 8, 144);
	}
}
