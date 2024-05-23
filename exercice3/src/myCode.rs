struct Note {
    note: i32,
    nom_etudiant: String,
}

impl Note {
    pub fn new(note: i32,nom_etudiant: String) -> Self {
        Note {
            note : note,
            nom_etudiant : nom_etudiant
        }
    }

    pub fn a_reussi(self) {
        if self.note > 75 {
            println!("L'étudiant(e) {} a réussi(e)", self.nom_etudiant);
        } else {
            println!("L'étudiant(e) {} a échoué(e)", self.nom_etudiant);
        }
    }
}

fn main() {
    let note_1 = Note::new(80, "Julien".to_string());
    note_1.a_reussi();

    let note_2 = Note::new(35, "Amélie".to_string());
    note_2.a_reussi();
}