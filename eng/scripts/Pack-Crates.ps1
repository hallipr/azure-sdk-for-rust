#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$OutputPath,
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')

if ($OutputPath) {
  $OutputPath = New-Item -ItemType Directory -Path $OutputPath -Force | Select-Object -ExpandProperty FullName
}

function Get-OutputPackageNames($workspacePackageNames) {
  $names = @()

  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      $names = $PackageNames
    }

    'PackageInfo' {
      $packageInfoFiles = Get-ChildItem -Path $PackageInfoDirectory -Filter '*.json' -File
      foreach ($packageInfoFile in $packageInfoFiles) {
        $packageInfo = Get-Content -Path $packageInfoFile.FullName | ConvertFrom-Json
        $names += $packageInfo.name
      }
    }

    default {
      return $workspacePackageNames
    }
  }

  foreach ($name in $names) {
    if (-not $workspacePackageNames.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace"
      exit 1
    }
  }

  return $names
}

function Get-CargoMetadata() {
  cargo metadata --no-deps --format-version 1 --manifest-path "$RepoRoot/Cargo.toml" | ConvertFrom-Json -Depth 100 -AsHashtable
}

function Get-PackagesToBuild() {
  $metadata = Get-CargoMetadata
  $outputPackageNames = Get-OutputPackageNames $metadata.packages.name

  # We start with output packages, then recursively add unreleased dependencies to the list of packages that need to be built
  [array]$packagesToBuild = $metadata.packages | Where-Object { $outputPackageNames.Contains($_.name) }

  return $packagesToBuild
}

Push-Location $RepoRoot
try {
  #$localRegistryPath = Initialize-VendorDirectory

  [array]$packages = Get-PackagesToBuild

  #Add-PathVersions $packages

  $packageArgs = $packages | ForEach-Object { "--package $($_.name)" } | Join-String -Separator ' '

  Invoke-LoggedCommand `
    -GroupOutput `
    -Command "cargo +nightly -Zpackage-workspace publish --dry-run $packageArgs"

  if ($OutputPath) {
    foreach ($package in $packages) {
      $packageName = $package.name
      $packageVersion = $package.version

      $packageOutputPath = "$OutputPath/$packageName"
      $targetPackagePath = "$RepoRoot/target/package/$packageName-$packageVersion"

      if (Test-Path -Path $packageOutputPath) {
        Remove-Item -Path $packageOutputPath -Recurse -Force
      }

      Write-Host "Copying package '$packageName' to '$packageOutputPath'"
      New-Item -ItemType Directory -Path $packageOutputPath -Force | Out-Null
      Copy-Item -Path $targetPackagePath/* -Destination $packageOutputPath -Recurse
    }
  }
}
finally {
  Pop-Location
}
