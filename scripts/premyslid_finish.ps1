<#
.SYNOPSIS
    Finish a Premyslid content session: validate, commit, push, open a pull request.

.DESCRIPTION
    Refuses to commit if the content does not validate, or if anything outside the two
    writable folders has been changed. Continuous integration checks both again, but failing
    here is faster and gives the model a chance to fix its own work.

    The commit carries a Model trailer. That trailer is the only record of which model wrote
    which content; git preserves everything else, but not that.

        pwsh scripts/premyslid_finish.ps1 -Model qwen2.5-coder:7b
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $Model,

    [switch] $NoPullRequest
)

$ErrorActionPreference = 'Stop'

$repositoryRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repositoryRoot

$taskId = if (Test-Path '.premyslid-session') { (Get-Content '.premyslid-session' -Raw).Trim() } else { 'unknown' }

Push-Location 'python'
try {
    & python -m suvorov.tools.validate
    if ($LASTEXITCODE -ne 0) {
        throw "The content does not validate. Fix the problems above, then run this script again."
    }
}
finally {
    Pop-Location
}

$changed = git status --porcelain | ForEach-Object { $_.Substring(3).Trim('"') } | Where-Object { $_ -ne '.premyslid-session' }
if (-not $changed) {
    throw "Nothing changed. If the task could not be done, write games/premyslid/research/blocked-$taskId.md and run this again."
}

$allowed = $changed | Where-Object { $_ -like 'games/premyslid/content/*' -or $_ -like 'games/premyslid/research/*' }
$forbidden = $changed | Where-Object { $_ -notin $allowed }
if ($forbidden) {
    Write-Host 'These files are outside the writable folders:' -ForegroundColor Red
    $forbidden | ForEach-Object { Write-Host "  $_" }
    throw "Revert them with 'git restore <file>'. Only content/ and research/ may be changed by a content session."
}

git add 'games/premyslid/content' 'games/premyslid/research'

$message = @"
content(premyslid): $taskId

Written by an automated content session. The task text came from
python -m suvorov.tools.next and the content validates against
games/premyslid/schema.

Task: $taskId
Model: $Model
"@

git commit -m $message

$branch = (git rev-parse --abbrev-ref HEAD).Trim()
git push -u origin $branch

if (-not $NoPullRequest) {
    if (Get-Command gh -ErrorAction SilentlyContinue) {
        gh pr create --fill --base main --head $branch
    }
    else {
        Write-Host "The gh CLI is not installed, so no pull request was opened. The branch is pushed." -ForegroundColor Yellow
    }
}

Remove-Item '.premyslid-session' -ErrorAction SilentlyContinue
Write-Host ''
Write-Host "Session finished: $taskId by $Model." -ForegroundColor Green
Write-Host 'Stop here. Do not start another task in this session.'
