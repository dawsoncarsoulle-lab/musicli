# install.ps1

Write-Host "🎵 Installation de MusicLI pour Windows" -ForegroundColor Cyan
Write-Host "======================================"
Write-Host ""

# 1. Définir le dossier d'installation (C:\Users\VotreNom\bin)
$InstallDir = "$env:USERPROFILE\bin"
$SourceFile = "target\x86_64-pc-windows-gnu\release\musicli.exe"

# Vérifier si le fichier compilé existe
if (-not (Test-Path $SourceFile)) {
    Write-Error "Le fichier musicli.exe est introuvable dans $SourceFile."
    Write-Host "Avez-vous lancé la compilation Docker ?" -ForegroundColor Yellow
    exit 1
}

# 2. Créer le dossier bin s'il n'existe pas
if (-not (Test-Path $InstallDir)) {
    Write-Host "1️⃣  Création du dossier $InstallDir..."
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
} else {
    Write-Host "1️⃣  Dossier $InstallDir déjà existant."
}

# 3. Copier l'exécutable
Write-Host "2️⃣  Copie de l'exécutable..."
Copy-Item -Path $SourceFile -Destination "$InstallDir\musicli.exe" -Force
Write-Host "   ✓ Fichier copié avec succès." -ForegroundColor Green

# 4. Ajouter au PATH (de manière permanente pour l'utilisateur)
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "3️⃣  Ajout au PATH..."
    # Sur Windows, le séparateur est ; et non :
    $NewPath = "$UserPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "   ✓ PATH mis à jour." -ForegroundColor Green
    Write-Host "   ⚠️  IMPORTANT : Redémarrez votre terminal pour que la commande fonctionne." -ForegroundColor Yellow
} else {
    Write-Host "3️⃣  Le dossier est déjà dans le PATH."
}

Write-Host ""
Write-Host "✅ Installation terminée !" -ForegroundColor Green
