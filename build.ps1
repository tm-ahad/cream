$name = "cream"
$joined = $args[1..($args.Count - 1)] -join ","
cargo build $joined 

# if (Test-Path "/usr/bin/cream" -PathType Container) {
#     Remove-Item "/usr/bin/cream" -Recurse -Force
# }

if ($args -contains "--release") {
    Move-Item -Path ./target/release/$name -Destination /usr/local/bin/$name -Force
} else {
    Move-Item -Path ./target/debug/$name -Destination /usr/local/bin/$name -Force
}
