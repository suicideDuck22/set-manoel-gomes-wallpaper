use std::{fs, env};
use reqwest;
use wallpaper;

fn main(){
    let dir = env::temp_dir();
    let path = format!("{}{}", dir.to_str().unwrap(), "manoel.jpg");
    let copy_path = format!("{}{}", dir.to_str().unwrap(), "manoel2.jpg");
    println!("{}", path);

    let mut file = fs::File::create(&path).unwrap();
    reqwest::blocking::get("https://oimparcial.com.br/app/uploads/2021/04/azul-caneta.jpg").unwrap().copy_to(&mut file).unwrap();
    fs::copy(&path, &copy_path).unwrap();

    wallpaper::set_from_path(&copy_path).unwrap()
}