# PassFX

Simple CLI password generator written in Rust 🦀.

## Usage guide

PassFX is very easy to use. All you need to do is navigate to the parent directory and run it. There are a couple of arguments you might want to use:

- --name [name] : Change the output file's name.
- --length [len] : Change the password's length (in characters).

It will then generate a password of [len] and will output it to a file named '[name].txt' in the same directory.

**Note:** If no arguments are provided (IE: You just double-clicked the executable), it will generate a 32 character long password to a file named 'password.txt'.

**Supported characters:**
 ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "!", "@", "#", "$", "%", "&"]
