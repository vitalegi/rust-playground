param (
    [String]$project_name="placeholder"
)
Write-Host "Project: $project_name"

cargo new $project_name
Write-Host ""
Write-Host "cd $project_name"
cd $project_name
