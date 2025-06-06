# BombParty CLI
This project aims to implement all features of bombparty from Sparklin Labs / https://jklm.fun (or soon https://croco.games).

## Roadmap:
- [x] Obtain a full dictionary (this is done, however it is not the official one. it is a combination of the english_450k dict from [monkeytypegame/monkeytype](https://github.com/monkeytypegame/monkeytype) and all of the dictionaries in [NachozQ/BombParty-Lists](https://github.com/NachozQ/BombParty-Lists). This means there are more words in here than in the original bombparty, and it includes proper nouns.)
- [x] Parse words into syllables
- [ ] Main game:
  - [x] Show syllable prompt
  - [x] Accept and verify answer
  - [x] Show alphabet board
  - [ ] Display lives
  - [ ] Use `crossterm` to show a tui
  - [ ] Make everything uppercase to have that bombparty vibe
  - [ ] Highlight the first instance of the syllable in the word
  - [ ] Multiplayer (I have no clue if this will ever be implemented, very low possibility.)

## How to build/run (and even install!) bombparty-cli

to just install:
```console
cargo install https://github.com/TurtleMeds/bombparty-cli
```
and just run with `bombparty`.

to build and play around with the source code:
```console
git clone https://github.com/TurtleMeds/bombparty-cli
cd bombparty-cli
cargo build # or run cargo run to run it directly
```
