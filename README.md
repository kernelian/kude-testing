This is the development repository of The Kernelian UNIX Desktop Environment.

This just calls X. Nothing much.



How to install on Arch Linux:

Install Xorg:

`sudo pacman -S xorg xorg-xinit`

Install Rust:

"https://www.rust-lang.org/tools/install"

Clone the repo, for this, you will need to have git installed.

`git clone https://github.com/kernelian/kude-testing`

CD to the directory of KUDE, and add the x11rb crate for everything to work.

`cargo add x11rb`

Edit the ~/.xinit file, for this, i will use nano as an example:

`nano ~/.xinitrc`

Remove the containing of xinitrc, and put:

`<PATH TO KUDE SRC MAIN.RS FILE> & (Include the ampersand)`

`exec <Put the X11 Application name you want to run here>`

Then, type `startx`, and you should be good to go!
