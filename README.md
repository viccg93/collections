la libreria estadar de Rust permite trabajar con estructuras de datos llamadas collections

estas estructuras a diferencias de las que Rust provee en el propio lenguaje como los arrays se alamacenan en el heap
lo que quiere decir que no es necesario conocer su tamaño durante la compilacion y permite que su tamaño aumente
durante la ejecucion.

las principales colecciones son:

- vectores: permite almacenar un numero variable de valores
- Strings: coleccion de caracteres
- Hash map (implementacion del tipo general map): parejas key:value

## Strings

los Strings son una coleccion compleja que enfrenta algunos retos que suele no tener el peso correspondiente
- Rust es propenso a exponer posibles errores
- los String son estructuras de datos mas complicadas de lo que se les da credito
- UTF-8

En Rust los Strings son colecciones de bytes y se proveen algunos metodos que brindan funcionalidad necesaria
ademas de que temas como la indexacion presenta retos que son unicos de los Strings

algo que es comun cuando se entiende la diferencia entre la interpretacion de String entre humanos y computadoras

En el core de Rust solo existe un tipo string y es str, que se entiende como un string slice, que son referencias
a cadenas UTF-8 encoded almacenadas en alguna ubicacion, por ejemplo las literales string que se almacenan
en el binario

el tipo String se encuentra en la libreria estandar en lugar de estar programado en el core de Rust y este tipo
se defino como una cadena UTF-8 encoded de tamaño dinamico, mutable y owned.