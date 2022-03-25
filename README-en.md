# keyboard recorder

> 语言： 这是一个英文的README，中文请见README.md.

## Intro
key-recorder is a very simple keyboard-input-recorder. It will silently record user kb input events. It could record for 81 keys that are used often.

When recieving a SIGINT (or CTRLC) signal, it will store the recorded events from starting till ending in a txt file under `./records/`.

The format for the txt files is "record-{starttime}-{endtime}"。

The 1st line of the txt file is starttime, 2nd is endtime, 3rd is 81 integers, representing the number of keyboard down for every key, 
4th is a serial of kb events during this time.

About in actual which key does a integer in 3rd line represent for, you could find in the 347 line in `main.rs`.

## Dependency
This project depends on `device_query-1.0.0`, `chrono-0.4.19`, `ctrlc-3.2.1`. Since `divice_query` is a cross-platform crate, this project will also be cross-platform.

## Limitation
This is not a globally keyboard detector. 

When some other Apps (especially full-screen App) running, key-recorder will not work. I have tested several games and Apps on Win10 including _Genshin Impact_, 
_NARAKA_.

I presently thought the reason is that under windows these Apps monopolize kb, of they hung-up key-recorder forcely.

This problem may be solved by Hook in Winapi. I plan to implement this by Rust, but C++ of .NET will be probably easier.

I haven't investigate how to solve this problem under Linux, but it will be likely easier.

## Extend
If you are in interest, you could do whatever you want to the txt file.

You could also make other programs, running this program backgroundly.

Because I have intended to make a globally keyboard detector, I haven't do these jobs on this project.
