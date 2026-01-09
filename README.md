# Rust DHCP Program

Un programme Rust simple pour scanner et découvrir les appareils connectés sur votre réseau local.

## Description

Ce programme analyse votre sous-réseau local en effectuant un ping sur toutes les adresses IP possibles (de .1 à .254) et tente de résoudre les noms d'hôtes via DNS inverse pour les appareils qui répondent.

## Fonctionnalités

- Détection automatique de votre adresse IP locale
- Scan du sous-réseau /24 complet
- Ping asynchrone pour une détection rapide
- Résolution DNS inverse pour obtenir les noms d'hôtes
- Affichage clair des résultats

## Prérequis

- Rust (édition 2021 ou supérieure)
- Windows/Linux/macOS avec accès réseau

## Installation

1. Clonez ce dépôt :
   ```bash
   git clone <url-du-depot>
   cd rustdhcpprogram
   ```

2. Construisez le projet :
   ```bash
   cargo build --release
   ```

## Utilisation

Lancez le programme :
```bash
cargo run
```

Ou avec l'exécutable compilé :
```bash
./target/release/rustdhcpprogram
```

Le programme affichera :
- Votre adresse IP locale
- La progression du scan
- Une liste des appareils connectés avec leur IP et nom d'hôte

## Comment ça marche

1. **Détection IP** : Utilise `local-ip-address` pour trouver l'adresse IP locale
2. **Scan réseau** : Ping toutes les adresses IP du sous-réseau (1-254)
3. **Résolution DNS** : Pour chaque IP répondante, effectue une recherche DNS inverse
4. **Affichage** : Présente les résultats dans un tableau formaté

## Dépendances

- `local-ip-address` : Pour la détection d'IP locale
- `dns-lookup` : Pour la résolution DNS inverse

## Limitations

- Fonctionne uniquement sur des réseaux /24 (masque 255.255.255.0)
- Nécessite les droits d'exécution de ping (généralement disponible)
- La résolution DNS peut échouer pour certains appareils

## Licence

MIT