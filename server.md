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
        player_count,
        
    }
```
- because this will be a quick paced multiplayer game, it will use UDP
