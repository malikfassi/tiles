# 0002 — Standard NFT : CW721 standard plutôt que sg721

- Statut : accepté
- Date : 2026-09-28

## Contexte
Le contrat actuel s'appuie sur `sg721` et `sg721-base` (version 3.15), l'extension de collection propre à Stargaze L1.
Ces crates ne sont plus maintenues et ne sont plus la référence de Stargaze 2.0 : les collections du Cosmos Hub
sont des contrats CW721 avec une extension de collection (`collection_info_extension` contenant `royalty_info`).
La logique propre du projet (pixels, expiration, prix évolutif, répartition des paiements) est un sur-ensemble de CW721.

## Décision
Le contrat utilise **`cw721-base`** (CW721 standard) pour la partie NFT et conserve sa logique de tuiles par-dessus.
Les métadonnées de collection et les royalties suivent le schéma documenté par Stargaze 2.0
(`collection_info_extension` → `royalty_info` : `payment_address`, `share` décimale, maximum 0.10).

## Conséquences
- Le contrat est compatible avec tout marketplace CW721 lisant `royalty_info` : Stargaze 2.0, mais aussi tout autre marché CW721 compatible.
- Les royalties restent déclaratives : c'est le marketplace qui les applique, pas le contrat NFT.
- Le contrat demeure portable vers toute chaîne CosmWasm 2.x — utile si la cible changeait.
- Perte des spécificités sg721 inutiles ici (freeze metadata, migration fee, minter vending) : elles n'étaient pas utilisées par la logique de tuiles.
- La compilation et les tests passent par `cw-multi-test` au lieu de `sg-multi-test`.

## Alternatives écartées
- Garder `sg721` : dépendance morte, plus déployable, aucun bénéfice fonctionnel pour tiles.
- Réécrire entièrement la couche NFT à la main : réintroduirait des bugs de sécurité (approvals, operators) sur du code standard éprouvé.
- Utiliser `cw721-base` seul sans extension de royalties : incompatible avec l'économie du projet.
