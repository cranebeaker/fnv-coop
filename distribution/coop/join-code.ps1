# Shows the host's join code: it waits for the line of the game's newest log that holds it.
# "Play Co-op.cmd" runs this after it starts the game. NVDIR is the package's folder.
$logs = Join-Path $env:NVDIR 'userdata\logs'
$from = (Get-Date).AddSeconds(-5)
for ($i = 0; $i -lt 900; $i++) {
    $log = Get-ChildItem -LiteralPath $logs -Filter 'game-*.log' -ErrorAction SilentlyContinue |
        Where-Object { $_.LastWriteTime -ge $from } |
        Sort-Object LastWriteTime |
        Select-Object -Last 1
    if ($log) {
        $line = Select-String -LiteralPath $log.FullName -Pattern 'NVRS-[A-Za-z0-9-]+' | Select-Object -First 1
        if ($line) {
            $code = $line.Matches[0].Value
            Write-Host ''
            Write-Host "  Join code: $code"
            try { Set-Clipboard -Value $code; Write-Host '  It is on the clipboard.' } catch { }
            Write-Host '  Send it to your friends, then start a new game.'
            exit 0
        }
    }
    Start-Sleep -Seconds 1
}
Write-Host '  No join code was found. Look at the newest game log in userdata\logs.'
exit 1
