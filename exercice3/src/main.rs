struct Note {
    note: i32,
    nom_etudiant: String,
}

mod note {
    use super::*;

    pub fn new(note: i32,nom_etudiant: String) -> Note {
        Note { note, nom_etudiant }
    }

    pub fn a_reussi(this: Note) {
        if this.note > 75 {
            println!("L'étudiant(e) {} a réussi(e)", this.nom_etudiant);
        } else {
            println!("L'étudiant(e) {} a échoué(e)", this.nom_etudiant);
        }
    }
}

impl Note {
    

    
}

fn main() {
    let note_1 = note::new(80, "Julien".to_string());
    note::a_reussi(note_1);

    let note_2 = note::new(35, "Amélie".to_string());
    note::a_reussi(note_2);
}
