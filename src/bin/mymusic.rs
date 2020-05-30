use mymusic::crawlers;

fn main() {
    let mydirs = crawlers::get_dirs("/Users/jleyba/Documents/test");  
    println!("Dirs: {:?}", mydirs); 
}
