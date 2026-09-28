## Introducción general

En los últimos tiempos, los modelos de IA están llegando a niveles que permiten trabajar con ellos de forma colaborativa. Personas humanas y agentes IA están comenzando a trabajar "codo con codo", formando un "equipo" de trabajo.

Al igual que las personas, los agentes IA también necesitan tener unas **directrices claras** cuando trabajan en un equipo. Para evitar incertidumbres y malentendidos.

En este repositorio se pretende explorar cómo se pueden explicitar esas directrices en este momento (*Septiembre 2026*). Las ideas recogidas son más o menos válidas para trabajar cualquier proveedor de IA; pero, concretamente, nos hemos centrado en los entornos de trabajo de [Anthropic](https://www.anthropic.com/):
- tanto en **sesiones interactivas de charla** ([Claude](https://claude.com/docs)),
- como en **agentes para desarrollo de software** ([Claude Code](https://code.claude.com/docs/es/overview)).

nota: El uso de **agentes de escritorio** está todavía en sus primeros pasos. Hay iniciativas tales como, por ejemplo: Claude Desktop, [Claude Cowork](https://claude.com/docs/cowork/overview), [Claude in Slack](https://claude.com/docs/claude-tag/overview), [Claude in Chrome](https://claude.com/claude-in-chrome) o [Claude for M365](https://claude.com/docs/office-agents/overview). Pero aún se están explorando formas de explicitar directrices y salvaguardas cuando se trabaja esos entornos. 

> En el campo de los agentes de escritório, por ahora, prácticamente solo tenemos los mecanismos base:
- Las directrices generales (`System Prompts`) que se expliciten para los modelos.
- Las directrices marcadas a cada tipo de agente que definamos.
- El cuidado que pongamos las personas humanas en los `prompts` que escribamos en cada sesión.

nota importante: La eficacia de los agentes IA hace que resulte prácticamente inviable hacer que todos sus resultados sean verificados detenidamente por parte de personas humanas. Lo cual nos lleva a la necesidad de disponer de otros **agentes que nos ayuden a verificar** el trabajo. 


## Entrando en harina,

- Unos apuntes generales detallando cómo explicitar directrices y definir agentes:  [`./documentacion/configuracion_de_directrices_para_Claude.md`](./documentacion/configuracion_de_directrices_para_Claude.md)

- Una muestra de definición de agentes: [`.claude/agents`](./.claude/agents). 

- Una muestra de utilización de agentes dentro de un bucle de trabajo automatizado: [`.claude/skills/programacion_en_bucle_automatico`](.claude/skills)).

- Una muestra de salvaguardas y reglas fijas de obligado cumplimiento: [`.claude/settings.json`](./.claude/settings.json)


## Comentario final

Todo esto de la IA está avanzando a velocidad vertiginosa. Y también su documentación evoluciona con rapidez. Enlaces concretos pueden variar, por ello solo cito aquí las entradas raíz a la documentación de Anthropic:
- Claude: [https://claude.com/docs](https://claude.com/docs)
- Claude Code: [https://code.claude.com/docs/es/overview](https://code.claude.com/docs/es/overview)