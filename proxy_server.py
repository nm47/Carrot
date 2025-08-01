#!/usr/bin/env python3
"""
Simple CORS proxy server for the Carrot recipe parser.
This allows the frontend to fetch recipe URLs without CORS issues.
"""

import http.server
import socketserver
import urllib.request
import urllib.parse
import json
import os
from urllib.error import URLError, HTTPError

class CORSProxyHandler(http.server.SimpleHTTPRequestHandler):        
    def do_GET(self):
        # Handle proxy requests to /proxy?url=...
        if self.path.startswith('/proxy?'):
            self.handle_proxy_request()
        else:
            # Serve static files normally
            super().do_GET()
    
    def handle_proxy_request(self):
        # Parse the URL parameter
        parsed_path = urllib.parse.urlparse(self.path)
        params = urllib.parse.parse_qs(parsed_path.query)
        
        if 'url' not in params:
            self.send_error(400, "Missing 'url' parameter")
            return
        
        target_url = params['url'][0]
        
        try:
            print(f"Fetching: {target_url}")
            
            # Create request with user agent to avoid blocking
            req = urllib.request.Request(
                target_url,
                headers={
                    'User-Agent': 'Mozilla/5.0 (compatible; Carrot Recipe Parser)'
                }
            )
            
            # Fetch the URL
            with urllib.request.urlopen(req, timeout=30) as response:
                content = response.read().decode('utf-8', errors='replace')
            
            # Send CORS headers and response
            self.send_response(200)
            self.send_header('Access-Control-Allow-Origin', '*')
            self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
            self.send_header('Access-Control-Allow-Headers', 'Content-Type')
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            
            # Return JSON response similar to allorigins
            response_data = {
                'contents': content,
                'status': {'url': target_url, 'content_type': 'text/html'}
            }
            
            self.wfile.write(json.dumps(response_data).encode('utf-8'))
            print(f"Successfully fetched {len(content)} bytes")
            
        except HTTPError as e:
            self.send_error(e.code, f"HTTP Error: {e.reason}")
        except URLError as e:
            self.send_error(500, f"URL Error: {e.reason}")
        except Exception as e:
            self.send_error(500, f"Server Error: {str(e)}")
    
    def do_OPTIONS(self):
        # Handle preflight CORS requests
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

def run_server(port=8001):
    # Change to the directory where this script is located
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    with socketserver.TCPServer(("", port), CORSProxyHandler) as httpd:
        print(f"Carrot development server running on http://localhost:{port}")
        print(f"Proxy endpoint: http://localhost:{port}/proxy?url=<recipe_url>")
        print("Press Ctrl+C to stop")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped")

if __name__ == "__main__":
    run_server()
