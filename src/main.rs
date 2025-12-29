use canopie_themes::{get_theme_overview, get_themes};

fn main() -> () {
    println!("{:?}", get_themes());
    println!("{:?}", get_theme_overview("morningstar".to_string()));
}
