// funksiyalar programin en esas hissesidir. Funksiyalar kodu modullashdirir ve yeniden istifade olunmasina imkan verir.
//fn funksiyanin teyin edilmesi ucun istifade olunur, funksiya adini ve parantezleri daxil edir
fn funksiya(){
    println!("Bu, funksiyadir!"); // funksiyanin icindeki kod parcasi
}

// return donduren funksiyalar ucun sonuncu ifadeden sonra ; isaresi qoyulmur


fn funksiya_parametrli(x: i32, y: i32) { 
    // not: birden cox parametrli funksiyalar ucun parametrlər vergüllə ayrılır
    // not: her parametrin tipi qeyd edilmelidir
    // funksiyaya iki parametr verilir: x ve y, her ikisi i32(tam say) tipindedir
    let cem = x + y; // x ve y-nin cemi cem deyisenine verilir
    println!("Parametrli funksiyada cem: {}", cem); // cem deyiseninin qiymeti ekrana cap edilir
}

fn funksiya_return(a: i32, b: i32) -> i32 {
    a + b // a ve b-nin cemi funksiyanin return deyeridir
}

//geriye deyer donduren funksiya
fn funksiya_geri_donus_deyeri() -> i32 { 
    // not: funksiyanin dondureceyi tip "->" isaresinden sonra qeyd edilir
    let deyisen = 10; // deyisen teyin edilir ve 10 qiymeti verilir
    deyisen // deyisenin qiymeti funksiyanin return deyeridir, sonuncu ifadede ; isaresi olmamalidir
}

// ic ice funksiyalar 🔗
// not: funksiya icinde baska bir funksiya cagira bilerik ama baska bir funksiyanin icinde funksiya teyin ede bilmirik
fn funksiya_1(){
    println!("Funksiya 1 isledi");
    funksiya_2(); // funksiya_2 funksiyasi funksiya_1-in icinde cagirilib
}


fn funksiya_2(){
    println!("Funksiya 2 isledi");
}

fn main(){
    funksiya(); // funksiyanin cagirilməsi main funksiyasinda
    funksiya_parametrli(10, 20); // parametrli funksiyanin cagirilməsi main funksiyasinda
    funksiya_1(); // ic ice funksiyanin cagirilməsi main funksiyasinda
    funksiya_geri_donus_deyeri(); // geriye deyer donduren funksiyanin cagirilməsi main funksiyasinda
    let netice = funksiya_return(15, 25); // return donduren funksiyanin cagirilməsi ve neticeninin deyisene verilməsi
    println!("Return donduren funksiyanin neticesi: {}", netice); // neticenin ekrana cap edilmesi
}
