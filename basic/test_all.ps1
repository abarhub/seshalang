function Execute-Programme
{
    param (
        $parametres,
        $resultatAttendu
    )
    $erreur=$false

    Write-Host "programm: $parametres"

    Write-Host "attendu: $resultatAttendu"

    $out = New-TemporaryFile
    $err = New-TemporaryFile

    Write-Host "out: $out"
    Write-Host "err: $err"
    try
    {
        $p = Start-Process cargo -ArgumentList "run --package seshalang --bin seshalang -- $parametres" `
                    -NoNewWindow -Wait -PassThru `
                    -RedirectStandardOutput $out -RedirectStandardError $err
        $sortie = Get-Content $out
        $erreurs = Get-Content $err

        $exit=$p.ExitCode

        $sortie=$sortie -join "`n"

        Write-Host "sortie: $sortie"

        if($sortie -ceq $resultatAttendu) {
            if ($exit -eq 0)
            {
                Write-Host "OK"
            } else {
                $Host.UI.WriteErrorLine("Erreur: dans le code retour:`n$exit")
                $erreur=$true
            }
        } else {
            $erreur=$true
            $Host.UI.WriteErrorLine("Erreur: dans le résultat du programme:`n$sortie")
        }

    }
    finally
    {
        Remove-Item $out
        Remove-Item $err
    }
    return $erreur
}

$nbErreur=0

$n=Execute-Programme "exemple_basic/exemple1.bas" " 5 `n 12 "

if ($n) {
    $nbErreur++
}

if ($nbErreur -eq 0){
    Write-Host "Pas d'erreur"
} else {
    Write-Error "Il y a $nbErreur erreurs"
}

