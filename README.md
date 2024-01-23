# House Of Cards

Survive the chaos in this card themed casual bullet hell roguelike twin stick shooter for mobile, desktop, and the web!

## Controls

- Movement: WASD or hold right mouse button
- Aim: arrow keys or mouse
- Shoot: left click or space
- Switch between active cards: 1, 2, or 3
- Reset after death: R
- Pause: escape or p
- Choose powerup/card: 8, 9, 0 or click on powerup/card
- Swap: Enter (or click button)
- Discard all: backspace/delete or click button
- Toggle auto shoot: Q (off by default)
- Toggle music: M (on by default)

## Mobile Controls

- Note: might need auto rotate on (game works best in landscape mode)
- Movement: left screen joystick
- Aim: right screen joystick
- Switch between active cards: touch card
- Pause: touch top left corner of screen (touch anywhere to unpause)
- Choose powerup/card: touch powerup/card
- Swap: touch swap button
- Discard all: touch discard all button
- Reset after death: touch screen anywhere
- Toggle music: button in pause menu

## Extra mechanics

- Damage
	- Joker => 50% chance to one shot (8 sec)
	- Ace => one shot (20 sec)
	- Face => 2 * the "number" value of the card (0.5 sec)
	- Else => number value of card (0.2 sec)
- Score and XP
	- 1 per enemy killed
- There is a movement speed penalty while shooting

### Powerups

Powerups can stack and are calculated independently (even for 2 of the same time of powerup).
You get a choice of a new card every level up and a card buff every time you kill a super enemy.

#### Card Buffs

- Blue 1: Diamonds pierce one additional enemy
- Blue 2: Hearts have 2% chance to heal 1 hp (won't increase max hp)
- Blue 3: Clubs stun for 0.1 seconds
- Blue 4: Spades have 33% chance to double damage

## Credits

- Pixel Art Cards: https://www.reddit.com/r/PixelArt/comments/i1t1gn/pixel_art_playing_card_designs_created_in_aseprite/
- Pixel Art Chess Pieces: https://spheya.artstation.com/projects/QnaVO3
- Math for rotated rectangle hitbox: https://chat.openai.com/share/f0826594-c5e9-4ea6-a1cf-0a010295fbfa
- Font: https://fonts.google.com/specimen/Assistant
- Music: https://www.youtube.com/watch?v=ROcups0YaHE

## TODO

- Main
    - Textures
        - Sizing info????
            - Might be different per piece :(
            - 26 top of image to bottom px
            - 10 top of image to top px (10 blank)
            - 28 wide
        - Enemies are chess peices
        - Player might be the king (opposite color, keep the direction arrow)
        - Mask like collsions?
    - Damage numbers
    - 3 card hand vs 2 card hand
- Feedback
    - Enemy variance (comes in swarms, moves slower/harder hits)
    - More generally fun mechanics
    - Alternate ways of healing?
    - Some mechanic to add together cards of the same suit?
    - Joker should be the strongest card
- Balancing 
    - Suit buffs (clubs/hearts op)
    - Super enemies fire rate gets too high
    - Enemies start too slow then get too fast?
- Tweak
    - XP and score system
    - Scaling system and numbers
    - Super enemy numbers and mechanics
- Improve aim feel (mouse, arrow keys, joystick)
    - Issue: mouse is relative to player position (really camera posistion) not world position
        - Like locked camera in League
- Loading time/screen on web
    - Either calculate the loading time for the sound file or ?
- Purpose?
    - Trying to get somewhere?
    - Trying to protect something?
    - Should make you move around dynamically
        - Instead of a straight line
