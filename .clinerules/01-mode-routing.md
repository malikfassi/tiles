# Routage modèle : Plan vs Act

Cline ne permet pas d'assigner un modèle différent à chaque rôle. Le seul levier natif est le split **Plan / Act**,
un modèle par mode (icône engrenage → "Use different models for Plan and Act modes").

| Mode | Provider / modèle | Effort | Pour |
|---|---|---|---|
| **Plan** | Z.AI Coding Plan — GLM-5.3 | High | Architecture, décisions structurantes, planification, relecture |
| **Act** | DeepSeek — DeepSeek V4.1 Flash | — | Implémentation, tests, QA, exécution routinière |

## Quel rôle dans quel mode

| Rôle (`.clinerules/skills/<rôle>/`) | Mode | Pourquoi |
|---|---|---|
| `product` | Plan | Cadrage produit, arbitrages — beaucoup de jugement |
| `architect` | Plan | Décisions structurantes, ADR, difficiles à défaire |
| `planner` | Plan | Découpage en tâches à partir de specs validées |
| `reviewer` | Plan | Relecture exigeante : trouver les bugs demande du raisonnement |
| `rust-dev` | Act | Implémentation Rust/CosmWasm d'une tâche déjà découpée |
| `qa` | Act | Lancer build/tests et résumer les erreurs : exécution |
| `researcher` | Act | Chercher une réponse technique précise et l'écrire dans une note |

Règle pratique : avant de charger un skill de rôle, vérifie que tu es dans le mode indiqué par ce tableau.
Ne cherche pas à simuler un modèle par rôle : Cline ignore silencieusement un champ `model:` dans les frontmatters de skill.
