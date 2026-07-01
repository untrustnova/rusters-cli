version: '3.8'
services:
  rusters_core:
    build: .
    ports:
      - "{{ BACKEND_PORT }}:{{ BACKEND_PORT }}"
      - "{{ FRONTEND_PORT }}:{{ FRONTEND_PORT }}"
    env_file:
      - .env
    volumes:
      - ./database:/app/database
    restart: unless-stopped
