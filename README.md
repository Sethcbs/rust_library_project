Encapsulation:
Encapsulation is all about bundling together data and being mindful of what you let other
files or users access from that data. In this program, similarly to the python one, we used 
structs and field modifiers, like pub, to practice encapsulation. Structs help us bundle 
together information, like keeping Book, Id, and Dvd information bundled within their own
structs. Fields in a struct are private by default, so using pub only when necessary lets
us control what information or funcitonality we are exposing to other files.

Composition over Inheritance:
Rust doesn't use inheritance, so instead is established "has-a" relationships between 
objects. An example of this is that Books and Dvds have an Id, but they aren't Ids.
This helps to seperate their identities and responsabilities.

Polymorphism:
This is the ability to change how we treat objects based on the form they are currently 
taking. Traits are the way this is represented in this project. A specific example is 
when we use Box<dyn Item>, because this means that we aren't specifying one type of 
Item, rather we are just expecting any object that implements Item and we will treat it
accordingly.
