RewriteEngine On

# Allow API calls to pass through to the multiplexed Rust backend
RewriteCond %{REQUEST_URI} ^/api [NC]
RewriteRule ^(.*)$ - [L]

# Allow Rusters framework endpoints
RewriteCond %{REQUEST_URI} ^/_rusters [NC]
RewriteRule ^(.*)$ - [L]

# Serve static files directly if they exist
RewriteCond %{DOCUMENT_ROOT}/frontend/public%{REQUEST_URI} -f [OR]
RewriteCond %{DOCUMENT_ROOT}/frontend/public%{REQUEST_URI} -d
RewriteRule ^(.*)$ frontend/public/$1 [L]

# Fallback to index.html for SPA routing and Rust Template evaluation
RewriteRule ^(.*)$ frontend/index.html [QSA,L]
