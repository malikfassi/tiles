---
name: adr
description: Enregistre une décision technique ou produit sous forme d'ADR court dans docs/adr/.
argument-hint: "[décision en une phrase]"
disable-model-invocation: true
model: haiku
effort: low
---

Décision : $ARGUMENTS

1. Trouve le prochain numéro libre dans `docs/adr/` (`ls docs/adr`).
2. Crée `docs/adr/NNNN-titre-court.md` en suivant `docs/adr/0000-template.md`, statut `accepté`, date du jour. 25 lignes maximum.
3. Si la décision remplace un ADR, passe l'ancien en `remplacé par NNNN`.
4. Réponds avec le chemin du fichier, rien d'autre.
