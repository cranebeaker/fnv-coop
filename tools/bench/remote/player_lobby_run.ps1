param([string]$Release = "C:\fnv\player\FNV-CoOp-current", [int]$HostPort = 47101, [int]$GuestPort = 47102)
# Player-path check on the second PC, desktop session (start it with task.ps1): two terminal launchers from the RELEASE folder, one for game
# copy A (host) and one for B (friend), each with its own settings folder. No window: tools/bench/remote/player_lobby.py drives them over
# their local web API (the same calls the launcher page makes), so nothing simulates mouse or keyboard input.
$py = "C:\Program Files\Python311\pythonw.exe"
foreach ($p in @(@{Port = $HostPort; Cfg = "$env:APPDATA\FNVCoOp-lab-host"; Log = "C:\fnv\player\launcher_host.log"},
                 @{Port = $GuestPort; Cfg = "$env:APPDATA\FNVCoOp-lab-guest"; Log = "C:\fnv\player\launcher_guest.log"})) {
    $env:FNVCOOP_LAUNCHER_DIR = $p.Cfg
    Start-Process -FilePath $py -ArgumentList @("launcher\fnv_terminal.py", "--no-window", "--port", $p.Port, "--key", "lab") `
        -WorkingDirectory $Release -RedirectStandardOutput $p.Log -WindowStyle Hidden
}
