---
name: next-task
description: Prend la prochaine tâche du plan, la fait implémenter puis relire et tester, et met à jour son statut.
argument-hint: "[ID de tâche, optionnel]"
disable-model-invocation: true
---

Tâche demandée : $ARGUMENTS (si vide : la première `à faire` de `docs/plan/tasks.md` dont les dépendances sont `fait`).

Procédure complète : `.clinerules/workflows/next-task.md`. Applique-la étape par étape (les rôles sont dans `.clinerules/skills/`).

Points à ne pas oublier :
- Se resynchroniser sur `origin/master` avant de choisir la tâche, et vérifier qu'elle n'est pas déjà faite ailleurs.
- Ne prendre ni une tâche `Manuel`, ni une tâche `attente-manuel`.
- Passe la tâche à `en cours`, commit ce statut seul, puis implémente (`rust-dev`), relis (`reviewer`), teste (`qa` : `cargo fmt --check`, `cargo clippy`, `cargo test`).
- Si la tâche est finie mais exige une action de Malik (déploiement, signature, explorateur) : `attente-manuel`, pas `fait`.
- Rapport final en 5 lignes. Une décision qui revient à Malik se pose en premier, sans y répondre à sa place.
