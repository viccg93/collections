pub fn get_strings_code(){
    //muchas de las operaciones disponibles con Vec<T> estan disponibles para String
    //ya que String es un wrapper de un Vect de bytes con garantias, restricciones y funcionalidades añadidas

    //creacion de una instancia mutable de String
    let mut s = String::new();

    //uso del metodo to_string() que esta disponible en cualquier tipo que implemente el trait Display
    //uno de los tipos que implementa Display son las literales

    let data = "hey rustaceans!";
    let s = data.to_string();

    //incluso se puede llamar directamente en la literal

    let s = "Rust".to_string();

    //la funcion from del tipo String es equivalente, elegir una u otra es cuestion de estilo y legibilidad

    let s = String::from("Hey again rustaceans");

    //recordemos que el tipo string es UTF-8 encoded por lo que las siguientes cadenas son completamente validas
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //un String puede crecer en tamañoo cambiar su contenido
    //para concatenar valores a un String podemos usar el operador '+' o el macro format!

    //para hacer un appen a un String podemos usar el metodo push_str()
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);//push_str() recibe la referencia a un slice, por lo que no toma el ownership
    println!("el valor de s2 es {}", s2);//por ende s2 sigue siendo un owner valido

    //podemos usar el operador '+' como alternativa a push_str()
    //hay que tener cuidado al usar ese operador ya que podemos pasar el owner o la referencia

    let a = String::from("Hey ");
    let b = String::from("rustaceans");
    //la razon de que el segundo valor del operador sea una referencia, es por que el operador '+' llama al metodo add()
    //que en su signature el segundo parametro es &str, si usamos dos valores o dos referencias tendremos un error de compilacion
    //&b es una referencia de String y no una referencia de str, que es la que espera el metodo add, pero funciona
    //por que Rust usa deref coercion y convierte &s2 en &s2[..]
    //mas que una copia, este metodo esta tomando el ownership de a y le concatena una copia de b, el ownership pasa a c
    //por loq ue esta implementacion es mas eficiente que hacer muchas copias
    let c = a + &b; //despues de esta linea la variable a fue movida y ya no puede ser usada por que se paso su ownership

    //si queremos concatenar varios String, solo el primero mueve su valor, los demas son referencias

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;

    //tambien podemos usar el macro format!()
    let s1 = String::from("tic");

    let s4 = format!("{}-{}-{}", s1,s2,s3); // en este caso todos los argumentos son referencias

    //Rust no soporta el acceso a los caracteres de un String por medio de indices debido a como se almacenan los String en memoria
    //considerando el siguiente String

    let hello = String::from("Hola");

    //tenemos que la cadena tiene un len de 4 y el espacio reservado para este String es de 4 bytes
    //pero en el siguiente String

    let hello = "Здравствуйте";

    //podriamos pensar que el tamaño es 12, pero es 24, ya que para cada caracter escalar unicode se requieren 2 bytes para almacenarlo
    //como el valor en bytes de los indices no corresponderian y por tanto se pueden dar funcionamientos no esperados
    //esa es la razon de que String no permite el acceso por indices, para evitar posibles errores

    //en Rust, en lo referente a UTF-8 tenemos 3 formas de ver a los strings, como bytes, como valores escalares y como graphemes

    //la palabra en hindi “नमस्ते” se puede representar como
    //un vector de bytes (u8) [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //unicode escalares (como las computadoras almacenan la informacion) ['न', 'म', 'स', '्', 'त', 'े']
    //2 de los elementos en esa coleccion no son letras, son diacriticos que no tienen sentido solos
    //como clusters de graphemes seria ["न", "म", "स्", "ते"]

    //de tal manera que Rust brinda diferentes formas de almacenar un raw string, permitiendo que cada programa decida la interpretacion
    //que necesita, sin estar amarrado a un lenguaje en especifico
    //finalmente existe una razon mas para no indexar estas cadenas y es que las operaciones con indices deben de ser O(1) y no es posible
    //garantizar ese performance con las strings

    //slicing

    //si se requiere obtener ciertos indices de un String, lo cual no se recomienda
    //Rust pedira ser mas especifico y solo aceptara rangos, ademas estos rangos estan sujetos a la cantidad minima de bytes
    //que requiere un caracter para su almacenamiento

    //en este ejemplo cada caracter en cirilico requiere 2 bytes, por lo que acceder a rangos multiplos de 2 esta permitido

    let hello = "Здравствуйте";
    let s = &hello[0..4]; //obtendriamos 4 bytes que equivalen a Зд

    //pero acceder a un solo indice o a un numero de bytes que no es multiplo de 2, resulta en un panic en tiempo de ejecucion
    //por lo que se debe de tener cuidado al crear string slices con rangos, ya que puede ocasionar un program crashing
    //let s = &hello[0..1]; //manda un error en tiempo de ejecucion

    //la mejor forma de operar con partes de un String es ser explicito si se quiere obtener chars o bytes
    //los graphemes salen del alcance de la libreria estandar y dicha funcionalidad se encuentra en crates.Into

    //obtener los chars
    for c in "Зд".chars() {//devuelve 2 elementos
        println!("{}", c); 
    }

    //obtener los bytes
    for b in "Зд".chars() {//devuelve 4 elementos
        println!("{}", b);
    }

    //es importante recordar que un caracter escalar unicode puede requerir mas de 1 byte para almacenarse
}