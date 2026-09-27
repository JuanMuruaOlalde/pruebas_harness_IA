## Introducción general

En los últimos tiempos, los modelos de IA están llegando a niveles que permiten trabajar con ellos de forma colaborativa. Personas humanas y agentes IA están comenzando a trabajar "codo con codo", formando un "equipo" de trabajo.

Al igual que las personas, los agentes IA también necesitan tener unas **directrices claras** cuando trabajan en un equipo. Para evitar incertidumbres y malentendidos.

En este repositorio se pretende explorar cómo se pueden explicitar esas directrices en este momento (*Septiembre 2026*). Las ideas recogidas son más o menos válidas para trabajar cualquier proveedor de IA; pero más concretamente, nos hemos centrado en los entornos de trabajo de [Anthropic](https://www.anthropic.com/); tanto en **sesiones interactivas de charla** ([Claude](https://claude.com/docs)) como en **agentes para desarrollo de software** ([Claude Code](https://code.claude.com/docs/es/overview)).

nota: El uso de **agentes de escritorio** está todavía comenzando. Hay iniciativas tales como, por ejemplo: Claude Desktop, [Claude Cowork](https://claude.com/docs/cowork/overview), [Claude in Slack](https://claude.com/docs/claude-tag/overview), [Claude in Chrome](https://claude.com/claude-in-chrome) o [Claude for M365](https://claude.com/docs/office-agents/overview). Pero aún se están explorando las formas de explicitar directrices y salvaguardas cuando se trabaja con estos agentes de escritorio. En este campo, por ahora, prácticamente tenemos solo:
- Las directrices generales (`System Prompts`) que se expliciten para los modelos.
- Las directrices marcadas a cada tipo de agente que definamos.
- El cuidado que pongamos las personas humanas en los `prompts` que escribamos en cada sesión.

nota: La eficacia de los agentes en el trabajo hace que resulte prácticamente inviable hacer que todos sus resultados sean verificados detenidamente por parte de personas humanas. Lo cual nos lleva a la necesidad de disponer de otros **agentes que nos ayuden a verificar** el trabajo. 

## Entrando en harina,

- Apuntes generales de cómo explicitar directrices y definir agentes están recogidos en el documento [`./documentacion/configuracion_de_directrices_para_Claude.md`](./documentacion/configuracion_de_directrices_para_Claude.md)

- Una muestra de definición de agentes está en la carpeta [`.claude/agents`](./.claude/agents). 

- Esos agentes se pueden usar por separado o dentro de un bucle de trabajo automatizado (ver en la carpeta [`.claude/skills`](.claude/skills)).

- Una muestra de explicitar salvaguardas y reglas fijas de obligado cumplimiento está en el archivo [`.claude/settings.json`](./.claude/settings.json)

   Para más detalles sobre salvaguardas y reglas fijas, consultar la documentación de Anthropic: [archivos de configuración y precedencia -settings-](https://code.claude.com/docs/es/settings)


## Comentario final

Todo esto de la IA está avanzando a velocidad vertiginosa; y la documentación también evoluciona casi de semana en semana. 

Por ello, es posible que algunos de los enlaces en este documento dejen de ser válidos.

Por si acaso, las entradas raíz a la documentación de Anthropic son:
- Claude: [https://claude.com/docs](https://claude.com/docs)
- Claude Code: [https://code.claude.com/docs/es/overview](https://code.claude.com/docs/es/overview)