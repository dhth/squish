Output demo.gif
Set FontSize 18
Set Width 1000
Set Height 400
Set Theme "GruvboxDark"

Set Shell zsh
Type@50ms "ls -lh test"
Enter
Sleep 2s
Type@50ms "squish test/input.jpg -o test/output.jpg -v"
Enter
Sleep 3s
Type@50ms "ls -lh test"
Enter
Sleep 2s
