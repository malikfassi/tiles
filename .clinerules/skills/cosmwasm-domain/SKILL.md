---
name: cosmwasm-domain
description: Connaissances du domaine NFT CosmWasm / CW721 et de l'écosystème Cosmos 2026 - standard CW721, royalties, storage, gas, cible Stargaze 2.0 sur Cosmos Hub. À charger pour toute spec, règle ou code du contrat.
---

# Domaine : NFT CosmWasm / CW721

Ces règles sont générales et s'appliquent au contrat tiles, pas à une chaîne particulière.

## Cible et contraintes d'écosystème (2026)
- Stargaze n'est plus une chaîne : c'est une application sur le **Cosmos Hub** (`cosmoshub-4`, gas ATOM). L'ancienne L1
  et son testnet `elgafar-1` sont morts ; le dépôt `public-awesome/stargaze` est archivé depuis le 11 juin 2026.
- **`sg721` n'existe plus** comme cible. Le standard des collections Stargaze 2.0 est **CW721 + extension de collection**
  (`collection_info_extension` avec `royalty_info` : `payment_address` + `share`, maximum 10 %).
- Les collections officielles passent par Stargaze Studio 2.0 (wizard, stockage Arweave permanent, royalties on-chain).
- Détail et sources : `docs/notes/ecosysteme-cosmos-2026.md`.

## Standard CW721
- Un token = un `token_id` (String), un `token_uri` (métadonnées hors chaîne) et une `extension` (données on-chain).
- Transfers, approvals et operators sont fournis par `cw721-base` : ne pas les réécrire.
- `SendNft` permet d'envoyer un NFT à un contrat : c'est le mécanisme standard d'intégration marketplace/staking.
  Un contrat qui reçoit des NFT doit implémenter `Receiver` et refuser proprement ce qu'il ne comprend pas.
- Les royalties ne sont **pas** appliquées par le contrat NFT : c'est le marketplace qui lit `royalty_info` et paie.
  Ne jamais promettre dans le contrat une garantie que le marché n'applique pas.

## Économie on-chain
- Toute répartition de fonds doit être **exacte** : la somme des parts versées égale le montant reçu.
  Gérer explicitement le reste d'arrondi (le dernier bénéficiaire reçoit le reliquat), jamais de perte silencieuse.
- Les pourcentages se calculent en points de base (entiers) plutôt qu'en flottants.
- Ne jamais faire confiance aux fonds envoyés : `info.funds` est vérifié, jamais supposé.

## Storage et gas
- `cw-storage-plus` uniquement (`Item`, `Map`, `IndexedMap`). Une clé = une donnée, pas de structure géante rechargée à chaque appel.
- Une écriture coûte ~10× une lecture : préférer `update` à `load` + `save`, batcher les écritures d'une même transaction.
- Éviter les `Vec` de taille variable dans l'état quand une taille fixe suffit (100 pixels par tuile).
- Un `execute` doit rester borné : pas de boucle sur une collection dont la taille n'est pas bornée par le message.

## Sécurité
- Vérifier ownership et droits **avant** toute mutation ; ne jamais dériver l'autorisation d'un champ fourni par l'appelant.
- Toute expiration (`expires`) est comparée au temps de la chaîne (`env.block.time`), jamais à une valeur passée en argument.
- Les bornes métier (index de pixel, couleur, durée) sont validées dans le handler, avec des variantes d'erreur dédiées.
- Pas de panique sur entrée utilisateur : seulement des `ContractError`.
