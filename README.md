# Riko


<p align="center">
  <img src="assets/riko.png" alt="Riko the Ermine" width="200">
</p>

A small, blazingly fast interpreted language. Cute name, serious intent.

---

## Features

- TODO

## Example

```riko
~ Riko: The Tiny Warrior Language

summon math from "std"

keep MAX_HEALTH = 100

form Warrior {
    name: str
    health: int
    power: int

    strike attack(target) ->
        say("{name} strikes {target.name}!")
        target.health = target.health - power
        when target.health <= 0 ->
            say("{target.name} has fallen!")
        otherwise ->
            say("{target.name} has {target.health} HP left.")

    strike heal(amount) ->
        health = math.min(health + amount, MAX_HEALTH)
        say("{name} heals to {health} HP.")

    strike is_alive() ->
        give back health > 0
}

strike battle(a, b) ->
    say("⚔️ Battle begins!")
    
    stalk while a.is_alive() and b.is_alive() ->
        a.attack(b)
        when b.is_alive() ->
            b.attack(a)
    
    hold winner = a.is_alive() ?? a :: b
    say("🏆 {winner.name} wins!")
    give back winner

~ Create warriors
hold riko = Warrior { name: "Riko", health: 100, power: 25 }
hold enemy = Warrior { name: "Shadow", health: 80, power: 20 }

~ Fight!
try ->
    battle(riko, enemy)
dodge err ->
    say("Battle interrupted: {err}")
```

## Roadmap

- [ ] TODO

## License

MIT

---

*Riko the ermine — small but has bite.*
