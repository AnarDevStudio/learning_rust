// funksiyalar programin en esas hissesidir. Funksiyalar kodu modullashdirir ve yeniden istifade olunmasina imkan verir.
//fn funksiyanin teyin edilmesi ucun istifade olunur, funksiya adini ve parantezleri daxil edir
fn funksiya(){
    println!("Bu, funksiyadir!"); // funksiyanin icindeki kod parcasi
}

fn main(){
    funksiya(); // funksiyanin cagirilməsi main funksiyasinda
    funksiya_parametrli(10, 20); // parametrli funksiyanin cagirilməsi main funksiyasinda
    funksiya_1(); // ic ice funksiyanin cagirilməsi main funksiyasinda
}

fn funksiya_parametrli(x: i32, y: i32) { 
    // not: birden cox parametrli funksiyalar ucun parametrlər vergüllə ayrılır
    // not: her parametrin tipi qeyd edilmelidir
    // funksiyaya iki parametr verilir: x ve y, her ikisi i32(tam say) tipindedir
    let cem = x + y; // x ve y-nin cemi cem deyisenine verilir
    println!("Parametrli funksiyada cem: {}", cem); // cem deyiseninin qiymeti ekrana cap edilir
}


// ic ice funksiyalar 🔗

fn funksiya_1(){
    println!("Funksiya 1 isledi");
    funksiya_2(); // funksiya_2 funksiyasi funksiya_1-in icinde cagirilib
}


fn funksiya_2(){
    println!("Funksiya 2 isledi");
}
