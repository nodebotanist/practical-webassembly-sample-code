import http.server
import socketserver

PORT = 8084

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map = {
    '.html': 'text/html',
    '.bg.wasm': 'application/javascript',
    '.wasm': 'application/wasm',
    '.js': 'application/javascript',
    '': 'application/octet-stream',
}

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print("serving at port", PORT)
    httpd.serve_forever()