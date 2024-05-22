struct Livre {
    titre: String,
    auteur: String,
    prix: f64
}

impl Livre {
    pub fn new(titre: String, auteur: String, prix: f64) -> Self {
        Livre {
            titre : titre,
            auteur : auteur,
            prix : prix
        }
    }

    pub fn afficher_informations(self) {
        println!("Le livre intitulé '{}', écrit par l'auteur '{}' se ved à {} euros.", self.titre, self.auteur, self.prix);
    }
}

fn main() {
    let livre_1 = Livre::new("100 Exercices Python pour s'entrainer".to_string(), "Laurenttine K.Masson".to_string(), 10.99);
    livre_1.afficher_informations();
}
