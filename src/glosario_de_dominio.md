# Glosario de dominio

Nomenclatura a emplear. En caso de preferir otro nombre para algo contemplado aquí, hay que acordarlo previamente antes de utilizarlo.

## De uso general

- **edificio**.
- **planta**. Siendo 0 la planta principal por donde se accede al edificio principalmente; con números positivos por encima de ella y números negativos por debajo de ella.
- **planta de acceso principal**: la planta 0.
- **distancia** (entre dos plantas): número de plantas que las separan, sin importar el sentido.
- **ascensor**.
- **identificador de ascensor**: número que distingue a cada ascensor del edificio. Se numeran desde 0.
- **usuario**: persona que utiliza un ascensor para desplazarse.

## Del control de tráfico

- **control de tráfico**: decide qué ascensor atiende cada llamada, registra los movimientos en el histórico y reubica los ascensores libres.
- **botón de llamada**: botón que pulsa el usuario en una planta para pedir un ascensor.
- **llamada**: petición de un ascensor que hace un usuario al pulsar el botón de llamada en una planta. La atiende el ascensor libre más cercano.
- **libre**: estado de un ascensor que no está atendiendo a ningún usuario. Solo los ascensores libres atienden llamadas y se reubican.
- **ocupado**: estado de un ascensor desde que se le envía a atender una llamada hasta que lleva al usuario a su planta de destino.
- **planta de origen**: planta en la que está el ascensor al empezar un movimiento.
- **planta de destino**: planta en la que queda el ascensor al terminar un movimiento. Cuando lleva a un usuario, es la planta a la que el usuario quiere ir.
- **reubicación**: movimiento de un ascensor libre a otra planta para agilizar las llamadas futuras. El ascensor sigue libre.
- **optimización** (de la posición de los ascensores libres): reubicación de los ascensores libres en las plantas desde las que más se ha llamado en el mismo día de la semana y la misma franja horaria durante el último mes.

## Del histórico

- **movimiento**: desplazamiento de un ascensor desde una planta de origen hasta una planta de destino. Queda registrado con su fecha y hora y su motivo.
- **motivo del movimiento**: por qué se ha movido el ascensor: atender una llamada, llevar al usuario a su destino o reubicar un ascensor libre.
- **histórico** (de movimientos): registro permanente de todos los movimientos, en el orden en que se produjeron. Nunca se borra ninguno.
- **fecha y hora**: momento en que ocurre algo, con precisión de segundos.
- **reloj**: indica la fecha y hora actual.
- **franja horaria**: cada una de las 24 horas en que se divide el día. La franja 0 va de 00:00 a 00:59.
- **último mes**: los 30 días anteriores a un momento dado, ambos límites incluidos.
