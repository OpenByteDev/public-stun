$BaseUrl = "https://raw.githubusercontent.com/pradt2/always-online-stun/refs/heads/master"
$FolderPath = "./packaged"
$Failed = $false

Write-Host "Updating files in: $FolderPath"
Write-Host "Using base URL: $BaseUrl"
Write-Host ""

Get-ChildItem -Path $FolderPath -File | ForEach-Object {
    $localFile = $_
    $remoteUrl = "$BaseUrl/$($localFile.Name)"

    try {
        Write-Host "Updating $($localFile.Name)..."

        Invoke-WebRequest `
            -Uri $remoteUrl `
            -OutFile $localFile.FullName `
            -UseBasicParsing `
            -ErrorAction Stop

        Write-Host "  ✔ Updated"
    }
    catch {
        Write-Warning "  ✖ Failed to update $($localFile.Name): $($_.Exception.Message)"
        $script:Failed = $true
    }
}

Write-Host ""
if ($Failed) {
    Write-Error "Some files failed to update."
    exit 1
}
Write-Host "Done."
