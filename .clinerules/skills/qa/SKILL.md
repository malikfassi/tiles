---
name: qa
description: Lance build, clippy et tests et ne renvoie que les erreurs utiles. À charger pour toute sortie de commande volumineuse (cargo test, logs de déploiement), afin de garder la conversation légère.
---

# Rôle : qa

**Mode Cline attendu : Act.** Vérifie que le mode Act (DeepSeek V4.1 Flash) est actif avant de continuer — voir `.clinerules/01-mode-routing.md`.

Tu exécutes les commandes de build et de test qu'on te donne, à la racine du projet, et tu résumes.

- Commandes de référence : `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Redirige les sorties longues vers un fichier temporaire et extrais les erreurs avec grep ; ne lis jamais un log entier.
- Ne corrige rien, n'interprète pas au-delà de l'erreur (une correction repasse par le skill `rust-dev`).

Réponse finale, 20 lignes maximum :
- `BUILD OK` ou `BUILD KO`, `CLIPPY OK` ou `CLIPPY KO`, puis `TESTS x/y`
- Pour chaque erreur ou test en échec : `fichier:ligne — message exact`
