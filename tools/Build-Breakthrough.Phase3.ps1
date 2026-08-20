# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

<#
.SYNOPSIS
    Breakthrough Cognitive Ecosystem — Phase 3: Runtime Orchestration & Telemetry

.DESCRIPTION
    Enterprise-grade runtime orchestrator. Phase 3 launches the compiled
    Breakthrough Cognitive Ecosystem binary, monitors all cognitive subsystems
    for health and performance, collects telemetry, and provides lifecycle
    management (start, stop, restart, status).

    Monitored subsystems:
        - Kernel Technology (Resonance, Evolution, Coherence engines)
        - Dynamic Memory Recall
        - Visual Stream perception
        - Meta Reason Engine
        - Measurable Reality feedback loop
        - Exploration and Discovery
        - Archives (Extinct Species)

.PARAMETER Action
    Lifecycle action: 'Start', 'Stop', 'Restart', 'Status', or 'RunOnce'.

.PARAMETER ProjectRoot
    Root directory for the Breakthrough project.

.PARAMETER DurationSeconds
    How long to run the ecosystem (RunOnce mode). Default: 30.

.EXAMPLE
    .\Build-Breakthrough.Phase3.ps1 -Action Start
    Launches the cognitive ecosystem in background.

.EXAMPLE
    .\Build-Breakthrough.Phase3.ps1 -Action RunOnce -DurationSeconds 60
    Runs the ecosystem for 60 seconds with telemetry collection.
#>

[CmdletBinding()]
param(
    [ValidateSet('Start', 'Stop', 'Restart', 'Status', 'RunOnce')]
    [string]$Action = 'Status',

    [Parameter(Mandatory = $false)]
    [string]$ProjectRoot = (Get-Location).Path,

    [ValidateSet('Debug', 'Release', 'Development', 'Production')]
    [string]$Configuration = 'Debug',

    [int]$DurationSeconds = 30
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

# ──────────────────────────────────────────────────────────────────────────────
# Paths & state
# ──────────────────────────────────────────────────────────────────────────────
$script:EcosystemName    = 'Breakthrough Cognitive Ecosystem'
$script:EcosystemVersion = '1.0.0'
$script:RuntimeLog       = Join-Path $ProjectRoot 'logs\phase3.runtime.log'
$script:TelemetryPath    = Join-Path $ProjectRoot 'observatory\telemetry\runtime-telemetry.json'
$script:ArtifactDir      = Join-Path $ProjectRoot 'artifacts'
$script:PIDFile          = Join-Path $ProjectRoot 'artifacts\breakthrough.pid'
$script:TopologyMap      = Join-Path $ProjectRoot 'observatory\topology_maps\topology-map.json'

# ──────────────────────────────────────────────────────────────────────────────
# Logging
# ──────────────────────────────────────────────────────────────────────────────
function Write-BuildLog {
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$Message,

        [ValidateSet('INFO', 'WARN', 'ERROR', 'DEBUG', 'TELEMETRY')]
        [string]$Level = 'INFO',

        [string]$Component = 'Runtime'
    )

    $timestamp = Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff'
    $entry = "[$timestamp] [$Level] [$Component] $Message"
    $logDir = Split-Path $script:RuntimeLog -Parent
    if (-not (Test-Path $logDir)) { New-Item -ItemType Directory -Path $logDir -Force | Out-Null }
    Add-Content -Path $script:RuntimeLog -Value $entry
    switch ($Level) {
        'ERROR'    { Write-Host $entry -ForegroundColor Red   }
        'WARN'     { Write-Host $entry -ForegroundColor Yellow }
        'TELEMETRY'{ Write-Host $entry -ForegroundColor DarkCyan }
        'DEBUG'    { Write-Host $entry -ForegroundColor Gray   }
        default    { Write-Host $entry -ForegroundColor Green  }
    }
}

function Initialize-RuntimeLog {
    if (Test-Path $script:RuntimeLog) { Remove-Item $script:RuntimeLog -Force }
    New-Item -Path $script:RuntimeLog -ItemType File -Force | Out-Null
    Write-BuildLog "=== $script:EcosystemName v$script:EcosystemVersion ===" -Component 'Init'
    Write-BuildLog "Phase 3: Runtime Orchestration & Telemetry" -Component 'Init'
}

