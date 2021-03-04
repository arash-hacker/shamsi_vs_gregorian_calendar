#GREGORIAN #WESTERN #SHAMSI #PERSIAN #SOLAR #MILADI #Jalali, #Persian, #Khorshidi      
```rust
use gregorian2shamsi::*;

fn main(){
    let d =Date { day: 15, month: 12, year: 1399 };
    println!("{:?}",
        convert_to_gregorian(d)
    );
}

assert_eq!(
        Date{year:1398,month:10,day:11},
        convert_to_shamsi(Date { day: 1, month: 1, year: 2020 }));
```