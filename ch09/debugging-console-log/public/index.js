import initRustWasm, {attackroll} from '../pkg/debugging_console_log.js'

(async () => {
    await initRustWasm()

    attackroll('Goblin', 10, 10, true)
})()