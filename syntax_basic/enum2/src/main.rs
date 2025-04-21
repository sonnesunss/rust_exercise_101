fn main() {
    /// rust enum并不像swift enum那般的复杂， raw value case 和关联数据就是它的全部

    // 不携带关联数据的枚举类型Week
    #[derive(Debug)]
    enum Week {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    let _week1 = Week::Monday;
    let _week2 = Week::Tuesday;

    // 携带数据的枚举类型Months
    #[derive(Debug)]
    enum Months {
        Jan(String),
        Feb(String),
        Mar(String),
        Apr(String),
        May(String),
        Jun(String),
        Jul(String),
        Aug(String),
        Sep(String),
        Oct(String),
        Nov(String),
        Dec(String),
    }

    let month1 = Months::Jan("一月份".to_owned());
    let month3 = Months::Mar("三月份".to_owned());

    println!("{:?}", month3);
    println!("{:?}", month1);
}
