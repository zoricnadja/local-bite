$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
$dbUserLine = Get-Content .env | Where-Object { $_ -match '^POSTGRES_USER=' } | Select-Object -First 1
$dbUser = if ($dbUserLine) { ($dbUserLine -split '=',2)[1].Trim('"',"'") } else { 'postgres' }
foreach ($sourceName in @('auth','raw-materials','productions','products','orders')) {
    $database = $sourceName.Replace('-','_') + '_db'
    & docker exec "$sourceName-db" psql -U $dbUser -d $database -v ON_ERROR_STOP=1 -c 'UPDATE integration_outbox SET published_at=NULL WHERE published_at IS NOT NULL;'
    if ($LASTEXITCODE -ne 0) { throw "Could not schedule replay for $sourceName" }
}
Write-Host 'Retained events scheduled for replay. Business records are unchanged; duplicate and stale events are safe.'
