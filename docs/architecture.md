# Architecture — tiles

> À mettre à jour par le rôle `architect` (`.clinerules/skills/architect/SKILL.md`).
> Les décisions structurantes sont dans `docs/adr/`.

## Vue d'ensemble

Un seul contrat CosmWasm. Une **tuile** (`Tile`) est un NFT qui contient un état de 100 pixels ; un pixel a une couleur
et une date d'expiration. Le propriétaire d'une tuile peut payer pour écrire la couleur d'un pixel ; le prix dépend
du taux de remplissage de la tuile ; les fonds reçus sont répartis entre le propriétaire, les royalties et la plateforme.

## Décisions de fond

| ADR | Décision |
|---|---|
| 0001 | Cible : Stargaze 2.0 sur le Cosmos Hub |
| 0002 | Standard CW721 + extension de collection (royalties ≤ 10 %) |
| 0003 | Portage CosmWasm 2.x avant toute nouvelle fonctionnalité |

## Structure du code (état 2025, à réviser en T-002)

```
src/lib.rs                modules + entry points
src/contract/
  msg.rs                  ExecuteMsg, QueryMsg, InstantiateMsg
  state.rs                état stocké
  error.rs                ContractError
  instantiate.rs          instanciation
  execute.rs              dispatch des messages execute
  query.rs                dispatch des queries
  tiles/                  responsabilités métier
    mint.rs               création d'une tuile
    set_pixel_color.rs    écriture d'un pixel (cœur du produit)
    update_price_scaling.rs  paramétrage de l'évolution des prix
src/core/
  pricing.rs              calcul de prix
  tile/metadata.rs        métadonnées de tuile (hash, couleur, timestamps)
src/events/               construction des événements (mint, pixel, répartition, prix)
src/defaults/             constantes (taille de tuile, parts, bornes)
tests/                    intégration cw-multi-test (contract/, core/, utils/)
```

## Dette identifiée (voir aussi `TODO.md` à la racine)

- Dépendances Stargaze (`sg721-base`, `sg-std`, `vending-*`) : à retirer (ADR 0002).
- `cosmwasm-std` 1.5 : à porter en 2.x (ADR 0003).
- Validation encore dans les structures de données (`validate_integrity`, `validate_for_tile`) : à déplacer dans les handlers.
- Erreurs génériques formatées : à remplacer par des variantes dédiées.
- `Vec<PixelData>` de taille variable : à remplacer par une structure à taille fixe interrogeable.
- Identifiant de pixel en `u32` : à ramener à `u8` (100 pixels par tuile).
- Workspace cassé : `Cargo.toml` déclare des membres `vendor/*` qui n'existent pas (T-001).
- `scripts/` et `scripts/state/*.state` ciblent `elgafar-1` : historiques, à remplacer pour le testnet du Hub.
