# House Of Cards

Twin stick shooter using Rust and Macroquad with the theme of cards

> Unleash Chaos in this Casual Bullet Hell Rougelike Twin Stick Shooter!
> -ChatGPT

## Controls

- Movement: WASD or hold right mouse button
- Aim: arrow keys or mouse
- Shoot: left click or space
- Reload: R
- Reset after death: R
- Pause: escape or p
- Choose powerup: 1, 2, 3 or left click
- Toggle auto shoot: F (off by default)
- Toggle auto reload: T (on by default)

## Mobile Controls

- Note: might need auto rotate on (game works best in landscape mode)
- Movement: left screen joystick
- Aim: right screen joystick
- Reload: touch deck icons in top right (auto reload enabled)
- Pause: touch top left corner of screen (touch anywhere to unpause)
- Choose powerup: touch powerup
- Reset after death: touch screen anywhere

## TODO

- 0.7.x
    - `F` (or some key) to toggle auto shoot
    - `T` (or some key) to toggle auto reload
- 0.8.0
    - Make a system that detects if on mobile and differentiate on that
        - For pause menu:
            - Display current status on toggles in pause menu
        - Draw default joystick if touch
- Else
    - Play nice with alt-tab
    - Better font
        - And make it visible on mobile
    - Tweak
        - XP and score system
        - Scaling system and numbers
        - Super enemy numbers and mechanics
    - Improve aim feel (mouse, arrow keys, joystick)
	- Sound
- Maybe
    - Draw default joystick if no touch
        - Only draw on mobile!
    - Other screens?
        - Show settings on pause menu?
    - Other weapons?
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
