# scheduleoff
Wrapper around `pmset schedule` for easy near-term sleep and shutdown time setting.

`sudo pmset schedule sleep "$(date -v+5M '+%D %T')"`
`sudo pmset schedule cancel shutdown "03/13/2023 15:27:41"`
`sudo pmset -g sched`
https://www.macos.utah.edu/documentation/administration/pmset.html

`scheduleoff` is designed for cases of "I'd like my Mac to shutdown in 45 minutes."
`pmset schedule` requires the time to be specified as a big clunky datetime, e.g. "03/13/23 22:16:13".
`whoami`
`sudo visudo`
`info visudo` -- 
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

`which pmset`
https://www.sudo.ws/docs/man/1.8.15/sudoers.man/