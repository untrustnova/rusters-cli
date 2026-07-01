<VirtualHost *:80>
    ServerName {{ PROJECT_ID }}.local
    DocumentRoot /path/to/{{ PROJECT_NAME }}/frontend/public

    # Proxy API calls to the Rusters multiplexed backend
    ProxyRequests Off
    ProxyPreserveHost On

    <Location /api>
        ProxyPass http://127.0.0.1:{{ BACKEND_PORT }}/api
        ProxyPassReverse http://127.0.0.1:{{ BACKEND_PORT }}/api
    </Location>

    # Serve Rusters health endpoint
    <Location /_rusters>
        ProxyPass http://127.0.0.1:{{ BACKEND_PORT }}/_rusters
        ProxyPassReverse http://127.0.0.1:{{ BACKEND_PORT }}/_rusters
    </Location>

    <Directory "/path/to/{{ PROJECT_NAME }}/frontend/public">
        Options Indexes FollowSymLinks
        AllowOverride All
        Require all granted
    </Directory>

    ErrorLog /var/log/httpd/{{ PROJECT_ID }}-error.log
    CustomLog /var/log/httpd/{{ PROJECT_ID }}-access.log combined
</VirtualHost>
