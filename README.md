# Versioning Tool in Rust

Bienvenue dans mon premier projet en Rust! Ce projet consiste en un outil de versioning, conçu pour gérer les versions
de fichiers ou de projets de manière simple et efficace. C'est un projet ambitieux, mais je prends beaucoup de plaisir à
le développer. Il y a une grande disparité de qualité de code entre ce que j'ai fait au début et ce que je suis capable
de produire maintenant.
Je change le code et l'améliore au fur et à mesure.

## 🎯 Objectif du projet

L’objectif est de fournir un outil de versioning léger et rapide, adapté aux développeurs souhaitant gérer les versions
de leurs fichiers ou projets de manière simplifiée. Ce projet est une exploration des capacités de Rust en termes de
gestion de fichiers, de performances, et de sécurité mémoire.

## 🛠️ Fonctionnalités

- **Suivi des versions** : Capture et enregistre les modifications sur vos fichiers.
- **Gestion des branches** : Crée et manipule des branches pour des flux de travail parallèles.
- **Fusion** : Permet la fusion de branches avec gestion des conflits.
- **Historique des versions** : Visualisez un historique complet des modifications.
- **Facilité d’utilisation** : Interface en ligne de commande simple et intuitive.

## 🚀 Installation

Pour utiliser cet outil, vous devrez avoir installé [Rust](https://www.rust-lang.org/tools/install) sur votre système.

Clonez le projet depuis GitHub et compilez-le:

```bash
git clone https://github.com/divinoschaeffer/D.I.T.git
cd dit
cargo build --release
```

L'exécutable sera disponible dans le dossier `target/release`.

## 📖 Utilisation

Voici un aperçu des commandes principales :

- **Initialiser un nouveau dépôt** :
  ```bash
  dit init
  ```

- **Ajouter des fichiers** :
  ```bash
  dit add <nom-du-fichier>
  ```

- **Faire un commit** :
  ```bash
  dit commit -m "Message de commit"
  ```

- **Créer une branche** :
  ```bash
  dit branch <nom-de-la-branche>
  ```

- **Fusionner une branche** :
  ```bash
  dit merge <nom-de-la-branche>
  ```
- **Changer de branche** :
    ```bash
    dit checkout <nom-de-la-branche>
    ```
- **Retourner à l'état d'un commit** :
    ```bash
    dit revert <id-commit>
    ```
- **Afficher l'abre de commit** :
    ```bash
    dit commit -s
    ```

## 🛠️ Technologies Utilisées

- **Langage** : [Rust](https://www.rust-lang.org/) pour sa performance et sa sécurité.
- **Gestion des dépendances** : [Cargo](https://doc.rust-lang.org/cargo/).

## 🤝 Contribuer

Les contributions sont les bienvenues ! Si vous souhaitez apporter des améliorations, n'hésitez pas à forker le projet
et à soumettre une pull request.

1. Forkez le projet.
2. Créez votre branche de fonctionnalité (`git checkout -b feature/ma-fonctionnalite`).
3. Commitez vos modifications (`git commit -m 'Ajoute une fonctionnalité'`).
4. Poussez sur la branche (`git push origin feature/ma-fonctionnalite`).
5. Ouvrez une Pull Request.

## 📄 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 📧 Contact

Pour toute question, n'hésitez pas à me contacter à [my-email@example.com](divino.schaeffer@gmail.com).
