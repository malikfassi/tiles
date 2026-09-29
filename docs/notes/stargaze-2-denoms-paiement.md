# Denoms de paiement chez Stargaze 2.0

Date : 2026-09-28. Sources : docs officielles Stargaze 2.0. **Ne pas refaire cette recherche.**

## Question
Peut-on forcer `uatom` comme denom de paiement d'un contrat autonome sur le Cosmos Hub,
ou faut-il prévoir un denom configurable, voire plusieurs denoms ?

## Sources

- `docs.stargaze.zone/start-here/paying-with-different-tokens` — page « Paying with Different Tokens ».
- `docs.stargaze.zone/collect/fees` — grille des frais marketplace.
- `docs.stargaze.zone/collect/minting-and-trading` — mint et listing.

## Ce que dit la documentation

**ATOM est le token principal.** Le tableau des tokens supportés donne ATOM comme
« Cosmos Hub native token (**primary**) », aux côtés de TIA, BTC (IBC), STARS et USDC.
La page précise : « Stargaze supports multi-token payments, so you're not locked into a single currency. »

**Le multi-token est une couche d'application, pas du contrat.** Le mécanisme documenté :
le vendeur affiche son prix dans le token qu'il préfère, l'acheteur choisit son token de paiement,
et « the swap happens automatically at current market rates » (swap propulsé par **Skip**),
le tout en une transaction. C'est le frontend/marketplace Stargaze qui orchestre la conversion.

**Les frais sont versés dans le denom de la vente.** L'exemple officiel de la grille des frais :
vente de **100 ATOM** avec 5 % de royalties → vendeur 92 ATOM, créateur 5, marketplace 2.

**Le gas est un flux séparé du prix.** Les frais de transaction sont payés en ATOM par l'acheteur
et le vendeur, en plus du prix de vente. La documentation insiste : « You'll always need a small
amount of ATOM for transaction fees. » Un vendeur peut donc être payé en USDC et payer son gas en ATOM.

**Le denom d'un listing est choisi par le vendeur** (« Set your price and token »), pas imposé
par la plateforme.

## Conclusion

1. **Payer en `uatom` est le comportement de référence** de Stargaze 2.0, pas une anomalie : ATOM est
   le token principal et l'exemple canonique de répartition des frais est en ATOM.
2. **Verser la part collection dans le denom des frais n'est pas un problème** : c'est exactement ce que
   fait Stargaze 2.0. Prix et gas sont deux flux distincts de la transaction.
3. **Supporter plusieurs denoms n'est pas une capacité du CW721** mais une couche d'orchestration
   (swap Skip) qui vit hors contrat. Un contrat autonome ne peut pas la répliquer sans dépendre
   d'un routeur externe, ce que les règles du projet interdisent.

## Décision qui en découle
ADR 0005, option B retenue par Malik : `uatom` est le denom de paiement, en dur, sans configuration.

## Reste à vérifier plus tard (non bloquant)
- Références de code de l'option C (multi-denom) si le produit l'exige un jour.
- Comportement exact du swap Skip (coût, slippage) : hors périmètre du contrat.
