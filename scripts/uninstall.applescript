-- Uninstall StatusLight
-- Double-clickable uninstaller for macOS

on run
	set dialogResult to display dialog "This will uninstall StatusLight and remove:" & linefeed & linefeed & ¬
		"  • CLI tools (/usr/local/bin/statuslight, statuslightd)" & linefeed & ¬
		"  • Launch daemon" & linefeed & ¬
		"  • StatusLight.app from Applications" & linefeed & linefeed & ¬
		"Your configuration at ~/.config/statuslight/ will be preserved." & linefeed & linefeed & ¬
		"Continue?" with title "Uninstall StatusLight" buttons {"Cancel", "Uninstall"} default button "Cancel" with icon caution

	if button returned of dialogResult is "Uninstall" then
		doUninstall()

		set purgeResult to display dialog "Remove configuration files too?" & linefeed & linefeed & ¬
			"This deletes ~/.config/statuslight/ (settings, Slack token, custom presets)." with title "Uninstall StatusLight" buttons {"Keep Config", "Remove Config"} default button "Keep Config"

		if button returned of purgeResult is "Remove Config" then
			do shell script "rm -rf ~/.config/statuslight/"
		end if

		display dialog "StatusLight has been uninstalled." with title "Uninstall StatusLight" buttons {"OK"} default button "OK" with icon note
	end if
end run

on doUninstall()
	-- 1. Quit the running app if open
	try
		tell application "StatusLight" to quit
	end try
	-- Wait for app to exit (up to 5 seconds), then force kill
	try
		do shell script "for i in 1 2 3 4 5 6 7 8 9 10; do pgrep -x StatusLight >/dev/null 2>&1 || exit 0; sleep 0.5; done; killall -9 StatusLight 2>/dev/null; exit 0"
	end try

	-- 2. Unload LaunchAgent
	set plistPath to (POSIX path of (path to home folder)) & "Library/LaunchAgents/com.statuslight.daemon.plist"
	try
		do shell script "launchctl unload -w " & quoted form of plistPath
	end try
	try
		do shell script "rm -f " & quoted form of plistPath
	end try

	-- 3. Kill statuslightd and statuslight processes, wait for confirmed exit
	try
		do shell script "killall statuslightd 2>/dev/null; for i in 1 2 3 4 5 6 7 8 9 10; do pgrep -x statuslightd >/dev/null 2>&1 || break; sleep 0.5; done; killall -9 statuslightd 2>/dev/null; killall statuslight 2>/dev/null; for i in 1 2 3 4 5 6 7 8 9 10; do pgrep -x statuslight >/dev/null 2>&1 || break; sleep 0.5; done; killall -9 statuslight 2>/dev/null; exit 0"
	end try

	-- 4. Turn off the light (now no other process holds the HID handle)
	try
		do shell script "/usr/local/bin/statuslight off"
	on error
		try
			do shell script "/Applications/StatusLight.app/Contents/MacOS/statuslight off"
		end try
	end try

	-- 5. Remove symlinks (requires admin) and app bundle
	try
		do shell script "rm -f /usr/local/bin/statuslight /usr/local/bin/statuslightd && rm -rf /Applications/StatusLight.app" with administrator privileges
	on error
		display dialog "Could not remove CLI tools (admin access denied)." & linefeed & "You can remove them manually:" & linefeed & linefeed & "  sudo rm -f /usr/local/bin/statuslight /usr/local/bin/statuslightd" with title "Uninstall StatusLight" buttons {"OK"} default button "OK" with icon caution
	end try

	-- 6. Remove install markers (so reinstall works correctly)
	try
		do shell script "rm -f ~/.config/statuslight/.installed-*"
	end try
end doUninstall
