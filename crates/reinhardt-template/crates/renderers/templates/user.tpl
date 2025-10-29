<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User Profile</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
        }
        .profile {
            border: 1px solid #ddd;
            border-radius: 8px;
            padding: 20px;
            background-color: #f9f9f9;
        }
        h1 {
            color: #333;
            margin-top: 0;
        }
        .info {
            margin: 10px 0;
        }
        .label {
            font-weight: bold;
            color: #666;
        }
        .status {
            padding: 5px 10px;
            border-radius: 4px;
            display: inline-block;
            margin-top: 10px;
        }
        .status.adult {
            background-color: #d4edda;
            color: #155724;
        }
        .status.minor {
            background-color: #fff3cd;
            color: #856404;
        }
    </style>
</head>
<body>
    <div class="profile">
        <h1>{{ name }}</h1>

        <div class="info">
            <span class="label">Email:</span> {{ email }}
        </div>

        <div class="info">
            <span class="label">Age:</span> {{ age }}
        </div>

        {% if age >= 18 %}
            <div class="status adult">
                Adult User
            </div>
        {% else %}
            <div class="status minor">
                Minor User
            </div>
        {% endif %}
    </div>
</body>
</html>
