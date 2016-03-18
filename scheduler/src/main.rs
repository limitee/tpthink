extern crate cron;
use cron::CronSchedule;

fn main() {
  //min     hour     day  month    year
  let expression = "2,17,51 * * * *";
  let schedule = CronSchedule::parse(expression).unwrap();
  println!("Upcoming fire times for '{}':", expression);
  for datetime in schedule.upcoming().take(12) {
    println!("-> {}", datetime);
  }

}