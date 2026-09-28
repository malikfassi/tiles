# 0003 — Portage sur CosmWasm 2.x

- Statut : accepté
- Date : 2026-09-28

## Contexte
Le contrat est en `cosmwasm-std` 1.5 / `cw-storage-plus` 1.2 / `cw721` 0.18. Toutes les chaînes Cosmos vivantes
(Cosmos Hub 2026 compris) embarquent désormais un module wasm en CosmWasm **2.x**, dont les binaires 1.x ne sont plus
la cible naturelle. Migrer maintenant coûte moins cher que migrer après le portage CW721 + le déploiement testnet.

## Décision
Le contrat est porté sur **`cosmwasm-std` 2.x**, avec `cw-storage-plus` 2.x, `cw721-base` 1.x et `cw-multi-test` à jour,
en une seule passe, avant toute nouvelle fonctionnalité.

## Conséquences
- Changements mécaniques à prévoir : entry points (feature `library`, `cosmwasm_std::entry_point`), `Response`/`SubMsg` typés,
  `to_json_binary`/`from_json` à la place des helpers dépréciés, `Decimal`/`Uint128` explicites, `Empty` pour les extensions CW721 non utilisées.
- L'état stocké reste compatible en lecture si les structs ne changent pas de forme — mais un test de migration est ajouté.
- La chaîne d'outils de déploiement (`cargo wasm`, `cosmwasm-check`) passe aux versions 2.x.
- Le portage 1.x → 2.x est une tâche isolée, sans changement de comportement : les tests existants doivent passer sans être réécrits
  (hors ajustements d'API de test).

## Alternatives écartées
- Rester en 1.x le temps du portage CW721 : double migration, et risque de devoir migrer dans l'urgence au moment du testnet.
- Attendre de connaître la cible de production exacte : aucune chaîne cible actuelle ne justifie 1.x, le choix n'apporterait aucun gain.
