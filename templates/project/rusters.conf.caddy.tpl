{{ PROJECT_ID }}.local {
    # Serve frontend static assets
    root * /path/to/{{ PROJECT_NAME }}/frontend/public
    file_server

    # Proxy API calls to the Rusters multiplexed backend
    handle /api/* {
        reverse_proxy localhost:{{ BACKEND_PORT }}
    }

    # Rusters framework endpoints
    handle /_rusters/* {
        reverse_proxy localhost:{{ BACKEND_PORT }}
    }

    # SPA fallback
    handle {
        try_files {path} {path}/ /index.html
        file_server
    }

    log {
        output file /var/log/caddy/{{ PROJECT_ID }}.log
        format json
    }
}
