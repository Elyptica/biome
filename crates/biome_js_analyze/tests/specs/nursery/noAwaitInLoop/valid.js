async function foo() { await bar; }
async function foo() { for (var bar in await baz) { } }
async function foo() { for (var bar of await baz) { } }
async function foo() { for await (var bar of await baz) { } }
async function foo() { for (var bar = await baz in qux) {} }
// While loops
async function foo() { while (true) { async function foo() { await bar; } } } // Blocked by a function declaration
// For loops
async function foo() { for (var i = await bar; i < n; i++) {  } }
// Do while loops
async function foo() { do { } while (bar); }
// Blocked by a function expression
async function foo() { while (true) { var y = async function() { await bar; } } }
// Blocked by an arrow function
async function foo() { while (true) { var y = async () => await foo; } }
async function foo() { while (true) { var y = async () => { await foo; } } }
// Blocked by a class method
async function foo() { while (true) { class Foo { async foo() { await bar; } } } }
// Asynchronous iteration intentionally
async function foo() { for await (var x of xs) { await f(x) } }
