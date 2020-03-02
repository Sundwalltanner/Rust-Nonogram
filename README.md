# Rust-Nonogram
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Sundwalltanner/Rust-Nonogram/blob/master/LICENSE)

First and foremost, this is being developed and tested in Windows. If it doesn't work on another platform, I'm sorry. At the very least, I'll try to get around to testing it in Linux. It should work fine, but I can't be certain until I've tried, and I haven't.

Nonograms, sometimes referred to as Picross (I don't know what the plural form of this is), are a type of picture-based numbers puzzle. Basically, you're given a grid with some numbers to the side of each row and column and you have to use that information to determine which boxes in the grid are filled in. Typically the end result is that the filled in boxes form the shape of something. That end result can be in black and white or in color based on the ruleset. [Look, here's a Wikipedia page about nonograms](https://en.wikipedia.org/wiki/Nonogram). Mine only uses the black and white ruleset for now.

## Why?
A video game developer named Jupiter has made like 100 different Picross games, most of which are exclusive to the Nintendo 3DS. I got addicted to [Picross S3](https://www.nintendo.com/games/detail/picross-s3-switch/) for the Nintendo Switch this term, so I wanted to try and make that in Rust.

## How to install
First, install Rust by following the directions found here:
https://www.rust-lang.org/tools/install

Once you've got Rust properly installed, clone this repository, navigate to its directory on your machine, and enter this into your terminal:

```
cargo run
```

This will install the packages located in the ```Cargo.toml``` file, create an executable out of the files in the ```src``` folder, and execute it. It might take a while to install the necessary packages. Piston is rather hefty.

Once you've done all that, you're ready to learn how to play.

## Controls
These are the default controls. They can be easily manipulated by the user, and there will soon be a menu where the user can change the keybindings from within the game.

The mouse controls are fairly basic:
* ```Left Mouse Click``` - Fill box if clear. Clear box if not clear. Interact with buttons.
* ```Right Mouse Click``` - Mark box if clear. Clear box if not clear.

These two mouse buttons can also be held down in order to fill/clear multiple boxes. They will maintain the same command that was executed on the first box for convenience. This means that if you press the left mouse button and your cursor is hovering over a cleared box, as long as you keep that left mouse button held down, any other cleared box your cursor hovers over will be filled. Anything besides cleared boxes won't be altered.

This also means that commands are executed on mouse button down rather than on mouse button release.

There's also optional keyboard controls:
* ```WASD``` - Move to a different box on the board.
* ```J``` - Fill box if clear. Clear box if not clear.
* ```K``` - Mark box if clear. Clear box if not clear.
* ```R``` - Restart. Generates a new game board. If the board dimensions have been altered, the new board will have those dimensions.
* ```Up Arrow``` - Increase board dimensions. Press ```R``` to generate a new board with these dimensions.
* ```Down Arrow``` - Decrease board dimensions. Press ```R``` to generate a new board with these dimensions.

Marking boxes is simply for player convenience. Marked boxes are supposed to indicate which boxes you believe are definitely clear in the solution. You can reach the win condition without marking a single box as long as you have the correct boxes filled.

This probably sounds confusing, and I'm sorry. It makes more sense when you play it. I borrowed this control setup from the Picross games on the Nintendo Switch, because they're my favorite version of this puzzle. There are numerous versions of this online, but so far I haven't really found one that I like nearly as much as the Nintendo Switch games.

## How to play
There are numbers near each column and row of the board. These numbers indicate sequences of filled in blocks. For example, take a look at the image below:

![Example image of 5x5 solution](https://i.imgur.com/YRhhWxf.png)

This is the final solution to this particular Nonogram. From glancing at this, we can see that sequences of filled in boxes maintain the same order given by the order of the numbers nearby. We can also see that sequences of filled in boxes need at least one white box of separation. In order to assist you in remembering which boxes cannot be filled in, you can ```right click``` a box in order to mark it with an orange-ish X (cross). You don't need to mark any boxes in order to win. They're just there to help you.

The win condition is based entirely on which boxes you've filled in. Every time you fill or clear a box, the game checks to see if your board state matches that of the winning board. This takes into account the fact that some boards might not have a unique solution.

The progress tracker doesn't indicate correctness. You can fill an incorrect box, and it will still add that box to your progress.

You'll notice as you progress that the hint numbers will automatically change to a darker color. This indicates that the game thinks that particular sequence is taken care of. This exists in order to assist the player so they don't have to remember which sequences they've completed. The code which is doing this is very poor at the moment. Sometimes it can be very misleading. I'm sorry. I'll try to fix it, but I can't wrap my head around how to get this working properly. The image below shows this feature in action:

![Dynamic hint number crossout](https://i.imgur.com/UtBIlRv.png)

When you win, a screen like this will pop up. This UI is subject to change:

![Win screen](https://i.imgur.com/1VoIUzt.png)

As seen in the win screen image above, the final image of the board is shown to the user along with a randomly generated comment about how my version of Picross doesn't actually use real pictures of anything, some stats about the previous game, and a button at the bottom that allows the user to start a new game.

Here's a gif of me solving a basic 5x5 Nonogram from start to finish:

![Gif of solving 5x5 Nonogram](https://i.imgur.com/wxxDn44.gif)

## Save progress
Progress is automatically saved to a file named ```savedata.json``` in the main directory for this project whenever the program is exited. This includes pressing the ```X``` in the top right corner of the window, ```ALT+F4'ing```, killing it with task manager, etc...

Eventually this will keep track of wins or something, but for now, it just keeps track of current progress.

Currently, if I make any updates to what is tracked within savedata.json, if you have an older version of that file, the program will crash and inform you that you need to delete your savedata.json file. I'll try and figure out a way to get around this, but for now, that's just the way it is.

## What are you still working on?
This isn't due for a few more weeks, so I've probably got plenty of time to hit these stretch goals:

* Fix save state logic. As mentioned above, if a new variable is saved, any old save data will cause the program to crash, and that old save data will need to be deleted. This isn't ideal. Either the program should overwrite the old save data, or we should be capable of at least keeping the save data that's compatible, and trying to concatenate the new variable that is now being saved.
* Pause menu with buttons:
    * Resume - Resume the game in progress.
    * How to Play - Tries to tell the user how to play the game.
    * Edit Keybindings - Tells the user what the current keybindings are and allows them to rebind actions to different keys.
    * Quit - Exit the game.
* Adapt locations and sizes of everything to window size. Right now everything is static.
* Potentially look into generating only unique goal states? This didn't end up being a huge issue, because of the method I'm using to generate the goal state. It will be easy enough to compare the current state to the goal state for win conditions. But it might be worthwhile to look into an algorithm for generating nonogram puzzles with unique goal states, as mine currently has the ability to produce a goal state that can be reached by more than one method.
* Tests, better comments, make Rust happy, make Clippy happy, etc... There's always stuff to work on...

## Other future goals
These are significantly less likely to be reached by the due date of this project:

* Allow image files to be read in and converted into a board goal state.
* Allow the user to play the color version as well. The ruleset of this is that there's basically more colors than black and white, hint numbers indicate color, and segments of different colors don't need whitespace to separate them. There's a lot more to it than that, and it would require quite a bit of work.

## References
These are URLs dropped in throughout the code's comments, gathered in one place for my convenience:

* Material icons: https://material.io/resources/icons/?style=baseline
* Unicode character codes: https://github.com/google/material-design-icons/blob/master/iconfont/codepoints
* Hex code color transparency: https://css-tricks.com/8-digit-hex-codes/
* Piston input ```GenericEvent``` traits: https://docs.rs/piston/0.49.0/piston/index.html#traits