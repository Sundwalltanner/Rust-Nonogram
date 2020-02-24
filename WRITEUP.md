## What was built
For my CS 410P Rust Winter 2020 final project at Portland State University, I've built a Nonogram/Picross game purely in the Rust programming language.

Nonograms, sometimes referred to as Picross, are a type of picture-based numbers puzzle. Basically, you're given a grid with some numbers to the side of each row and column and you have to use that information to determine which boxes in the grid are filled in. Typically the end result is that the filled in blocks form the shape of something. That end result can be in black and white or in color based on the ruleset. [Look, here's a Wikipedia page about nonograms](https://en.wikipedia.org/wiki/Nonogram).

Currently, this program features:
* Randomly generated Nonogram puzzles of varying dimensions.
* Dropdown menu allowing user to change dimensions of generated board.
* Button allowing user to generate a new board.
* Various keybinding functions:
    * ```Left click``` mouse to fill boxes and interact with buttons.
        * Support for ```left click hold``` to dynamically perform functions based on first box interacted with.
    * ```Right click``` mouse to mark boxes.
        * Support for ```right click hold``` to dynamically perform functions based on first box interacted with.
    * ```Up key``` to increase dimensions.
    * ```Down key``` to decrease dimensions.
    * ```R key``` to restart and generate a new board.
* Stats screen upon puzzle completion with option to start again.
    * In-game timer, with final completion time displayed at the end.
    * Puzzle complexity parameters such as filled / unfilled ratio.
* Progress automatically saved in ```savedata.json``` file when program is exited.

## Related work
I intentionally avoided Googling ```Rust Nonogram``` until hitting the very end of this project, and even then, I basically only found Nonogram solvers. I largely based the UI of my program off of [Jupiter's](https://en.wikipedia.org/wiki/Jupiter_Corporation) [Picross S3 on the Nintendo Switch](https://www.nintendo.com/games/detail/picross-s3-switch/), as I've put nearly 30 hours into that game during this term alone and have found it to be the best implementation of Picross so far.

There's a bunch of web-based implementations that I looked at:
* [Picross by a dude who works at Google](http://liouh.com/picross/)
    * Unique click and hold functionality. Not really a fan.
    * By default, capable of randomly generating a puzzle without a unique solution, yet checks for the unique solution, so if you don't give the solution it wants, it tells you that you're wrong.
    * The ```allow multiple solutions``` optional mode doesn't automatically check to see if your solution is correct.
    * I really like the seed approach, and wanted to implement that into mine, but Piston doesn't make it easy to create an input field.
* [Hanjie Star Picross](https://www.hanjie-star.com/)
    * Really similar to mine and the Nintendo Switch game's approach.
* [puzzle-nonograms.com](https://www.puzzle-nonograms.com/)
    * Doesn't dynamically determine what a press and hold should do. If you press and hold left click or right click, it overwrites everything. Really annoying design.
    * Doesn't crossout numbers. Doesn't even have the basic functionality to crossout a line you've completed.
    * Doesn't automatically detect a win. You need to click a button to indicate you want to submit your answer.
* [nonograms.org](https://www.nonograms.org/)
    * Capable of taking a GIF, PNG, BMP, JAC, JCD, or JMP and converting it into a black and white or color Nonogram puzzle. Each individual pixel would correspond to one square of the goal in the puzzle. Color Nonograms deal with slightly different rules. Mine deals with black and white.

I also took a look at some free mobile implementations:
* [Konami's Pixel Puzzle Collection](https://play.google.com/store/apps/details?id=jp.konami.mo.pvt.aww)
    * Really well made. But it tells me that this isn't a game that works all that well on mobile. Needing to press a button to select a different input type is annoying, but it's the only solution for this platform.
* [Hungry Cat Picross](https://play.google.com/store/apps/details?id=com.tuesdayquest.logicart&hl=en_US)
    * This isn't even Picross.

Finally, I've been looking through some of [the more popular Github repositories under the topic of Nonogram](https://github.com/topics/nonogram):
* [HandsomeOne's Nonogram editor and solver w/ no dependencies](https://github.com/HandsomeOne/Nonogram)
    * This one's really fun to play with. No need to clone it. There's [a github.io where you can mess with it](https://handsomeone.github.io/Nonogram/).
    * It's cool to see the process the solver goes through as it tries to solve a puzzle, though it's unfortunate that as far as I can tell, this doesn't deal with the creation of nonogram puzzles with unique solutions, as the solver gets stumped by anything larger than a 10x10 puzzle. The algorithm that's attempting to solve these puzzles is solving them using the same methods that a human would use. I don't know if there's a better approach, but this leads to getting stuck somewhere where a human would normally have to guess. It also gets stuck on the last step if there is more than one possible solution.
* [Izaron's very fast Japan crosswords solver and generator](https://github.com/Izaron/Nonograms)
    * This one comes bundled with [an incredibly useful post about his process](https://izaron.github.io/post/solving-colored-japanese-crosswords-with-the-speed-of-light/). I'm not concerned with something so heavily focused on solving Nonograms, but they touch upon the fact that almost all the puzzles in the post were generated by the code, as it's capable of taking an image and producing a Nonogram puzzle out of it. I've seen a couple examples capable of doing this, and would have loved to attempt it myself.

## How it works
From the beginning, I was looking for a game engine in Rust that was both easy to learn, and would provide me with exactly what I needed in order to build a Nonogram game. I ended up choosing [Piston](https://www.piston.rs/) after taking a look at it, [Amethyst](https://amethyst.rs/), and [ggez](https://ggez.rs/). After recently realizing the game is basically just a GUI, I regret not just using something like [Conrod](https://docs.rs/conrod/0.61.1/conrod/).

So basically, in [main.rs](https://github.com/Sundwalltanner/Rust-Nonogram/blob/master/src/main.rs) there's a ```while``` loop that's constantly wiping the screen, drawing to it, and detecting input as long as the program is the active window. This ends up leading to noticeable problems such as the timer not visually updating if the window isn't active. It keeps track of the time correctly, and will jump ahead when the window is reactivated. It's just a weird problem I wasn't able to fix due to the way Piston works.

I'm also using the following dependencies:
* [rand](https://docs.rs/rand/0.7.3/rand/)
    * Bernoulli distribution for generating a puzzle. Travels through cells of grid, determining on an individual cell basis whether or not it's going to be filled in based on a probability that's currently set to 0.5. So there's a 50% chance that a cell will be filled. This is just about the lowest this can be set in order to generate solvable puzzles. I'm thinking of setting it a bit higher though.
    * Less important than the puzzle generation, but upon puzzle completion, the final solution is displayed for the user, and a randomly generated critique of the image is displayed beneath it. This is a comment on what this type of game is supposed to be. The reasoning behind the game's name is that the filled in squares are supposed to produce an image of something, whereas my game just generates them randomly and the odds that they'll produce something resembling anything are very low.
* [Serde](https://serde.rs/)
    * Player progress is saved as a JSON file, and Serde is basically the most recommended method to read and write information in this format using Rust. [serde_json](https://docs.serde.rs/serde_json/) alone is enough to write the file, but in order to interpret the information in the file and read it back out efficiently, I had to use [Serde's derive feature](https://serde.rs/derive.html).

## What doesn't work
There's not a whole lot to this section. I gave myself a lot of time to work on this project, and basically everything I've done beyond the first week of working on it has been a stretch goal because I'm genuinely enjoying working on this. The things in this list are just things I would like to add if given more time:

* As mentioned, due to the way Piston works, the window isn't updating unless it's considered active. This means the timer doesn't display the correct time until the window is activated again. This doesn't mean time is tracked incorrectly, this just means the time will jump forward to the correct time suddenly when the window is reactivated.
* The way I'm reading the save file in prevents old save files from being compatible. By old save files, I mean that if I change any of the info that's saved, users will have to delete their current save file in order for the program to run. If I want to add ```number_of_wins``` at some point, old save files without this stat will cause the program to crash.
* Dropdown menu doesn't currently work.

## What lessons were learned
I knew from the beginning due to the research I did before writing my initial project proposal that the largest obstacle wouldn't be learning Rust, but learning Piston and other external libraries I would have to rely on. That ended up being completely true, and as I said, if I started again, I probably wouldn't use Piston. A lot of documentation is outdated, doesn't include any examples, and there's generally nobody really talking about this game engine on the internet.

In the last CS 410P course I took, the topic was C++, and I made [a terminal-based chess game](https://github.com/Sundwalltanner/Ascii-Chess), because terminal-based programs are basically all I've made so far. It was enjoyable working outside my comfort zone with this project, and while I know I'm not all that great at UI design or coming up with nice looking color schemes, I appreciate having the ability to display info on the screen without needing to print it to a terminal, and allowing for the user to interact with the program in ways beyond terminal input.

I posted a question in a subreddit related to Rust game development, and nobody knew the answer. I wanted an easy way to align text to the right, because all positioning in Piston is based on the top left corner of the screen, and fortunately Rust's standard ```format!``` macro [supports string alignment](https://doc.rust-lang.org/std/fmt/#fillalignment) by allowing you to indicate a minimum number of characters in a string, and filling the remainder with whitespace in order to align the text to a specific area. Unfortunately, Piston is doing something in the background that takes whitespace and automatically spreads it evenly between both sides of the text, so no matter what, any amount of whitespace centers a string when drawn to the screen. Nobody online is asking about this, and the people in the subreddit didn't know why it was happening. I ended up solving the issue by finding the width of the text and using some math instead.