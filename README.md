# House Of Cards

Unleash Chaos in this card themed casual bullet hell roguelike twin stick shooter for mobile, desktop, and the web!

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

- Rework card system
    - Jokers need a redesign (right now would never pick them)
        - 50% chance to 
    - You have a hand of 5 cards
    - Instead of a stat buff card card, you get the option of replacing a card in your hand
        - Random choice of 3 cards
    - You have 1 active card which is what you are shooting
        - Have their own independent cooldowns (fire rate = 1 / cooldown)
            - Joker: 10 seconds     (0.1)
            - Face: 5 second        (0.2)
            - Ace: 1 second         (1.0)
            - Else: 0.2 seconds     (5.0)
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
	- Joker => -5 damage
	- Face => 10 damage
	- Ace => One shot
	- Else => number value of card
- Score and XP
	- 1 per enemy killed
- There is a movement speed penalty while shooting and reloading

### Powerups

Powerups can stack and are calculated independently (even for 2 of the same time of powerup).
You get a stat buff card every level up and a card buff every time you kill a super enemy.

#### Stat Buffs

- Red: Flat +1 damage to all cards
- Green: +2 hp and increase max hp by 2
- Orange: 5% faster fire rate
- Purple: 5% movement speed buff

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