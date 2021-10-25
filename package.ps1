$ErrorActionPreference = "Stop"

$outputDir = "./release"
$licenseFile = Join-Path $outputDir "credits.html"

cargo build --release

if (!$?) {
    throw "build failed"
}

if (Test-Path $outputDir) {
    Remove-Item $outputDir -Recurse
}

New-Item $outputDir -ItemType Directory
Copy-Item "./target/release/lonely-star.exe" $outputDir
Copy-Item "./SDL2.dll" $outputDir
Copy-Item "./resources" $outputDir -Recurse -Exclude icon.ico

cargo about generate credits.hbs > $licenseFile

if (!$?) {
    throw "failed to get licenses"
}