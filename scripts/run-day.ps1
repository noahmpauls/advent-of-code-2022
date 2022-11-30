param (
	[Parameter(Position=0)]
    [String] 
	$PackageFilter,

	[Parameter(Position=1)]
    [String]
	$Part,

	[Parameter(Position=2)]
    [String]
	$Data,

	[Switch]
	$release = $false,

	[Switch]
	$skipbuild = $false
)

$Packages = Get-ChildItem -Directory -Filter "*${PackageFilter}*"
if ($Packages.Count -eq 0) {
	Write-Host "no packages match that filter"
} elseif ($Packages.Count -gt 1) {
	Write-Host "multiple packages match that filter"
} else {
	$PackageFolder = $Packages[0].Name
	if ($PackageFolder -match '^[0-9]{2}_') {
		$SplitFile = $PackageFolder -split "_",2
		$Package = $SplitFile[1]
	}
	if ($release) {
		if (!($skipbuild) -or !(Test-Path -Path "./target/release/$Package.exe" -PathType leaf)) {
			cargo build --package $Package --release
		}
		& ".\target\release\$Package.exe" --part $Part --file ".\$PackageFolder\input\$Data.txt"
	} else {
		cargo run --package $Package -- --part $Part --file ".\$PackageFolder\input\$Data.txt"
	}
}

