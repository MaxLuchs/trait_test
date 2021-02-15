Exercises to:
https://www.twitch.tv/videos/903845418

# Just using ```Box<dyn trait>```
Example: ```let people: Vec<Box<dyn trait>> = vec![josef, maria, gabriel]```

Cons:
- Getting from ```Box<dyn trait>``` back to the concrete Type is easily possible anymore.

# Alternative solution for using ```Box<dyn trait>``` for Traits: https://bennetthardwick.com/blog/dont-use-boxed-trait-objects-for-struct-internals/
Define an Enum containing all subtypes and implementing the Trait

Cons:
- Enum not extensible anymore afterwards on the fly for new concrete types

Example: 
```
enum Personality { Father(Father), Mother(Mother), Angel(Angel) }
impl Personality for People {
    fn say_hello(&self) {
        match(*self) {
            Father(father) => father.say_hello(),
            Mother(mother) => mother.say_hello(),
            Angel(angel) => angel.say_hello()
        }
    }
}
```




