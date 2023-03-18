import initRustWasm from '../pkg/ch07.js'
import {greet} from '../pkg/ch07.js'

(async () => {

    
    await initRustWasm()
    greet()
 
})()