fn main() {
    let mut sum: i64 = 0;
    let ranges_string = "655-1102,2949-4331,885300-1098691,1867-2844,20-43,4382100-4484893,781681037-781860439,647601-734894,2-16,180-238,195135887-195258082,47-64,4392-6414,6470-10044,345-600,5353503564-5353567532,124142-198665,1151882036-1151931750,6666551471-6666743820,207368-302426,5457772-5654349,72969293-73018196,71-109,46428150-46507525,15955-26536,65620-107801,1255-1813,427058-455196,333968-391876,482446-514820,45504-61820,36235767-36468253,23249929-23312800,5210718-5346163,648632326-648673051,116-173,752508-837824";
    let ranges = ranges_string.split(",");
    for range in ranges {
        let mut ids = range.split("-");
        let start = ids.next().expect("err").parse::<i64>().expect("err");
        let end = ids.next().expect("err").parse::<i64>().expect("err");
        // println!("range: {range}; start: {start}; end: {end}");
        let mut i = start;
        while i <= end {
            // println!("{}", i);
            if f64::floor(f64::log10(i as f64)) % 2.0 != 0.0 {
                // println!("dsada");
                let i_string = i.to_string();
                let i_tuple = i_string.split_at(i_string.len() / 2);
                if i_tuple.0 == i_tuple.1 {
                    println!("range {range} invalid: found pattern {i_string}");
                    sum += i;
                }
            };
            i += 1;
        }
    }
    println!("the sum is {sum}")
}
