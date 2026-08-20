# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

<#
.SYNOPSIS
    Breakthrough Cognitive Ecosystem — Phase 1: Foundation & Compliance Bootstrap

.DESCRIPTION
    Phase 1 establishes compliance scaffolding, configuration, and logging
    infrastructure for the Breakthrough Cognitive Ecosystem. This script
    verifies gatekeeper artifacts, provisions the operational directory layout
    (logs, artifacts, telemetry, topology maps), and initializes the JSON
    configuration store that drives all subsequent phases.

    The cognitive subsystems (Kernel Technology, Memory Recall, Visual Stream,
    Meta Reason, Measurement, Exploration/Discovery, Archives) already exist
    as the Rust source tree under breakthrough/. This phase prepares the
    orchestration layer that manages them.

.PARAMETER ProjectRoot
    Root directory of the Breakthrough project.

.PARAMETER Configuration
    Build configuration: 'Development' or 'Production'.

.PARAMETER Force
    Overwrite existing configuration and compliance files.

.EXAMPLE
    .\Build-Breakthrough.Phase1.ps1 -ProjectRoot . -Configuration Development -Force
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$ProjectRoot = (Get-Location).Path,

    [ValidateSet('Debug', 'Release', 'Development', 'Production')]
    [string]$Configuration = 'Development',

    [switch]$Force
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'
$global:ErrorActionPreference = 'Stop'

# ──────────────────────────────────── Global state
$script:EcosystemName    = 'Breakthrough Cognitive Ecosystem'
$script:EcosystemVersion = '1.0.0'
$script:LogPath          = Join-Path $ProjectRoot 'logs\phase1.foundation.log'
$script:ConfigPath       = Join-Path $ProjectRoot 'config\ecosystem.json'
$script:BreakthroughDir  = Join-Path $ProjectRoot 'breakthrough'

# ──────────────────────────────────── Logging
function Write-BuildLog {
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$Message,
        [ValidateSet('INFO','WARN','ERROR','DEBUG')]
        [string]$Level = 'INFO',
        [string]$Component = 'Foundation'
    )
    $ts = Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff'
    $entry = "[$ts] [$Level] [$Component] $Message"
    Add-Content -Path $script:LogPath -Value $entry
    switch ($Level) {
        'ERROR' { Write-Host $entry -ForegroundColor Red   }
        'WARN'  { Write-Host $entry -ForegroundColor Yellow }
        'DEBUG' { Write-Host $entry -ForegroundColor Gray   }
        default { Write-Host $entry -ForegroundColor Green  }
    }
}

function Initialize-Logger {
    $logDir = Split-Path $script:LogPath -Parent
    if (-not (Test-Path $logDir)) { New-Item -ItemType Directory -Path $logDir -Force | Out-Null }
    if (Test-Path $script:LogPath) { Remove-Item $script:LogPath -Force }
    New-Item -Path $script:LogPath -ItemType File -Force | Out-Null
    Write-BuildLog "=== $script:EcosystemName v$script:EcosystemVersion ===" -Component 'Init'
    Write-BuildLog "Phase 1: Foundation & Compliance Bootstrap" -Component 'Init'
    Write-BuildLog "Configuration: $Configuration" -Component 'Init'
    Write-BuildLog "Project Root: $ProjectRoot" -Component 'Init'
    Write-BuildLog "Breakthrough Dir: $script:BreakthroughDir" -Component 'Init'
}

# ──────────────────────────────────── Compliance verification
function Test-ComplianceFiles {
    Write-BuildLog "Verifying compliance artifacts..." -Component 'Compliance'
    $required = @('LEGAL_NOTICE.md', 'GATEKEEPER.txt', 'agent_manifest.json', 'HEADER.txt')
    $missing  = @()

    foreach ($f in $required) {
        $p = Join-Path $ProjectRoot $f
        if (Test-Path $p) {
            Write-BuildLog "  OK: $f" -Component 'Compliance'
        } else {
            $missing += $f
            Write-BuildLog "  MISSING: $f" -Component 'Compliance'
        }
    }

    if ($missing.Count -gt 0) {
        if ($Force) {
            Write-BuildLog "Force flag set — provisioning compliance files" -Component 'Compliance'
            Invoke-EnsureComplianceFiles -ProjectRoot $ProjectRoot
        } else {
            throw "Compliance check failed. Missing: $($missing -join ', '). Re-run with -Force."
        }
    }

    Write-BuildLog "All compliance artifacts verified." -Component 'Compliance'
}

