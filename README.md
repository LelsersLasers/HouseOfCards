# House Of Cards

Twin stick shooter using Rust and Macroquad with the theme of cards

## Controls

- Movement: WASD or hold right mouse button
- Aim: arrow keys or mouse
- Shoot: left click or space
- Reload: R
- Reset after death: R
- Pause: escape or p

## TODO

- 0.3.0
    - Waveless spawn system
- Else
    - Improve mouse feel
	- Improve hitboxes
	- Player XP/levels
	- Power up selection
	- Sound
	- Other screens
    - Other weapons
- Maybe
	- Particles?
	- Spatial hash for collision detection?
	- Update world build locations less often?
	- Touch controls?
    - Art/animations for everything else

## Extra mechanics

- End of wave
	- Gain 1 hp
	- If would increase max hp, increase max hp to new current hp
- Damage
	- Joker => -5 damage
	- Face => 10 damage
	- Ace => One shot
	- Else => number value of card
- Score
	- 1 point per enemy killed
	- (Wave number)^2 points at the end of the wave

## Planning

- World
	- Floor is colored tiles
		- Red-Purple: floor
		- Black: walls
	- Endless enemies
- Player:
	- WASD to move
	- Mouse to aim
		- Left click to shoot
	- Or use arrow keys to aim
		- Space to shoot
	- Health
	- Weapon
		- Rifle, shotgun, sniper
	- Ammo: deck of cards
		- Damage: number on card
	- XP/levels
		- Power ups: agument cards
			- Ex: +1 damage to all cards, diamond cards piece, etc
- Window scaling
	- Fills entire window
	- Scaling based on largest square that can fit in window
- Pages:
	- Main menu
		- Play
		- Settings
		- Quit
	- Settings
		- ?
	- Game

## Credits

- Pixel Art Cards: https://www.reddit.com/r/PixelArt/comments/i1t1gn/pixel_art_playing_card_designs_created_in_aseprite/
- Math for rotated rectangle hitbox: https://chat.openai.com/share/f0826594-c5e9-4ea6-a1cf-0a010295fbfa
- Font: https://fonts.google.com/specimen/Annie+Use+Your+Telescope