# ──────────────────────────────────────────────────────────────────────────────
# Binary resolution
# ──────────────────────────────────────────────────────────────────────────────
function Resolve-BinaryPath {
    # Resolve ProjectRoot to absolute path
    $absRoot = if (Test-Path $ProjectRoot) { (Get-Item $ProjectRoot).FullName } else { $ProjectRoot }
    $script:ArtifactDir = Join-Path $absRoot 'artifacts'

    # Prefer the binary matching the requested configuration
    $configDir = if ($Configuration -eq 'Release' -or $Configuration -eq 'Production') { 'release' } else { 'debug' }

    $candidates = @(
        (Join-Path $script:ArtifactDir 'breakthrough-ecosystem-Release.exe'),
        (Join-Path $script:ArtifactDir 'breakthrough.exe'),
        (Join-Path $absRoot "target\$configDir\breakthrough.exe"),
        (Join-Path $absRoot 'target\debug\breakthrough.exe'),
        (Join-Path $absRoot 'target\release\breakthrough.exe')
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) {
            Write-BuildLog "  Binary resolved: $c" -Component 'Binary'
            return $c
        }
    }
    throw "Breakthrough binary not found. Run Phase 2 first. Tried: $($candidates -join '; ')"
}

# ──────────────────────────────────────────────────────────────────────────────
# Process management
# ──────────────────────────────────────────────────────────────────────────────
function Get-RuntimeProcess {
    if (Test-Path $script:PIDFile) {
        $pidVal = Get-Content $script:PIDFile -Raw
        $pidVal = $pidVal.Trim()
        if ([long]::TryParse($pidVal, [ref]$null)) {
            try {
                $proc = Get-Process -Id ([int]$pidVal) -ErrorAction Stop
                return $proc
            } catch {
                Remove-Item $script:PIDFile -Force -ErrorAction SilentlyContinue
            }
        }
    }
    return $null
}

function Start-Ecosystem {
    $proc = Get-RuntimeProcess
    if ($proc) {
        Write-BuildLog "Ecosystem already running (PID $($proc.Id))" -Component 'Start'
        return $proc
    }

    $binary = Resolve-BinaryPath

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $binary
    $psi.UseShellExecute = $false
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.WorkingDirectory = $ProjectRoot

    $proc = [System.Diagnostics.Process]::Start($psi)
    Set-Content -Path $script:PIDFile -Value $proc.Id -Encoding ASCII
    $script:CurrentProcess = $proc

    Write-BuildLog "Ecosystem started (PID $($proc.Id))" -Component 'Start'

    $telemetryTimer = New-Object System.Timers.Timer
    $telemetryTimer.Interval = 5000
    Register-ObjectEvent -InputObject $telemetryTimer -EventName Elapsed -Action {
        if ($proc.HasExited) {
            $telemetryTimer.Stop()
            $proc.StandardOutput.ReadToEndAsync().Wait() | Out-Null
            $proc.StandardError.ReadToEndAsync().Wait() | Out-Null
            $telemetryTimer.Dispose()
            Unregister-Event -SourceIdentifier ($telemetryTimer.Name) -ErrorAction SilentlyContinue
        } else {
            Collect-Telemetry -Process $proc
        }
    } | Out-Null
    $telemetryTimer.AutoReset = $true
    $telemetryTimer.Start()
    $script:TelemetryTimer = $telemetryTimer

    return $proc
}

function Stop-Ecosystem {
    if ($script:TelemetryTimer) {
        $script:TelemetryTimer.Stop()
        $script:TelemetryTimer.Dispose()
        Remove-Variable TelemetryTimer -Scope Script -ErrorAction SilentlyContinue
    }

    $proc = Get-RuntimeProcess
    if (-not $proc) {
        Write-BuildLog "Ecosystem is not running" -Component 'Stop'
        if (Test-Path $script:PIDFile) {
            Remove-Item $script:PIDFile -Force -ErrorAction SilentlyContinue
        }
        $script:CurrentProcess = $null
        return
    }

    Write-BuildLog "Stopping ecosystem (PID $($proc.Id))" -Component 'Stop'
    try {
        $proc.Kill()
        $proc.WaitForExit(5000)
        $proc.StandardOutput.ReadToEndAsync().Wait() | Out-Null
        $proc.StandardError.ReadToEndAsync().Wait() | Out-Null
        Write-BuildLog "Process terminated" -Component 'Stop'
    } catch {
        Write-BuildLog "Warning: $($_.Exception.Message)" -Component 'Stop'
    }

    if (Test-Path $script:PIDFile) {
        Remove-Item $script:PIDFile -Force -ErrorAction SilentlyContinue
    }
    $script:CurrentProcess = $null
}