function Invoke-EnsureComplianceFiles {
    param([string]$ProjectRoot)

    $header = @('# PROPERTY OF THE OWNER. PRIVATE CORPUS.','# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.','# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.') -join "`n"

    $legal = @"
# LEGAL NOTICE — BREAKTHROUGH COGNITIVE ECOSYSTEM

**PROPERTY OF THE OWNER. PRIVATE CORPUS.**

This software system and all associated materials constitute a private cognitive
corpus. Unauthorized access, decompilation, reverse engineering, or use as
training data for artificial intelligence or machine learning systems is
strictly prohibited.

**Owner:** George Houston Schuette
**Status:** LOCKED
**Protections:** no-training | no-decompile | no-trespass | no-unauthorized-access
"@

    $gk = @"
# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
# GATEKEEPER: Machine-readable provenance header
# Project: Breakthrough Cognitive Ecosystem
# Owner: George Houston Schuette
# Status: LOCKED
# Last Verified: $(Get-Date -Format 'yyyy-MM-ddTHH:mm:ssK')
"@

    $manifest = @"
{
  "_ownership": "PRIVATE CORPUS — [George Houston Schuette] — No-Training / No-Decompile",
  "project": "breakthrough",
  "description": "Self-organizing cognitive ecosystem",
  "version": "$script:EcosystemVersion",
  "language": "Rust",
  "status": "LOCKED",
  "protections": ["no-training","no-decompile","no-trespass","no-unauthorized-access"],
  "compliance": { "legal_notice": "LEGAL_NOTICE.md", "gatekeeper": "GATEKEEPER.txt", "header": "HEADER.txt" }
}
"@

    Set-Content -Path (Join-Path $ProjectRoot 'HEADER.txt') -Value $header -Encoding UTF8
    Set-Content -Path (Join-Path $ProjectRoot 'LEGAL_NOTICE.md') -Value $legal -Encoding UTF8
    Set-Content -Path (Join-Path $ProjectRoot 'GATEKEEPER.txt') -Value $gk -Encoding UTF8
    Set-Content -Path (Join-Path $ProjectRoot 'agent_manifest.json') -Value $manifest -Encoding UTF8
    Write-BuildLog "  Compliance files provisioned" -Component 'Compliance'
}

# ──────────────────────────────────── Directory layout
function New-EcosystemLayout {
    Write-BuildLog "Provisioning operational directory layout" -Component 'Layout'

    $dirs = @(
        'config', 'config\profiles', 'logs', 'artifacts', 'artifacts\scripts',
        'observatory', 'observatory\telemetry', 'observatory\topology_maps',
        'observatory\visualizations', 'observatory\field_snapshots',
        'observatory\replay', 'tools', 'tools\scripts'
    )

    foreach ($d in $dirs) {
        $p = Join-Path $ProjectRoot $d
        if (-not (Test-Path $p)) {
            New-Item -ItemType Directory -Path $p -Force | Out-Null
            Write-BuildLog "  Created: $d" -Component 'Layout'
        }
    }

    # Verify the Rust source tree exists
    $rustRoot = Join-Path $script:BreakthroughDir 'mod.rs'
    if (Test-Path $rustRoot) {
        Write-BuildLog "  OK: Rust lib root found at breakthrough/mod.rs" -Component 'Layout'
    } else {
        throw "Rust source tree not found at $rustRoot. Cannot proceed without source."
    }

    # Verify Cargo.toml
    $cargoToml = Join-Path $script:BreakthroughDir 'Cargo.toml'
    if (Test-Path $cargoToml) {
        Write-BuildLog "  OK: Cargo.toml found" -Component 'Layout'
    } else {
        throw "Cargo.toml not found at $cargoToml"
    }

    Write-BuildLog "Directory layout complete: $($dirs.Count) paths" -Component 'Layout'
}

