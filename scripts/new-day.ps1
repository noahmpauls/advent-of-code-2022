param (
    [String] $D,
    [String] $N
)

$Folder = $D + "_" + $N

if (!(Test-Path -Path "./target/release/tools.exe" -PathType leaf)) {
	Write-Host "building tools..."
	cargo build --package tools --release
}
.\target\release\tools.exe add-member $Folder

if (Test-Path -Path "./$Folder" -PathType Container) {
    Write-Host "folder $Folder already exists"
} else {
	mkdir $Folder
    Copy-Item -Path "./template/*" -Destination $Folder -Recurse
	(Get-Content ./$Folder/Cargo.toml).replace("template", $N) | Set-Content ./$Folder/Cargo.toml
}

