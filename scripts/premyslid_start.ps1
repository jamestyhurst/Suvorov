<#
.SYNOPSIS
    Start a Premyslid content session: make a branch and print one task.

.DESCRIPTION
    Content sessions are run by small models that are far better at writing a file than at
    driving git. This script does the git work so the model only has to edit files. It picks
    a task, makes a branch named after it, and prints the task.

    Run from the repository root:
        pwsh scripts/premyslid_start.ps1
#>

[CmdletBinding()]
param(
    [int] $Seed
)

$ErrorActionPreference = 'Stop'

$repositoryRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repositoryRoot

if (-not (Test-Path 'games/premyslid/plan/regions.json')) {
    throw "Run this from the Suvorov repository: games/premyslid was not found."
}

# Start from the latest main so that the task list reflects work already merged.
$currentBranch = (git rev-parse --abbrev-ref HEAD).Trim()
if ($currentBranch -ne 'main') {
    Write-Host "Switching from '$currentBranch' to main." -ForegroundColor Yellow
    git switch main
}
git pull --ff-only

Push-Location 'python'
try {
    $arguments = @('-m', 'suvorov.tools.next', '--json')
    if ($PSBoundParameters.ContainsKey('Seed')) { $arguments += @('--seed', $Seed) }
    $response = & python @arguments | ConvertFrom-Json
}
finally {
    Pop-Location
}

if ($null -eq $response.task) {
    Write-Host "No tasks outstanding. Tell James: the queue is empty, which should not happen."
    exit 0
}

$taskId = $response.task.task_id
$branch = "content/$taskId"

if ((git branch --list $branch)) {
    # Someone already worked this task locally. Give the branch a distinct name rather than
    # resuming a branch whose state is unknown.
    $branch = "$branch-$(git rev-parse --short HEAD)"
}
git switch -c $branch

Set-Content -Path '.premyslid-session' -Value $taskId -Encoding utf8

if ($response.milestone) {
    Write-Host ''
    Write-Host $response.milestone -ForegroundColor Cyan
}

Write-Host ''
Write-Host "Branch: $branch"
Write-Host ''
Write-Host ($response.task.what)
Write-Host ''
Write-Host "File:   $($response.task.file)"
if ($response.task.schema) { Write-Host "Schema: $($response.task.schema)" }
if ($response.task.rules) {
    Write-Host ''
    Write-Host 'Rules:'
    foreach ($rule in $response.task.rules) { Write-Host "  - $rule" }
}
Write-Host ''
Write-Host 'When the files are written, run:  pwsh scripts/premyslid_finish.ps1 -Model <your model name>'
