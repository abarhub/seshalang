#[derive(Debug)]
pub struct Sortie {
    liste_sortie: Vec<String>,
    dernier_retour_ligne: bool,
    directe: bool,
}

impl Sortie {
    pub(crate) fn new(directe: bool) -> Sortie {
        Sortie {
            liste_sortie: vec![],
            dernier_retour_ligne: true,
            directe,
        }
    }

    pub(crate) fn ajoute(&mut self, s: String, retour_ligne_fin: bool) {
        if self.directe {
            if retour_ligne_fin {
                println!("{}", s);
            } else {
                print!("{}", s);
            }
        } else {
            if !self.dernier_retour_ligne && self.liste_sortie.len() > 0 {
                let pos = self.liste_sortie.len() - 1;
                let s2 = self.liste_sortie[pos].clone() + s.as_str();
                self.liste_sortie[pos] = s2;
            } else {
                self.liste_sortie.push(s);
            }
            self.dernier_retour_ligne = retour_ligne_fin;
        }
    }

    pub fn sortie(&self) -> Vec<String> {
        self.liste_sortie.clone()
    }
}
