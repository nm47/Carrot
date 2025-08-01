#!/usr/bin/env python3
import urllib.request
import json

# Test the proxy functionality directly
url = "https://www.budgetbytes.com/wprm_print/orange-julius"

print(f"Testing direct fetch of: {url}")

try:
    req = urllib.request.Request(
        url,
        headers={'User-Agent': 'Mozilla/5.0 (compatible; Carrot Recipe Parser)'}
    )
    
    with urllib.request.urlopen(req, timeout=30) as response:
        content = response.read().decode('utf-8', errors='replace')
    
    print(f"✓ Successfully fetched {len(content)} bytes")
    print("✓ Content preview:")
    print(content[:200] + "...")
    
    # Test JSON serialization
    response_data = {
        'contents': content,
        'status': {'url': url, 'content_type': 'text/html'}
    }
    
    json_str = json.dumps(response_data)
    print(f"✓ JSON serialization works, size: {len(json_str)} bytes")
    
except Exception as e:
    print(f"✗ Error: {e}")