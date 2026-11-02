<#
.SYNOPSIS
Replays the shared acceptance routes in the release viewer and reports
pass/fail for each. Every pull request that touches the game must pass
them (CONTRIBUTING.md).

.DESCRIPTION
Routes (commands and success lines documented in docs/GOODSPRINGS_ROUTE.md,
docs/PATHING.md and docs/VANCE_GUN_ROUTE.md):
  doc    Doc Mitchell walks the west rooms to his chair spot and talks
         (GSDocMitchellHouse, VCG01 stage 110).
  vcg02  Back in the Saddle: Sunny's walks, wells, reward (the quest's own
         scripts; quest start, bottle hits, some gecko kills and the
         player's following are console lines).
  vms16  Ghost Town Gunfight with Trudy's help: the gangers come in and
         die, stage 100.
  vance  Vance's Gun: investigate the missing museum gun and get the Wins'
         safe unlocked through the quest's own activation/dialogue scripts.
A route passes when every one of its success lines appears in the
viewer's output and no panic does.
Every route runs with the viewer's --answer-boxes test aid, which answers
the prompts the game opens as a player would, by rule, as soon as each is
shown: a message box with one button (OK) with it; a box with more buttons
with the next of --box-answers (button numbers in the box's own order, 0
the first); a tutorial box closed; the name entry accepted with Enter (the
name in it). The routes give --box-answers 2: VCG01's "Before you venture
deeper into the wasteland, you may revise your character." box answered
"Finished - Travel Onward" (buttons: Edit Name, Rebuild Character,
Finished - Travel Onward). With the DLCs installed their start-up quests
also open message boxes on a new game (Dead Money's signal, the pre-order
packs' "items added", the level-cap notices, ...), as the game does; left
open, a prompt holds the AI and the input and the route never finishes.
Each answer is logged as "--answer-boxes: ..."; a box with several
buttons and no choice left is left open and logged as waiting. Outputs (logs, screenshots) go to
-Out; they are private, never commit them.

.EXAMPLE
powershell -File scripts\acceptance.ps1 -Data "D:\Steam\steamapps\common\Fallout New Vegas\Data"
powershell -File scripts\acceptance.ps1 -Routes vms16 -Build
powershell -File scripts\acceptance.ps1 -Background
#>
param(
    # The game's Data folder (or set NV_DATA).
    [string]$Data = $env:NV_DATA,
    # Which routes to run, comma-separated (doc, vcg02, vms16, vance).
    [string]$Routes = 'doc,vcg02,vms16',
    # Where logs and screenshots go (default: %USERPROFILE%\nv-re\acceptance\<time>).
    [string]$Out,
    # Build the release viewer first.
    [switch]$Build,
    # Keep the runs out of the way (the viewer's --background: its window
    # behind the others, without the focus, the mouse left alone).
    [switch]$Background
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if (-not $Data) {
    $default = 'C:\Games\Steam\steamapps\common\Fallout New Vegas\Data'
    if (Test-Path $default) { $Data = $default } else { throw 'Give -Data (the game''s Data folder) or set NV_DATA.' }
}
if (-not $Out) {
    $Out = Join-Path $env:USERPROFILE ("nv-re\acceptance\" + (Get-Date -Format 'yyyyMMdd-HHmmss'))
}
New-Item -ItemType Directory -Force $Out | Out-Null

if ($Build) {
    Push-Location (Join-Path $root 'viewer')
    try {
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw 'viewer release build failed' }
    } finally { Pop-Location }
}
$viewer = Join-Path $root 'viewer\target\release\nv-viewer.exe'
if (-not (Test-Path $viewer)) { throw "No release viewer at $viewer (run with -Build)." }

function Run-Line([string[]]$lines) {
    $a = @()
    foreach ($l in $lines) { $a += '--run'; $a += $l }
    $a
}
function Run-At([int]$at, [string]$line) { @('--run-at', "$at", $line) }

$routeArgs = @{
    doc   = @{
        Args    = @('GSDocMitchellHouse', '--stage', 'VCG01', '110', '--wait', '50')
        Success = @('Dialogue menu: talking to Doc Mitchell')
    }
    vcg02 = @{
        Args    = @('WastelandNV', '--at', '-68250,5800,8480,180') +
            (Run-Line @('SetStage VCG02 5', 'set VFreeformGoodsprings.bMetSunny to 1', 'StartQuest VCG02',
                'SetObjectiveCompleted VCG02 3 1', 'SetObjectiveDisplayed VCG02 5 1', 'SetStage VCG02 10')) +
            (Run-At 40 'set VCG02.nTargetCount to 3') + (Run-At 40 'SetObjectiveCompleted VCG02 10 1') +
            (Run-At 40 'SunnyREF.evp') +
            (90, 110, 130, 150, 170, 190, 210 | ForEach-Object { Run-At $_ 'player.MoveTo SunnyREF' }) +
            (Run-At 230 'player.MoveTo VCG02SunnySneakMarkerREF') +
            (Run-At 240 'VCG02Gecko1REF.Kill') + (Run-At 241 'VCG02Gecko2REF.Kill') +
            (270, 290, 310 | ForEach-Object { Run-At $_ 'player.MoveTo SunnyREF' }) +
            # Well 3's geckos (and well 2's if still alive).
            (Run-At 320 '0010A210.Kill') + (Run-At 321 '0010ABB0.Kill') + (Run-At 322 '0010A20F.Kill') +
            (Run-At 323 '0010A211.Kill') + (Run-At 324 '0010A212.Kill') + (Run-At 325 '0010A213.Kill') +
            (Run-At 340 'SunnyREF.StartConversation player') +
            @('--say', "Okay, I'm in.", '--say', "Sure, I'll come with you.", '--say', "Couldn't hurt.", '--wait', '370')
        Success = @('Completed: Talk to Sunny about your reward', 'XP +50')
    }
    vms16 = @{
        Args    = @('WastelandNV', '--at', '-67845,3000,8400,180') +
            (Run-Line @('set VMS16.bTrudyHelp to 1', 'SetStage VMS16 65', 'SunnyRef.ResetHealth',
                'SunnyRef.MoveTo SunnySpawnMarker', 'SunnyRef.AddScriptPackage SunnyTriggerGunfightDialoguePackage',
                'SunnyRef.RemoveScriptPackage', 'GoodspringsPowderGangMarker.Enable',
                'SunnyRef.AddScriptPackage SunnyTravelPackage', 'RingoRef.AddScriptPackage RingoTravelPackage',
                'RingoRef.AddToFaction GoodspringsFaction 1', 'SetStage VMS16 70', 'set VMS16.bGunFightStart to 1',
                'player.ModAV Health 5000')) +
            @('--wait', '330', '--walk', '--weapon', 'WeapNV9mmPistol')
        Success = @('XP +50')
    }
    vance = @{
        Args    = @('VikkiAndVance', '--at', '2074,3626,7232,180', '--use', '000E288A') +
            (Run-Line @('player.SetAV Science 50', 'player.SetAV Speech 55')) +
            (Run-At 70 'player.MoveTo PrimmSlimREF') +
            (Run-At 70 'PrimmSlimREF.StartConversation player') +
            @('--say', "Did you know that Vance's gun is missing?",
              # This is the visible Science 50 choice; the DIAL topic's
              # internal name is "What happened to Vance's Gun?".
              '--say', 'Scan your data registry',
              '--say', 'Goodbye.') +
            # FalloutNV.esm has no link that adds this non-top-level topic:
            # INFO 000BACC4 does not AddTopic it, and no parsed script does.
            # Add it explicitly so the following real quest INFOs can run.
            (Run-At 155 'player.AddTopic 000E32E7') +
            (Run-At 160 'player.MoveTo PaulineWinsREF') +
            # Let the cross-cell move spawn Wins Residence and its talkers
            # before asking Pauline to start the conversation.
            (Run-At 165 'PaulineWinsREF.StartConversation player') +
            @('--say', "I know the two of you stole Vance's gun down in Primm.",
              '--say', "What's your plan?",
              '--say', "That's the greatest plan I've ever heard in my entire life.",
              '--say', 'go') +
            # The success INFO changes SammyPauline but does not add Sammy's
            # non-top-level conversation topic either.
            (Run-At 255 'player.AddTopic 000E32E5') +
            (Run-At 260 'player.MoveTo SammyWinsREF') +
            (Run-At 260 'SammyWinsREF.StartConversation player') +
            @('--say', 'Pauline says you should give me the gun.',
              '--say', 'go', '--wait', '320')
        Success = @('XP +50', 'XP +55',
            "The gun's in the safe. The combination's 05-23-34.",
            'result script: SammySafeREF.setownership / SammySafeREF.unlock / set nVPrimmVanceGun.VanceGunCase to 3 / set nVPrimmVanceGun.SammyPauline to 2')
    }
}

$chosen = @($Routes.Split(',') | ForEach-Object { $_.Trim() } | Where-Object { $_ })
foreach ($r in $chosen) {
    if (-not $routeArgs.ContainsKey($r)) { throw "Unknown route '$r' (doc, vcg02, vms16, vance)." }
}

$results = @()
foreach ($r in $chosen) {
    $spec = $routeArgs[$r]
    $log = Join-Path $Out "$r.log"
    $shot = Join-Path $Out "$r.png"
    # --answer-boxes: prompts answered as they appear, as a player would
    # (see the header); --box-answers 2: VCG01's "revise your character"
    # box answered "Finished - Travel Onward" (its third button).
    $a = @($Data) + $spec.Args + @('--answer-boxes', '--box-answers', '2', '--screenshot', $shot)
    if ($Background) { $a += '--background' }
    # Each route is its own viewer run: a new game, so the DLC start
    # messages show again in every route that starts outside Doc's house.
    Write-Host "== $r (new game)"
    $start = Get-Date
    # Windows PowerShell turns a native program's stderr lines into errors.
    $ErrorActionPreference = 'Continue'
    & $viewer @a > $log 2>&1
    $code = $LASTEXITCODE
    $ErrorActionPreference = 'Stop'
    $secs = [int]((Get-Date) - $start).TotalSeconds
    $text = Get-Content -Raw $log
    $missing = @($spec.Success | Where-Object { $text -notmatch [regex]::Escape($_) })
    $panic = $text -match 'panicked at'
    $pass = ($missing.Count -eq 0) -and -not $panic -and $code -eq 0
    $why = if ($panic) { 'panic' } elseif ($code -ne 0) { ('exit code 0x{0:X8}' -f $code) } elseif ($missing.Count) { 'missing: ' + ($missing -join '; ') } else { '' }
    $results += [pscustomobject]@{ Route = $r; Pass = $pass; Seconds = $secs; Detail = $why; Log = $log }
}
$results | Format-Table -AutoSize | Out-String | Write-Host
if ($results | Where-Object { -not $_.Pass }) { exit 1 }
exit 0
