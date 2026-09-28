# Tiles — produit

> Document de cadrage produit. Seul le rôle `product` écrit ici (voir `.clinerules/skills/product/SKILL.md`).
> La section **Entrées** est la source de vérité : elle n'est modifiée que par Malik.

## Entrées

- 2026-09-28 — Malik : « on repart sur le nouveau Stargaze » (Cosmos Hub). Reprise du projet après abandon (dernier commit : janvier 2025).
- 2026-09-28 — Malik : le smart contract existe déjà et doit être porté ; l'interface web et l'orchestration restent à construire.
- 2026-09-28 — Malik : **tout le monde peut payer pour colorier** un pixel (canvas collaboratif ouvert, sans autorisation du propriétaire — ADR 0004).
- 2026-09-28 — Malik : **l'écrasement est interdit tant que le bail de couleur est valide** ; à expiration le pixel redevient libre (ADR 0004).

## Vision

Une plateforme de pixel art collaborative en NFT. Chaque **tuile** est un NFT ; sur chaque tuile, le propriétaire
peut changer la couleur des pixels pendant un temps limité, en payant — le prix évolue selon le remplissage de la tuile,
et les revenus sont répartis entre le propriétaire, les royalties de la collection et la plateforme.

Le plaisir vient de deux choses : **un canvas partagé** que tout le monde voit évoluer, et **une économie locale**
où colorer tôt coûte moins cher que colorer tard.

## À compléter

- User stories et critères d'acceptation (à produire par le rôle `product`)
- Périmètre V1 (MoSCoW)
- Questions ouvertes
