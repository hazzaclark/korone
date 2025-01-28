# korone
A Lightweight Endianess Parser and Lexer

![image](https://github.com/user-attachments/assets/7e120d96-6a6e-4afc-9203-7b76f55e2546)

# Motive:

The ambition and intention moreover, with this project is to providem myself with a lightweight solution for being able
to evaluate and parse data in Rust - providing an opportunity to get a feel for the environment encompsssing Rust.

The main encompassing feature is the byte-aware parser, allowing for an effective means of attributing EOS/EOF when being fed into stdin

Such is the idea of potentially working on an implementation to read and parse Binary formats to handle certain operations

# Building:

```
git clone this repo

cargo build --release

cargo run
```

### Usage:

``./korone <INPUT_FILE> <ENDIANNESS>`` 

![image](https://github.com/user-attachments/assets/5d8c0abd-2268-40d6-a5b0-bd44b560df48)


The following is an example of using a SEGA Mega Binary ROM file to read the Endianness level accordingly

The example provided is using one of BigEvilCorp's, [Mega Drive Sample's](https://github.com/BigEvilCorporation/megadrive_samples) 

[Output Paste](https://pastebin.com/MQsigh4D)

![200w](https://github.com/user-attachments/assets/8ceb109a-49a1-4caa-a3a0-5770d0911c81)
