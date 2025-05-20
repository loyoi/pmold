set windows-shell := ["powershell.exe", "-Command"]

@link:
  gsudo New-Item -ItemType SymbolicLink -Path "E:/mybin/pmold.exe" -Target (Resolve-Path "./target/release/pmold.exe")