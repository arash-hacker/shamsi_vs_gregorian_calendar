

#[derive(Debug,PartialEq)]
pub struct Date{
    pub day:i64,
    pub month:i64,
    pub year:i64,
}
impl Date{
    #[allow(dead_code)]
    fn get_date(self)->i64{
        self.day
    }
}

pub fn convert_to_shamsi(d:Date)->Date{
    
    let miladi_year:i64 = d.year ;
    let miladi_month:i64 = d.month ;
    let miladi_day:i64 = d.day;
    

    let buf1 = [0,31,59,90,120,151,181,212,243,273,304,334];
    let buf2 = [0,31,60,91,121,152,182,213,244,274,305,335];

    let mut date ;
    let  month;
    let  year ;

    if (miladi_year % 4) != 0 {

        date = buf1[miladi_month as usize- 1] + miladi_day;

        if date > 79 {
            date = date - 79;
            if date <= 186 {
                match date % 31 {
                     0=>{
                         month = date / 31;
                         date = 31;
                     },
                    _=>{
                        month = (date / 31) + 1;
                        date = date % 31;
                    }
                }
                year = miladi_year - 621;
            } else {
                date = date - 186;

                match date % 30 {
                    0=>{
                        month = (date / 30) + 6;
                        date = 30;
                    },
                    _=>{
                        month = (date / 30) + 7;
                        date = date % 30;
                    }
                }
                year = miladi_year - 621;
            }
        } else {
            let ld;
            if (miladi_year > 1996) && (miladi_year % 4) == 1 {
                ld = 11;
            } else {
                ld = 10;
            }
            date = date + ld;

            match date % 30 {
                0=>{
                    month = (date / 30) + 9;
                    date = 30;
                },
                _=>{
                    month = (date / 30) + 10;
                    date = date % 30;
                }
            }
            year = miladi_year - 622;
        }
    } else {

        date = buf2[miladi_month as usize - 1] + miladi_day;
        let ld;
        if miladi_year >= 1996 {
            ld = 79;
        } else {
            ld = 80;
        }
        if date > ld {
            date = date - ld;

            if date <= 186 {
                match date % 31 {
                    0=>{
                        month = date / 31;
                        date = 31;
                    },
                    _=>{
                        month = (date / 31) + 1;
                        date = date % 31;
                    }
                }
                year = miladi_year - 621;
            } else {
                date = date - 186;

                match date % 30 {
                    0=>{
                        month = (date / 30) + 6;
                        date = 30;
                    },
                    _=>{
                        month = (date / 30) + 7;
                        date = date % 30;
                    }
                }
                year = miladi_year - 621;
            }
        } else {
            date = date + 10;

            match date % 30 {
                0=>{
                    month = (date / 30) + 9;
                    date = 30;
                },
                _=>{
                    month = (date / 30) + 10;
                    date = date % 30;
                }
            }
            year = miladi_year - 622;
        }

    }

    Date{year:year,month:month,day:date}
    
}

pub fn convert_to_gregorian(d:Date)->Date{
    let gregorian_days_in_month = [ 31, 28, 31, 30, 31, 30, 31,31, 30, 31, 30, 31];
    let jalali_days_in_month=[ 31, 31, 31, 31, 31, 31, 30, 30,30, 30, 30, 29 ];
    let jalali=Date{year:d.year-979,month:d.month-1,day:d.day-1};

    let mut jalali_day_no = 365 * jalali.year + (jalali.year / 33)* 8 + (((jalali.year % 33) + 3) / 4) ;

    for i in 0..jalali.month {
        jalali_day_no += jalali_days_in_month[i as usize];
    }

    jalali_day_no += jalali.day;

    let mut gregorian_day_no = jalali_day_no + 79;

    let mut gregorian_year = 1600 + 400 * (gregorian_day_no / 146097); 
    gregorian_day_no = gregorian_day_no % 146097;

    let mut leap = 1;
    if gregorian_day_no >= 36525 {
        gregorian_day_no=gregorian_day_no-1;
        gregorian_year += 100 * (gregorian_day_no / 36524);
        gregorian_day_no = gregorian_day_no % 36524;

        if gregorian_day_no >= 365 {
            gregorian_day_no=gregorian_day_no+1;
        } else {
            leap = 0;
        }
    }

    gregorian_year += 4 * (gregorian_day_no / 1461); 

    gregorian_day_no = gregorian_day_no % 1461;

    if gregorian_day_no >= 366 {
        leap = 0;

        gregorian_day_no=gregorian_day_no-1;
        gregorian_year += gregorian_day_no / 365;
        gregorian_day_no = gregorian_day_no % 365;
    }

    let mut i:i64=0;
    while gregorian_day_no >= gregorian_days_in_month[i as usize]+ if i == 1 && leap == 1  {i} else {0} {
          gregorian_day_no -= gregorian_days_in_month[i as usize] + if i == 1 && leap == 1  {i} else {0};
          i+=1;
    }
    
    let gregorian_month = i +1;
    let gregorian_day = gregorian_day_no + 1;

    Date{year:gregorian_year, month:gregorian_month, day:gregorian_day}

}
