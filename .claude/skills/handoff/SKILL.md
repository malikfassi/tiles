---
name: handoff
description: Met à jour docs/STATUS.md en fin de session pour que la session suivante reparte sans ré-explorer.
disable-model-invocation: true
model: sonnet
effort: low
---

Réécris `docs/STATUS.md` (40 lignes maximum) à partir de cette session :

- **Où on en est** : phase, tâches faites, ce qui marche vraiment (testé ou non).
- **Prochaine étape** : une action précise, avec la commande (`/next-task T-0xx`, `/kickoff …`).
- **Décisions en attente de Malik.**
- **Pièges connus** : ce qui a coûté du temps et qu'il ne faut pas refaire.

Ne recopie pas ce qui est déjà dans les autres docs : renvoie vers eux. Date du jour en tête.
Ensuite, conseille à Malik de lancer `/clear` avant la tâche suivante.
