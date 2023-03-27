# what is this game?
- this is a top down simple ascii art styled dungeon crawler. You as a player, advance through
stages to win.

# TODO
-- CREATE FULL REWRITE --
1. Upgraded to bevy version 0.9 && completed reimplementation of player controllers
2. Rewrite Player.rs and Main.rs 
3. fix ldtk to spawn collisions properly -- this is what we are working on
4. remove unnecessary files/functions
5. implement brand new AI 
6. fix player controller
---
1. [x] Create a character with basic movement
2. [x] Create a tilemap with collisions for your player to move through
3. [ ] add in enemies
    * use big-brain plugin by zkat 
4. [ ] add in weapon types 
    * or make it a bow focused game
5. [ ] add in an inventory system
6. [ ] add in UI 
7. [ ] add in main menu/pause menu
8. [ ] make it prettier
    * color palette for the whole game
      --> 120B0D - EEE0CB - BAA898 - 848586 - C2847A
    * make wall textures/ground textures
    * draw enemies
9. [ ] add in player health
    * added in health field within player struct
    * made player health set to 100 when spawning
---

## Server

- the multiplayer protocol should look something like this: 
```    
struct Protocol {
        Position,
        Velocity,
        port,
        IP,
        health,
        damage,
        text,
        
    }
```
- because this will be a quick paced multiplayer game, it will use UDP
