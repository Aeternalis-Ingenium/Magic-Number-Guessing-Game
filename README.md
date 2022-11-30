<h1 align=center><strong>Guessing Magic Number</strong></h1>

---

This is a game where you can challenge your guessing instinct. The game is implemented fully in `Rust` and contains the following features:

* Choose between 4 difficulties as follow:

    * 1 for easy $\rightarrow$ 13 Guessesing Chances
    * 2 for normal $\rightarrow$ 10 Guessesing Chances
    * 3 for hard $\rightarrow$ 7 Guessesing Chances
    * 4 for hardcore $\rightarrow$ 3 Guessesing Chances
* Choosing difficulty number that is greater than 4 will result in a loop of prompt for the number until the guess is between 1 and 4.
* Guessing the magic number greater than 100 will result in a loop of prompt for the number until the guess is between 1 and 100.
* Win $\rightarrow$ Whenever the magic number is guessed within the above chances
* Lost $\rightarrow$ Whenever the chances run out!

---

## **Set Up Guide**

1. Clone the repository

    * SSH: `git@github.com:Ma6icFin6erz/Magic-Number-Guessing-Game.git`
    * HTTPS: `https://github.com/Ma6icFin6erz/Magic-Number-Guessing-Game.git`

<br>

2. Go into the working directory

    * `cd Magic-Number-Guessing-Game`

<br>

3. Build the program

    * `cargo build`

<br>

4. Execute the binary

    * `./target/debug/magical_guessing_game`

---

Enjoy the game!
