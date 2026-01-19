fn main() {

    // if — şərt ifadəsidir (şərtli operator)
    // if yalnız true və ya false qəbul edir

    if 10 > 5 {
        // 10 > 5 ifadəsi doğrudur (true)

        println!("10 boyukdur");
    }

    // İndi else hissəsinə baxaq

    if 3 > 5 {
        // bu şərt yanlışdır (false)

        println!("3 boyukdur");
    } else {
        // əgər if false olarsa, bura işləyir

        println!("3 boyuk deyil");
    }

    // else if — bir neçə şərti yoxlamaq üçün

    let reqem = 7;

    if reqem > 10 {
        println!("10-dan boyukdur");
    } else if reqem > 5 {
        println!("5-dən boyukdur");
    } else {
        println!("5-dən kicikdir");
    }


    // Rust-da if dəyər qaytara bilər (çox vacib mövzu)

    let netice = if 10 > 5 {
        // burada ; YOXDUR
        "dogrudur"
    } else {
        "yanlisdir"
    };

    println!("Netice: {}", netice);


    // Məntiqi operatorlar

    let yas = 20;
    let vesige = true;

    if yas >= 18 && vesige == true {
        println!("Masin sure biler");
    }

    // NOT operatoru

    if !vesige {
        println!("Vesige yoxdur");
    }

}
