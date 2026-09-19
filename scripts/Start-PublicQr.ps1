param([switch]$Build)
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
$composeArgs = @('compose','-f','docker-compose.yml','-f','docker-compose.public.yml')
if ($Build) {
    & docker @composeArgs build public-gateway
    if ($LASTEXITCODE -ne 0) { throw 'Public frontend build failed.' }
}
& docker @composeArgs up -d --no-build public-gateway public-tunnel
if ($LASTEXITCODE -ne 0) { throw 'Could not start public QR access.' }
$tunnelId = & docker @composeArgs ps -q public-tunnel
$startedAt = & docker inspect --format '{{.State.StartedAt}}' $tunnelId
$publicUrl = $null
for ($attempt=0; $attempt -lt 30; $attempt++) {
    # Docker emits normal container logs on stderr; Windows PowerShell must not
    # treat those log lines as terminating errors.
    $ErrorActionPreference = 'Continue'
    $logText = (& docker logs --since $startedAt $tunnelId 2>&1 | Out-String)
    $logsExitCode = $LASTEXITCODE
    $ErrorActionPreference = 'Stop'
    if ($logsExitCode -ne 0) { throw 'Could not read tunnel logs.' }
    $matchesFound = [regex]::Matches($logText, 'https://[a-z0-9-]+\.trycloudflare\.com')
    if ($matchesFound.Count -gt 0) { $publicUrl = $matchesFound[$matchesFound.Count-1].Value; break }
    Start-Sleep -Seconds 2
}
if (!$publicUrl) { throw 'Tunnel did not return a public URL. Check Docker logs for public-tunnel.' }
$envPath = Join-Path (Get-Location) '.env'
$envContents = [System.IO.File]::ReadAllText($envPath)
$setting = "PUBLIC_TRACE_URL=$publicUrl/trace"
if ($envContents -match '(?m)^PUBLIC_TRACE_URL=') {
    $envContents = [regex]::Replace($envContents, '(?m)^PUBLIC_TRACE_URL=[^\r\n]*', $setting)
} else { $envContents += "`n$setting`n" }
[System.IO.File]::WriteAllText($envPath,$envContents,[System.Text.UTF8Encoding]::new($false))
& docker compose up -d --no-build --no-deps products-service
if ($LASTEXITCODE -ne 0) { throw 'Could not synchronize QR base URL.' }
& docker compose restart nginx
if ($LASTEXITCODE -ne 0) { throw 'Could not refresh local gateway.' }
Write-Host "Public QR base: $publicUrl/trace"
Write-Host 'Open/download each QR again to use the current URL. Previously printed codes keep their original URL.'
Write-Host 'Keep this computer, Docker and the tunnel running. For permanent printed codes, use a stable hostname.'
