Lo mejor es usar el hook pre-commit del propio git, no un hook de Claude Code. Lo he probado en un repositorio temporal y bloquea los dos casos. No he tocado el proyecto.

## Por qué un hook de git y no uno de Claude Code

- Git lo ejecuta en el momento del commit, cuando los archivos ya están añadidos, así que revisa exactamente lo que va a entrar. Un hook PreToolUse de Claude Code se ejecuta antes del comando entero. Si el agente escribe git add -A && git commit … en una sola línea, el hook comprobaría el estado anterior al git add y dejaría pasar lo recién añadido.
- Vale también para las personas: es una directriz de empresa, no solo de los agentes.
- El validador ve el motivo: git muestra el mensaje en la salida de git commit, y ahí lo lee el agente.

## El script: .githooks/pre-commit
```
#!/usr/bin/env bash
# Comprueba, antes de cada commit, las directrices de la carpeta directrices/:
# - ningún archivo mayor de 10 MB
# - ningún archivo binario ejecutable
set -eu

limite_de_tamano_en_bytes=$((10 * 1024 * 1024))
hay_incumplimientos=0

while IFS= read -r -d '' ruta_del_archivo; do
    # Se examina la versión preparada para el commit (el índice), no la del disco.
    tamano_en_bytes=$(git cat-file -s ":$ruta_del_archivo")
    if (( tamano_en_bytes > limite_de_tamano_en_bytes )); then
        echo "Supera 10 MB ($tamano_en_bytes bytes): $ruta_del_archivo" >&2
        hay_incumplimientos=1
    fi

    tipo_de_contenido=$(git cat-file blob ":$ruta_del_archivo" | file --brief --mime-type -)
    case "$tipo_de_contenido" in
        application/x-executable | application/x-pie-executable | application/x-sharedlib | \
        application/x-dosexec | application/vnd.microsoft.portable-executable | application/x-mach-binary)
            echo "Binario ejecutable ($tipo_de_contenido): $ruta_del_archivo" >&2
            hay_incumplimientos=1
            ;;
    esac
done < <(git diff --cached --name-only --diff-filter=ACMRT -z)

if (( hay_incumplimientos )); then
    echo "Commit bloqueado: incumple las directrices de la carpeta directrices/." >&2
    exit 1
fi
```

- Tamaño: mide la versión preparada para el commit (git cat-file -s), no la del disco. He tomado 10 MB como 10 × 1024 × 1024 bytes.
- Binarios: identifica el formato por el contenido con file (ejecutables de Linux, Windows y macOS). No se fija en el permiso de ejecución, así que los scripts como este mismo pasan sin problema.
- Activación: después de guardarlo y darle permiso de ejecución (chmod +x), hay que ejecutar una vez git config core.hooksPath .githooks. Ese ajuste no se versiona, así que cada copia del repositorio tiene que hacerlo; conviene apuntarlo en el README o en CLAUDE.md.

## Resultado de la prueba

Caso	Resultado
Texto + el propio script ejecutable	Pasa
Archivo de 11 MB	Bloqueado
Binario ELF con espacios en el nombre	Bloqueado
git add -A && git commit con un script .sh	Pasa
git add -A && git commit con un binario	Bloqueado

## Qué añadir en Claude Code

- Cerrar el atajo: el hook se puede saltar con git commit --no-verify. Puedes añadir "Bash(git commit *--no-verify*)" a deny en settings.json e indicárselo al validador. No es infalible, porque también se salta con -n o con git -c core.hooksPath=….
- Una ventaja extra: el validador ya no necesita abrir documentacion/ para comprobar tamaños, porque lo hace git. Eso encaja con tu regla nueva de CLAUDE.md.
- Si aun así quieres un hook de Claude Code: sería un PreToolUse con "matcher": "Bash" e "if": "Bash(git commit *)", que ejecute el mismo script y termine con código 2 para bloquear. Pero tendría el problema de git add … && git commit descrito arriba.

¿Lo añado al proyecto (el script, su activación y la regla deny)?