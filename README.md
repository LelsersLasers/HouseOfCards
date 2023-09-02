# House Of Cards

Twin stick shooter using Rust and Macroquad with the theme of cards, works on mobile and desktop without a download!

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
- Toggle auto shoot: Q (off by default)
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

- 0.12.x
    - Load screen on web
        - Display `landscape please` message on mobile?
- 0.13.0
    - Increase firerate instead of reload speed powerup
- Else
    - Tweak
        - XP and score system
        - Scaling system and numbers
        - Super enemy numbers and mechanics
    - Improve aim feel (mouse, arrow keys, joystick)
    - Purpose
        - Trying to get somewhere?
- itch.io page
    - Write up a short summary
    - Add screenshots + icon pic
    - Finish typing out extra mechanics

## Extra mechanics

- Damage
	- Joker => -5 damage
	- Face => 10 damage
	- Ace => One shot
	- Else => number value of card
- Score and XP
	- 1 per enemy killed
- There is a movement speed penalty while shooting and reloading

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
- Font: https://fonts.google.com/specimen/Assistant
- Music: https://www.youtube.com/watch?v=ROcups0YaHE