---
name: rust-conventions
description: Conventions de code Rust et CosmWasm du projet tiles - structure des modules, messages, erreurs, tests. À charger pour toute écriture ou relecture de code du contrat.
---

# Conventions : Rust + CosmWasm (tiles)

Cible : Rust edition 2021, **cosmwasm-std 2.x**, `cw-storage-plus`, `cw721-base`, `cw-multi-test`. Aucune dépendance Stargaze.

## Structure
- Découpage existant, à respecter :
  - `src/lib.rs` : déclaration des modules et ré-export des entry points (`instantiate`, `execute`, `query`, `migrate`).
  - `src/contract/` : `msg.rs` (messages), `state.rs` (état), `error.rs`, puis un fichier par message execute (`execute.rs`, `query.rs`, `instantiate.rs`).
  - `src/contract/tiles/` : un fichier par responsabilité métier (`mint.rs`, `set_pixel_color.rs`, `update_price_scaling.rs`).
  - `src/core/` : logique métier pure, sans dépendance à `Deps` ni `Env` quand c'est possible (testable en Rust standard).
  - `src/events/` : construction des événements, isolée du métier.
  - `src/defaults/` : constantes nommées, aucune valeur magique ailleurs.
- Pas de module dans un module du même nom (`contract/contract/`) — TODO.md le signale déjà.

## Entry points
- `#[cosmwasm_std::entry_point]` sur les quatre handlers, signatures standard :
  `instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<Response, ContractError>`.
- Les handlers orchestrent, valident et délèguent ; ils ne contiennent pas de logique métier longue.
- Chaque attribut de réponse est nommé en `snake_case` et porte une valeur exploitable par un indexeur
  (pas de JSON sérialisé dans un attribut d'événement).

## Messages et validation
- `#[cw_serde]` partout ; champs en `snake_case`, noms complets et lisibles.
- **Ne jamais valider dans les structures de données.** La validation vit dans le handler, en trois temps :
  forme du message, cohérence de l'état, règles métier. (TODO.md §2)
- Les adresses reçues sont validées et converties en `Addr` avant tout usage.
- Les bornes métier (index de pixel < `PIXELS_PER_TILE`, format de couleur, durée d'expiration min/max)
  sont vérifiées explicitement, avant toute écriture d'état.

## Erreurs
- Un variant nommé par cas d'erreur dans `ContractError` (`thiserror::Error`), jamais `StdError::generic_err` avec un message formaté.
- Variantes typiques à privilégier : `InvalidColorFormat`, `InvalidPixelId`, `InvalidExpirationTooShort`,
  `InvalidExpirationTooLong`, `Unauthorized`, `InsufficientFunds`, `TileNotFound`, `PixelUpdateNotAllowed`.
- Message d'erreur en anglais, court, sans donnée utilisateur interpolée.

## État
- `Item` pour les singletons (config, compteur), `Map`/`IndexedMap` pour les collections indexées par clé naturelle.
- Index de pixel en `u8` (100 pixels par tuile) ; les identifiants d'entités en `u32` seulement si réellement nécessaire.
- `&str` plutôt que `String` quand la valeur est de longueur fixe (couleur hexadécimale).
- Timestamps : `Timestamp` de cosmwasm, jamais un `u64` brut en secondes.
- `update` plutôt que `load` + `save` ; une seule écriture par entité et par message.
- Tout état écrit doit être lisible par une query : si l'état n'est pas interrogeable, la fonctionnalité est incomplète.

## Funds et économie
- `info.funds` : vérifier le montant exact attendu, une seule dénomination.
- Répartition : somme des parts == montant reçu, reliquat d'arrondi attribué explicitement.
- Arrondis : calcul en entiers (points de base), jamais en flottants.

## Tests
- `#[cfg(test)]` dans le fichier testé pour la logique pure ; `tests/` pour l'intégration `cw-multi-test`.
- Un test par règle métier, un test par variante d'erreur.
- Nommage : `test_<comportement>_<condition>` en anglais (`test_set_pixel_rejects_out_of_range_id`).
- Les tests vérifient **l'état résultant** et **les événements émis**, pas seulement le `Result`.
- `cargo test` doit passer sans nœud ni réseau.
- Fixtures de déploiement partagées dans `tests/utils/` plutôt que dupliquées par fichier.

## Style
- `cargo fmt` et `cargo clippy --all-targets -- -D warnings` propres.
- Pas de `unwrap()`/`expect()` hors tests et hors cas prouvés impossibles (avec un commentaire justificatif).
- Commentaires en anglais, uniquement pour expliquer un « pourquoi » non évident. Pas de commentaire qui paraphrase le code.
