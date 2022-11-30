param (
	[Parameter(Position=0)]
    [String] 
    $Day,

	[Parameter(Position=1)]
    [String] 
    $Name
)

$Folder = $Day + "_" + $Name

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
	(Get-Content ./$Folder/Cargo.toml).replace("template", $Name) | Set-Content ./$Folder/Cargo.toml
}

