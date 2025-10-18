<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Browsable API</title>
    <style>
      body {
        font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        margin: 0;
        padding: 20px;
        background-color: #f5f5f5;
      }
      .container {
        max-width: 1200px;
        margin: 0 auto;
        background-color: white;
        padding: 30px;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      }
      h1 {
        color: #333;
        border-bottom: 2px solid #007bff;
        padding-bottom: 10px;
      }
      .response {
        background-color: #f8f9fa;
        border: 1px solid #dee2e6;
        border-radius: 4px;
        padding: 20px;
        margin-top: 20px;
      }
      pre {
        background-color: #282c34;
        color: #abb2bf;
        padding: 15px;
        border-radius: 4px;
        overflow-x: auto;
        font-family: "Courier New", monospace;
        line-height: 1.5;
      }
      .status {
        display: inline-block;
        padding: 5px 10px;
        border-radius: 4px;
        font-weight: bold;
        margin-bottom: 10px;
      }
      .status-success {
        background-color: #d4edda;
        color: #155724;
      }
      .status-error {
        background-color: #f8d7da;
        color: #721c24;
      }
    </style>
  </head>
  <body>
    <div class="container">
      <h1>Reinhardt Browsable API</h1>
      <div class="response">
        <div class="status {{status_class}}">{{status}}</div>
        <h2>Response</h2>
        <pre>{{content}}</pre>
      </div>
    </div>
  </body>
</html>
