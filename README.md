# rust-fem

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.0.0. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.

Add Typescript:

```bash
 bun init -y
 bun add ts-node typescript @types/node tsc --init
```

- **&** reference means read only
- **& mut** mutable reference means write and read
- **value** means the thing itself
- **()** is a unit is "nothing" value (not undefined)

- push (js) vs pull (rust)

### There are THREE rules you must have in your head at all times.

- There can only be **one** value owner
- There can be **unlimited** immutable borrows (reference) with no mutable references
- There can be only **one** mutable reference and **no** immutable references

### There is one rule for Lifetimes

- A reference **cannot** outlive its value

- Stated differently:

  - One var owns the the data

  - One var can change the data

  - Many vars can look at the data

  - You cannot look and change the data simultaneously

  - You cannot refer to something that has been dropped (released in memory)
