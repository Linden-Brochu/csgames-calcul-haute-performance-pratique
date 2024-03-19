# Projet

Ce projet a servi de pratique pour la compétition de calcul haute performance des CS Games 2024.
Il s'agit de la somme des connaissances acquisses des deux équipes de l'ÉTS pour la compétition, soit le multithreading
et la programmation GPU avec le framework / librairie WGPU

## Tâche

En 3h, faire une app CLI pour multiplier des matrices sur GPU

## Threads

### Thread main

- Initialisation des threads

### Thread 1

- Lire à la console les commandes

### Thread 2

- Initialiser WGPU
- Faire les calculs si demander

## Commandes

- `create <name> <rows> <cols>`: Créer une matrice
- `delete <name>`: Supprimer une matrice
- `print <name>`: Afficher une matrice
- `multiply <name1> <name2> <name3>`: Multiplier deux matrices et stocker le résultat dans une troisième matrice
- `quit`: Quitter l'application
- `set <name> <row> <col> <value>`: Modifier une valeur dans une matrice