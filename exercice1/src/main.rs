struct Biscuit {
    name: String,
    forme: String,
}

impl Biscuit {
    pub fn new(name: String, forme: String) -> Self {
        Biscuit {
            name : name,
            forme : forme,
        }
    }

    pub fn cuir(self) {
        println!("Ce {} a été  cuit sous forme d'une {} \nBonne dégustation",self.name, self.forme);
    }
}


fn main() {
    let biscuit_1 = Biscuit::new("cookie pépite de chocolat".to_string(), "étoile".to_owned());
    biscuit_1.cuir();
}
