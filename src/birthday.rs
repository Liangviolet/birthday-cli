use chrono::{Datelike,NaiveDate};

#[derive(Clone)]
pub struct Birthday{
    pub id:i32,
    pub name:String,
    pub date:NaiveDate,
}

impl Birthday{
    pub fn age(&self,today:NaiveDate)->Option<u32>{
        today.years_since(self.date)
    }

    pub fn next(&self,today:NaiveDate)->NaiveDate{
        let birthday_this_year=self.date.with_year(today.year()).unwrap();
        let birthday_next_year=self.date.with_year(today.year()+1).unwrap();
        if birthday_this_year > today {
            birthday_this_year
        } else {
            birthday_next_year
        }
    }

    pub fn is_today(&self,today:NaiveDate)->bool{
        self.date.month()==today.month()&&self.date.day()==today.day()
    }
}