# world_simulator
a world building tool designed to be fairly generic using a dedicated DSL

this repo is still in the very early stages of devlopment

# syntax

classes and inhertance

```
class location;
class economic_entity {
	float wealth;
	float food;
}
class city extends location,economic_entity {
	string name;
}

class merchant extends economic_entity{
	string name;
}

class road{
	source city;
	target city;
}
```

rendering (expiremental)
```

render {
	graph_render(nodes=city,edge_start = road.target,edge_end = road.end)
}

```

# technical decsions
it makes a lot of sense to use some sort of a qurying engine for runing the underlying representation.
since that gives us parllalisem for free and all sorts of other niceties.

unfortunatly they all take way too long to compile... so what we probably want with this is a setup where there is a subcrate that compiles a base data structure for us to use then the rest of the code just refrences it...