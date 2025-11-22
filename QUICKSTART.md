# 🚀 Quick Start Guide - extstat

## Installation complète (5 minutes)

### 1. Installer Rust (si pas déjà fait)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Télécharger et builder extstat

```bash
# Extraire le projet (tu as déjà les fichiers)
cd extstat

# Builder (première fois = télécharge les dépendances, ~1-2 min)
./build.sh

# OU manuellement :
cargo build --release
```

### 3. Tester

```bash
# Test rapide sur le répertoire courant
./target/release/extstat . -c -n 10
```

### 4. Installer (optionnel)

```bash
# Copier le binaire dans ton PATH
sudo cp target/release/extstat /usr/local/bin/

# Maintenant utilisable partout
extstat /path/to/analyze
```

## Premier test

```bash
# Analyser ton home avec stats détaillées
extstat ~ -c -n 20

# Analyser /data en ignorant petits fichiers (< 1MB)
extstat /data -s 1048576 -c
```

## Comprendre le code Rust (bases)

### Structure du projet

```
extstat/
├── Cargo.toml          # Dépendances (comme package.json ou requirements.txt)
├── src/
│   └── main.rs         # Code source principal
└── target/
    └── release/
        └── extstat     # Binaire compilé
```

### Concepts Rust dans le code

**1. Structures de données**
```rust
struct Args {
    path: PathBuf,      // Type safe pour chemins
    min_size: u64,      // Unsigned 64-bit integer
    top: usize,         // Taille (comme size_t en C)
}
```

**2. Pattern matching (très puissant)**
```rust
path.extension()              // Option<&OsStr>
    .and_then(|s| s.to_str()) // Option<&str>
    .map(|s| format!(".{}", s))
    .unwrap_or_else(|| "[no extension]".to_string())
// Si une étape échoue → valeur par défaut
```

**3. Itérateurs parallèles (la magie de Rayon)**
```rust
files.par_iter()              // Traite en parallèle
    .filter_map(|entry| {     // Filtre + map en un
        // ...
    })
    .fold(|| HashMap::new(), |acc, x| {  // Accumule par thread
        // ...
    })
    .reduce(|| HashMap::new(), |a, b| {  // Fusionne les résultats
        // ...
    })
```

**4. Gestion d'erreurs avec Result**
```rust
fn main() -> Result<()> {     // Peut retourner une erreur
    let metadata = entry.metadata()?;  // ? = propage l'erreur
    Ok(())                     // Succès
}
```

### Modifier le code

**Changer les couleurs ?**
Ligne ~120 :
```rust
Cell::new(&stat.extension).fg(Color::Green),  // Change Green
Cell::new(size_str).fg(Color::Yellow),        // Change Yellow
```

**Changer la largeur de la barre ?**
Ligne ~170 :
```rust
let bar_width = 30;  // Change à 50 pour barre plus longue
```

**Ajouter un tri par nombre de fichiers ?**
Ligne ~90 :
```rust
// Remplace
ext_stats.sort_by(|a, b| b.total_size.cmp(&a.total_size));
// Par
ext_stats.sort_by(|a, b| b.file_count.cmp(&a.file_count));
```

## Commandes utiles

```bash
# Builder en mode debug (plus rapide à compiler)
cargo build

# Lancer sans installer
cargo run -- /path -c

# Vérifier le code sans compiler
cargo check

# Tester avec différents arguments
cargo run -- . -n 5
cargo run -- /var -s 10000000 -c

# Nettoyer les builds
cargo clean
```

## Performance attendue

Sur un système moderne (SSD + multi-core) :
- **~500k fichiers/seconde** en scan
- **~2-3 secondes** pour 1 million de fichiers
- **Utilisation mémoire** : ~50-100MB pour 1M fichiers

## Prochaines étapes

Une fois que tu es à l'aise avec la v1 :

1. **Ajouter une progress bar** (facile)
2. **Export JSON** (moyen)
3. **Mode interactif TUI** (avancé)
4. **Drill-down par extension** (avancé)

Dis-moi ce qui t'intéresse et je t'aide à le coder !

## Debugging

Si ça ne compile pas :

```bash
# Vérifier Rust est installé
rustc --version
cargo --version

# Nettoyer et rebuild
cargo clean
cargo build --release

# Voir les erreurs détaillées
cargo build --verbose
```

## Ressources Rust

- [Rust Book (français)](https://jimskapt.github.io/rust-book-fr/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## Questions fréquentes

**Q: Pourquoi `cargo build` est long la première fois ?**
R: Il télécharge et compile toutes les dépendances. Ensuite c'est rapide.

**Q: Différence entre `cargo build` et `cargo build --release` ?**
R: Release = optimisé (2-5x plus rapide) mais compile plus lentement.

**Q: Comment débugger ?**
R: Ajoute `println!("debug: {:?}", variable);` dans le code, puis `cargo run`.

**Q: Puis-je cross-compiler pour un autre OS ?**
R: Oui ! Avec `cross` ou `cargo build --target ...`

Besoin d'aide ? Demande-moi ! 🦀
