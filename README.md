# Wire

Wire is a simple application that lets you make your own desktop environment really simply.


# How it works

Wire is an application that calls X, reads your startup.conf for applications to start, and it runs them, and you can make your own desktop environment!


# How to use the binary?

You don't need Rust installed if you don't want to compile it yourself, a ready to go binary is already provided in the binaries folder.

Here's how to use it:

1. Install xorg and xorg-xinit if you don't have them

   Arch: `sudo pacman -S xorg xorg-xinit`


2. Move the binary to any location you want, or you can just keep it.

3. Edit .xinitrc, i will use nano as an example for this

   `nano ~/.xinitrc`
 
   To run the binary:

   `exec your/path/to/the/binary/here`

   And that's all for .xinitrc!

4. Don't startx (or how you want to launch it) just yet, make a folder named wire and create a startup.conf file:

   `nano ~/.config/wire/startup.conf`

5. Edit startup.conf to start the applications, be it panels or anything you want!

6. Now, do `startx`, and you're all set!

Enjoy!

# How to compile it and use it?

Compiling it is straightforward, you just need Rust installed.

You just go to the Wire directory, and do `cargo build --release` or any other parameter you want to use, and then just follow the binary instructions.
