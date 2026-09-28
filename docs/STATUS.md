# STATUS — 2026-09-28

## Où on en est
Reprise du projet après abandon (dernier commit : 2025-01-03). Objectif de session : retrouver le fil et remettre le projet sur les rails.

**Fait :**
- État des lieux complet du code et de l'écosystème ; comparaison avec le projet voisin `mosaic` (à `/Users/malikfassifihri/dev/mosaic`).
- Décision de cible : **Stargaze 2.0 sur le Cosmos Hub** (ADR 0001), standard **CW721** (ADR 0002), portage **CosmWasm 2.x** (ADR 0003).
- Pilotage installé : `.clinerules/` (rôles, workflows, conventions) + miroir `.claude/skills/`, `docs/` (product, architecture, ADR, plan, notes).
- Note d'écosystème sourcée : `docs/notes/ecosysteme-cosmos-2026.md` (Stargaze L1 morte, Hub permissionné, précédents de gouvernance).
- **T-001 fait** : workspace réparé (membres `vendor/*` fantômes retirés, dépendance `sg_std` retirée de `build.rs`, denom `uatom` en constante locale).
- **T-002 fait** : inventaire complet du contrat dans `docs/architecture.md` (structure, messages, problèmes identifiés et classés par gravité).

**Ce qui marche vraiment (testé le 2026-09-28) :** `cargo build` OK, `cargo test` **48/48 verts**, `cargo fmt --check` propre, `cargo clippy --all-targets` sans erreur (7 warnings mineurs). Le projet recompile et ses tests passent de nouveau, tel quel, avant tout portage.

**Découverte importante de T-002 :** `validate_for_tile` (`src/core/tile/metadata.rs:116`) retourne toujours `Ok(())` — **l'expiration des pixels n'est jamais appliquée**. Un pixel encore valide peut être réécrit par n'importe qui, au prix d'un pixel libre. C'est la règle économique centrale du produit : à corriger avant toute mise en ligne (T-008).

## Prochaine étape
`/next-task T-003` — portage CosmWasm 2.x (dépendances, entry points, helpers). Vérification en Plan recommandée avant de coder.

## Décisions en attente de Malik
1. **Qui peut colorier un pixel ?** Aujourd'hui `SetPixelColor` n'exige pas d'être le propriétaire de la tuile : n'importe qui peut payer pour colorier la tuile d'un autre (le paiement va bien au propriétaire).
   - (a) **Tout le monde peut colorier** en payant : c'est le canvas collaboratif, le propriétaire encaisse.
   - (b) **Seul le propriétaire** (ou ses approbations) colorie : la tuile est une œuvre privée, personne ne peut la modifier de l'extérieur.
   - (c) **Le propriétaire choisit à l'instanciation/l'achat** : plus riche mais plus de code et de tests.
   Impacte T-008 (règle de couleur) et T-009 (répartition).
2. **Mode de production** (à trancher plus tard, quand le contrat sera complet) : Studio 2.0 + logique hors chaîne vs proposition de gouvernance — ADR à ouvrir.

## Pièges connus (ne pas refaire)
- **Ne plus chercher Stargaze L1** : `elgafar-1` est mort, `starsd` n'est plus la CLI, le repo `public-awesome/stargaze` est archivé. Tout le dossier `launchpad/` est une copie morte du projet Stargaze.
- **Ne pas réintroduire `sg721`/`sg-std`/`vending-*`** : ce sont ces dépendances qui rendent le contrat impossible à déployer (ADR 0002).
- **Ne pas perdre de temps sur les explorateurs web** (Mintscan, forum Discourse, DuckDuckGo) : SPA/captchas. Les docs officielles passent par GitBook et sont lisibles avec `.md` (ex. `docs.stargaze.zone/developers/overview.md`).
- Le code réseau sortant depuis le shell de cette machine a échoué pendant la session (curl/LCD bloqués) : privilégier `fetch_web_content`.
- `mosaic/` (décembre 2024) est un **prédécesseur**, pas un doublon : le garder comme archive pour le frontend (`mosaic/frontend`, spec dans `mosaic/TECHNICAL_SPEC.md`), sans y travailler.
