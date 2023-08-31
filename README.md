# House Of Cards

Twin stick shooter using Rust and Macroquad with the theme of cards

## Controls

- Movement: WASD or hold right mouse button
- Aim: arrow keys or mouse
- Shoot: left click or space
- Reload: R
    - Auto reload is enabled if you run out of cards
- Reset after death: R
- Pause: escape or p

## TODO

- 0.4.0
	- Player XP/levels
		- Show level bar under health bar
	- Put text like # / # for hp and xp bars
	- Power up selection
		- Better font colors
		- Can use mouse to select
	- Display current powerups on screen
	- Scale enemy movement speed based on wave
	- Score = xp
	- Improve gaining xp/score system
- Else
	- Level up: stat upgrade choice (damage, hp, reload speed, etc)
	- Once per wave: super enemy that drops chest that has a passive upgrade
	- Make chances in power up feel better
    - Improve mouse feel
	- Sound
	- Other screens
    - Other weapons?
- Maybe
	- Improve hitboxes?
	- Particles?
	- Spatial hash for collision detection?
	- Update world build locations less often?
	- Touch controls?
    - Art/animations for everything else

## Extra mechanics

- Damage
	- Joker => -5 damage
	- Face => 10 damage
	- Ace => One shot
	- Else => number value of card
- Score
	- 1 point per enemy killed

## Power ups

- Flat +1 damage to all cards
- +2 hp and increase max hp by 2
- 33% faster reload
- 5% movement speed buff
- Diamonds pierce one additional enemy
- Hearts have 5% chance to heal 1 hp (won't increase max hp)
- Clubs stun for 0.25 seconds
- Spades have 10% chance to double damage


+2 hp, max hp +2
33% quicker reload
+5% movement speed
Diamonds +1 enemy pierce
Hearts: 5% chance to +1 hp
Clubs: 0.25s stun
Spades: 10% chance double damage

## Credits

- Pixel Art Cards: https://www.reddit.com/r/PixelArt/comments/i1t1gn/pixel_art_playing_card_designs_created_in_aseprite/
- Math for rotated rectangle hitbox: https://chat.openai.com/share/f0826594-c5e9-4ea6-a1cf-0a010295fbfa
- Font: https://fonts.google.com/specimen/Annie+Use+Your+Telescope
