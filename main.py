from pyframe import create_webframe,WindowHandle

html = """
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Console Input Demo</title>
  <style>
    body {
      font-family: Arial, sans-serif;
      padding: 2rem;
      background-color: #f5f5f5;
    }
    .styled-input {
      padding: 0.5rem;
      font-size: 1rem;
      border: 2px solid #ccc;
      border-radius: 4px;
      width: 200px;
      margin-right: 1rem;
      transition: border-color 0.3s;
    }
    .styled-input:focus {
      border-color: #007BFF;
      outline: none;
    }
    .styled-button {
      padding: 0.6rem 1.2rem;
      font-size: 1rem;
      color: #fff;
      background-color: #007BFF;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      transition: background-color 0.3s;
    }
    .styled-button:hover {
      background-color: #0056b3;
    }
  </style>
</head>
<body>

  <h2>Enter Something:</h2>
  <input id="myInput" type="text" class="styled-input" placeholder="Type here…">
  <button id="myButton" class="styled-button">Submit</button>

  <script>
    // When the button is clicked, grab the input value and send it to Python
    document.getElementById('myButton').addEventListener('click', function() {
      const val = document.getElementById('myInput').value;
      const obj = { input: val };
      // <<< fixed: added the missing parenthesis here
      window.ipc.postMessage(JSON.stringify(obj));
    });
  </script>

</body>
</html>
"""

window = WindowHandle()
async def process_incoming_frontend_messages(data):
    # This will be called with the JSON string from your HTML
    print("Received from frontend:", data)
    title = await window.get_title()
    print("Window Title:", title)

if __name__ == "__main__":
    # Launch the frame — note no extra parentheses after create_webframe(...)
    create_webframe(
        handler=process_incoming_frontend_messages,
        html=html
    )
