# adventofcode2022
These are my, [Niklas Hallqvist](https://github.com/niklasha) solutions to
[Advent of code 2022](https://adventofcode.com/2022).
They are written in [Rust](https://rust-lang.org).

My reason for doing these are, besides the fact that I like puzzle solving, I want to test my skills in Rust.
Earlier years I had never done Rust for anything serious, this year I have actually developped in Rust for a real system so I cannot claim being a novice anymore.

You need Rust, [rustup](https://rustup.rs/) is the suggested way to install Rust, that is about it.
You may need to add some SSL libraries, depending on operating system, but the installation process will tell you, if so.

Run all the days with:
```
cargo run input/
```

Where "input/" is a prefix for the days' inputs, named 01, 02, etc.
The tests (the examples given in the days' descriptions) can be run with:
```
cargo test
```

For every day, the first commit will be the solution with which I solved the puzzle.
After that, I may still revise the code to be more idiomatic or just nicer.


```
My results were:
      --------Part 1--------   --------Part 2--------
Day       Time   Rank  Score       Time   Rank  Score
 24   18:05:57   8440      0   23:45:46   8807      0
 23   02:27:32   3482      0   02:46:49   3502      0
 22   02:51:56   4006      0   16:47:33   5827      0
 21   00:54:22   3998      0   02:38:36   4059      0
 20   17:04:13  12184      0   17:12:28  11277      0
 19   02:13:58   1371      0          -      -      -
 18   00:58:55   4160      0   03:16:19   4372      0
 17   03:30:12   4229      0   10:21:45   5083      0
 16       >24h  15862      0          -      -      -
 15   01:28:01   5515      0   03:57:49   5947      0
 14   01:42:44   6832      0   01:53:24   6263      0
 13   02:19:14   7721      0   04:58:44  11662      0
 12   03:12:27   9739      0   07:21:50  15920      0
 11   02:54:49  11935      0   05:13:57  12306      0
 10   00:44:09   8403      0   01:11:12   7428      0
  9   01:46:29  12157      0   01:48:30   7753      0
  8   00:53:17   9565      0   02:36:07  14340      0
  7   12:27:08  44509      0   16:20:01  52130      0
  6   00:23:06  12337      0   00:30:25  12839      0
  5   01:39:39  16019      0   01:44:24  15196      0
  4   00:21:03   9318      0   00:25:17   8370      0
  3   00:19:10   6904      0   00:44:44   9747      0
  2   00:35:16  12655      0   00:46:25  11993      0
  1   00:38:39  10869      0   00:43:09  10284      0```
