<#
.SYNOPSIS
    Finder install script.
.DESCRIPTION
    This script is used to install Finder on Windows from the command line.

    It is an adapted version of
    https://github.com/prefix-dev/pixi/blob/main/install/install.ps1.
.PARAMETER FinderVersion
    Specifies the version of Finder to install. The default value is 'latest'. You can
    also specify it by setting the environment variable 'PIXI_VERSION'.
.PARAMETER FinderHome
    Specifies Finder's home directory. The default value is $PWD'. You can also specify
    it by setting the environment variable 'FINDER_HOME'.
.PARAMETER NoPathUpdate
    If specified, the script will not update the PATH environment variable.
.LINK
    https://github.com/tobiasraabe/hamburg-einbuergerungstest-terminfinder
.NOTES
    Version: v0.0.4
#>
param (
    [string] $FinderVersion = 'latest',
    [string] $FinderHome = "$PWD",
    [switch] $NoPathUpdate
)

Set-StrictMode -Version Latest

function Publish-Env {
    if (-not ("Win32.NativeMethods" -as [Type])) {
        Add-Type -Namespace Win32 -Name NativeMethods -MemberDefinition @"
[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(
    IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam,
    uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
"@
    }

    $HWND_BROADCAST = [IntPtr] 0xffff
    $WM_SETTINGCHANGE = 0x1a
    $result = [UIntPtr]::Zero

    [Win32.Nativemethods]::SendMessageTimeout($HWND_BROADCAST,
        $WM_SETTINGCHANGE,
        [UIntPtr]::Zero,
        "Environment",
        2,
        5000,
        [ref] $result
    ) | Out-Null
}

if ($Env:PIXI_VERSION) {
    $FinderVersion = $Env:PIXI_VERSION
}

if ($Env:FINDER_HOME) {
    $FinderHome = $Env:FINDER_HOME
}

if ($Env:PIXI_NO_PATH_UPDATE) {
    $NoPathUpdate = $true
}

# Repository name
$REPO = 'tobiasraabe/hamburg-einbuergerungstest-terminfinder'
$ARCH = 'x86_64'
$PLATFORM = 'pc-windows-msvc'

$BINARY = "finder-$ARCH-$PLATFORM"

if ($FinderVersion -eq 'latest') {
    $DOWNLOAD_URL = "https://github.com/$REPO/releases/latest/download/$BINARY.zip"
} else {
    $DOWNLOAD_URL = "https://github.com/$REPO/releases/download/$FinderVersion/$BINARY.zip"
}

$BinDir = Join-Path $FinderHome ''

Write-Host "This script will automatically download and install Finder ($FinderVersion) for you."
Write-Host "Getting it from this url: $DOWNLOAD_URL"
Write-Host "The binary will be installed into '$BinDir'"

$TEMP_FILE = [System.IO.Path]::GetTempFileName()

try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TEMP_FILE

    # Create the install dir if it doesn't exist
    if (!(Test-Path -Path $BinDir)) {
        New-Item -ItemType Directory -Path $BinDir | Out-Null
    }

    $ZIP_FILE = $TEMP_FILE + ".zip"
    Rename-Item -Path $TEMP_FILE -NewName $ZIP_FILE

    # Extract finder from the downloaded zip file
    Expand-Archive -Path $ZIP_FILE -DestinationPath $BinDir -Force
} catch {
    Write-Host "Error: '$DOWNLOAD_URL' is not available or failed to download"
    exit 1
} finally {
    Remove-Item -Path $ZIP_FILE
}
