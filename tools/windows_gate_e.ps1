param(
    [string]$TestBinary = ".\himsat-core-tests.exe",
    [string]$ManifestPath = ".\manifest.json",
    [string]$EvidenceDir = ".\gate-e-evidence",
    [switch]$RunLiveSmoke,
    [switch]$ConsentLiveCapture
)

$ErrorActionPreference = "Stop"

function Write-JsonFile {
    param([string]$Path, [object]$Value)
    $Value | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $Path -Encoding utf8
}

if ($env:OS -ne "Windows_NT") {
    throw "Gate E hardware bundle must run on Windows."
}

if (Test-Path -LiteralPath $EvidenceDir) {
    $existingEvidence = @(Get-ChildItem -LiteralPath $EvidenceDir -Force)
    if ($existingEvidence.Count -ne 0) {
        throw "Evidence directory is not empty. Preserve the existing run and choose a new evidence directory: $EvidenceDir"
    }
} else {
    New-Item -ItemType Directory -Path $EvidenceDir | Out-Null
}
$manifest = Get-Content -LiteralPath $ManifestPath -Raw | ConvertFrom-Json
if (-not (Test-Path -LiteralPath $TestBinary)) {
    throw "Missing prebuilt test binary: $TestBinary"
}

$actualSha = (Get-FileHash -Algorithm SHA256 -LiteralPath $TestBinary).Hash.ToLowerInvariant()
if ($manifest.test_binary_sha256 -ne $actualSha) {
    throw "Test binary digest mismatch. Expected $($manifest.test_binary_sha256), got $actualSha"
}

$audioService = Get-Service -Name Audiosrv -ErrorAction Stop
$pnpError = $null
$endpoints = @()
try {
    $endpoints = @(Get-PnpDevice -Class AudioEndpoint -PresentOnly -ErrorAction Stop)
} catch {
    $pnpError = $_.Exception.Message
}

$sanitizedEndpoints = @(
    $endpoints | ForEach-Object {
        [ordered]@{
            friendly_name = $_.FriendlyName
            status = [string]$_.Status
            class = [string]$_.Class
        }
    }
)

$testListPath = Join-Path $EvidenceDir "test-list.txt"
& $TestBinary --list *>&1 | Tee-Object -FilePath $testListPath
if ($LASTEXITCODE -ne 0) {
    throw "Prebuilt test binary could not list tests."
}

$micTest = "capture_windows::windows_tests::live_stream_open_reports_frames_or_classified_fault"
$loopbackTest = "capture_windows_system_audio::windows_tests::live_loopback_open_reports_frames_or_classified_fault"

$testList = Get-Content -LiteralPath $testListPath -Raw
foreach ($required in @($micTest, $loopbackTest)) {
    if ($testList -notmatch [regex]::Escape($required)) {
        throw "Required Gate E live test is absent from prebuilt binary: $required"
    }
}

$preflight = [ordered]@{
    schema = "himsat-gate-e-preflight-v1"
    source_sha = $manifest.source_sha
    test_binary_sha256 = $actualSha
    os = [System.Environment]::OSVersion.VersionString
    powershell = $PSVersionTable.PSVersion.ToString()
    audio_service = [ordered]@{
        name = $audioService.Name
        status = [string]$audioService.Status
    }
    endpoint_query = [ordered]@{
        count = $sanitizedEndpoints.Count
        error = $pnpError
        endpoints = $sanitizedEndpoints
    }
    live_smoke_requested = [bool]$RunLiveSmoke
    explicit_live_capture_consent = [bool]$ConsentLiveCapture
    generated_at_utc = [DateTimeOffset]::UtcNow.ToString("o")
}
Write-JsonFile (Join-Path $EvidenceDir "preflight.json") $preflight

if (-not $RunLiveSmoke) {
    Write-Host "PREFLIGHT_COMPLETE"
    Write-Host "SOURCE_SHA=$($manifest.source_sha)"
    Write-Host "AUDIO_ENDPOINT_COUNT=$($sanitizedEndpoints.Count)"
    exit 0
}

if (-not $ConsentLiveCapture) {
    throw "Live microphone/loopback smoke was requested without -ConsentLiveCapture."
}
if ($audioService.Status -ne "Running") {
    throw "Windows Audio service is not running."
}
if ($sanitizedEndpoints.Count -eq 0) {
    throw "No present AudioEndpoint device is visible to this session."
}

$results = @()

$env:HIMSAT_LIVE_MIC_TEST = "1"
$micLog = Join-Path $EvidenceDir "live-mic.txt"
& $TestBinary $micTest --exact --nocapture *>&1 | Tee-Object -FilePath $micLog
$micExit = $LASTEXITCODE
Remove-Item Env:HIMSAT_LIVE_MIC_TEST -ErrorAction SilentlyContinue
$results += [ordered]@{ test = $micTest; exit_code = $micExit; log = "live-mic.txt" }

$env:HIMSAT_LIVE_LOOPBACK_TEST = "1"
$loopbackLog = Join-Path $EvidenceDir "live-loopback.txt"
& $TestBinary $loopbackTest --exact --nocapture *>&1 | Tee-Object -FilePath $loopbackLog
$loopbackExit = $LASTEXITCODE
Remove-Item Env:HIMSAT_LIVE_LOOPBACK_TEST -ErrorAction SilentlyContinue
$results += [ordered]@{ test = $loopbackTest; exit_code = $loopbackExit; log = "live-loopback.txt" }

Write-JsonFile (Join-Path $EvidenceDir "live-smoke.json") ([ordered]@{
    schema = "himsat-gate-e-live-smoke-v1"
    source_sha = $manifest.source_sha
    test_binary_sha256 = $actualSha
    results = $results
    generated_at_utc = [DateTimeOffset]::UtcNow.ToString("o")
})

if (($results | Where-Object { $_.exit_code -ne 0 }).Count -ne 0) {
    throw "One or more live Gate E smoke tests failed. Preserve the evidence directory."
}

Write-Host "LIVE_SMOKE_COMPLETE"