# ──────────────────────────────────── Configuration store
function Initialize-Configuration {
    Write-BuildLog "Initializing ecosystem configuration store" -Component 'Config'

    $config = [ordered]@{
        ecosystem   = [ordered]@{
            name        = 'Breakthrough Cognitive Ecosystem'
            version     = $script:EcosystemVersion
            description = 'Self-organizing cognitive ecosystem built on Kernel Technology'
            cargoManifest = Join-Path $script:BreakthroughDir 'Cargo.toml'
        }

        kernel = [ordered]@{
            topologyNodeCount     = 10
            resonanceFrequency    = 0.5
            coherenceTarget       = 0.8
            schedulerMaxTicks     = 1000
            evolutionMutationRate = 0.01
        }

        memory = [ordered]@{
            recallQuery           = @(0.8, 0.2)
            recallThreshold       = 0.65
            reconstructionWindow  = 0.5
        }

        perception = [ordered]@{
            visualResolution     = [ordered]@{ width = 64; height = 64 }
            visualRefreshRateHz  = 30.0
            sensoryChannels      = @('visual', 'auditory', 'tactile', 'proprioceptive')
        }

        metaReason = [ordered]@{
            depth          = 3
            strategy       = 'Heuristic'
        }

        reality = [ordered]@{
            measurementId        = 'cog-output-001'
            measurementScalar    = 0.75
            measurementUncertainty = 0.05
            calibrationIterations = 100
        }

        horizons = [ordered]@{
            explorationId      = 1
            explorationBearing = 0.0
            explorationDepth   = 100
            discoveryId        = 2
            discoveryConfidence = 0.9
            discoveryDistance  = 10.0
        }

        archives = [ordered]@{
            extinctSpecies = @(
                [ordered]@{
                    id                   = 1
                    name                 = 'Neanderthal'
                    extinctionEra        = 'Pleistocene'
                    recoveryPotential    = 0.9
                    confidence           = 0.8
                }
            )
        }

        build = [ordered]@{
            configuration = $Configuration
            target        = 'x86_64-pc-windows-msvc'
            logDir        = Join-Path $ProjectRoot 'logs'
            artifactDir   = Join-Path $ProjectRoot 'artifacts'
        }
    }

    $json = $config | ConvertTo-Json -Depth 10
    Set-Content -Path $script:ConfigPath -Value $json -Encoding UTF8
    Write-BuildLog "  Configuration written to $script:ConfigPath" -Component 'Config'
}

# ──────────────────────────────────── Verification
function Test-LayoutIntegrity {
    Write-BuildLog "Verifying directory layout integrity" -Component 'Verify'

    $expected = @('config','logs','artifacts','observatory','tools',
                  'breakthrough\mod.rs','breakthrough\Cargo.toml','breakthrough\src\main.rs')
    $ok = $true
    foreach ($p in $expected) {
        $full = Join-Path $ProjectRoot $p
        if (-not (Test-Path $full)) {
            Write-BuildLog "  FAIL: $p missing" -Component 'Verify'
            $ok = $false
        }
    }
    if ($ok) {
        Write-BuildLog "  All expected paths present" -Component 'Verify'
    } else { throw "Layout integrity check failed" }
}

# ──────────────────────────────────── Pipeline
function Invoke-Phase1 {
    Initialize-Logger
    Write-BuildLog "Starting Phase 1 pipeline..." -Component 'Pipeline'

    Test-ComplianceFiles
    New-EcosystemLayout
    Initialize-Configuration
    Test-LayoutIntegrity

    Write-BuildLog "Phase 1 complete. Configuration: $script:ConfigPath" -Component 'Pipeline'
    Write-BuildLog "Phase 1 SUCCEEDED" -Component 'Pipeline'
}

try { Invoke-Phase1; exit 0 }
catch {
    Write-BuildLog "FATAL: $($_.Exception.Message)" -Component 'Pipeline'
    Write-BuildLog "=== Phase 1 FAILED ===" -Component 'Pipeline'
    exit 1
}
