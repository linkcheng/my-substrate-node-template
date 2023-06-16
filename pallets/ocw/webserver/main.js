const http = require('http');

const server = http.createServer((req, res) => {
  if (req.url === '/data') {
    res.writeHead(200, {'Content-Type': 'application/json'});
    res.end(JSON.stringify({price: 10}));
  } else {
    res.writeHead(404);
    res.end();
  }
});

server.listen(8000);

console.log('Server running on port 8000');
