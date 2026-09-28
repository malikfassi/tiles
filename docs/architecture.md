# Architecture — tiles

> À mettre à jour par le rôle `architect` (`.clinerules/skills/architect/SKILL.md`).
> Les décisions structurantes sont dans `docs/adr/`.
> Inventaire T-002 réalisé le 2026-09-28 sur l'état de janvier 2025 (1 387 lignes, 48 tests verts).

## Vue d'ensemble

Un seul contrat CosmWasm. Une **tuile** (`Tile`) est un NFT qui contient l'état de 100 pixels ; un pixel a une couleur
et une date d'expiration. **N'importe qui** peut payer pour écrire la couleur d'un pixel pendant une durée choisie ;
le prix suit cette durée via `PriceScaling` ; les fonds reçus vont au propriétaire de la tuile et à l'adresse de royalties.
Une couleur payée est protégée jusqu'à son expiration (ADR 0004).

## Décisions de fond

| ADR | Décision |
|---|---|
| 0001 | Cible : Stargaze 2.0 sur le Cosmos Hub |
| 0002 | Standard CW721 + extension de collection (royalties ≤ 10 %) |
| 0003 | Portage CosmWasm 2.x avant toute nouvelle fonctionnalité |
| 0004 | Coloriage ouvert à tous, bail de couleur protégé jusqu'à expiration |

## Structure du code

```
src/lib.rs                 4 modules : contract, core, defaults, events
src/contract/
  contract.rs              entry points (instantiate, execute, query)
  msg.rs                   InstantiateMsg (= Sg721InstantiateMsg), ExecuteMsg (= sg721::ExecuteMsg<Extension, TileExecuteMsg>), QueryMsg
  state.rs                 une seule entrée : PRICE_SCALING (Item<PriceScaling>)
  error.rs                 ContractError (8 variantes + Std + Base/sg721)
  instantiate.rs           initialise sg721 + PRICE_SCALING par défaut
  execute.rs               dispatch manuel de 12 variantes sg721 + 2 variantes propres
  query.rs                 dispatch manuel de 13 queries (12 déléguées à sg721, 1 propre)
  tiles/
    mint.rs                mint direct : hash des pixels par défaut dans l'extension
    set_pixel_color.rs     cœur du produit : validation, prix, paiement, mise à jour du hash
    update_price_scaling.rs  mise à jour des prix, réservée à l'adresse de royalties
src/core/
  pricing.rs               PriceScaling (4 paliers) + interpolation linéaire + prix quadratique
  tile/mod.rs              struct Tile { tile_hash: String } — l'extension du NFT
  tile/metadata.rs         PixelData, TileMetadata (Vec de 100), PixelUpdate, hash SHA-256
src/events/                6 événements typés + trait EventData (into_event / try_from_event)
src/defaults/constants.rs  constantes, dont CHAIN_ID/NODE_URL encore sur elgafar-1 et le denom Stargaze
tests/                     48 tests verts (contract/, core/, utils/)
```

## Ce que fait le contrat, précisément

| Message | Effet |
|---|---|
| `Instantiate` | Initialise sg721 (collection info, royalties, minter) + `PRICE_SCALING` par défaut. Émet `instantiate_price_scaling`. |
| `Mint` | Crée une tuile : calcule le hash des 100 pixels par défaut, l'écrit dans l'extension. Émet `mint_metadata`. |
| `SetPixelColor { token_id, current_metadata, updates }` | Vérifie le hash fourni contre celui stocké, calcule le prix total, vérifie les fonds, répartit royalties/owner, applique les mises à jour, écrit le nouveau hash. Émet `pixel_update`, `metadata_update`, `payment_distribution`. |
| `UpdatePriceScaling` | Réservé à `royalty_info.payment_address`. Émet `price_scaling_update`. |
| 11 autres variantes | Déléguées telles quelles à sg721 (transfer, send, approve, burn, update_collection_info…). |

## Problèmes identifiés (base du portage)

