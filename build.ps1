docker-compose down
docker-compose run -d --service-port nginx

$update = 2

$pwd = Get-Location
$pwd = $pwd.Path

$watcher_toml = [System.IO.FileSystemWatcher]::new()
$watcher_toml.Path = $pwd
$watcher_toml.Filter = "Cargo.toml"
$watcher_toml.NotifyFilter = [System.IO.NotifyFilters]::LastWrite -bor [System.IO.NotifyFilters]::FileName -bor [System.IO.NotifyFilters]::DirectoryName
$_ = Register-ObjectEvent -InputObject $watcher_toml -EventName Changed -Action { $global:update = 2 }

$watcher_src = [System.IO.FileSystemWatcher]::new()
$watcher_src.Path = [System.IO.Path]::Combine($pwd, "src")
$watcher_src.NotifyFilter = [System.IO.NotifyFilters]::LastWrite -bor [System.IO.NotifyFilters]::FileName -bor [System.IO.NotifyFilters]::DirectoryName
$watcher_src.IncludeSubdirectories = $true
$_ = Register-ObjectEvent -InputObject $watcher_src -EventName Changed -Action { $global:update = 2 }
$_ = Register-ObjectEvent -InputObject $watcher_src -EventName Created -Action { $global:update = 2 }
$_ = Register-ObjectEvent -InputObject $watcher_src -EventName Deleted -Action { $global:update = 2 }
$_ = Register-ObjectEvent -InputObject $watcher_src -EventName Renamed -Action { $global:update = 2 }

$watcher_node = [System.IO.FileSystemWatcher]::new()
$watcher_node.Path = [System.IO.Path]::Combine($pwd, "node/src")
$watcher_node.NotifyFilter = [System.IO.NotifyFilters]::LastWrite -bor [System.IO.NotifyFilters]::FileName -bor [System.IO.NotifyFilters]::DirectoryName
$watcher_node.IncludeSubdirectories = $true
$_ = Register-ObjectEvent -InputObject $watcher_node -EventName Changed -Action { if ($global:update -lt 1) { $global:update = 1 } }
$_ = Register-ObjectEvent -InputObject $watcher_node -EventName Created -Action { if ($global:update -lt 1) { $global:update = 1 } }
$_ = Register-ObjectEvent -InputObject $watcher_node -EventName Deleted -Action { if ($global:update -lt 1) { $global:update = 1 } }
$_ = Register-ObjectEvent -InputObject $watcher_node -EventName Renamed -Action { if ($global:update -lt 1) { $global:update = 1 } }

$watcher_toml.EnableRaisingEvents = $true
$watcher_src.EnableRaisingEvents = $true
$watcher_node.EnableRaisingEvents = $true

while ($true) {
    Write-Output "Waiting for update."
    while ($update -eq 0) {
        Start-Sleep 1
    }
    $tmp = $update
    $update = 0
    Write-Output "Some file updated."
    Write-Output "Let's build!"
    $to_cargo = $tmp -gt 1
    $to_node = $true
    if ($to_cargo) {
        docker-compose run --rm cargo
        $to_node = $to_node -and $?
    }
    if ($to_node) {
        docker-compose run --rm nodewebpack
    }
}