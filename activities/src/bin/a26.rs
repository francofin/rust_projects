// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::*;
use humantime::format_duration;
use std::time::Duration;
use demo::print_from_lib;
use demo::group_one;

fn main() {
    println!("Hello");
    print_from_lib();
    group_one::g1::print_from_g1();
    let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now();

    println!("UTC Time is {:?}", utc);
    println!("Local Time is {:?}", local);

    let dt_1 = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
    // July 8 is 188th day of the year 2014 (`o` for "ordinal")
    // assert_eq!(dt, Utc.yo(2014, 189).and_hms(9, 10, 11));
    // July 8 is Tuesday in ISO week 28 of the year 2014.
    // assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms(9, 10, 11));
    println!("This prints date Time 1 at {:?}", dt_1);
    

    let dt = Utc.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12); // `2014-07-08T09:10:11.012Z`
    // assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000));
    // assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000));
    println!("This prints date Time at {:?}", dt);

    // dynamic verification
    // assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
    //         LocalResult::Single(Utc.ymd(2014, 7, 8).and_hms(21, 15, 33)));
    // assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
    // assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

    // other time zone objects can be used to construct a local datetime.
    // obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
    let local_dt = Local.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12);
    println!("This prints local Time at {}", local_dt.format("%a %b %e %T %Y").to_string());
    let fixed_dt = FixedOffset::east(9 * 3600).ymd(2014, 7, 8).and_hms_milli(18, 10, 11, 12);
    println!("This prints fixed Time at {}", fixed_dt);
    // assert_eq!(dt, fixed_dt);
}
