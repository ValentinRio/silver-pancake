/*
* Data structures
*/

const PI = 3.14 // Static immutable value

// Type is the enum equivalent
type TokenKind =
    FLOAT
    INT
    OTHER: char // variant of a type can be tagged

struct Token =
    kind: TokenKind
    val: string

struct Vector =
    x, y: float

struct Rect =
    pos, size: Vector

make_rect min, max: Vector -> Rect
    Rect {{min.x, min.y}, {max.x - min.x, max.y - min.y}}  // To instantiate a struct specifing fields is not required 

make_rect min, max: Vector -> Rect
    Rect {pos = {x = min.x, y = min.y}, size = {x = max.x - min.x, y = max.y - min.y}}  // This is the same as above 

make_vect {a, b} {c, d}



/*
* Function declarations, recursion and temporary variables
*/

multiply x, y: int -> int  // Commas in function declarations can be used to reduce boilerplate
    x * y

fact_rec n: int -> int
    if n == 0
        1
    else
        n * fact_rec n-1

add_one x: int -> int
    let
        y: int = 1  // let can be used to instantiate temporary variables that can be used in the followed expression
    in
    x + y



/*
* Modules imports
*/

// You can either import a public member of a module
import foo::greet

greet "Silver pancake"

// Or import the whole module and use one of it public member
import foo

foo::greet "Silver pancake"



/*
* Modules exports
*/ 

greet str: string -> string
    `Hello ${str}`

export =
    greet
