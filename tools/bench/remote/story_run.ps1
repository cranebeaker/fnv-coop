# The story run (docs/tools/story.md) on the second PC, started in the logged-in desktop session by task.ps1:
#   task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/story_run.ps1 -Wait 5 [-ScriptArgs "-Steps S03,S05"]
# One pair (copies 0 and 1 in C:\fnv\fnv-instances). stdout = the summary, stderr = progress; the files are out\story\<stamp>\results.json and summary.md.
# C:\fnv\story_done.txt appears when the run ends (exit code inside). Share the PC through C:\fnv\dino.lock (create it BEFORE launching, delete it after).
# -IniExtra "section.key=value+...": extra nvcoop.ini keys for the copies (the release defaults: "companions.enabled=1+live.social=1").
# -Probe NAME [-SaveAs NAME2]: load that save on the pair, print where it stands and (with -SaveAs) save it through the bridge on both copies instead of running the steps.
param([string]$Steps = "", [int]$BudgetMin = 25, [string]$Save = "", [switch]$StopOnFail, [string]$Probe = "", [string]$SaveAs = "", [string]$IniExtra = "")
$py = "C:\Program Files\Python311\python.exe"
Set-Location C:\fnv\fnv-coop
$env:PYTHONIOENCODING = "utf-8"
$env:NVCOOP_LEAN = "0"
$env:NVCOOP_ADAPTER = "NVIDIA GeForce RTX 3070"          # an ssh/task session cannot enumerate display adapters
$env:PROOF_MAX_PAIRS = "1"
$env:PROOF_NVSE_DIR = "C:\fnv\fnv-coop\out\cache\deps\nvse"
$env:LAB_INI_EXTRA = $IniExtra                    # more nvcoop.ini keys for the copies (tools/lab/lab_live.py reads them)
if (Test-Path C:\fnv\story_done.txt) { Remove-Item C:\fnv\story_done.txt -Force }
$a = @("-I", "tools\story\story.py", "run", "--budget-min", "$BudgetMin", "--reserve-gb", "2")
if ($Save -ne "") { $a += @("--save", $Save) }
if ($Probe -ne "") {
  $a = @("-I", "tools\story\story.py", "probe", "--save", $Probe, "--reserve-gb", "2")
  if ($SaveAs -ne "") { $a += @("--save-as", $SaveAs) }
}
if ($Steps -ne "") { $a += @("--steps", $Steps) }
if ($StopOnFail) { $a += "--stop-on-fail" }
& $py @a 1> C:\fnv\story_out.txt 2> C:\fnv\story_err.txt
"done rc=$LASTEXITCODE " + (Get-Date) | Set-Content C:\fnv\story_done.txt
