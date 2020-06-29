# AB

A number guessing game implemented in Rust. Also available at http://ab.muan.co.

🦀

```
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target/debug/ab`
-----------------------------------------------------------------------
Guess a 4-digit number with the least number of attempts possible!
All four digits will be different.

With each guess, you will get a hint in the form of As and Bs.

A means: n digits match perfectly.
B means: n digits match but are not at the correct position.

For example, with an answer of 7130, a guess of 3610 will receive 1A2B.
1*A for the 0, rightly in the 4th position; 2*B are 1 and 3.
-----------------------------------------------------------------------
> (0) Guess a 4-digit number:
0123
0A1B
> (1) Guess a 4-digit number:
3456
2A0B
> (2) Guess a 4-digit number:
3478
1A1B
> (3) Guess a 4-digit number:
5490
1A0B
> (4) Guess a 4-digit number:
349
⚠️  Less than 4 digits.
> (4) Guess a 4-digit number:
3401
1A1B
> (5) Guess a 4-digit number:
0452
1A0B
> (6) Guess a 4-digit number:
3591
0A1B
> (7) Guess a 4-digit number:
3678
0A2B
> (8) Guess a 4-digit number:
3721
0A2B
> (9) Guess a 4-digit number:
3795
0A1B
> (10) Guess a 4-digit number:
7289
1A0B
> (11) Guess a 4-digit number:
5794
0A2B
> (12) Guess a 4-digit number:
5289
0A0B
> (13) Guess a 4-digit number:
1356
1A1B
> (14) Guess a 4-digit number:
1347
0A3B
> (15) Guess a 4-digit number:
4179
0A3B
> (16) Guess a 4-digit number:
4170
0A3B
> (17) Guess a 4-digit number:
7416
The answer is 7416! You got it in 17 tries. 👾
```
