<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ title }}</title>
    <link
      rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css"
    />
    <style>
      .api-header {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 1rem 0;
        margin-bottom: 2rem;
      }
      .method-badge {
        font-size: 0.9rem;
        padding: 0.5rem 1rem;
        margin-right: 1rem;
      }
      .method-GET {
        background-color: #61affe;
      }
      .method-POST {
        background-color: #49cc90;
      }
      .method-PUT {
        background-color: #fca130;
      }
      .method-PATCH {
        background-color: #50e3c2;
      }
      .method-DELETE {
        background-color: #f93e3e;
      }
      pre code {
        display: block;
        padding: 1rem;
        background-color: #f8f9fa;
        border-radius: 0.25rem;
        overflow-x: auto;
      }
      .string {
        color: #50a14f;
      }
      .number {
        color: #c18401;
      }
      .boolean {
        color: #4078f2;
      }
      .null {
        color: #a626a4;
      }
      .brace,
      .bracket {
        color: #383a42;
        font-weight: bold;
      }
      .user-info {
        background-color: rgba(255, 255, 255, 0.1);
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
      }
    </style>
  </head>
  <body>
    <nav class="api-header">
      <div class="container">
        <div class="d-flex justify-content-between align-items-center">
          <h1 class="h3 mb-0">{{ api_name }}</h1>
          {{ #if user }}
          <div class="user-info">
            <i class="bi bi-person-circle"></i> {{ user }}
          </div>
          {{ /if }}
        </div>
      </div>
    </nav>

    <div class="container">
      <div class="row">
        <div class="col-12">
          <div class="card shadow-sm mb-4">
            <div class="card-header bg-white">
              <div class="d-flex align-items-center">
                <span class="badge method-{{ method }}">{{ method }}</span>
                <code class="flex-grow-1">{{ endpoint }}</code>
              </div>
            </div>
            <div class="card-body">
              <ul class="nav nav-tabs mb-3" role="tablist">
                <li class="nav-item" role="presentation">
                  <button
                    class="nav-link active"
                    data-bs-toggle="tab"
                    data-bs-target="#response"
                    type="button"
                  >
                    Response
                  </button>
                </li>
                <li class="nav-item" role="presentation">
                  <button
                    class="nav-link"
                    data-bs-toggle="tab"
                    data-bs-target="#headers"
                    type="button"
                  >
                    Headers
                  </button>
                </li>
              </ul>

              <div class="tab-content">
                <div
                  class="tab-pane fade show active"
                  id="response"
                  role="tabpanel"
                >
                  <pre><code>{{ data }}</code></pre>
                </div>
                <div class="tab-pane fade" id="headers" role="tabpanel">
                  <pre><code>{{ headers }}</code></pre>
                </div>
              </div>
            </div>
          </div>

          <!-- Form Fields Section -->
          {{ form_fields }}

          <!-- API Endpoints Navigation -->
          {{ endpoints }}

          <!-- Filter Controls -->
          {{ filters }}

          <!-- Pagination -->
          {{ pagination }}
        </div>
      </div>
    </div>

    <footer class="mt-5 mb-3 text-center text-muted">
      <small>Powered by Reinhardt Framework</small>
    </footer>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
  </body>
</html>
