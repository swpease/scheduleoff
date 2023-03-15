# scheduleoff
Wrapper around `pmset schedule` for easy near-term sleep and shutdown time setting.

## Motivation
With macOS Ventura, Apple made the flabbergasting decision to remove the ability to schedule sleeps/shutdowns/restarts with their handy UI as part of their Battery Preferences(?) panel, and instead instructs users to [use the Terminal](https://support.apple.com/en-ae/guide/mac-help/mchl40376151/mac).

I often like to schedule my Mac to shutdown in an hour or so to have some background audio as I go to bed. I don't want to do datetime math as I'm going to bed (or ever, really), and also don't want to have to open up my Terminal, and also don't want to issue a `sudo` command and enter my password, and also don't want to have to remember what `pmset` is called, and also don't want to have to remember its syntax. 

Last night, I opened up my Terminal and issued the command:

>`sudo pmset schedule shutdown "$(date -v+50M '+%D %T')"`

to set-up a shutdown in 50 minutes. It's the user-friendly reliability of Apple that has kept me coming back.

## What `scheduleoff` Does
Rather than type out that pile of bullshit seen above, I wrote `scheduleoff` as a wrapper around it, simplifying the above call to:

>`scheduleoff shutdown 50`

### An Extra Detail
If you used the old UI that Apple removed, you'll remember that there is actually a 10-minute countdown dialog box that pops up at the scheduled time, giving you the option to cancel or proceed immediately. `pmset`, and therefore `scheduleoff`, have this same feature.

### Why Is It Called `scheduleoff`?
Well, I've been using `pmset` quite often over the last several days, and still can't remember what it's name is. There seems to be a contest to remove as many characters as possible from command line tools, which in the modern world of TAB-completion seems outdated. It's supposed to schedule stuff, so I thought "schedule"-something, and it only handles initiating sleeps and shutdowns, so I thought, "off". I imagine I'll wind up thinking, "It's 'schedule'-something, right?", and type in my Terminal "sch[TAB]", then pause, then "[TAB][TAB]" to see everything that starts with "sch" pop up, then see "scheduleoff", and write "sched[TAB]", and then that'll work (for me).

## Installation
### Rust
If you are a Rust user, you can use `cargo install scheduleoff`.
### Here
Pre-built binaries are available for download in the "builds" folder. I do not have an x86 Mac anymore, so I can't tell if the x86 version actually works.

After downloading, 
  1. Store it in whichever folder you want. It would be easiest to just store it in the folder that your Terminal opens up to by default. You can find this by opening your Terminal and typing `open .`, which will open it in Finder. 
  2. Rename the file to your liking. I suggest "scheduleoff".
  3. To run, type `./<executable> [commands]` where `<executable>` is whatever you named that file, and see below in Use for commands.
  4. You may need to change permissions, using `chmod 744 <executable>` in Terminal in the folder containing the file you downloaded, where `<executable>` is whatever you named that file.
  5. If you try to run it and Apple won't let you, right-click on it in Finder and click "Open".

## Use
`scheduleoff`s intended use case is one-off shutdown or sleep scheduling in the near-term. So the only options are

`scheduleoff shutdown MINUTES`

`scheduleoff sleep MINUTES`

where `MINUTES` is some number of minutes, e.g. `scheduleoff sleep 90`. 

`MINUTES` must be greater than 0, because if it is 0, then your scheduled time will be missed by `pmset` because it will have already passed by.

For any other aspect that the old scheduling UI could handle, you'll have to find some other tool, or use `pmset`. For information on how to use `pmset`, I recommend [this tutorial](https://www.macos.utah.edu/documentation/administration/pmset.html).

### Canceling a Scheduled Shutdown/Sleep
You have two options:
  1. Wait for the 10-minute countdown dialog.
  2. Use `pmset`
  
For option 2, see the above linked tutorial, but you'll be writing something like: `sudo pmset schedule cancel shutdown "03/13/2023 15:27:41"`.

## `sudo`
You'll still have to type in your password to use `scheduleoff`, because it uses `sudo`. If you think that's annoying, then there are (at least) two options:
  1. Configure it so that you can use Touch ID for `sudo`.
  2. Make it so that `sudo pmset` doesn't require a password.
  
I opted for option 2. Below is a description of what I did, though I suggest you **read other resources to get comfortable with the process, because bad things will happen if you make a mistake**.

### Password-less `sudo` Overview
It amounted to customizing a `sudo`-related file with the added line:
> swpease ALL = (root) NOPASSWD: /usr/bin/pmset

Where you would replace "swpease" with your user name. If you're unsure, use `whoami` in your Terminal. I suppose you could also verify that `pmset` is in the same location with `which pmset`. 

To add this line to the file you're supposed to add it to, use `sudo visudo`, which will enter you into a `vim` editor of the file (you should know how to use `vim` before you start). It is **EXTREMELY IMPORTANT** not to make a mistake with this. From the `visudo` documentation:
> visudo parses the sudoers file after editing and will not save the
       changes if there is a syntax error.  Upon finding an error, visudo will
       print a message stating the line number(s) where the error occurred and
       the user will receive the “What now?” prompt.  At this point the user
       may enter ‘e’ to re-edit the sudoers file, ‘x’ to exit without saving
       the changes, or ‘Q’ to quit and save changes.  The ‘Q’ option should be
       used with extreme caution because if visudo believes there to be a
       syntax error, so will sudo and no one will be able to run sudo again
       until the error is fixed.  If ‘e’ is typed to edit the sudoers file
       after a syntax error has been detected, the cursor will be placed on
       the line where the error occurred (if the editor supports this
       feature).
       
### Resources
  - [sudoers Manual](https://www.sudo.ws/docs/man/1.8.15/sudoers.man/)
  - [Digital Ocean](https://www.digitalocean.com/community/tutorials/how-to-edit-the-sudoers-file)
  - [StackExchange](https://unix.stackexchange.com/questions/18830/how-to-run-a-specific-program-as-root-without-a-password-prompt)
  - [Jeff Triplett](https://jefftriplett.com/2022/enable-sudo-without-a-password-on-macos/)
  - [StackExchange #2](https://superuser.com/questions/430880/is-there-a-way-to-remove-root-requirements-for-a-specific-command-in-linux-if-y)
