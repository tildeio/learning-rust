# Learning Rust via Ruby

### A text-based adventure game being built in Rust and Ruby (simultaneously)

If you want to play it:

1. clone this repository
2. cd into it (i.e. enter `cd learning-rust` in your terminal)
3. for the Ruby version: enter `ruby lib/main.rb` in your terminal and hit return. For the Rust version: enter `cargo run` in your terminal and hit return
4. play :)

The actual narrative still needs to be ported over & some prettifying still needs to happen, but you can play a super simplistic placeholder game right now. Both games should basically work at this point, but please submit an issue if you notice something's broken! The Ruby side seems to be especially fragile.

Built mostly by @lizbaillie with help from @wycats

TODO:
- [ ] Ruby `Game.take` and `Player.remove_from_inventory` methods don't break anymore, but NPC/Player inventory item is not removed (should be removed). This bug allows players to keep taking the same items from the same NPCs and using them over and over. Not the end of the world, but not great.
- [ ] Make the map in both games an actual map and not just a list of room names.
- [ ] Format the Rust stuff more neatly. Currently for things like `look around` we kind of just spit out the available objects and it's pretty ugly.
- [ ] Add an actual story and not just a bunch of weird rooms :P
- [ ] figure out how to test both Rust and Ruby games
