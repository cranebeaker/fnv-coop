# The lab run (docs/tools/lab.md) on the second PC, started in the logged-in desktop session by task.ps1:
#   task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/lab_run.ps1 -Wait 5
# Three pairs at once (6 copies in C:\fnv\fnv-instances). stdout = the summary, stderr = progress; the files are out\lab\<stamp>\results.json and summary.md.
# C:\fnv\lab_done.txt appears when the run ends (exit code inside). Needs runs\gameslot_config.json {"slots": 3}.
# -Net good_wifi,bad_wifi adds the network pass: the rooms run again with the client joined THROUGH the chaos relay (tools/chaos) with each profile; the whole
# run is capped by -TotalMin (default 15). -Split adds nothing here: the two-PC smoke has its own script, lab_split.ps1.
# The standard network run is the four profiles below: `-Net std` (or `-NetStd`; a PowerShell string parameter cannot be given without a value, so the
# "default list" is a word) is `-Net good_wifi,bad_wifi,mobile_tether,transatlantic`, clean + 4 profiles, 3 pairs, about 12 to 15 minutes.
# -Visible shows the copies' windows (never minimized) and -NoUnthrottle starts them without the opt-out of Windows power throttling (NVCOOP_UNTHROTTLE=0): both are for
# comparing the frame rate of the copies (docs/tools/lab.md "Frame rate of the copies"); a normal run uses neither.
param([int]$Pairs = 3, [int]$BudgetMin = 20, [string]$Rooms = "", [string]$Net = "", [int]$TotalMin = 15, [switch]$NetStd, [switch]$Visible, [switch]$NoUnthrottle, [string]$GuestTake = "", [string]$IniExtra = "", [string]$ExtraIni = "")
$StdNet = "good_wifi,bad_wifi,mobile_tether,transatlantic"
if ($NetStd -or $Net -eq "std") { $Net = $StdNet }
$py = "C:\Program Files\Python311\python.exe"
Set-Location C:\fnv\fnv-coop
$env:PYTHONIOENCODING = "utf-8"
$env:NVCOOP_LEAN = "0"
$env:NVCOOP_TEST_HELPERS = "1"
if ($GuestTake -ne "") { $env:LAB_GUEST_TAKE = $GuestTake }
if ($ExtraIni -ne "") { $IniExtra = (@($IniExtra, $ExtraIni) | Where-Object { $_ -ne "" }) -join "+" }   # -ExtraIni is the same option under the name the avatar docs use
if ($IniExtra -ne "") { $env:LAB_INI_EXTRA = $IniExtra }          # extra nvcoop.ini keys for this run, e.g. "actors.norepeatfinal=1+actors.noavaudit=1" (an A/B run of a fix switch)
if ($Visible) { $env:NVCOOP_VISIBLE = "1" }
if ($NoUnthrottle) { $env:NVCOOP_UNTHROTTLE = "0" }
$env:NVCOOP_ADAPTER = "NVIDIA GeForce RTX 3070"          # an ssh/task session cannot enumerate display adapters
$env:PROOF_MAX_PAIRS = "$Pairs"
$env:PROOF_NVSE_DIR = "C:\fnv\fnv-coop\out\cache\deps\nvse"
if (Test-Path C:\fnv\lab_done.txt) { Remove-Item C:\fnv\lab_done.txt -Force }
$labArgs = @("-I", "tools\lab\lab.py", "run", "--pairs", "$Pairs", "--budget-min", "$BudgetMin", "--reserve-gb", "2")
if ($Rooms -ne "") { $labArgs += @("--rooms", $Rooms) }
if ($Net -ne "") { $labArgs += @("--net", $Net, "--total-min", "$TotalMin") }
& $py @labArgs 1> C:\fnv\lab_out.txt 2> C:\fnv\lab_err.txt
"done rc=$LASTEXITCODE " + (Get-Date) | Set-Content C:\fnv\lab_done.txt
