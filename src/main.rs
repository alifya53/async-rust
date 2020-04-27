// use std::thread;
// // use futures::executor::block_on;
// use std::time::Duration;
// fn dance_1(){
//     thread::sleep(Duration::from_secs(3));
//     println!("I am Dancing on the song");  
// }
// fn main(){
// fn dance(){
//     thread::sleep(Duration::from_secs(5));
//     println!("I am Dancing on the song on dance function"); 
// }    
// learn_song();
// sing_song();
// dance(); 
// dance_1();

// }
// fn sing_song() {
//     thread::sleep(Duration::from_secs(2));
//     println!("I am singing the song");
// }
// fn learn_song(){
//     thread::sleep(Duration::from_secs(2));
//     println!("I am learning the song");
// }

// use futures::executor::block_on;
// use std::thread;
// use std::time::Duration;

// async fn IR_SENSOR()-> bool {
//     thread::sleep(Duration::from_secs(2));
//     println!{"sending data from IR sensor..."};
    
//     true
// }
// async fn ultrasonic_sensor()-> i32{
//     thread::sleep(Duration::from_secs(5));
//     println!{"sending data from ultrasonic..."};
//    22
// }
// async fn combining_data(){
//     // let IR = IR_SENSOR();
//     // let ultra = ultrasonic_sensor();
//     let s1 = IR_SENSOR().await;
//     let s2 = ultrasonic_sensor().await;
//     // futures::join(s1,s2);
//     println!{"Final data {:#?} {:#?}",s1,s2};
// }
// fn main() {
// block_on(combining_data());
// }

use async_std::task;
use futures::executor::block_on;
use std::thread;
use std::time::Duration;

async fn IR_SENSOR()-> bool {
    task::sleep(Duration::from_secs(2)).await;
    println!{"sending data from IR sensor..."};
    
    true
}
async fn ultrasonic_sensor()-> i32{
    task::sleep(Duration::from_secs(5)).await;
    println!{"sending data from ultrasonic..."};
   22
}
async fn combining_data(){
    // let IR = IR_SENSOR();
    // let ultra = ultrasonic_sensor();
    let s1 = IR_SENSOR().await;
    let s2 = ultrasonic_sensor().await;
    // futures::join(s1,s2);
    println!{"Final data {:#?} {:#?}",s1,s2};
}
fn main() {
block_on(combining_data());
}