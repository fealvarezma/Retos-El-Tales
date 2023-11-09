fn main() {
    //println!("Cualquier palabra"); // Solución punto 1

    //let p2= "hola";
    //p1::fp();
    //print!("{}", p2);
    //print!("    ");

    for numero in 1..=10 {
        print!("{:2}",numero);
    }

    //creación de la lista
    let mut _lista: Vec<i32> = Vec::new();
    for num in 1..=10 {
        _lista.push(num);
    }
    print!("{:?}",_lista);

    for i in _lista {

        if i % 2 != 0 {
            print!("{:3}", i);
        }
           
    }
 
}