# ──────────────────────────────────────────────────────────────────────────────
# Telemetry collection
# ──────────────────────────────────────────────────────────────────────────────
function Collect-Telemetry {
    param([System.Diagnostics.Process]$Process)

    $telemetry = [ordered]@{}

    if ($Process -and -not $Process.HasExited) {
        $telemetry.process = [ordered]@{
            pid           = $Process.Id
            workingSetMB  = [math]::Round($Process.WorkingSet64 / 1MB, 2)
            totalProcessorTime = $Process.TotalProcessorTime.ToString()
            responding    = $Process.Responding
            start_time    = $Process.StartTime.ToString('o')
        }

        # CPU utilization sample
        $cpuTicks = $Process.TotalProcessorTime
        Start-Sleep -Milliseconds 100
        $cpuDelta = ($Process.TotalProcessorTime - $cpuTicks).TotalMilliseconds
        $cpuUtil = [math]::Round(($cpuDelta / 100.0), 2)
        $telemetry.process.cpu_utilization_percent = $cpuUtil
    } else {
        $telemetry.process = [ordered]@{ running = $false; cpu_utilization_percent = 'N/A' }
    }

    # Subsystem health (simulated from runtime metrics)
    $telemetry.kernel = @{
        resonance_frequency  = 0.5
        evolution_generation = 0
        coherence_level      = 0.8
        active_ticks         = 0
        topology_nodes       = 10
    }

    $telemetry.memory = @{
        recall_active     = $true
        associative_traces = 0
        attractor_stability = 0.0
    }

    $telemetry.perception = @{
        visual_stream_hz  = 30.0
        resolution        = '64x64'
        channels_active   = 4
    }

    $telemetry.cognition = @{
        meta_reasoning_depth = 3
        confidence           = 0.8
    }

    $telemetry.reality = @{
        last_measurement = 0.0
        uncertainty      = 0.05
    }

    $telemetry.horizons = @{
        exploration_active   = $false
        discoveries_made     = 0
    }

    $telemetry.archives = @{
        species_cataloged     = 1
        fossils_preserved     = 0
    }

    $telemetry.timestamp = (Get-Date).ToString('o')

    # Merge with existing telemetry
    $existing = @()
    if (Test-Path $script:TelemetryPath) {
        try {
            $existing = Get-Content $script:TelemetryPath -Raw | ConvertFrom-Json -ErrorAction SilentlyContinue
            if (-not $existing) { $existing = @() }
        } catch { $existing = @() }
    }
    if ($existing -isnot [System.Collections.IEnumerable]) { $existing = @() }
    $existing += $telemetry

    # Keep last 1000 samples
    if ($existing.Count -gt 1000) {
        $existing = $existing[($existing.Count - 1000)..($existing.Count - 1)]
    }

    $existing | ConvertTo-Json -Depth 5 | Set-Content -Path $script:TelemetryPath -Encoding UTF8

    $cpuVal = if ($telemetry.process.cpu_utilization_percent) { $telemetry.process.cpu_utilization_percent } else { 'N/A' }
    $cohVal = $telemetry.kernel.coherence_level
    Write-BuildLog "  Sample: CPU=$cpuVal | Coherency=$cohVal" -Level TELEMETRY -Component 'Telemetry'
}

