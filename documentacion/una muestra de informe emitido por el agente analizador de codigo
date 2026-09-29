Se ha lanzado `@.claude/agents/analizar_codigo_y_determinar_estructura.md`

# El resumen del agente principal que gestiona la sesión

El agente ha terminado correctamente (**OK**). Ha escrito el análisis en [vista_general_del_codigo-20260929T214603-.md](trabajo/analisis/vista_general_del_codigo-20260929T214603-.md) (246 líneas, sobre el commit `0b8783a`). Como avisé, el nombre del archivo acaba en `-.md`.

**Estado del código:** según el agente, `cargo fmt --check` y `cargo clippy` no dan avisos y pasan los 100 tests. Los nombres son claros y siguen el glosario. El dominio no depende de los adaptadores.

**Problemas que señala, de más a menos grave:**
1. **Si falla el registro en el histórico, el estado queda incoherente.** El ascensor se mueve antes de registrar el movimiento. Si el registro falla, se devuelve el error, pero el ascensor ya se ha movido y el histórico no lo recoge. Lo he confirmado en [control_de_trafico.rs:66-78](src/dominio/control_de_trafico.rs#L66-L78) y en [control_de_trafico.rs:88-100](src/dominio/control_de_trafico.rs#L88-L100). Es el único problema que da un resultado incorrecto hoy, y ya lo tienes como tarea pendiente.
2. **La elección del ascensor está en el sitio equivocado.** Según el glosario la decide el control de tráfico, pero la hace `Edificio`. Además, la regla del "más cercano" está duplicada en los dos.
3. **Cualquiera puede mover un ascensor sin registrarlo.** [edificio.rs:103](src/dominio/edificio.rs#L103) es público y permite moverlo aunque esté ocupado, sin pasar por el histórico. Además, libre/ocupado es un `bool` y las plantas y el número de ascensores están fijos en constantes.
4. **`ControlDeTrafico` tiene tres responsabilidades:** asignar llamadas, registrar movimientos y optimizar posiciones. Esto va contra la S y la O de SOLID.
5. **No hay puertos de entrada.** La interfaz gráfica depende directamente de `ControlDeTrafico` y de cómo están anidados sus errores.
6. **La "llamada" no está modelada** aunque está en el glosario. Las llamadas que no se atienden no dejan rastro.

**Por dónde empezar, según el agente:** por el punto 1. Es acotado y ya existe el doble de test `HistoricoQueSiempreFalla` para probarlo. Antes hay que decidir cómo debe comportarse: ¿el ascensor no se mueve si falla el registro, o se deshace el movimiento? Después irían los puntos 2 y 3. El informe marca qué sugerencias añaden términos nuevos al dominio y conviene que las valides tú.

Hay dos cosas nuevas sin commitear: el informe y la memoria del agente en `.claude/agent-memory/analizar_codigo_y_determinar_estructura/` (4 archivos). El agente no ha tocado el código.

# El informe que ha escrito el subagente analizador

# Vista general del código: simulador de ascensores

- **Fecha del análisis:** 2026-09-29 21:46:03
- **Rama y commit analizados:** `pruebas_con_el_bucle_automatico`, `0b8783a` ("registro de lo realizado"), con el árbol de trabajo limpio.
- **Alcance:** todo el código de `src/`, `Cargo.toml`, `main.rs`, la configuración de `.claude/` y los registros de `trabajo/3_historico/`. Siguiendo `CLAUDE.md`, no he leído `documentacion/`, `trabajo/0_funcionalidades_y_tareas_pendientes/` ni `zz - trozos de codigo descartados - guardados por si acaso/`.

---

## 1. Resumen

Es un proyecto pequeño (unas 2 600 líneas de Rust, de las que algo más de la mitad son tests) y está sano:

- `cargo fmt --check` no encuentra nada que cambiar.
- `cargo clippy --all-targets` no da ningún aviso.
- Pasan los 100 tests de `cargo test`.
- La nomenclatura sigue el glosario y es muy descriptiva, como pide `CLAUDE.md`.
- Ya se ve una arquitectura hexagonal incipiente: el dominio define los puertos `Reloj` y `HistoricoDeMovimientos`, y los adaptadores los implementan.

Los principales problemas no son de estilo, sino de **dónde vive cada responsabilidad** y de **qué invariantes protege el dominio**:

1. **La elección del ascensor que atiende una llamada está en `Edificio`.** El glosario se la asigna al *control de tráfico*. Además, la lógica de "el ascensor libre más cercano" está duplicada en `Edificio` y en `ControlDeTrafico`.
2. **Una operación puede dejar el sistema a medias.** Primero se mueve el ascensor y después se registra el movimiento. Si el registro falla, el ascensor queda movido (y ocupado) pero el movimiento no consta en el histórico, que según el glosario es un "registro permanente de **todos** los movimientos".
3. **`ControlDeTrafico` acumula tres responsabilidades:** asignar llamadas, registrar movimientos y la estadística y optimización de la reubicación. Esto va contra la S y la O de SOLID.
4. **El agregado `Edificio` no protege sus invariantes.** `mover_ascensor_a_la_planta` es público y permite mover un ascensor ocupado, o mover cualquier ascensor sin registrarlo en el histórico.
5. **Falta la capa de aplicación (puertos de entrada).** La interfaz gráfica depende directamente del struct concreto `ControlDeTrafico` y de la estructura interna de sus errores.
6. **La *llamada* no existe como concepto explícito del dominio.** Se deduce de los movimientos con motivo `AtenderUnaLlamada`, así que las llamadas que no se atienden (cuando no hay ningún ascensor libre) no quedan registradas y la optimización infravalora la demanda.

---

## 2. Estado de salud

| Comprobación | Resultado |
|---|---|
| `cargo fmt --check` | Sin cambios pendientes |
| `cargo clippy --all-targets` | Sin avisos |
| `cargo test` | 100 tests; pasan todos |
| Dependencias | Solo `eframe = "0.36.1"`, con `edition = "2024"` |
| Directrices de `trabajo/directrices/` | Se cumplen: no hay secretos, no hay archivos de más de 10 MB y no hay ejecutables versionados. `target/` y `historico_de_movimientos.txt` están en `.gitignore`. |

Reparto del código entre producción y tests, en líneas (aproximado, contando desde `#[cfg(test)]`):

| Archivo | Producción | Tests |
|---|---:|---:|
| `src/dominio/control_de_trafico.rs` | 290 | 428 |
| `src/dominio/edificio.rs` | 182 | 466 |
| `src/dominio/fecha_y_hora.rs` | 188 | 104 |
| `src/dominio/planta.rs` | 22 | 28 |
| `src/dominio/movimiento.rs`, `reloj.rs`, `historico_de_movimientos.rs` | 40 | 0 |
| `src/adaptadores/historico_en_archivo.rs` | 141 | 154 |
| `src/adaptadores/historico_en_memoria.rs`, `reloj_simulado.rs`, `reloj_del_sistema.rs` | 73 | 0 |
| `src/adaptadores/interfaz_grafica/panel_del_simulador.rs` | 92 | 249 |
| `src/adaptadores/interfaz_grafica/ventana_del_simulador.rs` | 69 | 24 |
| `src/main.rs`, `src/lib.rs` y los `mod.rs` | 64 | 0 |

---

## 3. Estructura de carpetas (primeros niveles)

```
pruebas_harness_IA/
├── Cargo.toml, Cargo.lock      Paquete `pruebas_harness`: biblioteca + binario
├── CLAUDE.md                   Directrices del proyecto para los agentes
├── README.md                   Presenta el experimento del harness, no el simulador
├── .gitignore                  Ignora /target y historico_de_movimientos.txt
├── historico_de_movimientos.txt  Histórico real que genera `cargo run` (ignorado por git)
├── .claude/                    Configuración del harness agéntico
│   ├── agents/                 6 agentes (analizar, costuras, evaluar, programar, revisar, validar)
│   ├── skills/programacion_en_bucle_automatico/  Orquestador del bucle de trabajo
│   ├── agent-memory/           Memorias persistentes de los agentes con memoria de proyecto
│   ├── workflows/              Vacía (solo .gitkeep)
│   └── settings.json           Permisos: allow / ask (git commit) / deny (git push, documentacion, directrices)
├── src/                        Código Rust del simulador (ver más abajo)
├── trabajo/                    Tablero kanban del bucle de trabajo
│   ├── 0_funcionalidades_y_tareas_pendientes/  Pendientes (no leída, por CLAUDE.md)
│   ├── 1_listo_para_implementar/                Vacía
│   ├── 2_en_curso/                              Vacía
│   ├── 3_historico/            Especificaciones ya hechas + registros `realizado_*.md` del bucle
│   ├── analisis/               Salida de este agente
│   └── directrices/            Directrices de seguridad y de empresa (solo lectura)
├── documentacion/              Apuntes sobre el harness e imágenes (no leída, por CLAUDE.md)
├── zz - trozos de codigo descartados - guardados por si acaso/  Ignorada por CLAUDE.md
└── target/                     Artefactos de compilación (ignorados)
```

### 3.1 `src/` en detalle

```
src/
├── lib.rs                      Expone los módulos `dominio` y `adaptadores`
├── main.rs                     Raíz de composición: monta ControlDeTrafico + RelojDelSistema +
│                               HistoricoEnArchivo y abre la ventana eframe
├── glosario_de_dominio.md      Lenguaje ubicuo (DDD)
├── dominio/
│   ├── planta.rs               Objeto valor `Planta(i8)` y `distancia_hasta`
│   ├── edificio.rs             `Edificio` (agregado), `IdentificadorDeAscensor`, `ErrorDeEdificio`,
│   │                           struct privado `Ascensor { planta, esta_libre: bool }`.
│   │                           Constantes fijas: plantas -2..7 y 3 ascensores.
│   │                           Contiene también la elección del "libre más cercano".
│   ├── control_de_trafico.rs   Servicio que atiende llamadas, lleva al usuario a su destino,
│   │                           registra movimientos y optimiza la posición de los libres
│   ├── movimiento.rs           `Movimiento` (datos con campos públicos) y `MotivoDelMovimiento`
│   ├── fecha_y_hora.rs         `FechaYHora` hecha a mano (algoritmo de H. Hinnant), `DiaDeLaSemana`,
│   │                           `FranjaHoraria`, "último mes"
│   ├── reloj.rs                PUERTO de salida `Reloj`
│   └── historico_de_movimientos.rs  PUERTO de salida `HistoricoDeMovimientos` + `ErrorDeHistorico`
└── adaptadores/
    ├── reloj_del_sistema.rs    Reloj real en UTC
    ├── reloj_simulado.rs       Reloj de hora fija, compartida con Rc<Cell<…>> (se usa en tests)
    ├── historico_en_memoria.rs Histórico en un Vec (se usa en tests)
    ├── historico_en_archivo.rs Histórico en un archivo de texto `;`-separado, solo añade líneas
    └── interfaz_grafica/
        ├── panel_del_simulador.rs    Modelo de la vista, sin egui: plantas, ascensores por planta,
        │                             último `Aviso` y "pulsar el botón de llamada"
        └── ventana_del_simulador.rs  Capa fina egui/eframe que dibuja el panel
```

### 3.2 Mapa de la arquitectura actual

La sección "Mapa de la arquitectura actual" de `CLAUDE.md` está vacía. Esta es la estructura que se ve hoy:

```
      ADAPTADORES DE ENTRADA                NÚCLEO (src/dominio)               ADAPTADORES DE SALIDA
┌─────────────────────────────┐   ┌──────────────────────────────────┐   ┌──────────────────────────┐
│ VentanaDelSimulador (egui)  │   │  ControlDeTrafico  (struct)      │   │ RelojDelSistema          │
│        │                    │   │    ├── Edificio (agregado)       │   │ RelojSimulado            │
│        ▼                    │──▶│    ├── Box<dyn Reloj> ───────────┼──▶│   implementan Reloj      │
│ PanelDelSimulador           │   │    └── Box<dyn Historico…> ──────┼──▶│ HistoricoEnArchivo       │
│   (depende del struct       │   │  Planta, Movimiento, FechaYHora  │   │ HistoricoEnMemoria       │
│    concreto y de sus errores)│   │  Puertos: Reloj, Historico…     │   │   implementan Historico… │
└─────────────────────────────┘   └──────────────────────────────────┘   └──────────────────────────┘
                     main.rs = raíz de composición (lo monta todo)
```

- **Puertos de salida:** existen, `Reloj` y `HistoricoDeMovimientos`.
- **Puertos de entrada:** no existen. No hay capa de aplicación ni casos de uso como trait.
- **Dirección de las dependencias:** en el código de producción es correcta: `dominio` no importa nada de `adaptadores`, ni de `std::fs` o `std::time`, ni de egui. La única excepción está en los tests de `control_de_trafico.rs` (líneas 294-295), que importan `HistoricoEnMemoria` y `RelojSimulado` desde `adaptadores`.

---

## 4. Principales problemas respecto a `CLAUDE.md`

La severidad es orientativa: **Alta** significa que afecta a la corrección o a una invariante del dominio; **Media**, que afecta al diseño o la mantenibilidad; **Baja**, que es un detalle.

### 4.1 Arquitectura hexagonal (Ports & Adapters)

| # | Sev. | Problema | Evidencia |
|---|---|---|---|
| H1 | Media | **Faltan los puertos de entrada.** La interfaz depende del struct concreto `ControlDeTrafico`. No se puede probar el panel con un doble del caso de uso, y todos sus tests son en realidad tests de integración que pasan por `Edificio`. | `panel_del_simulador.rs:36-39`, `:75-90` |
| H2 | Media | **El adaptador de entrada conoce la estructura interna de los errores del dominio.** Hace `match` sobre `ErrorDeControlDeTrafico::Edificio(ErrorDeEdificio::NingunAscensorLibre)`, así que cambiar cómo se anidan los errores rompe la interfaz. | `panel_del_simulador.rs:84` |
| H3 | Media | **El adaptador de entrada contiene conocimiento del dominio.** Recorre él mismo el rango de plantas (`planta_mas_baja..=planta_mas_alta`) y filtra los ascensores de cada planta. Estas consultas pertenecen al edificio o a un caso de uso de consulta. Ya se propuso `Edificio::plantas()`, según `realizado_20260929T161545.md`. | `panel_del_simulador.rs:54-69` |
| H4 | Baja | **`ControlDeTrafico` expone el puerto `historico()` solo para los tests,** lo que filtra un detalle de infraestructura en su API pública. | `control_de_trafico.rs:56-58`; se usa solo en tests (`:324`, `panel_del_simulador.rs:270`) |
| H5 | Baja | **Los tests del núcleo dependen de adaptadores** (`HistoricoEnMemoria`, `RelojSimulado`). Funciona, pero invierte la regla de dependencias de la hexagonal dentro de los tests. | `control_de_trafico.rs:294-295` |
| H6 | Baja | **Los puertos están mezclados con las entidades en `dominio/`,** sin un submódulo `puertos/` ni una capa `aplicacion/`, así que la arquitectura no se ve en la estructura de carpetas. | `src/dominio/mod.rs` |
| H7 | Baja | **La ruta del histórico es relativa al directorio desde el que se lanza el programa** y está fija en el código. | `main.rs:13` |

### 4.2 Principios SOLID

| # | Sev. | Principio | Problema | Evidencia |
|---|---|---|---|---|
| S1 | Media | **S** (responsabilidad única) | `ControlDeTrafico` hace tres cosas: (a) asignar llamadas y llevar usuarios, (b) registrar en el histórico y (c) contar las llamadas por planta, día y franja y decidir la reubicación. La parte (c) ocupa más de la mitad del código de producción del archivo. | `control_de_trafico.rs:104-232` y `:278-289` |
| S2 | Media | **O** (abierto/cerrado) | Las políticas están fijas: "libre más cercano, desempate por identificador" y "reubicación según día de la semana, franja y 30 días". Para probar otra estrategia, algo natural en un simulador, hay que modificar el código existente. | `edificio.rs:128-150`, `control_de_trafico.rs:192-232` |
| S3 | Media | **O** (abierto/cerrado) | La configuración del edificio son constantes del módulo: `-2`, `7` y `3` ascensores. `Edificio::new()` no admite parámetros, y los tests tienen que dar rodeos para montar un estado concreto (ver `control_con_historico_y_ascensores`, que ocupa un ascensor concreto "engañando" al algoritmo de envío). | `edificio.rs:3-5`, `:59-63`; `control_de_trafico.rs:463-500` |
| S4 | Baja | **I** (segregación de interfaces) | `HistoricoDeMovimientos::movimientos()` devuelve **todo** el histórico en un `Vec`, y la optimización lo filtra después en memoria. En el adaptador de archivo, eso supone releer y parsear el archivo completo cada vez, y el archivo crece sin límite porque "nunca se borra". Un método de consulta más específico, por ejemplo "las llamadas desde tal fecha", encajaría mejor. | `historico_de_movimientos.rs:10-13`; `historico_en_archivo.rs:45-58` |
| S5 | Baja | **D** (inversión de dependencias) | Está relacionado con H1: el módulo de alto nivel de la interfaz depende de una implementación concreta y no de una abstracción. | `panel_del_simulador.rs:1-3` |

### 4.3 DDD y glosario de dominio

| # | Sev. | Problema | Evidencia |
|---|---|---|---|
| D1 | Alta | **Invariante rota entre el edificio y el histórico.** En `atender_llamada_desde_la_planta` primero se muta el `Edificio` (el ascensor se mueve y queda ocupado) y después se registra. Si `registrar` falla, la operación devuelve error pero el ascensor ya está ocupado en la planta de la llamada. Lo mismo ocurre en `llevar_al_usuario_a_su_planta_de_destino`, y en `optimizar_…`, que puede quedarse a medias dentro del bucle. La interfaz muestra entonces "No se pudo atender la llamada" aunque el ascensor sí se ha movido y, como la interfaz no puede liberarlo, se pierde para el resto de la sesión. El problema ya está anotado como pendiente en `realizado_20260929T154454.md`. | `control_de_trafico.rs:66-78`, `:88-100`, `:118-124` |
| D2 | Media | **Responsabilidad mal ubicada.** Según el glosario, el *control de tráfico* "decide qué ascensor atiende cada llamada", pero esa decisión está en `Edificio::enviar_ascensor_libre_mas_cercano_a_la_planta`. | `edificio.rs:124-150` frente al glosario |
| D3 | Media | **Lógica duplicada del "más cercano".** `Edificio` usa `min_by_key(distancia_hasta)` sobre los libres, y `ControlDeTrafico::retirar_el_ascensor_mas_cercano_a_la_planta` hace lo mismo sobre una lista de candidatos. Son dos implementaciones de la misma regla de negocio. | `edificio.rs:136-143`; `control_de_trafico.rs:153-168` |
| D4 | Media | **La *llamada* está en el glosario, pero no está modelada.** No hay un tipo `Llamada`: se deduce de `Movimiento` con motivo `AtenderUnaLlamada`, contando su `planta_de_destino`. Por eso las llamadas que se rechazan por `NingunAscensorLibre` no dejan rastro, y la optimización no ve la demanda insatisfecha, justo la que más interesaría. | `control_de_trafico.rs:216-232`, `:280-289` |
| D5 | Media | **El agregado no protege sus invariantes.** `Edificio::mover_ascensor_a_la_planta` es `pub` y deja mover un ascensor **ocupado**, o cualquier ascensor, sin pasar por el control de tráfico y sin registrar nada en el histórico. En producción solo lo usan el propio `Edificio` y `reubicar_ascensor_libre`; en lo demás se usa para preparar los tests. | `edificio.rs:103-114`; `control_de_trafico.rs:177-178`; `panel_del_simulador.rs:218` |
| D6 | Baja | **El estado libre/ocupado es un `bool`.** El glosario define dos estados con nombre, *libre* y *ocupado*. Un `enum EstadoDelAscensor { Libre, Ocupado }` expresaría mejor el lenguaje ubicuo y permitiría añadir más estados en el futuro. | `edificio.rs:31-34` |
| D7 | Baja | **La entidad *ascensor* está escondida.** `Ascensor` es un struct privado dentro de `edificio.rs`, e `IdentificadorDeAscensor` también vive allí aunque lo usan `movimiento.rs` y los adaptadores. `IdentificadorDeAscensor` es además el índice del array, así que identidad y posición quedan acopladas. | `edificio.rs:7-19`, `:29-41`, `:170-180` |
| D8 | Baja | **Hay que reconstruir la planta de origen.** Como `Edificio` solo devuelve el identificador del ascensor enviado, `ControlDeTrafico` toma antes una "foto" de todas las plantas para saber de dónde salió. Si el agregado devolviera el desplazamiento realizado (algo parecido a un evento de dominio), esta foto no haría falta. | `control_de_trafico.rs:65-72` |
| D9 | Baja | **Días y franjas en UTC.** El reloj real es UTC (`reloj_del_sistema.rs:6`), así que "misma franja horaria" y "mismo día de la semana" se calculan en UTC y no en la hora local del edificio. El archivo `historico_de_movimientos.txt` muestra, por ejemplo, 19:38 cuando la hora local eran las 21:38. Ya está anotado como pendiente. | `reloj_del_sistema.rs:6-17`; `historico_de_movimientos.txt` |

### 4.4 Nomenclatura y estilo

En general está muy bien: los nombres son largos y descriptivos (`numero_de_llamadas_por_planta_en_la_franja_horaria_actual`, `dejar_en_su_sitio_los_ascensores_libres_que_ya_estan_en_una_planta_elegida`), los tests se leen como especificaciones y se usan los términos del glosario. Solo hay detalles menores:

- `FechaYHora::con(...)` y `Planta::con_numero(...)` son nombres cortos comparados con el resto. Algo como `FechaYHora::con_fecha_y_hora(...)` o `desde_componentes(...)` sería más coherente.
- Los errores no implementan `Display` ni `std::error::Error`. La interfaz enseña al usuario el formato `{:?}` (`panel_del_simulador.rs:29`) y `main.rs:20` hace lo mismo.
- `HistoricoEnArchivo` descarta la causa del error de E/S con `map_err(|_| ...)`, así que se pierde el diagnóstico (`historico_en_archivo.rs:40`, `:42`).
- El paquete se llama `pruebas_harness`, que no dice nada del dominio (algo como `simulador_de_ascensores` sí lo haría). Además, el `README.md` no explica cómo usar el simulador.

### 4.5 Flujo de trabajo y documentación

- **`CLAUDE.md` tiene secciones vacías:** "Mapa de la arquitectura actual" y "Flujo de trabajo". Los agentes no tienen escrito dónde va cada cosa ni qué ciclo seguir; este documento puede servir de base para rellenar el mapa.
- **No se siguió TDD:** según los registros de `trabajo/3_historico/`, el ciclo rojo/verde no se hizo test a test en ninguna de las tres funcionalidades.
- **Nadie ha abierto la ventana real:** el entorno de los agentes no tiene pantalla, así que la interfaz no se ha probado a mano (lo dice `realizado_20260929T161545.md`).
- **Solo un caso de uso llega a la aplicación:** de los tres casos de uso del dominio (atender llamada, llevar al usuario, optimizar), la interfaz solo expone el primero. Es una decisión de alcance que Juan aceptó, pero conviene tenerla presente: `optimizar_posicion_de_ascensores_libres` y `llevar_al_usuario_a_su_planta_de_destino` no se ejecutan nunca en producción.

### 4.6 Otras observaciones técnicas

- **`FechaYHora` es una implementación propia del calendario** (unas 190 líneas) para no añadir dependencias. Está bien probada, pero usar la hora local (D9) exige una base de zonas horarias. Lo razonable sería convertir a hora local **en el adaptador** `RelojDelSistema`, con una crate como `jiff` o `chrono`, y dejar el dominio sin cambios.
- **Uso de `expect` en el dominio:** está en `control_de_trafico.rs:72`, `:166`, `:241` y `:258`. Son invariantes internas razonables, pero serían innecesarios si `Edificio` ofreciera iteradores sobre sus ascensores en lugar de exigir que se consulte por identificador y se devuelva `Result`.
- **`HistoricoEnMemoria` y `RelojSimulado` se compilan en la biblioteca de producción,** aunque hoy solo los usan los tests. `RelojSimulado` sí puede tener sentido en un simulador (tiempo simulado). Si no se van a usar fuera de los tests, se podrían mover a un módulo de soporte de tests.

---

## 5. Sugerencias: por dónde empezar

El orden busca primero la corrección, luego abrir las costuras que facilitan el resto y, por último, la estética. Cada paso es pequeño y se puede verificar con los tests actuales.

1. **Hacer atómicas las operaciones del control de tráfico (D1).** Es lo único que hoy produce un estado incorrecto. Opciones:
   - calcular el movimiento sin mutar el edificio, registrarlo y solo entonces aplicarlo;
   - deshacer el cambio del edificio si el registro falla.

   Hay que acordar el comportamiento con Juan: ya hay un pendiente abierto y es una decisión de dominio. Es fácil de probar con el `HistoricoQueSiempreFalla` que ya existe en los tests del panel.

2. **Llevar la decisión de qué ascensor atiende al control de tráfico y unificar el "más cercano" (D2, D3).** `Edificio` se quedaría con operaciones sencillas y protegidas (por ejemplo, "ocupar el ascensor X y llevarlo a la planta P" y "liberar el ascensor X en la planta P"), y la regla de "libre más cercano con desempate" pasaría a una única función del dominio que usarían tanto la asignación de llamadas como la reubicación.

3. **Proteger las invariantes del agregado `Edificio` (D5, D6, D7):**
   - reducir la visibilidad de `mover_ascensor_a_la_planta` (`pub(crate)` o eliminarlo en favor de operaciones con significado de dominio);
   - cambiar `esta_libre: bool` por un `enum EstadoDelAscensor`;
   - añadir un constructor o *builder* del edificio que permita fijar el estado inicial, lo que simplifica mucho los fixtures de los tests (S3).

   La configuración del edificio (plantas y número de ascensores) se puede pasar como parámetro sin tocar el comportamiento por defecto.

4. **Extraer la optimización a su propio componente (S1, S2, S4).** Por ejemplo, una estrategia `PoliticaDeReubicacion` (trait) cuya implementación actual sea la de "día de la semana + franja + 30 días". Así `ControlDeTrafico` queda como coordinador y se pueden añadir otras estrategias sin modificarlo. Es buen momento para darle al puerto del histórico una consulta más acotada ("llamadas desde tal fecha").

5. **Introducir una capa de aplicación con puertos de entrada (H1-H3, S5).** Serían uno o varios traits de casos de uso ("atender llamada", "llevar al usuario", "optimizar", "consultar estado del edificio") que implementaría el servicio del núcleo. `PanelDelSimulador` dependería del trait y recibiría errores de aplicación "planos", pensados para mostrarse con `Display`, en lugar de la anidación interna. Con esta costura, el panel se puede probar con un doble.

6. **Modelar la *llamada* explícitamente (D4).** Registrar las llamadas, también las no atendidas, para que la optimización trabaje sobre la demanda real y no sobre los movimientos. Es un cambio de dominio y hay que acordarlo antes con Juan, porque afecta al glosario y al formato del histórico.

7. **Mejoras menores y documentación:**
   - implementar `Display` y `std::error::Error` en los errores;
   - conservar la causa en `HistoricoEnArchivo`;
   - convertir a hora local en `RelojDelSistema` (D9);
   - hacer configurable la ruta del histórico (H7);
   - rellenar "Mapa de la arquitectura actual" y "Flujo de trabajo" en `CLAUDE.md` (lo hará quien tenga permiso; este agente no toca `CLAUDE.md`).

**Primer paso concreto recomendado:** el punto 1. Es acotado, corrige un defecto real, ya tiene un pendiente abierto y un doble de test disponible, y obliga a tomar una decisión que condiciona el diseño de los puntos 2 y 3.

Varias de estas sugerencias cambian el comportamiento o el lenguaje de dominio (1, 4, 6 y en parte 3). Según la forma de trabajar del proyecto, cualquier término nuevo (`EstadoDelAscensor`, `PoliticaDeReubicacion`, un tipo `Llamada`...) debe acordarse antes de usarlo, como indica el propio glosario.

---

## 6. Qué no se ha analizado

- `documentacion/`, `trabajo/0_funcionalidades_y_tareas_pendientes/` y `zz - trozos de codigo descartados - guardados por si acaso/`, por las restricciones de `CLAUDE.md`. Los pendientes que cito los conozco por los registros de `trabajo/3_historico/`.
- La ejecución real de la ventana con `cargo run`, porque no hay pantalla.
- El contenido de las definiciones de agentes y de la skill más allá de sus cabeceras y del orquestador. No forman parte del código del simulador.
