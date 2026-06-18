param(
  [Parameter(Mandatory = $true)]
  [string]$Repo,
  [string]$Branch = "main",
  [string]$Tag = "v0.1.0",
  [string]$CommitMessage = "Prepare CamJongUn SDK release automation"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

gh auth status | Out-Null

if (-not (Test-Path ".git")) {
  git init
}

$currentBranch = git branch --show-current
if (-not $currentBranch) {
  git checkout -b $Branch
} elseif ($currentBranch -ne $Branch) {
  git branch -M $Branch
}

if (-not (git remote get-url origin 2>$null)) {
  git remote add origin $Repo
} else {
  git remote set-url origin $Repo
}

git add -A
git commit -m $CommitMessage
git push -u origin $Branch
git tag -f $Tag
git push -f origin $Tag

Write-Host "Pushed $Branch and $Tag to $Repo. The release workflow will build and publish GitHub release artifacts."
