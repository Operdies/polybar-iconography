# polybar-iconography
bspwm workspace module for Polybar which shows active windows on each workspace

![image](https://user-images.githubusercontent.com/14153598/189499776-6337c71d-f4ab-4f8e-a3ae-83c7dbde10fd.png)

Here is an example where the active workspace is highlighted with a underline accent,
and the active window on the workspace is highlighted with an overline accent.

Every icon is a unicode symbol, so [nerd fonts](https://www.nerdfonts.com/) are highly recommended.
Use the [cheat sheet](https://www.nerdfonts.com/cheat-sheet) to find appropriate icons.

## Dependencies
* The [hush shell](https://hush-shell.github.io/) 
* `xorg-xprop`

## Configuration

Icons for different applications must currently be configured directly in the code.
This is very flexible and allows setting the icon based on any property set by BSPWM.
Additional properties can be fetched using the X window ID.

Colors and separators can be configured using environment variables from the polybar config.
See my [personal configuration](https://github.com/Operdies/dotfiles/blob/ac3ac1ec5b46d02b985903052f28746cd07e6b3c/config/polybar/config.ini#L457)
for an example, or have a look in the code.
