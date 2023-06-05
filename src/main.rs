use std::io;

struct Rectangle {
    longueur: f64,
    largeur: f64,
}

impl Rectangle {
    fn new(longueur: f64, largeur: f64) -> Rectangle {
        Rectangle { longueur, largeur }
    }

    fn calculer_surface(&self) -> f64 {
        self.longueur * self.largeur
    }
}

struct Cercle {
    rayon: f64,
}

impl Cercle {
    fn new(rayon: f64) -> Cercle {
        Cercle { rayon }
    }

    fn calculer_surface(&self) -> f64 {
        std::f64::consts::PI * self.rayon.powi(2)
    }
}

struct Triangle {
    base: f64,
    hauteur: f64,
}

impl Triangle {
    fn new(base: f64, hauteur: f64) -> Triangle {
        Triangle { base, hauteur }
    }

    fn calculer_surface(&self) -> f64 {
        (self.base * self.hauteur) / 2.0
    }
}

fn main() {
    let rectangle = creer_rectangle();
    let cercle = creer_cercle();
    let triangle = creer_triangle();

    println!("Surface du rectangle: {}", rectangle.calculer_surface());
    println!("Surface du cercle: {}", cercle.calculer_surface());
    println!("Surface du triangle: {}", triangle.calculer_surface());
}

fn creer_rectangle() -> Rectangle {
    println!("Veuillez entrer la longueur du rectangle:");
    let longueur = lire_nombre();

    println!("Veuillez entrer la largeur du rectangle:");
    let largeur = lire_nombre();

    Rectangle::new(longueur, largeur)
}

fn creer_cercle() -> Cercle {
    println!("Veuillez entrer le rayon du cercle:");
    let rayon = lire_nombre();

    Cercle::new(rayon)
}

fn creer_triangle() -> Triangle {
    println!("Veuillez entrer la base du triangle:");
    let base = lire_nombre();

    println!("Veuillez entrer la hauteur du triangle:");
    let hauteur = lire_nombre();

    Triangle::new(base, hauteur)
}

fn lire_nombre() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");

    input.trim().parse().expect("Veuillez entrer un nombre valide")
}
