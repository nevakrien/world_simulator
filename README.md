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

class road extends unordered<city,city>{
	uint length;
}
```