from pyframe import create_webframe, WindowHandle

html = """
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Console Input Demo</title>
  <style>
    /* Simple styling for input and button */
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
      // Send JSON string over the IPC channel
      window.ipc.postMessage(JSON.stringify(obj));
    });
  </script>

</body>
</html>
"""

# Create a handle for interacting with the window (e.g. to change its title)
window = WindowHandle()

async def process_incoming_frontend_messages(data):
    """
    Async callback invoked whenever the frontend posts a message.

    Parameters:
    - data (str): The raw JSON string sent from the HTML/JS side.

    What this function does:
    1. Prints the received JSON to the Python console.
    2. Calls `window.set_title(new_title="Lucy")` to update the window title.
       - If the title change succeeds (returns "Ok"), logs a success message.
       - Otherwise, no further action is taken.

    This demonstrates a round-trip:
      Frontend → Python handler → Backend operation (set title) → Python log.
    """
    # Log the incoming message
    print("Received from frontend:", data)

    # Attempt to set the window title to "Lucy"
    title_setter = await window.set_title(new_title="Lucy")
    if title_setter == "Ok":
        print("Successfully set window title to 'Lucy'")
    else:
        print(f"Failed to set window title: {title_setter}")

if __name__ == "__main__":
    """
    Launch the webframe with:
    - handler: the async function above to process messages from JS
    - html: the HTML template string defined earlier
    """
    create_webframe(
        handler=process_incoming_frontend_messages,
        html=html
    )
