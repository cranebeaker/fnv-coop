# Load a save on the story pair, print where it stands and optionally save it as a ready save (docs/tools/story.md "The start save").
#   writes C:\fnv\story_probe_args.txt first: line 1 = the save to load, line 2 (optional) = the name to save it as
#   task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/story_probe.ps1 -Wait 5
# (task.ps1 cannot pass arguments with spaces through ssh, hence the file.) C:\fnv\story_done.txt appears when it ends; stdout is story_out.txt, progress story_err.txt.
$py = "C:\Program Files\Python311\python.exe"
Set-Location C:\fnv\fnv-coop
$env:PYTHONIOENCODING = "utf-8"
$env:NVCOOP_LEAN = "0"
$env:NVCOOP_ADAPTER = "NVIDIA GeForce RTX 3070"
$env:PROOF_MAX_PAIRS = "1"
$env:PROOF_NVSE_DIR = "C:\fnv\fnv-coop\out\cache\deps\nvse"
if (Test-Path C:\fnv\story_done.txt) { Remove-Item C:\fnv\story_done.txt -Force }
$l = @(Get-Content C:\fnv\story_probe_args.txt)
$a = @("-I", "tools\story\story.py", "probe", "--save", $l[0].Trim(), "--reserve-gb", "2")
if ($l.Count -gt 1 -and $l[1].Trim() -ne "") { $a += @("--save-as", $l[1].Trim()) }
& $py @a 1> C:\fnv\story_out.txt 2> C:\fnv\story_err.txt
"done rc=$LASTEXITCODE " + (Get-Date) | Set-Content C:\fnv\story_done.txt
