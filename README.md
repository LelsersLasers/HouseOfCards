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
- Choose powerup: 1, 2, 3 or left click

## TODO

- 0.6.x
	- Touch controls
        - Reload
        - Pause
    - `O` (or some key) to toggle auto shoot
    - `I` (or some key) to toggle auto reload
- Else
    - Tweak
        - XP and score system
        - Scaling system and numbers
        - Super enemy numbers and mechanics
    - Play nice with alt-tab
    - Improve aim feel (mouse, arrow keys, joystick)
	- Sound
	- Other screens?
        - Show settings on pause menu?
    - Other weapons?
    - Mobile controls work with resizing scale?
        - Is this needed?
- Maybe
	- Improve hitboxes?
	- Particles?
    - Damage numbers?
	- Spatial hash for collision detection?
	- Update world build locations less often?
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

## Credits

- Pixel Art Cards: https://www.reddit.com/r/PixelArt/comments/i1t1gn/pixel_art_playing_card_designs_created_in_aseprite/
- Math for rotated rectangle hitbox: https://chat.openai.com/share/f0826594-c5e9-4ea6-a1cf-0a010295fbfa
- Font: https://fonts.google.com/specimen/Annie+Use+Your+Telescope
