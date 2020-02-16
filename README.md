# Rust-Nonogram
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Sundwalltanner/Rust-Nonogram/blob/master/LICENSE)

Nonograms, sometimes referred to as Picross, are a type of picture-based numbers puzzle. Basically, you're given a grid with some numbers to the side of each row and column and you have to use that information to determine which boxes in the grid are filled in. Typically the end result is that the filled in blocks form the shape of something. That end result can be in black and white or in color based on the ruleset. [Look, here's a Wikipedia page about nonograms](https://en.wikipedia.org/wiki/Nonogram).

## Why?
I've been bored riding the MAX for about two hours every day I have class, so I was looking for an entertaining Nintendo Switch game to play. I ended up stumbling across Picross (get it?). A video game developer named Jupiter has made like 100 different Picross games, most of which are exclusive to the Nintendo 3DS. I couldn't ever really get into Sudoku, so I don't really know how similar the two are. It's incredibly addicting though, and it seemed like something I'd be capable of creating in Rust.

## Early Access
This is currently in development. In the meantime, here's a gif of it running:

![Gif of program running #1](https://i.imgur.com/MrIILoa.gif)

And here's an image of what it currently looks like when you start it, proving that it's generating the numbers for the columns and rows based on the randomly generated solution:

![Image of program running #2](https://i.imgur.com/DjNZi3A.png)

Now that I've redone the data structures holding the board and its cells together, I can resize it by altering the ```dimensions``` variable in [nonogram_board.rs](https://github.com/Sundwalltanner/Rust-Nonogram/blob/master/src/nonogram_board.rs). Ultimately, I want this to be capable of being modified by the user using a drop down menu or something. And they'll hopefully be capable of generating a new board as well with a big "generate" button or something. Anyways, here's a picture of a solved 30x30 nonogram. It didn't take me 13 seconds to solve. I just didn't have it wiped after it generated a random goal state.

![Image of program running #3](https://i.imgur.com/iIO3wDv.png)

## What are you still working on?
This isn't due for a few more weeks, so I've probably got plenty of time to hit these stretch goals:

* Most nonogram games have a feature that crosses out the numbers written on sides of the board dynamically based on whether or not the game logic thinks where you placed that filled in box is the only place it can be inserted. This is relatively complicated, so I've been holding off on trying to implement this.
* Check win state. Currently, the game doesn't even check to see if you've completed it. All I really need to do is compare 2 3D arrays. I've already got most of the code implemented. I just need to figure out what I want to do with the game when the player has won. I need to add a screen that pops up, informs the player that they've won, tell them the amount of time it took them, and then indicate some sort of keybinding they must hit in order to restart, at which point I need to have the game restart everything. It needs to restart the timer, the black counter, and generate a new board.
* Wipe the goal state. Currently as a means of debugging, the goal state isn't wiped. That means that when the program is executed, the player doesn't see a clean board. Instead, they see the randomly generated goal state.
* Potentially look into generating only unique goal states? This didn't end up being a huge issue, because of the method I'm using to generate the goal state. It will be easy enough to compare the current state to the goal state for win conditions. But it might be worthwhile to look into an algorithm for generating nonogram puzzles with unique goal states, as mine currently has the ability to produce a goal state that can be reached by more than one method.
* Currently, most aspects of this are coded in a way that doesn't allow one to easily alter the size of the board. I would like the player to be able to choose between 5x5, 5x10, 10x10, 10x15, 15x15, etc... board sizes like in most nonogram implementations.
* Tests, better comments, make Rust happy, make Clippy happy, etc... There's always stuff to work on...
