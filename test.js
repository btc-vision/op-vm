const app = require('.')


function add(a, b) {
    return {result: a + b}
}

console.log(app.hello(add))