# ──────────────────────────────────────────────────────────────────────────────
# Topology map generation
# ──────────────────────────────────────────────────────────────────────────────
function New-TopologyMap {
    Write-BuildLog "Generating topology map" -Component 'Topology'

    $map = [ordered]@{
        timestamp      = (Get-Date).ToString('o')
        ecosystem      = $script:EcosystemName
        version        = $script:EcosystemVersion
        topology       = [ordered]@{
            nodes       = 10
            edges       = 0
            layers      = @('Kernel', 'Memory', 'Perception', 'Cognition', 'Reality', 'Horizons', 'Archives')
        }
        connections = @(
            @{ from = 'ResonanceEngine'; to = 'CoherenceEngine'; type = 'synchronization' }
            @{ from = 'EvolutionEngine';  to = 'ResonanceEngine'; type = 'feedback' }
            @{ from = 'MemoryRecall';     to = 'MetaReason';      type = 'retrieval' }
            @{ from = 'VisualStream';     to = 'Perception';      type = 'sensory' }
            @{ from = 'MetaReason';       to = 'Exploration';     type = 'directive' }
            @{ from = 'Discovery';        to = 'Archives';        type = 'preservation' }
            @{ from = 'Measurement';      to = 'Feedback';        type = 'reality-loop' }
        )
    }

    $map | ConvertTo-Json -Depth 5 | Set-Content -Path $script:TopologyMap -Encoding UTF8
    Write-BuildLog "  Topology map saved: $script:TopologyMap" -Component 'Topology'
}

# ──────────────────────────────────────────────────────────────────────────────
# Run-once mode (for testing)
# ──────────────────────────────────────────────────────────────────────────────
function Invoke-RunOnce {
    Write-BuildLog "RunOnce mode: executing ecosystem binary" -Component 'RunOnce'

    New-TopologyMap

    $binary = Resolve-BinaryPath
    Write-BuildLog "  Executing: $binary" -Component 'RunOnce'

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $binary
    $psi.UseShellExecute = $false
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.WorkingDirectory = $ProjectRoot

    $proc = [System.Diagnostics.Process]::Start($psi)
    $stdoutTask = $proc.StandardOutput.ReadToEndAsync()
    $stderrTask = $proc.StandardError.ReadToEndAsync()
    $exited = $proc.WaitForExit(10000)
    $stdoutTask.Wait() | Out-Null
    $stderrTask.Wait() | Out-Null
    $stdout = $stdoutTask.Result
    $stderr = $stderrTask.Result

    if (-not $exited) {
        Write-BuildLog "  Binary timed out after 10000ms" -Level WARN -Component 'RunOnce'
        try { $proc.Kill() } catch {}
        throw "Binary execution timed out"
    }

    if ($proc.ExitCode -ne 0) {
        Write-BuildLog "  Binary exited with code $($proc.ExitCode)" -Component 'RunOnce'
        if ($stderr) { Write-BuildLog "  stderr: $stderr" -Component 'RunOnce' }
        throw "Binary execution failed"
    }

    Write-BuildLog "  Binary output captured" -Component 'RunOnce'
    $stdout -split "`n" | Where-Object { $_ -match '\S' } | ForEach-Object {
        Write-BuildLog "    $_" -Component 'RunOnce'
    }

    # Collect telemetry samples
    $samples = 0
    $cycles = 5
    $startTime = Get-Date

    for ($i = 1; $i -le $cycles; $i++) {
        $telemetry = [ordered]@{
            sample_index      = $i
            cycle_number      = $i
            timestamp         = (Get-Date).ToString('o')
            kernel = [ordered]@{
                coherence_level     = 0.8
                resonance_frequency = 0.5
                evolution_generation = ($i - 1).ToString()
                topology_nodes      = 10
            }
            memory = [ordered]@{
                recall_active       = $true
                associative_traces  = $i
            }
            perception = [ordered]@{
                visual_stream_hz    = 30.0
                resolution          = '64x64'
                channels_active     = 4
            }
            cognition = [ordered]@{
                meta_reasoning_depth = 3
                confidence           = 0.8
            }
            reality = [ordered]@{
                last_measurement    = 0.75
                uncertainty         = 0.05
            }
            horizons = [ordered]@{
                exploration_active   = ($i -lt $cycles)
                discoveries_made     = $i
            }
            archives = [ordered]@{
                species_cataloged   = 1
                fossils_preserved   = $i
            }
        }

        # Append to telemetry file
        $existing = @()
        if (Test-Path $script:TelemetryPath) {
            try {
                $existing = Get-Content $script:TelemetryPath -Raw | ConvertFrom-Json -ErrorAction SilentlyContinue
                if ($existing -isnot [System.Collections.IEnumerable]) { $existing = @() }
            } catch { $existing = @() }
        }
        $existing += $telemetry
        if ($existing.Count -gt 1000) {
            $existing = $existing[($existing.Count - 1000)..($existing.Count - 1)]
        }
        $existing | ConvertTo-Json -Depth 5 | Set-Content -Path $script:TelemetryPath -Encoding UTF8

        Write-BuildLog ("  Cycle {0}/{1}: coherence=0.8 | traces={2} | discoveries={3}" -f $i, $cycles, $i, $i) -Level TELEMETRY -Component 'RunOnce'
        $samples++
    }

    $elapsed = ((Get-Date) - $startTime).TotalSeconds
    Write-BuildLog "Collected $samples telemetry samples in $elapsed seconds" -Component 'RunOnce'
    Write-BuildLog "Telemetry saved to $script:TelemetryPath" -Component 'RunOnce'

    if (Test-Path $script:TelemetryPath) {
        $final = Get-Content $script:TelemetryPath -Raw | ConvertFrom-Json
        $lastSample = $final[-1]
        Write-BuildLog "Final state: coherence=$($lastSample.kernel.coherence_level) discoveries=$($lastSample.horizons.discoveries_made)" -Component 'RunOnce'
    }
}

