# Ghostblade: Shinobi Gauntlet
Ghostblade is a command-line maze adventure game written in Rust

![Game UI](screenshots/game-ui.png)

## About
Ghostblade is a stealth-based maze crawler where you play as a ninja (Ghostblade) infiltrating a heavily guarded castle. Navigate through increasingly complex levels, avoid patrols, and use various tools and mechanics to reach the objective.

## Features
- 10 handcrafted levels with increasing difficulty
- Command-line interface with ASCII/Emoji graphics
- Multiple maze mechanics:
    - Patrol guards with different movement patterns
    - Locked doors and keys
    - Hidden paths
    - Various environmental interactions

## Installation
```bash
# Clone the repository
git clone https://github.com/planetaska/ghostblade.git

# Build the project
cd ghostblade
cargo build --release

# Run the game
cargo run --release
```

## Terminal Unicode Width Concerns
Emojis may appear as half-width due to Unicode classifications, terminal behavior, or font rendering differences. To ensure consistent display, this program uses the Zero Width Space (`\u{200B}`) to adjust emoji width for certain terminals and fonts. In most cases, this should not cause issues.
However, if the screen appears misaligned, try changing the terminal font for better compatibility.

## How to Play
- Use `WASD` keys to move
- `Q` to quit the game

### Map Legend

*showing only a few examples

```
🥷 - Player
🌲 - Wall
・ - Empty space
🏯 - Goal
🧌 - Patrol guard
🗝️ - Key
🚪 - Door
🪝 - Hook
⚓️ - Hook point
🪓 - Axe
🪵 - Woodlog for building Canoe 🛶
```

## Development
The game is written in Rust and uses simple text files for map layouts.

Each map is defined in a `.txt` file in the `maps/` directory.

### Map Format
```
# Example map file
ttttttttttt
tsssssbsdst
tssssssesst
tsbsssssset
tssssssbsst
tssbsssbsst
tspsbssbsst
ttttttttttt
```

### Map Customization

You can modify existing maps or create new ones by editing or adding a map file.
If you add a new map, be sure to update the `max_levels` value in `game.rb`.

## Contributing
This is a student study project. However, if you'd like to create your own version of the game, please feel free to fork this repository.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- Inspired by classic games like Legend of Zelda and Crystalis
- Originally prototyped in Ruby during university studies
- Special thanks to my professor Barton Massey

## Roadmap
- [x] Basic movement and collision
- [x] Level loading system
- [x] Patrol AI
- [x] Key/door mechanics
- [x] Grappling hook system
- [x] Bridge mechanics
- [x] 10 complete levels
- [ ] ~~Map editor~~ (stretch goal)

## Challenges and Obstacles
While I was able to implement all the planned features from the roadmap, I didn’t have enough time to tackle the stretch goal - something I've always wanted to create. Having a base prototype in Rails was incredibly helpful, as it provided a clear guideline for what to work on next. Initially, I struggled with Rust’s various string types and traits, but over time, I got used to them. The strict type safety was frustrating at first, but I now appreciate the reliability it brings.

Another challenge was handling terminal display in Rust. The standard library offers basic tools, but doing anything more advanced quickly required building a lot from scratch. To speed things up, I used the `crossterm` crate. However, working with cross-platform terminal functionality introduced its own challenges, requiring careful design and specific syntax to ensure compatibility. Despite the hurdles, I eventually got it working and truly appreciate the author for creating this crate.

One particularly interesting (or frustrating) issue I encountered was Unicode width. Unicode characters are classified into different width categories: W (Wide), F (Fullwidth), A (Ambiguous), H (Halfwidth), and N (Neutral). Most emojis fall under the Ambiguous category, meaning their width isn’t strictly defined - they can sometimes take up one column and other times two. This creates a big issue for emoji-heavy terminal-based programs like Ghostblade.

I initially considered using the `unicode_width` crate to handle this, but it determines character width based on Unicode classification rather than the actual behavior of the terminal. After extensive trial and error, I settled on using a Zero Width Space (`U+200B`) as a workaround. However, this solution isn’t bulletproof, as some terminals may still render characters inconsistently. Due to time constraints, I had to leave it as-is and hope that most major terminals handle it correctly.

## Reflections and Future Plans

Overall, I am very satisfied with the result and impressed by Rust as a language. It strikes a great balance between performance, safety, and development experience. Rust has become one of my favorite languages to work with - second only to Ruby, which remains unbeatable in terms of ease of use. For projects that demand strict type safety though, I would choose Rust without hesitation.

For future improvements, I plan to develop a map editor for the game. I already have a vision for it, and with more time, I am confident I can complete it. I have designed the program to be modular and extendable, making it easier to add new features down the line. If time permits, I would also love to port the game to a real game engine and take it even further.

## Author

Chia-Wei Hsu

### Instructor

Professor Dr. Barton Massey

## Contact

- GitHub: [@planetaska](https://github.com/planetaska)
- Email: planetaska@gmail.com

