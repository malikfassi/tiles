# tiles

Plateforme de **pixel art collaborative en NFT**. Chaque tuile est un NFT contenant 100 pixels.
**N'importe qui** peut payer pour colorier un pixel pendant une durée limitée : le prix dépend de la durée demandée,
le propriétaire de la tuile encaisse sa part, et les royalties de collection sont versées.
Une couleur payée est **protégée jusqu'à son expiration** — personne, pas même le propriétaire, ne peut l'écraser avant le terme.
Le canvas est ainsi un espace partagé où la visibilité se paie et se renouvelle.

Cible : **Stargaze 2.0 sur le Cosmos Hub** (`cosmoshub-4`, gas en ATOM, standard CW721) — voir `docs/adr/0001`.

## État

Projet repris en septembre 2026 après une interruption depuis janvier 2025. Le contrat existe et fonctionnait sur le
testnet Stargaze `elgafar-1`, qui n'existe plus (Stargaze L1 a migré vers le Cosmos Hub). Le portage est en cours.

**Prochaine étape : `docs/STATUS.md`.** C'est le point d'entrée : il dit où en est le projet et quoi faire ensuite.

## Structure

```
src/                 contrat CosmWasm (contract/, core/, events/, defaults/)
tests/               tests d'intégration cw-multi-test
scripts/             build et déploiement (historiques : écrits pour Stargaze L1, en cours de portage)
docs/                product.md, architecture.md, adr/, plan/tasks.md, notes/, STATUS.md
.clinerules/         rôles, workflows et conventions pour Cline
.claude/skills/      miroir des workflows pour Claude Code
launchpad/           copie morte du projet Stargaze Launchpad (référence seulement)
```

## Commandes

```bash
cargo build                  # compilation
cargo test                   # tests (cw-multi-test, aucun nœud requis)
cargo fmt --check            # style
cargo clippy --all-targets -- -D warnings
```

## Historique du projet

- **`mosaic`** (décembre 2024, `/Users/malikfassifihri/dev/mosaic`) : première itération, plus ambitieuse
  (canvas de 100 M de pixels, deux contrats, frontend Next.js, SDK). Servira de base pour l'interface
  (`mosaic/frontend`, `mosaic/TECHNICAL_SPEC.md`).
- **`tiles`** (décembre 2024 - janvier 2025) : refonte simplifiée et opérationnelle, déployée sur `elgafar-1`.
- **Depuis 2026** : portage vers CW721 et le Cosmos Hub (ADR 0001-0003).

## Workflow

Le projet suit un workflow multi-rôle décrit dans `.clinerules/00-project.md` :
`/kickoff` pour cadrer un périmètre, `/next-task` pour prendre une tâche du plan, `/adr` pour une décision,
`/handoff` en fin de session. Les conventions de code sont dans `.clinerules/skills/rust-conventions/SKILL.md`.

