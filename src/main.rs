use std::thread;
// use futures::executor::block_on;
use std::time::Duration;
fn dance_1(){
    thread::sleep(Duration::from_secs(3));
    println!("I am Dancing on the song");  
}
fn main(){
fn dance(){
    thread::sleep(Duration::from_secs(5));
    println!("I am Dancing on the song on dance function"); 
}    
learn_song();
sing_song();
dance(); 
dance_1();

}
fn sing_song() {
    thread::sleep(Duration::from_secs(2));
    println!("I am singing the song");
}
fn learn_song(){
    thread::sleep(Duration::from_secs(2));
    println!("I am learning the song");
}