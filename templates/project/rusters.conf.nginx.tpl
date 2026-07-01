server {
    listen 80;
    server_name {{ PROJECT_ID }}.local;

    root /path/to/{{ PROJECT_NAME }}/frontend/public;
    index index.html;

    # Proxy API calls to the Rusters multiplexed backend
    location /api {
        proxy_pass http://127.0.0.1:{{ BACKEND_PORT }};
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_cache_bypass $http_upgrade;
    }

    # Rusters framework health / info endpoints
    location /_rusters {
        proxy_pass http://127.0.0.1:{{ BACKEND_PORT }};
        proxy_set_header Host $host;
    }

    # SPA fallback — serve index.html for all other routes
    location / {
        try_files $uri $uri/ /index.html;
    }

    error_log  /var/log/nginx/{{ PROJECT_ID }}-error.log;
    access_log /var/log/nginx/{{ PROJECT_ID }}-access.log;
}