### Bloquants pour la cible (ADR 0001/0002)
- `sg_std::StargazeMsgWrapper` dans les signatures de `instantiate` et `execute` : le contrat ne peut pas être déployé hors Stargaze L1.
- `Sg721Contract<Tile>` comme base, plus `CollectionInfoResponse` et `sg721_base::ContractError` : dépendance à une crate morte.
- 11 variantes de messages et 12 queries recopiées à la main dans `execute.rs` et `query.rs` (~200 lignes de boilerplate) : `cw721-base` gère ce dispatch lui-même, tout ce bloc disparaît.
- `CHAIN_ID = "elgafar-1"`, `NODE_URL` Stargaze, denom `ustars` dans `constants.rs` et `build.rs`.

### Correction fonctionnelle (le contrat ne fait pas ce qu'il annonce)
- `validate_for_tile` (`src/core/tile/metadata.rs:116-126`) **retourne toujours `Ok(())`** : l'expiration d'un pixel n'est jamais vérifiée. N'importe qui peut réécrire un pixel encore valide, au même prix qu'un pixel libre. C'est la règle économique centrale du produit : à corriger avant toute mise en ligne.
- Le hash fourni par l'appelant (`current_metadata`) sert d'optimistic locking, mais comme `validate_for_tile` n'oppose rien, l'ensemble ne protège qu'avec la coopération du client.

### Sécurité
- `set_pixel_color` interroge le contrat lui-même (`query_wasm_smart` sur `env.contract.address`) pour connaître le propriétaire : coûteux en gas et inutile, l'état est déjà accessible via `deps`.
- `info.funds[0].amount == total_price` : ne vérifie ni le denom ni la longueur de `funds` (un envoi multi-denom passe si le premier correspond).
- Aucune borne haute sur la part de royalties (`royalty_info.share`) ni contrôle que le paiement vient bien d'une seule dénomination.
- Accès direct à `contract.tokens` (état interne de sg721) : à réécrire avec l'API publique.
- Pas de contrôle que `info.sender` est le propriétaire de la tuile avant de modifier ses pixels — l'argent va au bon propriétaire, mais le tiers qui paie peut colorier la tuile d'autrui sans son accord (comportement voulu ou non, à trancher par ADR).

### Standard CosmWasm (TODO.md §1-3)
- Hash par `format!("{}:{}:…")` concaténé au lieu d'une sérialisation canonique : deux états différents peuvent produire la même chaîne, et tout changement de format invalide les tokens déjà mintés.
- Validation métier dans `PixelUpdate` (`validate_integrity`, `validate_for_tile`) au lieu des handlers (TODO.md §2).
- Erreurs génériques formatées (`InvalidPixelUpdate { reason: String }`) au lieu de variantes typées (TODO.md §3).
- `Vec<PixelData>` de taille variable : à remplacer par `[PixelData; 100]` (TODO.md §4).
- Identifiant de pixel en `u32` alors que 100 pixels suffisent (`u8`) (TODO.md §9).
- `PixelData::default()` utilise `Addr::unchecked("")` : adresse invalide dans l'état initial.
- Timestamps en `u64` (secondes) au lieu de `cosmwasm_std::Timestamp`.
- `unwrap()` dans `pricing.rs::calculate_price` et `instantiate.rs` : paniquent sur entrée utilisateur (`checked_mul`/`checked_div` sur des divisions par zéro théoriques).

### Dette de structure
- `src/contract/mod.rs` redéclare `pub mod contract` (module dans un module du même nom) : inutile (TODO.md §8).
- `state.rs` ne contient qu'une entrée : l'état des pixels vit dans l'extension du NFT, donc les pixels d'une tuile ne sont pas interrogeables sans charger le NFT entier.
- Aucune query `TilePixels { token_id }` pour lire les pixels sans transférer tout le NFT.
- Pas d'entry point `migrate` : impossible de faire évoluer l'état d'un contrat déjà déployé.
- 7 warnings clippy mineurs.

## Suite

Ordre retenu dans `docs/plan/tasks.md` : T-003 (CosmWasm 2.x) → T-004 (CW721) → T-005/T-006 (mint et état) →
T-007/T-008/T-009 (validation, couleur, répartition) → T-010 (tests) → T-011 (migration) → T-012 (scripts Cosmos Hub).


