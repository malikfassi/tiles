# 0005 — Denom de paiement des pixels

- Statut : accepté
- Date : 2026-09-28

## Contexte
`NATIVE_DENOM` vaut `uatom` : le prix des pixels, la part collection et la part plateforme sont
tous exprimés et versés dans ce denom. Or `uatom` est aussi le denom des frais de transaction
sur le Cosmos Hub, ce qui soulevait la crainte que la part « collection » se confonde avec un coût.

La question a été tranchée par une recherche documentée dans
`docs/notes/stargaze-2-denoms-paiement.md` : **ce n'est pas un problème, c'est le comportement
de référence de Stargaze 2.0.** ATOM y est le token principal, et l'exemple officiel de répartition
des frais du marketplace est lui-même libellé en ATOM (vente de 100 ATOM → 92 vendeur, 5 créateur, 2 marketplace).
Les frais de gas sont un flux distinct, payé par l'acheteur en plus du prix.

Le multi-token existe bien chez Stargaze 2.0 (ATOM, TIA, BTC, STARS, USDC), mais c'est une **couche
d'orchestration** : le frontend enchaîne un swap automatique (propulsé par Skip) avant l'achat.
Un contrat CW721 autonome ne peut pas la reproduire sans dépendre d'un routeur externe.

Le bug révélé par T-008 reste valable et est corrigé : le contrat doit refuser tout denom qu'il
ne reconnaît pas (test `an_unknown_denom_is_refused`).

## Décision
**Option B retenue (Malik, 2026-09-28) : `uatom` est le denom de paiement, en dur.**

1. Le prix d'un pixel, la part collection et la part plateforme sont exprimés en `uatom`
   (`defaults::constants::NATIVE_DENOM`), et le contrat paie ses trois parts dans ce denom.
2. Le contrat accepte **exactement un coin `uatom`** et refuse tout autre denom, tout paiement
   vide, tout paiement multi-denom, tout montant différent du prix annoncé.
3. Aucun champ de configuration de denom n'est ajouté. Le denom est une constante de compilation.

## Conséquences
- Le contrat reste conforme au standard de fait de Stargaze 2.0 et son comportement de paiement
  est celui de l'écosystème : une vente en ATOM verse ses parts en ATOM.
- Changer de denom un jour (par exemple un stablecoin) **exige un redéploiement**. C'est assumé :
  le coût d'un `Config` dénormalisable est jugé supérieur au bénéfice tant qu'aucun besoin produit
  ne le justifie.
- Les frais de transaction restent payés par l'appelant en ATOM, en plus du prix. Si le denom
  de paiement devenait un jour un stablecoin, ce serait toujours le cas.
- Le support multi-denom (option C) est **hors périmètre du contrat** : si le produit le veut un jour,
  l'orchestration (swap Skip) vit dans la couche web, pas dans le contrat.

## Alternatives écartées
- **Option A — denom configurable distinct des frais (ex. `uusdc`)** : écartée. Elle reposait sur
  l'hypothèse d'un problème (« la part collection finance les frais ») que la documentation invalide.
  Ajouter un champ, une migration et des scripts de déploiement pour un bénéfice nul.
- **Option C — plusieurs denoms avec prix par denom** : écartée. Le multi-token de Stargaze 2.0 est
  une couche d'application adossée à un routeur (Skip) ; la reproduire on-chain introduirait une
  dépendance externe, contraire aux règles du projet, pour un coût de tests multiplié.
- **Variante B+ (denom configurable, un seul denom accepté)** : écartée par Malik. Souplesse de
  déploiement marginale, complexité de configuration et de migration non justifiée à ce stade.