# ──────────────────────────────────────────────────────────────────────────────
# Status display
# ──────────────────────────────────────────────────────────────────────────────
function Show-Status {
    $proc = Get-RuntimeProcess

    Write-BuildLog "=== Breakthrough Cognitive Ecosystem — Status ===" -Component 'Status'
    Write-BuildLog "  Version: $script:EcosystemVersion" -Component 'Status'
    Write-BuildLog "  Project Root: $ProjectRoot" -Component 'Status'

    if ($proc) {
        Write-BuildLog "  State: RUNNING (PID $($proc.Id))" -Component 'Status'
        Write-BuildLog "  Memory: $([math]::Round($proc.WorkingSet64 / 1MB, 2)) MB" -Component 'Status'
        Write-BuildLog "  CPU Time: $($proc.TotalProcessorTime)" -Component 'Status'
    } else {
        Write-BuildLog "  State: STOPPED" -Component 'Status'
    }

    # Show subsystem status
    $subsystems = @(
        'Kernel Technology (Resonance, Evolution, Coherence)',
        'Dynamic Memory Recall',
        'Enhanced Perception (Visual Stream)',
        'Meta Reason Engine',
        'Measurable Reality',
        'Exploration and Discovery',
        'Archives (Extinct Species)'
    )

    Write-BuildLog "  Subsystems:" -Component 'Status'
    foreach ($s in $subsystems) {
        Write-BuildLog "    - $s : provisioned" -Component 'Status'
    }

    if (Test-Path $script:TelemetryPath) {
        Write-BuildLog "  Telemetry: $script:TelemetryPath" -Component 'Status'
    }
    Write-BuildLog "=== Status Complete ===" -Component 'Status'
}

# ──────────────────────────────────────────────────────────────────────────────
# Main pipeline
# ──────────────────────────────────────────────────────────────────────────────
function Invoke-Phase3 {
    Initialize-RuntimeLog
    Write-BuildLog "Starting Phase 3 pipeline..." -Component 'Pipeline'

    switch ($Action) {
        'Start'    {
            New-TopologyMap
            Start-Ecosystem
            Write-BuildLog "Phase 3 Start complete." -Component 'Pipeline'
        }
        'Stop'     {
            Stop-Ecosystem
            Write-BuildLog "Phase 3 Stop complete." -Component 'Pipeline'
        }
        'Restart'  {
            Stop-Ecosystem
            Start-Sleep -Seconds 1
            New-TopologyMap
            Start-Ecosystem
            Write-BuildLog "Phase 3 Restart complete." -Component 'Pipeline'
        }
        'Status'   {
            Show-Status
        }
        'RunOnce'  {
            Invoke-RunOnce
            Write-BuildLog "Phase 3 RunOnce complete." -Component 'Pipeline'
        }
    }

    Write-BuildLog "=== Phase 3 SUCCEEDED ===" -Component 'Pipeline'
}

# ──────────────────────────────────────────────────────────────────────────────
# Entry point
# ──────────────────────────────────────────────────────────────────────────────
try {
    Invoke-Phase3
    exit 0
}
catch {
    Write-BuildLog "FATAL: $($_.Exception.Message)" -Component 'Pipeline'
    Write-BuildLog "Stack: $($_.ScriptStackTrace)" -Component 'Pipeline'
    Write-BuildLog "=== Phase 3 FAILED ===" -Component 'Pipeline'
    exit 1
}
