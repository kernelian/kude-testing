# Wire

Wire is a simple application that calls X, giving unlimited creativity!


# How it works

It connects to X, meaning you can run anything you want on there! Be it any desktop or a WM you want!




How to install on Arch Linux:

Install Xorg:

`sudo pacman -S xorg xorg-xinit`

Install Rust:

"https://www.rust-lang.org/tools/install"

Clone the repo, for this, i will use git as an example.

`git clone https://github.com/kernelian/wire`

CD to the directory of Wire, and add the x11rb crate for everything to work.

`cargo add x11rb`

Edit the ~/.xinit file, for this, i will use nano as an example:

`nano ~/.xinitrc`

Remove the containing of xinitrc, and put:

`<PATH TO Wire SRC MAIN.RS FILE> & (Include the ampersand)`

`exec <Put the X11 Application name you want to run here>`

Then, type `startx`, and you should be good to go!
