# GAME DESIGN DOCUMENT
# Nom de code : [A DEFINIR]
## Simulation de vaisseau spatial - Sci-Fi

---

## 1. VISION

Un jeu de simulation de vaisseau spatial en 2D top-down, mêlant combat tactique inspiré de FTL, exploration libre inspirée de Freelancer/Elite/Star Citizen, économie et trading d'EVE Online, et profondeur stratégique des 4X. L'univers est imprégné de l'esthétique de la sci-fi classique (Dune, Star Wars).

Le joueur est un pilote indépendant qui navigue entre galaxies, systèmes solaires, planètes et stations. Il peut choisir sa voie : commerçant, chasseur de primes, mineur, explorateur, mercenaire, sauveteur, pirate...

**Mots clés** : Liberté totale, gestion de vaisseau profonde, univers vivant, multiples carrières.

---

## 2. INSPIRATIONS

| Jeu | Ce qu'on prend |
|---|---|
| **FTL** | Gestion interne du vaisseau (énergie, systèmes, crew, pause tactique) |
| **Star Citizen** | Simulation réaliste, multicrew, mining, professions variées |
| **EVE Online** | Économie, corporations, bounty hunting, univers persistant |
| **Freelancer** | Exploration libre entre systèmes, commerce, missions variées |
| **4X (Stellaris, etc.)** | Multiples galaxies, diplomatie, factions, découverte |
| **Dune** | Esthétique, politique, guildes, ressources rares, épices |
| **Star Wars** | Factions, diversité de vaisseaux, contrebande, chasseurs de primes |

---

## 3. BOUCLE DE JEU PRINCIPALE

```
[Station/Planète]
    │
    ├── Acheter/vendre cargo
    ├── Recruter crew
    ├── Réparer/upgrader vaisseau
    ├── Accepter missions (bounty, escort, rescue, delivery, exploration)
    │
    ▼
[Navigation système solaire]
    │
    ├── Voyager entre planètes/stations/astéroïdes
    ├── Minage d'astéroïdes
    ├── Rencontres aléatoires (pirates, patrouilles, épaves, anomalies)
    ├── Scanner / explorer
    │
    ▼
[Combat tactique] (temps réel + pause active)
    │
    ├── Gérer énergie (shields/weapons/engines)
    ├── Cibler / coordonner les tirs
    ├── Gérer le crew (postes, réparations, abordage)
    ├── Butin / récupération
    │
    ▼
[Saut hyperspatial] → Autre système / autre galaxie
```

---

## 4. SYSTEME DE COMBAT (V1 - Implémenté)

