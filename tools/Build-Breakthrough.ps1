# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

<#
.SYNOPSIS
    Breakthrough Cognitive Ecosystem — Master Orchestrator

.DESCRIPTION
    Single-entry-point script that orchestrates the full build pipeline:
        Phase 1 — Foundation & Compliance Bootstrap
        Phase 2 — Build Orchestration & Performance Gates
        Phase 3 — Runtime Orchestration & Telemetry

    Cognitive subsystems assembled:
        * Kernel Technology (Resonance Engine, Evolution Engine, Coherence Engine)
        * Dynamic Memory Recall (Associative memory retrieval)
        * Enhanced Perception (Visual Stream at high-resolution, optimal refresh)
        * Meta Reason Engine (multi-layered cognitive processing)
        * Measurable Reality (quantification with precision)
        * Exploration and Discovery (horizon modules, validation)
        * Archives (Extinct Species preservation, Neanderthal insights)

.PARAMETER Phases
    Comma-separated list of phases to run. Default: '1,2,3'.

.PARAMETER ProjectRoot
    Root directory for the Breakthrough project.

.PARAMETER Configuration
    Build configuration: 'Debug', 'Release', or 'Production'.

.PARAMETER SkipPerfGates
    Skip performance gate validation (development only).

.PARAMETER RunTests
    Run cargo tests during Phase 2 (default: true).

.PARAMETER RuntimeSeconds
    Duration for RunOnce telemetry collection in Phase 3.

.EXAMPLE
    .\Build-Breakthrough.ps1
    Full pipeline: bootstrap + build + runtime telemetry.

.EXAMPLE
    .\Build-Breakthrough.ps1 -Phases 1,2 -Configuration Release -SkipPerfGates
    Foundation + build, release mode, skip perf gates.
#>

[CmdletBinding()]
param(
    [string]$Phases = '1,2,3',

    [string]$ProjectRoot = (Get-Location).Path,

    [ValidateSet('Debug', 'Release', 'Production')]
    [string]$Configuration = 'Release',

    [switch]$SkipPerfGates,

    [switch]$RunTests = $true,

    [int]$RuntimeSeconds = 30,

    [int]$Phase2TimeoutSeconds = 600
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$toolsDir = Join-Path $ProjectRoot 'tools'

function Write-MasterLog {
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$Message,
        [string]$Component = 'Master'
    )
    $ts = Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff'
    $entry = "[$ts] [$Component] $Message"
    $logPath = Join-Path $ProjectRoot 'logs\master.build.log'
    $ld = Split-Path $logPath -Parent
    if (-not (Test-Path $ld)) { New-Item -ItemType Directory -Path $ld -Force | Out-Null }
    Add-Content -Path $logPath -Value $entry
    Write-Host $entry -ForegroundColor Cyan
}

Write-MasterLog "=== Breakthrough Cognitive Ecosystem — Master Build Pipeline ==="
Write-MasterLog "Phases: $Phases"
$phaseList = $Phases -split '[, ]+' | Where-Object { $_ }
Write-MasterLog "Configuration: $Configuration"
Write-MasterLog "Project Root: $ProjectRoot"

foreach ($phase in $phaseList) {
    $scriptPath = Join-Path $toolsDir "Build-Breakthrough.Phase$phase.ps1"
    if (-not (Test-Path $scriptPath)) {
        Write-MasterLog "ERROR: Phase $phase script not found at $scriptPath"
        throw "Phase $phase script missing"
    }

    Write-MasterLog ">>> Launching Phase $phase <<<"

    # Build argument string for pwsh -File invocation (avoids splatting issues with -File mode)
    $argStr = "-ProjectRoot `"$ProjectRoot`" -Configuration `"$Configuration`""
    if ($phase -eq '2') {
        if ($SkipPerfGates) { $argStr += ' -SkipPerfGates' }
        if ($RunTests)      { $argStr += ' -RunTests' }
    }
    if ($phase -eq '3') {
        $argStr += " -Action RunOnce -DurationSeconds $RuntimeSeconds"
    }

    $psi = [System.Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = (Get-Command pwsh).Source
    $psi.Arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$scriptPath`" $argStr"
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.WorkingDirectory = $ProjectRoot

    $proc = [System.Diagnostics.Process]::Start($psi)

    # Collect output asynchronously to prevent pipe deadlock (buffers fill)
    $stdoutTask = $proc.StandardOutput.ReadToEndAsync()
    $stderrTask = $proc.StandardError.ReadToEndAsync()

    $timeoutMs = if ($phase -eq '2') { $Phase2TimeoutSeconds * 1000 } else { 300000 }
    $exited = $proc.WaitForExit($timeoutMs)

    if (-not $exited) {
        Write-MasterLog "ERROR: Phase $phase timed out after $timeoutMs ms"
        try {
            taskkill /PID $proc.Id /T /F 2>$null
        } catch {}
        $proc.Dispose()
        exit 1
    }

    $stdoutTask.Wait() | Out-Null
    $stderrTask.Wait() | Out-Null
    $exitCode = $proc.ExitCode
    $stdout = $stdoutTask.Result
    $stderr = $stderrTask.Result

    $output = $stdout + "`n" + $stderr
    $output -split "`n" | Where-Object { $_ -match '\S' } | ForEach-Object {
        Write-MasterLog "  [P$phase] $_"
    }

    if ($exitCode -ne 0) {
        Write-MasterLog "ERROR: Phase $phase failed with exit code $exitCode"
        exit $exitCode
    }

    Write-MasterLog "<<< Phase $phase COMPLETE >>>"
}

Write-MasterLog "=== All phases complete. Ecosystem is operational. ==="
exit 0
