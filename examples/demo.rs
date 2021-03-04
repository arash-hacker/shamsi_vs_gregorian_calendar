use gregorian2shamsi::*;
fn main(){
    let d =Date { day: 15, month: 12, year: 1399 };
    println!("{:?}",
        convert_to_gregorian(d)
    );
}
#[test]
fn test_shamsi_2_gregori(){

    assert_eq!(
        Date { day: 21, month: 5, year: 1990 },
        convert_to_gregorian(Date{year:1369,month:2,day:31}));

    assert_eq!(
        Date { day: 1, month: 1, year: 2020 },
        convert_to_gregorian(Date{year:1398,month:10,day:11}));

    assert_eq!(
        Date { day: 20, month: 3, year: 2020 },
        convert_to_gregorian(Date{year:1399,month:1,day:1}));
    
    assert_eq!(
        Date { day: 23, month: 4, year: 2000 },
        convert_to_gregorian(Date{year:1379,month:2,day:4}));

    assert_eq!(
        Date{year:2021,month:3,day:4},
        convert_to_gregorian(Date { day: 14, month: 12, year: 1399 }));

    assert_eq!(
        Date{year:1967,month:10,day:11},
        convert_to_gregorian(Date { day: 19, month: 7, year: 1346 }));

    
    assert_eq!(
        Date{year:2008,month:2,day:29},
        convert_to_gregorian(Date { day: 10, month: 12, year: 1386 }));

    assert_eq!(
        Date{year:2013,month:5,day:14},
        convert_to_gregorian(Date { day: 24, month: 2, year: 1392 }));

    assert_eq!(
        Date{year:1990,month:3,day:31},
        convert_to_gregorian(Date { day: 11, month: 1, year: 1369 }));

    assert_eq!(
        Date{year:1906,month:1,day:7},
        convert_to_gregorian(Date { day: 17, month: 10, year: 1284 }));
}
#[test]
fn test_gregori_2_shamsi(){

    assert_eq!(
        Date{year:1398,month:10,day:11},
        convert_to_shamsi(Date { day: 1, month: 1, year: 2020 }));

    assert_eq!(
        Date{year:1399,month:1,day:1},
        convert_to_shamsi(Date { day: 20, month: 3, year: 2020 }));
    
    assert_eq!(
        Date{year:1379,month:2,day:4},
        convert_to_shamsi(Date { day: 23, month: 4, year: 2000 }));

    assert_eq!(
        Date { day: 14, month: 12, year: 1399 },
        convert_to_shamsi(Date{year:2021,month:3,day:4}));

    assert_eq!(
        Date { day: 19, month: 7, year: 1346 },
        convert_to_shamsi(Date{year:1967,month:10,day:11}));

    
    assert_eq!(
        Date { day: 10, month: 12, year: 1386 },
        convert_to_shamsi(Date{year:2008,month:2,day:29}));

    assert_eq!(
        Date { day: 24, month: 2, year: 1392 },
        convert_to_shamsi(Date{year:2013,month:5,day:14}));

    assert_eq!(
        Date { day: 11, month: 1, year: 1369 },
        convert_to_shamsi(Date{year:1990,month:3,day:31}));

    assert_eq!(
        Date { day: 17, month: 10, year: 1284 },
        convert_to_shamsi(Date{year:1906,month:1,day:7}));

}