### 4.1 Contrôle du vaisseau
- **WASD** : contrôle direct (W=avancer, A/D=tourner, S=freiner)
- **Souris** : clic gauche pour cibler un ennemi
- **T** : auto-target (cible automatique l'ennemi le plus proche)
- **F** : hold fire (cesser le feu)
- **Espace** : pause tactique totale
- **+/-** : vitesse du jeu (x0.25, x0.5, x1, x2)

### 4.2 Système d'énergie
- Réacteur avec cellules discrètes (10 par défaut, upgradeable)
- Trois systèmes : **Shields**, **Weapons**, **Engines**
- Touches 1/2/3 (+) et Shift+1/2/3 (-) pour redistribuer
- Fonctionne pendant la pause tactique

#### Effets par système :
- **Shields** : +50 max HP et +10 HP/s regen par cellule. 0 = shields offline
- **Weapons** : chaque arme a un coût en cellules, alimentées par priorité (index)
- **Engines** : +25% vitesse et +5% évasion par cellule. 0 = 30% vitesse de base

### 4.3 Armes (V1)

| Arme | Power | Cooldown | Damage | Range | Special |
|---|---|---|---|---|---|
| Pulse Laser | 1 | 0.8s | 20 | 150 | Rapide, bloqué par shields |
| Heavy Laser | 2 | 2.0s | 60 | 120 | Haute précision |
| Missile Launcher | 1 | 2.5s | 40 | 500 | Bypass shields, ammo limitée |
| Flak Cannon | 2 | 2.5s | 5x5 | 200 | Multi-projectiles, anti-shield |

#### Armes futures (V2+)
- **Beam Weapon** : balayage continu, DPS, bloqué par shields pleins
- **Ion Cannon** : désactive systèmes temporairement, pas de dégâts hull
- **Burst Laser** : 3 tirs en salve rapide
- **Torpedo** : lent, énormes dégâts, homing
- **Railgun** : très longue portée, très lent, perce shields partiellement
- **Tractor Beam** : immobilise cibles petites, attire cargo/débris

### 4.4 Layout vaisseau joueur V1

```
        [Pulse Laser]     <- index 0, power 1, cone avant
            |
  [Flak] --+-- [Missile]  <- index 1 (power 2), index 2 (power 1, ammo)
            |
        [Pulse Laser]     <- index 3, power 1, cone avant
```
Total power nécessaire : 5 (mais 3 allouées par défaut → choix tactique)

### 4.5 HUD Combat
- Barres HP (rouge) et Shield (bleu) en bas à gauche
- Panneau énergie (3 barres : Shields/Weapons/Engines)
- Info cible en haut à droite (HP, distance)
- Indicateur vitesse / pause / auto-target en haut à gauche

---

## 5. SYSTEME DE VAISSEAU (V2 - Design)

### 5.1 Modules internes (salles)

Chaque vaisseau a un plan interne avec des salles. Chaque salle contient un système :

| Salle | Système | Effet |
|---|---|---|
| Pont de commande | Pilotage | Évasion, contrôle du vaisseau |
| Salle des armes | Weapons | Contrôle des tourelles |
| Générateur shields | Shields | Boucliers |
| Salle des moteurs | Engines | Vitesse, évasion |
| Drone Bay | Drones | Contrôle des drones |
| Infirmerie | Medbay | Soins du crew |
| Soute | Cargo | Stockage marchandises |
| Camouflage | Cloaking | Invisibilité temporaire |
| Capteurs | Sensors | Détection, scan |
| Support vie | Life Support | Oxygène (boarding defense) |
| Téléporteur | Teleporter | Abordage / extraction |

### 5.2 Types de vaisseaux joueur

| Classe | HP | Shield | Crew Max | Armes | Drones | Cargo | Vitesse | Spécial |
|---|---|---|---|---|---|---|---|---|
| **Intercepteur** | 150 | 80 | 2 | 2 | 0 | 10 | Très rapide | Camouflage |
| **Corvette** | 200 | 100 | 3 | 3 | 0 | 20 | Rapide | Camouflage |
| **Frégate** | 300 | 200 | 4 | 4 | 1 | 40 | Moyen | Polyvalent |
| **Croiseur** | 500 | 300 | 6 | 6 | 2 | 60 | Lent | Puissance de feu |
| **Carrier** | 250 | 150 | 5 | 2 | 4 | 30 | Moyen | Essaim drones |
| **Cargo hauler** | 200 | 100 | 3 | 1 | 1 | 200 | Lent | Gros stockage |
| **Mineur** | 250 | 120 | 3 | 1 | 2 | 100 | Lent | Laser de minage |

### 5.3 Crew

- Chaque membre a des **compétences** (pilotage, shields, armes, moteurs, réparation, combat)
- Assignation aux postes : un membre au poste donne un **bonus** au système
- Les membres se **déplacent** entre les salles (pathfinding interne)
- En cas d'abordage : combat au corps à corps dans les salles
- Recrutement dans les stations
- Races / origines variées (bonus différents)

### 5.4 Drones

| Type | Effet |
|---|---|
| Defense Drone | Tire sur les projectiles/missiles entrants |
| Attack Drone | Cible automatique les ennemis |
| Repair Drone | Répare la hull lentement |
| Mining Drone | Mine les astéroïdes automatiquement |
| Shield Drone | Projette un bouclier supplémentaire |

### 5.5 Camouflage (Cloaking)

- Consomme de l'énergie (2-3 cellules)
- Durée limitée + cooldown
- Rend invisible aux capteurs IA
- Se désactive si le joueur tire

---

## 6. UNIVERS & EXPLORATION

### 6.1 Structure de l'univers

```
Univers
  └── Galaxie (plusieurs, connectées par des portails)
       └── Système solaire (étoile + planètes + astéroïdes + stations)
            └── Corps célestes
                 ├── Étoile (danger si trop proche, recharge énergie solaire)
                 ├── Planète (orbite, atmosphère, stations orbitales, landing zones)
                 ├── Lune (bases cachées, minage)
                 ├── Ceinture d'astéroïdes (minage, embuscades pirates)
                 ├── Station spatiale (commerce, missions, réparations, recrutement)
                 ├── Épave / Anomalie (exploration, loot, danger)
                 └── Portail hyperspatial (voyage entre systèmes/galaxies)
```

### 6.2 Navigation

- **Intra-système** : propulsion normale (WASD), on voit les planètes/stations sur la carte
- **Inter-système** : saut hyperspatial via portails ou drive FTL (nécessite fuel)
- **Inter-galaxie** : portails spéciaux ou tech avancée
- **Approche planétaire** : on peut s'approcher des planètes, voir leur orbite, scanner leur surface
- **Landing** : certaines planètes/stations ont des zones d'atterrissage (transition vers menu station)

### 6.3 Génération de l'univers

- **Procédural** : les galaxies, systèmes et planètes sont générés procéduralement
- **Points d'intérêt fixes** : certaines stations/planètes clés sont placées manuellement
- **Seed** : l'univers est déterministe à partir d'une seed (rejouable)
- **Densité variable** : zones civilisées (beaucoup de stations, sécurité) vs zones sauvages (danger, richesse)

---

## 7. PROFESSIONS / ACTIVITES

### 7.1 Trading / Commerce

- Acheter bas, vendre haut entre stations
- Chaque station a une économie locale (offre/demande dynamique)
- Marchandises : minerais, nourriture, tech, armes, luxe, contrebande
- Routes commerciales lucratives mais dangereuses
- Prix fluctuants selon les événements (guerre, pénurie, etc.)

### 7.2 Minage

- Ceintures d'astéroïdes riches en minerais
- **Laser de minage** : équipement spécial sur le vaisseau
- **Mining drones** : automatisent partiellement le processus
- Minerais raffinés valent plus cher
- Risque : pirates attirés par les mineurs
- Types de minerais : communs (fer, cuivre) → rares (cristaux, éléments exotiques)

### 7.3 Bounty Hunting / Chasseur de primes

- Contrats de bounty disponibles dans les stations
- Cibles : pirates, criminels recherchés, vaisseaux renégats
- Récompense proportionnelle à la difficulté
- Système de réputation (plus de bounties complétées → meilleures offres)
- Possibilité de capturer vivant (bonus) avec téléporteur

### 7.4 Missions de sauvetage (Rescue)

- Signaux de détresse (SOS) aléatoires dans l'espace
- Sauver des équipages de vaisseaux endommagés
- Escort de convois civils à travers zones dangereuses
- Évacuation de stations attaquées
- Récompense en crédits + réputation

### 7.5 Exploration

- Découverte de systèmes inconnus (first discovery bonus)
- Scanner les planètes pour trouver des ressources / anomalies
- Épaves anciennes à fouiller (loot rare, lore)
- Anomalies spatiales (trous noirs, nébuleuses, phénomènes inconnus)
- Cartographie : vendre les données de scan aux stations

### 7.6 Contrebande / Piraterie

- Transporter des marchandises illégales (épices, armes, esclaves ?)
- Éviter les patrouilles et scanners
- Attaquer des cargos pour voler leur marchandise
- Réputation négative avec les factions légales
- Bases pirates cachées dans les ceintures d'astéroïdes

### 7.7 Mercenaire

- Contrats militaires pour des factions
- Participer à des batailles entre factions
- Défendre des stations attaquées
- Escorte de VIP / convois militaires

---

## 8. ECONOMIE

### 8.1 Monnaie
- **Crédits** : monnaie universelle
- Possibilité de monnaies locales par faction (taux de change)

### 8.2 Marchandises

| Catégorie | Exemples | Notes |
|---|---|---|
| Minerais | Fer, cuivre, titane, cristaux | Base du minage |
| Raffinés | Alliages, composites, circuits | Valeur ajoutée |
| Nourriture | Rations, produits frais, luxe | Essentiel pour les colonies |
| Tech | Composants, armes, shields | Haute valeur |
| Données | Cartes, scans, blueprints | Poids nul, valeur variable |
| Contrebande | Épices, armes interdites | Illégal, très lucratif |
| Matériaux rares | Éléments exotiques | Nécessaires pour upgrades top |

### 8.3 Offre & Demande dynamique
- Les prix changent selon l'offre/demande locale
- Un système agricole vend la nourriture pas cher mais achète la tech cher
- Les événements (guerre, catastrophe) affectent les prix
- Le joueur peut influencer le marché (livraisons massives)

---

## 9. FACTIONS & REPUTATION

### 9.1 Factions principales (exemples)

| Faction | Style | Spécialité |
|---|---|---|
| **Empire Solaire** | Ordre, militaire | Vaisseaux lourds, shields puissants |
| **Guilde des Marchands** | Commerce, neutralité | Meilleures offres, routes protégées |
| **Alliance Libre** | Liberté, démocratie | Tech variée, exploration |
| **Syndicat de l'Ombre** | Crime, contrebande | Armes illégales, camouflage |
| **Nomades du Vide** | Mystère, anciens | Tech alien, anomalies |

### 9.2 Système de réputation
- Réputation par faction (-100 à +100)
- Actions positives : missions complétées, commerce, aide
- Actions négatives : attaquer leurs vaisseaux, contrebande, piraterie
- Niveaux : Hostile → Méfiant → Neutre → Amical → Allié
- Effets : accès aux stations, prix, missions exclusives, aide en combat

---

## 10. PROGRESSION

### 10.1 Progression du joueur
- Pas de niveau XP classique
- Progression par l'**argent** (acheter meilleur vaisseau/équipement)
- Progression par la **réputation** (débloquer missions/zones/équipements de faction)
- Progression par l'**exploration** (découvrir de nouvelles zones/tech)
- Progression par le **crew** (crew expérimenté = meilleurs bonus)

### 10.2 Upgrades vaisseau
- **Systèmes** : augmenter le max_level de chaque système (shields lvl 4→6, etc.)
- **Réacteur** : augmenter max_power (10→12→15...)
- **Armes** : acheter/remplacer des armes dans les slots
- **Modules** : ajouter des salles spéciales (drone bay, cloaking, teleporter)
- **Hull** : renforcer la coque (plus de HP)
- **Cargo** : agrandir la soute

### 10.3 Acheter un nouveau vaisseau
- Disponibles dans certaines stations
- On peut posséder un seul vaisseau actif (le reste en stockage station)
- Le crew peut être transféré
- Prix élevés → objectif long terme

---

## 11. ARCHITECTURE TECHNIQUE (Bevy ECS)

### 11.1 Modules actuels (V1)
```
src/
  player/           -- Contrôle joueur, énergie, HUD, camera
  combat/           -- Armes, shields, dégâts, projectiles, effets
  ai/               -- IA des vaisseaux ennemis
  movement/         -- Physique de mouvement
  fx/               -- Effets visuels (lasers, explosions, shields)
  templates/        -- Templates de vaisseaux et armes
  materials/        -- Shaders custom (couleur d'équipe)
```

### 11.2 Modules futurs (roadmap)
```
src/
  universe/         -- Génération galaxies/systèmes, navigation, portails
  economy/          -- Trading, prix, marchandises, offre/demande
  mining/           -- Minage d'astéroïdes, raffinage
  missions/         -- Système de missions/quêtes
  factions/         -- Factions, réputation, diplomatie
  crew/             -- Membres d'équipage, compétences, assignation
  stations/         -- Interface station (commerce, recrutement, réparations)
  celestial/        -- Planètes, étoiles, astéroïdes, orbites
  persistence/      -- Sauvegarde/chargement
  ui/               -- Menus, carte galactique, inventaire
```

### 11.3 Constantes modulables
Chaque module a son propre fichier de constantes pour faciliter le balancing :
- `player/constants/` : stats vaisseau, énergie, armes, contrôles, HUD
- `universe/constants/` : tailles systèmes, densité, distances
- `economy/constants/` : prix de base, fluctuations, marges
- `mining/constants/` : yield par minerai, vitesse de minage
- `missions/constants/` : récompenses, difficulté

---

## 12. ROADMAP

### V1 (Combat Core) ✅ FAIT
- [x] Vaisseau joueur contrôlable (WASD)
- [x] Système d'énergie (shields/weapons/engines)
- [x] 4 types d'armes (laser, heavy, missile, flak)
- [x] Auto-targeting + hold fire
- [x] Pause tactique + contrôle vitesse
- [x] HUD complet
- [x] Camera follow
- [x] Shield fix (instigator-based)

### V2 (Vaisseau avancé)
- [ ] Système de salles internes
- [ ] Crew (recrutement, assignation, compétences)
- [ ] Drones (defense, attack, repair)
- [ ] Camouflage (cloaking)
- [ ] Dégâts aux systèmes (salles endommagées)
- [ ] Abordage (teleporter + combat crew)
- [ ] Nouveaux types d'armes (beam, ion, burst)

### V3 (Univers)
- [ ] Génération procédurale de systèmes solaires
- [ ] Planètes, étoiles, astéroïdes (visuels + orbites)
- [ ] Navigation intra-système
- [ ] Saut hyperspatial entre systèmes
- [ ] Stations spatiales (interface commerce/réparation)

### V4 (Économie & Professions)
- [ ] Système de trading (achat/vente, offre/demande)
- [ ] Minage d'astéroïdes
- [ ] Système de missions
- [ ] Bounty hunting
- [ ] Exploration & scan

### V5 (Factions & Monde vivant)
- [ ] Factions avec réputation
- [ ] Patrouilles / pirates / convois IA
- [ ] Événements dynamiques (guerres, catastrophes)
- [ ] Contrebande
- [ ] Diplomatie

### V6 (Polish)
- [ ] Sauvegarde/chargement
- [ ] Multiple galaxies
- [ ] Son & musique
- [ ] Menus (menu principal, pause, carte galactique)
- [ ] Tutoriel
- [ ] Balancing complet

---

## 13. CONSTANTES GLOBALES DE DESIGN

- **Temps réel** avec pause tactique (jamais de tour par tour)
- **Single player** (multijoueur éventuel beaucoup plus tard)
- **2D top-down** (pas de 3D)
- **Procédural + handcrafted** : univers généré mais points clés placés
- **Pas de permadeath** : le joueur respawn à la dernière station visitée (avec perte de cargo)
- **Progression horizontale** : pas de power creep infini, les choix comptent plus que le niveau
- **Modularité** : chaque système est indépendant et tweak-able via constantes
