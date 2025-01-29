- λ calculus is universal, any computable function can be expressed and evaluated in terms of λ. Thus making it equivalent to a Turing machine.
- It emphasis on the transformation of expressions.

### Terminologies

- An expression can be recursively defined as:
	- `expression := <name> | <function> | <application>`
	- `function := λ<name>.<expression>`
	- `application := <expression> <expression>`

- `λx.x`
	-  This is an example of a lambda function and is called `identity function`.
	-  It has a name x defined and returns the name. `x` has no meaning on its own.
		- The names get replaced by an expression which is applied to the function.
		-  `(λx.x) y -> [y/x]y -> y`
			- `[y/x]` is the transformation notation which indicates to replace all occurrences of x with y .
	-  The name of a function doesn't matter when checking for equality.

### Free and Bound Variables

- A `name` is said to be `bound` in a function if is defined in the function name (preceded by `λ`).
	- `λx.x` -> x is bound as it defined in the function name.
	- `λx.xy` -> x is bound but y is not as it is not defined.
- A `name` can be both bound and free in an expression.
	- `(λx.xy)(λy.y)` -> y is `free` in the left expression while it is bound in right expression.

### Substitutions
- `(λx.x)(λy.y) -> [(λy.y)/x]x ->  λy.y`
	- Even an entire function can be given as a parameter to a function.
- In case of complicated functions, it is advised to rename the parameters with same names if it occurs across functions definitions. 
	- `(λx.(λy.xy))y -> [y/x]λy.xy ->λy.yy`
		- This is could be made more easier to solve if the innermost y was changed to something else such as `z`.
		- `(λx.(λz.xz))y -> [y/x]λz.xz ->λz.yz`

``` 
		(λx.(λy.(x(λx.xy))))y 
			-> (λx.(λz.(w(λx.xz))))y 
	                ->[y/x]λz.(w(λx.xz))  
	                     -> λz.w(λx.xz)
```
- We continue doing substitutions until no further can be possible.
### Arithmetic
- Numerals can be represented in church encoding in the form of `succ(succ(zero))` where `succ` is a function which can be written as `λs.s+1`. In other words numerals are represented in the form of `1+..+1+0`
	- `0 = λsz.z` -> 0 
	- `1 = λsz.s(z)` -> 1+0 
	- `2 = λsz.s(s(z))` -> 1+1+0

#### Successor function
- The successor function can be defined as `λwyx.y(wyx)`
- It will increment by 1 when given a number.
```
Lets try to increment zero (λsz.z)
(λwyx.y(wyx))(λsz.z) -> [(λsz.z)/w]yx.y(wyx) -> λyx.y((λsz.z)yx) 
	-> λyx.y(z) ; which is of form λsz.s(z)
```