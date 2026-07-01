<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <!-- Rusters Template Engine: values below are injected at runtime -->
    <title>{{ PROJECT_NAME }}</title>
    <meta name="description" content="{{ PROJECT_NAME }} — powered by Rusters" />
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <link rel="stylesheet" href="/src/styles.css" />
  </head>
  <body>
    <div id="app">
      <!-- Rusters Runtime Context -->
      <!-- {{ RUST_TEMPLATE_SLOT }} -->
    </div>
    <!-- Vite will inject the correct entry point based on frontend framework -->
    <script type="module" src="/src/{{ ENTRY_FILE }}"></script>
  </body>
</html>
