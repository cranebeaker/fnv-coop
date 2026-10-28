# The host side of the two-PC lab smoke (tools/lab/lab_split.py), started in the logged-in desktop session by task.ps1:
#   task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/lab_host_up.ps1 -Wait 5
# Like host_up.ps1 (one copy, instance 0, its bridge pipe), after putting NVCoopLab.esp into that copy. The caller pushed out\lab\NVCoopLab.esp first.
$py = "C:\Program Files\Python311\python.exe"
Set-Location C:\fnv\fnv-coop
$env:PYTHONIOENCODING = "utf-8"
$env:NVCOOP_VISIBLE = "1"
$env:NVCOOP_TEST_HELPERS = "1"
$env:NVCOOP_LEAN = "0"
$env:NVCOOP_ADAPTER = "NVIDIA GeForce RTX 3070"          # an ssh/task session cannot enumerate display adapters
& $py -I tools\lab\esp\install_lab.py --instance-dir C:\fnv\fnv-instances\0 *> C:\fnv\up_out.txt
& $py -I tools\runner\run.py up --n 1 --wait-pipe 180 *>> C:\fnv\up_out.txt
