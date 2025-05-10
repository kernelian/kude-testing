This is the development repository of The Kernelian UNIX Desktop Environment (The T falls off) for testing, and other stuff.


How to install on Arch Linux:

Install Xorg:
`sudo pacman -S xorg xorg-xinit`

Install Rust:
"https://www.rust-lang.org/tools/install"

Edit the ~/.xinit file, for this, i will use nano as an example:
`nano ~/.xinitrc`

Remove the containing of xinitrc, and put:
`<PATH TO KUDE SRC MAIN.RS FILE> & (Include the ampersand)
exec <Put the X11 Application name you want to run here>`

Then, type startx, and you should be good to go!
