# STATUS — 2026-09-28

## Où on en est
Reprise du projet après abandon (dernier commit : 2025-01-03). Objectif de session : retrouver le fil et remettre le projet sur les rails.

**Fait :**
- État des lieux complet du code et de l'écosystème ; comparaison avec le projet voisin `mosaic` (à `/Users/malikfassifihri/dev/mosaic`).
- Décision de cible : **Stargaze 2.0 sur le Cosmos Hub** (ADR 0001), standard **CW721** (ADR 0002), portage **CosmWasm 2.x** (ADR 0003).
- Pilotage installé : `.clinerules/` (rôles, workflows, conventions) + miroir `.claude/skills/`, `docs/` (product, architecture, ADR, plan, notes).
- Note d'écosystème sourcée : `docs/notes/ecosysteme-cosmos-2026.md` (Stargaze L1 morte, Hub permissionné, précédents de gouvernance).

**Ce qui marche vraiment :** rien de neuf à ce stade — le workspace est cassé (dossier `vendor/` référencé mais absent du `Cargo.toml`), le README était vide, les scripts et `.state` visent `elgafar-1` (testnet disparu).

## Prochaine étape
`/next-task T-001` — réparer le workspace et obtenir un `cargo test` vert (voir `docs/plan/tasks.md`).

## Décisions en attente de Malik
- Aucune bloquante pour avancer : les choix déjà tranchés (cible, CW721, CosmWasm 2.x) suffisent jusqu'à T-010.
- À venir plus tard : mode de production (Studio 2.0 + logique hors chaîne vs proposition de gouvernance) — ADR à ouvrir quand le contrat sera complet.

## Pièges connus (ne pas refaire)
- **Ne plus chercher Stargaze L1** : `elgafar-1` est mort, `starsd` n'est plus la CLI, le repo `public-awesome/stargaze` est archivé. Tout le dossier `launchpad/` est une copie morte du projet Stargaze.
- **Ne pas réintroduire `sg721`/`sg-std`/`vending-*`** : ce sont ces dépendances qui rendent le contrat impossible à déployer (ADR 0002).
- **Ne pas perdre de temps sur les explorateurs web** (Mintscan, forum Discourse, DuckDuckGo) : SPA/captchas. Les docs officielles passent par GitBook et sont lisibles avec `.md` (ex. `docs.stargaze.zone/developers/overview.md`).
- Le code réseau sortant depuis le shell de cette machine a échoué pendant la session (curl/LCD bloqués) : privilégier `fetch_web_content`.
- `mosaic/` (décembre 2024) est un **prédécesseur**, pas un doublon : le garder comme archive pour le frontend (`mosaic/frontend`, spec dans `mosaic/TECHNICAL_SPEC.md`), sans y travailler.
