# House Of Cards

Twin stick shooter using Rust and Macroquad with the theme of cards

## Controls

- Movement: WASD or hold right mouse button
- Aim: arrow keys or mouse
- Shoot: left click or space
- Reload: R

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
    - Converted to Nord palette using: https://ign.schrodinger-hat.it/