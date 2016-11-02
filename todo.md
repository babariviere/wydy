Scripts organized like:
scripts/update/system.sh
scripts/update/neovim.sh
scripts/update/git/<repo_name>.sh
scripts/install.sh

ex: update all => scripts/update/*
ex: update system => scripts/update/system

-------

If command start like a script, ask if user wants to add script.

ex: update <name>
[1] Search for "update <name>"
[2] Add script "update <name>"

-------

Add ability to select option by default.

ex: update <name> 
Search for "update <name>" by default, use --select to show select menu.

-------

add ability to remove hint message.
flags:  --no-hints

------

Add ability to send output to the clients.
flags:  --get-output=<stderr,stdin,stdout,all>
				--get-stdout
				--get-stdin
				--get-stderr

------

Add ability to execute wcommand locally.
flags:  --exec-locally

------

Add support for sdl2, gtk3, ncurses. (only for client side)

------ 

Add support for config file.
Contains default flags.

------

Add ability to use client config file.
