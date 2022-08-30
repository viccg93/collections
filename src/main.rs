fn main() {
    //los vectores deben de contener valores de un solo tipo
    //definicion de un vector vacio (es poco comun esta inicializacion)
    let mut vec: Vec<i32> = Vec::new();

    //cuando el vector se crea con valores, el compilador puede inferir el tipo
    //podemos usar la macro vec!

    let mut vec2 = vec![1,2,3]; // infiere que el tipo es i32

    //para actualizar un vector usamos el metodo push(value)
    //igul que en los tipos vistos previamente, para actualizar el valor de un owner, este debe ser mutable

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);

    //para acceder a los valores de un vector podemos usar vec[index] o vec.get(index)

    let second_element = &vec[1]; //usamos la referencia, por que no queremos mover
    println!("El segundo valor es {}", second_element);

    //retorna una referencia en forma de Option
    let last = vec.get(5);

    match last {
        Some(value) => println!("El ultimo valor es {}", value),
        None => println!("No existe valor en ese indice"),
    };

    //como previamente almacenamos en second_element una referencia a vec[1], operar sobre vec resulta en un error

    let ref_first_element_in_vec2  = &vec2[0]; //refencia inmutable

    vec2.push(4); // no deberia de funcionar en versiones anteriores de Rust, debido al borrow mutable cuando ya existe una referencia inmutable
    dbg!(vec2);

    //alternativamente podemos iterar con for, lo cual es preferible por su practicidad y evita index out of bounds
    //usamos la referencia del vector para mantener el ownership
    for element in &vec {
        println!("{}", element);
    }

    dbg!(&vec);

    //tambien podemos iterar accediendo a referencias mutables
    for element in &mut vec {
        *element += 1; //para alterar el valor, debemos hacer dereference a la referencia mutable
        //iterar sobre un vector mutable o inmutable es seguro, ya que intentar añadir o remover elementos mientras se itera, genera un error de compilacion
        //ya que la referencia que usa el for es una referencia al vector completo y esta en el mismo scope, cualquier modificacion no esta permitida
        //vec.push(1); //no compila
    }

    dbg!(&vec);

    //enums en vectores
    //los vectores presentan una "limitacion" y es que solo pueden almacenar un tipo, aunque esto se puede "cambiar" si se usa con enums
    //realmente la restriccion se mantiene por que las variantes de enum son conocidas en tiempo de compilacion, solo se abren las posibilidades

    //ejemplo de vector que pue recibir celdas de una hoja de calculo
    enum SpreadSheetCell {
        Int(u32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(2),
        SpreadSheetCell::Text(String::from("Vic")),
        SpreadSheetCell::Float(10.0)
    ];

    //en caso de no conocer los valores que almacenara el vector en tiempo de compilacion
    //se recomienda usar un objeto trait

    //al igual que cualquier otro struct, las instancias vector se liberan al salir de scope y se invalidad sus referencias

    //nota: Vector es un tipo generico
}

//modulo strings
mod strings;