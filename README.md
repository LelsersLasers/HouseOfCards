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

## TODO

- Main
    - Update player bars in death
    - UI updates
    - New enemy mechanic
    - Fix level up + super enemy death on same time
    - BALANCING
    - 3 card hand vs 2 card hand (vs 5 card hand)
- UI
    - Make PowerupPick rectangles the same size as the ChooseCard cards
    - Use `mq::Rect`s for the PowerupPick interactions
- New enemy mechanic
    - 1 of every ~10 enemies tries to predict player movement instead of slowly walking at the player
- Balance suit buffs (clubs/hearts op)
- Tweak
    - XP and score system
    - Scaling system and numbers
    - Super enemy numbers and mechanics
- Improve aim feel (mouse, arrow keys, joystick)
    - Issue: mouse is relative to player position (really camera posistion) not world position
        - Like locked camera in League
- Loading time
    - Either calculate the loading time for the sound file or ?
- Purpose?
    - Trying to get somewhere?
    - Trying to protect something?
    - Should make you move around dynamically
- Update itch.io page
    - Write up a short summary
    - Add screenshots + icon pic

## Extra mechanics

- Damage
	- Joker => 50% chance to one shot (8 sec)
	- Ace => one shot (20 sec)
	- Face => 20 damage (0.5 sec)
	- Else => number value of card (0.2 sec)
- Score and XP
	- 1 per enemy killed
- There is a movement speed penalty while shooting

### Powerups

Powerups can stack and are calculated independently (even for 2 of the same time of powerup).
You get a choice of a new card every level up and a card buff every time you kill a super enemy.

#### Card Buffs

- Blue 1: Diamonds pierce one additional enemy
- Blue 2: Hearts have 5% chance to heal 1 hp (won't increase max hp)
- Blue 3: Clubs stun for 0.25 seconds
- Blue 4: Spades have 20% chance to double damage


## Credits

- Pixel Art Cards: https://www.reddit.com/r/PixelArt/comments/i1t1gn/pixel_art_playing_card_designs_created_in_aseprite/
- Math for rotated rectangle hitbox: https://chat.openai.com/share/f0826594-c5e9-4ea6-a1cf-0a010295fbfa
- Font: https://fonts.google.com/specimen/Assistant
- Music: https://www.youtube.com/watch?v=ROcups0YaHE