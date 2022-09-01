use std::collections::HashMap;
pub fn get_hashmaps_code () {
    
    //los hashmap son colecciones de parejas key-value
    //las keys pueden ser de cualquier tipo

    //instancia de hashmao vacia
    let mut scores = HashMap::new();

    //insercion de elementos
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("yellow"),5);

    //todas la llaves deben de ser del mismo tipo y los elementos
    //se almacenan en el heap

    //el metodo get nos permite obtener el valor de acuerdo a la clave
    let my_team_name = String::from("blue");
    //obtiene un Option
    let my_score = scores.get(&my_team_name);

    let my_score:u32 = match my_score {
        Some(score) => *score, //se tiene que deferenciar el valor
        None => 0,
    };

    println!("El marcador del equipo {} es {}", my_team_name,my_score);

    //recorrido con for y duple
    //usar for con el valor y no la referencia, hace que pierda el ownership
    for (key, value) in &scores {
        println!("el equipo {} tiene {} puntos", key, value);
    }

    //cuando se insertan tipos que implementan copy se hace una copia en el hashmap
    //los demas tipos se mueven, como es el caso de String
    //se pueden insertar referencias, pero estas deben de ser validas al menos hasta que el hashmap
    //salga de scope

    //las llaves deben ser unicas y estar asociadas a un solo valor
    //el contenido de esos valores no tiene relevancia

    //sobreescritura de valores, cuando se inserta con la misma key

    scores.insert(String::from("yellow"), 10);

    for (key, value) in &scores {
        println!("el equipo {} tiene {} puntos", key, value);
    }

    //entry es un metodo que permite checar si una llave existe, regresa un enum Entry
    //con el metodo or_insert sobreescribe solo si la llave no tiene un valor asociado
    //en caso de que esta llave no tenga un valor asociado, inserta el entry
    //or_insert ademas devuelve una referencia mutable del valor del Entry
    scores.entry(String::from("blue")).or_insert(15); // no actualiza el valor por que la llave tiene un valor asignado
    let green_score = scores.entry(String::from("green")).or_insert(50); // inserta elemento en el hashmap
    println!("== el equipo green tiene {} puntos ==", green_score); //almacenamos en green_score la referencia que devuelve or_insert


    for (key, value) in &scores {
        println!("el equipo {} tiene {} puntos", key, value);
    }

    //actualizacion del valor de un hashmap de acuerdo al valor previo

    //esta es otra tecnica que aprovecha el metodo or_insert que devuelve una referencia mutable
    //hacemos dereference y actualizamos el valor

    //vamos a almacenar las palabras como key y actualizaremos el valor con cada ocurrencia
    let phrase = String::from("I was sitting there like fuck it was like fuck I thought fuck this shit fuck");
    let mut words = HashMap::new();
    for word in phrase.split_whitespace() {
        let count = words.entry(word).or_insert(0); // en la primera aparicion inserta la entrada con 0, y devuelve el valor
        *count += 1; //sumamos 1 al valor haciendo deference para actualizar la entrada y mantener su owner
    }
    
    dbg!(words);

    //la implementacion de la funcion de hash en hashmap puede ser un poco mas lenta, ya que ofrece mayor resistencia ante ataques dos
    //en caso de requerir mayor rendimiento se puede implementar una funcion distinta mediante Hasher que implementa BuildHasher
    //o utilizar alguna de crates.io
}